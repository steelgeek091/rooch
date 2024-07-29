// Copyright (c) RoochNetwork
// SPDX-License-Identifier: Apache-2.0

use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Instant;

use bitcoin::hashes::Hash;
use bitcoin::OutPoint;
use move_core_types::account_address::AccountAddress;
use xorf::{BinaryFuse8, Filter};
use xxhash_rust::xxh3::xxh3_64;

use moveos_store::MoveOSStore;
use moveos_types::move_std::option::MoveOption;
use moveos_types::move_std::string::MoveString;
use moveos_types::moveos_std::object::{ObjectID, ObjectMeta};
use moveos_types::moveos_std::simple_multimap::{Element, SimpleMultiMap};
use rooch_common::fs::file_cache::FileCacheManager;
use rooch_config::RoochOpt;
use rooch_db::RoochDB;
use rooch_types::bitcoin::ord::{derive_inscription_id, InscriptionID};
use rooch_types::into_address::IntoAddress;
use rooch_types::rooch_network::RoochChainID;

use crate::commands::statedb::commands::inscription::InscriptionSource;

pub mod export;
pub mod genesis;
pub mod genesis_utxo;
pub mod import;
mod inscription;
mod utxo;

pub const GLOBAL_STATE_TYPE_PREFIX: &str = "states";
pub const GLOBAL_STATE_TYPE_ROOT: &str = "states_root";
pub const GLOBAL_STATE_TYPE_OBJECT: &str = "states_object";
pub const GLOBAL_STATE_TYPE_FIELD: &str = "states_field";

const UTXO_SEAL_INSCRIPTION_PROTOCOL: &str =
    "0000000000000000000000000000000000000000000000000000000000000004::ord::Inscription";

fn init_job(
    base_data_dir: Option<PathBuf>,
    chain_id: Option<RoochChainID>,
) -> (ObjectMeta, MoveOSStore, Instant) {
    let start_time = Instant::now();

    let opt = RoochOpt::new_with_default(base_data_dir.clone(), chain_id.clone(), None).unwrap();
    let rooch_db = RoochDB::init(opt.store_config()).unwrap();
    let root = rooch_db
        .latest_root()
        .unwrap()
        .expect("statedb is empty, genesis must be initialed.");
    log::info!("original root object: {:?}", root);

    log::info!("job progress started");

    (root, rooch_db.moveos_store, start_time)
}

fn convert_option_string_to_move_type(opt: Option<String>) -> MoveOption<MoveString> {
    opt.map(MoveString::from).into()
}

#[derive(Clone, Default, PartialEq, Debug)]
pub(crate) struct OutpointInscriptions {
    outpoint: OutPoint,
    inscriptions: Vec<ObjectID>,
}

impl OutpointInscriptions {
    fn hash_key(&self) -> u64 {
        xxh3_outpoint(&self.outpoint)
    }
}

fn xxh3_outpoint(outpoint: &OutPoint) -> u64 {
    let mut bytes: Vec<u8> = Vec::with_capacity(32 + 4);
    bytes.extend(outpoint.txid.to_byte_array());
    bytes.extend(&outpoint.vout.to_le_bytes());
    xxh3_64(&bytes)
}

impl Display for OutpointInscriptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let inscriptions = self
            .inscriptions
            .iter()
            .map(|inscription| inscription.to_string())
            .collect::<Vec<String>>()
            .join(",");
        write!(f, "{}-{}", self.outpoint, inscriptions)
    }
}

impl FromStr for OutpointInscriptions {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.splitn(2, '-').collect();
        if parts.len() != 2 {
            return Err(anyhow::anyhow!("Invalid OutpointInscriptions format"));
        }
        let outpoint = OutPoint::from_str(parts[0])?;
        let inscriptions = parts[1]
            .split(',')
            .map(ObjectID::from_str)
            .collect::<Result<Vec<ObjectID>, _>>()?;
        Ok(OutpointInscriptions {
            outpoint,
            inscriptions,
        })
    }
}

fn derive_utxo_seal(inscriptions: Option<Vec<ObjectID>>) -> SimpleMultiMap<MoveString, ObjectID> {
    if let Some(obj_ids) = inscriptions {
        SimpleMultiMap {
            data: vec![Element {
                key: MoveString::from_str(UTXO_SEAL_INSCRIPTION_PROTOCOL).unwrap(),
                value: obj_ids,
            }],
        }
    } else {
        SimpleMultiMap::create()
    }
}

