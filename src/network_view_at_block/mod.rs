use near_primitives::types::{BlockId, BlockReference, Finality};
use std::str::FromStr;
use strum::{EnumDiscriminants, EnumIter, EnumMessage};

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
pub struct NetworkViewAtBlockArgs {
    ///What is the name of the network
    network_name: String,
    #[interactive_clap(subcommand)]
    next: ViewAtBlock,
}

impl NetworkViewAtBlockArgs {
    pub fn get_connection_config(&self) -> crate::common::ConnectionConfig {
        match self.network_name.as_str() {
            "testnet" => crate::common::ConnectionConfig::Testnet,
            "mainnet" => crate::common::ConnectionConfig::Mainnet,
            "betanet" => crate::common::ConnectionConfig::Betanet,
            _ => todo!(),
        }
    }

    pub fn get_block_ref(&self) -> BlockReference {
        match self.next.clone() {
            ViewAtBlock::Now => Finality::Final.into(),
            ViewAtBlock::AtBlockHash(at_block_hash) => BlockReference::BlockId(BlockId::Hash(
                near_primitives::hash::CryptoHash::from_str(at_block_hash.block_id_hash.as_str())
                    .unwrap(),
            )),
            ViewAtBlock::AtBlockHeight(at_block_height) => {
                BlockReference::BlockId(BlockId::Height(at_block_height.block_id_height))
            }
        }
    }
}

#[derive(Debug, EnumDiscriminants, Clone, interactive_clap::InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
///Сhoose block for view
pub enum ViewAtBlock {
    #[strum_discriminants(strum(message = "View account properties in the final block"))]
    ///View account properties in the final block
    Now,
    #[strum_discriminants(strum(message = "View account properties in the selected block"))]
    ///View account properties in the selected block
    AtBlockHeight(AtBlockHeight),
    #[strum_discriminants(strum(message = "Specify a block ID hash to view this account"))]
    ///Specify a block ID hash to view this account
    AtBlockHash(BlockIdHash),
}

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
pub struct AtBlockHeight {
    ///Type the block ID height for this account
    block_id_height: near_primitives::types::BlockHeight,
}

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
pub struct BlockIdHash {
    ///Type the block ID hash for this account
    block_id_hash: String,
}
