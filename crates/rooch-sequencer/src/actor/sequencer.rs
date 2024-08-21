// Copyright (c) RoochNetwork
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;
use std::time::SystemTime;

use crate::messages::{
    GetSequencerOrderMessage, GetTransactionByHashMessage, GetTransactionsByHashMessage,
    GetTxHashsMessage, SetStatusMessage, TransactionSequenceMessage,
};
use crate::metrics::SequencerMetrics;
use accumulator::{Accumulator, MerkleAccumulator};
use anyhow::Result;
use async_trait::async_trait;
use coerce::actor::{context::ActorContext, message::Handler, Actor};
use function_name::named;
use moveos_types::h256::{self, H256};
use prometheus::Registry;
use rooch_store::meta_store::MetaStore;
use rooch_store::transaction_store::TransactionStore;
use rooch_store::RoochStore;
use rooch_types::crypto::{RoochKeyPair, Signature};
use rooch_types::sequencer::SequencerInfo;
use rooch_types::service_status::ServiceStatus;
use rooch_types::transaction::{LedgerTransaction, LedgerTxData};
use tracing::info;

pub struct SequencerActor {
    last_sequencer_info: SequencerInfo,
    tx_accumulator: MerkleAccumulator,
    sequencer_key: RoochKeyPair,
    rooch_store: RoochStore,
    service_status: ServiceStatus,
    metrics: Arc<SequencerMetrics>,
}

impl SequencerActor {
    pub fn new(
        sequencer_key: RoochKeyPair,
        rooch_store: RoochStore,
        service_status: ServiceStatus,
        registry: &Registry,
    ) -> Result<Self> {
        // The sequencer info would be inited when genesis, so the sequencer info should not be None
        let last_sequencer_info = rooch_store
            .get_meta_store()
            .get_sequencer_info()?
            .ok_or_else(|| anyhow::anyhow!("Load sequencer info failed"))?;
        let (last_order, last_accumulator_info) = (
            last_sequencer_info.last_order,
            last_sequencer_info.last_accumulator_info.clone(),
        );
        info!("Load latest sequencer order {:?}", last_order);
        info!(
            "Load latest sequencer accumulator info {:?}",
            last_accumulator_info
        );
        let tx_accumulator = MerkleAccumulator::new_with_info(
            last_accumulator_info,
            rooch_store.get_transaction_accumulator_store(),
        );

        Ok(Self {
            last_sequencer_info,
            tx_accumulator,
            sequencer_key,
            rooch_store,
            service_status,
            metrics: Arc::new(SequencerMetrics::new(registry)),
        })
    }

    pub fn last_order(&self) -> u64 {
        self.last_sequencer_info.last_order
    }

    #[named]
    pub fn sequence(&mut self, mut tx_data: LedgerTxData) -> Result<LedgerTransaction> {
        let fn_name = function_name!();
        let _timer = self
            .metrics
            .sequencer_sequence_latency_seconds
            .with_label_values(&[fn_name])
            .start_timer();

        match self.service_status {
            ServiceStatus::ReadOnlyMode => {
                return Err(anyhow::anyhow!("The service is in read-only mode"));
            }
            ServiceStatus::DateImportMode => {
                if !tx_data.is_l1_block() && !tx_data.is_l1_tx() {
                    return Err(anyhow::anyhow!(
                        "The service is in date import mode, only allow l1 block and l1 tx"
                    ));
                }
            }
            ServiceStatus::Maintenance => {
                // Only the sequencer can send transactions in maintenance mode
                if let Some(sender) = tx_data.sender() {
                    if sender != self.sequencer_key.public().rooch_address()? {
                        return Err(anyhow::anyhow!("The service is in maintenance mode"));
                    }
                } else {
                    return Err(anyhow::anyhow!("The service is in maintenance mode"));
                }
            }
            _ => {}
        }

        let now = SystemTime::now();
        let tx_timestamp = now.duration_since(SystemTime::UNIX_EPOCH)?.as_millis() as u64;

        let tx_order = self.last_sequencer_info.last_order + 1;

        let hash = tx_data.tx_hash();
        let mut witness_data = hash.as_ref().to_vec();
        witness_data.extend(tx_order.to_le_bytes().iter());
        let witness_hash = h256::sha3_256_of(&witness_data);
        let tx_order_signature = Signature::sign(&witness_hash.0, &self.sequencer_key)
            .as_ref()
            .to_vec();

        // Calc transaction accumulator
        let _tx_accumulator_root = self.tx_accumulator.append(vec![hash].as_slice())?;
        self.tx_accumulator.flush()?;

        let tx_accumulator_info = self.tx_accumulator.get_info();
        let tx = LedgerTransaction::build_ledger_transaction(
            tx_data,
            tx_timestamp,
            tx_order,
            tx_order_signature,
            tx_accumulator_info.clone(),
        );

        let sequencer_info = SequencerInfo::new(tx.sequence_info.tx_order, tx_accumulator_info);
        self.rooch_store
            .save_sequencer_info(sequencer_info.clone())?;
        self.rooch_store.save_transaction(tx.clone())?;
        info!("sequencer tx: {} order: {:?}", hash, tx_order);
        self.last_sequencer_info = sequencer_info;

        Ok(tx)
    }
}

impl Actor for SequencerActor {}

#[async_trait]
impl Handler<TransactionSequenceMessage> for SequencerActor {
    async fn handle(
        &mut self,
        msg: TransactionSequenceMessage,
        _ctx: &mut ActorContext,
    ) -> Result<LedgerTransaction> {
        self.sequence(msg.tx)
    }
}

#[async_trait]
impl Handler<GetTransactionByHashMessage> for SequencerActor {
    async fn handle(
        &mut self,
        msg: GetTransactionByHashMessage,
        _ctx: &mut ActorContext,
    ) -> Result<Option<LedgerTransaction>> {
        self.rooch_store.get_transaction_by_hash(msg.hash)
    }
}

#[async_trait]
impl Handler<GetTransactionsByHashMessage> for SequencerActor {
    async fn handle(
        &mut self,
        msg: GetTransactionsByHashMessage,
        _ctx: &mut ActorContext,
    ) -> Result<Vec<Option<LedgerTransaction>>> {
        self.rooch_store.get_transactions_by_hash(msg.tx_hashes)
    }
}

#[async_trait]
impl Handler<GetTxHashsMessage> for SequencerActor {
    async fn handle(
        &mut self,
        msg: GetTxHashsMessage,
        _ctx: &mut ActorContext,
    ) -> Result<Vec<Option<H256>>> {
        let GetTxHashsMessage { tx_orders } = msg;
        self.rooch_store.get_tx_hashs(tx_orders)
    }
}

#[async_trait]
impl Handler<GetSequencerOrderMessage> for SequencerActor {
    async fn handle(
        &mut self,
        _msg: GetSequencerOrderMessage,
        _ctx: &mut ActorContext,
    ) -> Result<u64> {
        Ok(self.last_sequencer_info.last_order)
    }
}

#[async_trait]
impl Handler<SetStatusMessage> for SequencerActor {
    async fn handle(&mut self, msg: SetStatusMessage, _ctx: &mut ActorContext) -> Result<()> {
        self.service_status = msg.status;
        Ok(())
    }
}