pub(crate) struct OutpointInscriptionsMap {
    items: Vec<OutpointInscriptions>,
    key_filter: Option<BinaryFuse8>,
}

fn unbound_outpoint() -> OutPoint {
    OutPoint {
        txid: Hash::all_zeros(),
        vout: 0,
    }
}

impl OutpointInscriptionsMap {
    fn index(src: PathBuf) -> (Self, usize, usize, usize) {
        let buf_size = 8 * 1024 * 1024; // inscription maybe large, using larger buffer than usual
        let mut reader = BufReader::with_capacity(buf_size, File::open(src.clone()).unwrap());
        let mut is_title_line = true;

        // collect all outpoint:inscription pairs except unbounded
        let mut has_outpoint_count: usize = 0;
        let mut unbound_count: usize = 0;
        let mut items = Vec::with_capacity(80 * 1024 * 1024);
        for line in reader.by_ref().lines() {
            let line = line.unwrap();
            if is_title_line {
                is_title_line = false;
                if line.starts_with("# export at") {
                    continue; // skip block height info
                }
            }
            let src: InscriptionSource = InscriptionSource::from_str(&line);
            let txid: AccountAddress = src.id.txid.into_address();
            let inscription_id = InscriptionID::new(txid, src.id.index);
            let obj_id = derive_inscription_id(&inscription_id);
            let satpoint_output = OutPoint::from_str(src.satpoint_outpoint.as_str()).unwrap();
            if satpoint_output == unbound_outpoint() {
                unbound_count += 1;
                continue; // skip unbounded outpoint
            }
            items.push(OutpointInscriptions {
                outpoint: satpoint_output,
                inscriptions: vec![obj_id.clone()],
            });
            has_outpoint_count += 1;
        }

        let map = Self::new_with_unsorted(items);
        let (mapped_outpoint_count, mapped_inscription_count) = map.stats();
        assert_eq!(
            has_outpoint_count, mapped_inscription_count,
            "Inscription count mismatch after mapping"
        );
        (
            map,
            mapped_outpoint_count,
            mapped_inscription_count,
            unbound_count,
        )
    }

    fn index_and_dump(src: PathBuf, dump_path: Option<PathBuf>) -> (Self, usize, usize, usize) {
        let (map, mapped_outpoint_count, mapped_inscription_count, unbound_count) =
            Self::index(src.clone());
        if let Some(dump_path) = dump_path {
            map.dump(dump_path);
        }
        (
            map,
            mapped_outpoint_count,
            mapped_inscription_count,
            unbound_count,
        )
    }

    fn new_with_unsorted(items: Vec<OutpointInscriptions>) -> Self {
        Self::new(items, false)
    }

    fn new(items: Vec<OutpointInscriptions>, sorted: bool) -> Self {
        let mut map = OutpointInscriptionsMap {
            items,
            key_filter: None,
        };
        if !sorted {
            map.sort_and_merge();
        }
        map.add_outpoint_filter();
        map
    }

    fn sort_and_merge(&mut self) -> usize {
        let items = &mut self.items;
        if items.is_empty() {
            return 0;
        }
        // sort by outpoint
        items.sort_by(|a, b| a.outpoint.cmp(&b.outpoint));
        // merge inscriptions with same outpoint in place
        let mut write_index = 0;
        for read_index in 1..items.len() {
            if items[write_index].outpoint == items[read_index].outpoint {
                let drained_inscriptions: Vec<ObjectID> =
                    items[read_index].inscriptions.drain(..).collect();
                items[write_index].inscriptions.extend(drained_inscriptions);
            } else {
                write_index += 1;
                if write_index != read_index {
                    items[write_index] = std::mem::take(&mut items[read_index]);
                }
            }
        }
        // truncate the vector to remove the unused elements
        let new_len = write_index + 1;
        items.truncate(new_len);
        items.shrink_to_fit();
        new_len
    }

    fn add_outpoint_filter(&mut self) {
        let keys: Vec<u64> = self.items.iter().map(|item| item.hash_key()).collect();
        let filter = BinaryFuse8::try_from(&keys).unwrap();
        self.key_filter = Some(filter);
    }

    // check if the outpoint is in the filter, false positive is allowed
    fn contains(&self, outpoint: &OutPoint) -> bool {
        match &self.key_filter {
            Some(filter) => filter.contains(&xxh3_outpoint(outpoint)),
            None => true,
        }
    }

