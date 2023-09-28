// Copyright (c) RoochNetwork
// SPDX-License-Identifier: Apache-2.0

use crate::cli_types::CommandAction;
// use crate::commands::transaction::commands::get_tx_by_hash::GetByHashCommand;
use async_trait::async_trait;
use rooch_types::error::RoochResult;

pub mod commands;

/// Tool for interacting with transaction
#[derive(clap::Parser)]
pub struct Transaction {
    #[clap(subcommand)]
    cmd: TransactionCommand,
}

#[async_trait]
impl CommandAction<String> for Transaction {
    async fn execute(self) -> RoochResult<String> {
        match self.cmd {
            // TransactionCommand::GetByHash(cmd) => cmd.execute_serialized().await,
        }
    }
}

#[derive(clap::Subcommand)]
pub enum TransactionCommand {
    // GetByHash(GetByHashCommand),
}