    fn search(&self, outpoint: &OutPoint) -> Option<Vec<ObjectID>> {
        if !self.contains(outpoint) {
            return None;
        }

        let items = &self.items;
        items
            .binary_search_by_key(outpoint, |x| x.outpoint)
            .ok()
            .map(|index| items[index].inscriptions.clone())
    }

    #[allow(dead_code)]
    fn load(path: PathBuf) -> Self {
        let file = File::open(path.clone()).expect("Unable to open the file");
        let reader = BufReader::new(file);
        let mut items = Vec::new();

        for line in reader.lines() {
            let line = line.expect("Unable to read line");
            let item: OutpointInscriptions = line.parse().expect("Unable to parse line");
            items.push(item);
        }

        let file_cache_manager = FileCacheManager::new(path).unwrap();
        let _ = file_cache_manager.drop_cache_range(0, 1 << 40);

        OutpointInscriptionsMap::new(items, true)
    }

    fn dump(&self, path: PathBuf) {
        let file = File::create(path.clone()).expect("Unable to create the file");
        let mut writer = BufWriter::new(file.try_clone().unwrap());

        for item in &self.items {
            writeln!(writer, "{}", item).expect("Unable to write line");
        }

        writer.flush().expect("Unable to flush writer");
        file.sync_data().expect("Unable to sync file");
        let file_cache_manager = FileCacheManager::new(path).unwrap();
        let _ = file_cache_manager.drop_cache_range(0, 1 << 40);
    }

    fn stats(&self) -> (usize, usize) {
        let outpoint_count = self.items.len();
        let inscription_count = self.items.iter().map(|item| item.inscriptions.len()).sum();
        (outpoint_count, inscription_count)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use bitcoin::Txid;
    use rand::Rng;
    use tempfile::tempdir;

    use super::*;

    impl OutpointInscriptionsMap {
        fn is_sorted_and_merged(&self) -> bool {
            let items = &self.items;

            if items.is_empty() {
                return true;
            }
            items
                .windows(2)
                .all(|item| item[0].outpoint < item[1].outpoint) // using '<' here to ensure the order is strictly increasing
        }
    }

    fn random_outpoint() -> OutPoint {
        let mut rng = rand::thread_rng();
        let txid: Txid = Txid::from_slice(&rng.gen::<[u8; 32]>()).unwrap();
        let vout: u32 = rng.gen();

        OutPoint { txid, vout }
    }

    // all outpoints are unique
    fn random_outpoints(n: usize) -> Vec<OutPoint> {
        let mut outpoints = HashSet::new();
        while outpoints.len() < n {
            outpoints.insert(random_outpoint());
        }
        outpoints.into_iter().collect()
    }

    // all inscriptions are unique
    fn random_inscriptions(n: usize) -> Vec<ObjectID> {
        let mut inscriptions = HashSet::new();
        while inscriptions.len() < n {
            inscriptions.insert(ObjectID::random());
        }
        inscriptions.into_iter().collect()
    }

    #[test]
    fn outpoint_inscriptions_display() {
        let outpoint = random_outpoint();
        let inscriptions = random_inscriptions(2);
        let outpoint_inscriptions = OutpointInscriptions {
            outpoint,
            inscriptions: inscriptions.clone(),
        };

        let expected = format!("{}-{},{}", outpoint, inscriptions[0], inscriptions[1]);
        assert_eq!(outpoint_inscriptions.to_string(), expected);
    }

    #[test]
    fn outpoint_inscriptions_from_str() {
        let outpoint = random_outpoint();
        let exp = OutpointInscriptions {
            outpoint,
            inscriptions: random_inscriptions(3),
        };

        let act = OutpointInscriptions::from_str(&exp.to_string()).unwrap();
        assert_eq!(exp, act);
    }

    fn random_items_default(n: usize) -> Vec<OutpointInscriptions> {
        random_items(n, 2)
    }

    fn random_items(n: usize, inscriptions_per_outpoint: usize) -> Vec<OutpointInscriptions> {
        let mut items = Vec::new();
        let outpoints = random_outpoints(n);
        if inscriptions_per_outpoint == 0 {
            for outpoint in outpoints {
                items.push(OutpointInscriptions {
                    outpoint,
                    inscriptions: vec![],
                });
            }
            return items;
        }

        let inscriptions = random_inscriptions(n * inscriptions_per_outpoint);
        for (i, outpoint) in outpoints.iter().enumerate() {
            let start = i * inscriptions_per_outpoint;
            let end = start + inscriptions_per_outpoint;
            items.push(OutpointInscriptions {
                outpoint: *outpoint,
                inscriptions: inscriptions[start..end].to_vec(),
            });
        }
        items
    }

    #[test]
    fn outpoint_inscriptions_map_stats() {
        let items = random_items_default(10);
        let map = OutpointInscriptionsMap::new_with_unsorted(items);
        let (outpoint_count, inscription_count) = map.stats();
        assert_eq!(outpoint_count, 10);
        assert_eq!(inscription_count, 20);
    }

    #[test]
    fn outpoint_inscriptions_map_is_sorted_and_merged() {
        let mut items = random_items_default(10);
        items[0] = items[9].clone(); // make it unmerged
        let mut map = OutpointInscriptionsMap {
            items,
            key_filter: None,
        };
        assert!(!map.is_sorted_and_merged());

        map.sort_and_merge();
        assert!(map.is_sorted_and_merged());
        assert_eq!((9, 20), map.stats());
        // check items is sorted manually
        let items = &map.items;
        for i in 0..8 {
            assert!(items[i].outpoint < items[i + 1].outpoint);
        }
    }

    #[test]
    fn contains_false_positive() {
        let sample_size = 1024 * 1024;
        let items = random_items(sample_size, 0);
        // won't search later, so it's ok to not sort and merge
        let mut map = OutpointInscriptionsMap::new(items.clone(), true);
        map.add_outpoint_filter();
        // ensure no false negative
        for item in items.iter() {
            assert!(map.contains(&item.outpoint));
        }
        // false positive rate
        let false_positives: usize = (0..sample_size)
            .map(|_| random_outpoint())
            .filter(|n| map.contains(n))
            .count();
        let fp_rate: f64 = false_positives as f64 / sample_size as f64;
        assert!(fp_rate < 0.1, "False positive rate is {}", fp_rate); // < 10% is acceptable
    }

    #[test]
    fn outpoint_inscriptions_map_search_not_found() {
        let map = OutpointInscriptionsMap::new_with_unsorted(Vec::new());
        let utxo = random_outpoint();
        assert!(map.search(&utxo).is_none());
    }

    #[test]
    fn outpoint_inscriptions_map_search_found() {
        let items = random_items_default(10);

        let map = OutpointInscriptionsMap::new_with_unsorted(items.clone());
        for item in items.iter() {
            let inscriptions = map.search(&item.outpoint).unwrap();
            assert_eq!(inscriptions, item.inscriptions);
        }
    }

    #[test]
    fn outpoint_inscriptions_map_search_found_merged() {
        let items = random_items_default(10);
        let mut unmerged_items = Vec::new();
        for item in items.iter() {
            let outpoint_inscriptions = OutpointInscriptions {
                outpoint: item.outpoint,
                inscriptions: vec![item.inscriptions[0].clone()],
            };
            unmerged_items.push(outpoint_inscriptions);
            let outpoint_inscriptions = OutpointInscriptions {
                outpoint: item.outpoint,
                inscriptions: vec![item.inscriptions[1].clone()],
            };
            unmerged_items.push(outpoint_inscriptions);
        }

        let map = OutpointInscriptionsMap::new_with_unsorted(unmerged_items);
        for item in items.iter() {
            let inscriptions = map.search(&item.outpoint).unwrap();
            assert_eq!(inscriptions, item.inscriptions);
        }
    }

    #[test]
    fn outpoint_inscriptions_map_index_and_dump() {
        let items = random_items_default(10);
        let map = OutpointInscriptionsMap::new_with_unsorted(items.clone());
        let (mapped_outpoint_count, mapped_inscription_count) = map.stats();
        let tempdir = tempdir().unwrap();
        let dump_path = tempdir
            .path()
            .join("outpoint_inscriptions_map_index_and_dump");
        map.dump(dump_path.clone());
        let map_from_load = OutpointInscriptionsMap::load(dump_path.clone());
        assert!(map_from_load.is_sorted_and_merged());
        let (mapped_outpoint_count2, mapped_inscription_count2) = map_from_load.stats();
        assert_eq!(mapped_outpoint_count, mapped_outpoint_count2);
        assert_eq!(mapped_inscription_count, mapped_inscription_count2);
        assert_eq!(map.items, map_from_load.items);
    }
}
