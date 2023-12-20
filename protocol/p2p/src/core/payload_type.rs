use crate::pb::kashd_message::Payload as KashdMessagePayload;

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub enum KashdMessagePayloadType {
    Addresses = 0,
    Block,
    Transaction,
    BlockLocator,
    RequestAddresses,
    RequestRelayBlocks,
    RequestTransactions,
    IbdBlock,
    InvRelayBlock,
    InvTransactions,
    Ping,
    Pong,
    Verack,
    Version,
    TransactionNotFound,
    Reject,
    PruningPointUtxoSetChunk,
    RequestIbdBlocks,
    UnexpectedPruningPoint,
    IbdBlockLocator,
    IbdBlockLocatorHighestHash,
    RequestNextPruningPointUtxoSetChunk,
    DonePruningPointUtxoSetChunks,
    IbdBlockLocatorHighestHashNotFound,
    BlockWithTrustedData,
    DoneBlocksWithTrustedData,
    RequestPruningPointAndItsAnticone,
    BlockHeaders,
    RequestNextHeaders,
    DoneHeaders,
    RequestPruningPointUtxoSet,
    RequestHeaders,
    RequestBlockLocator,
    PruningPoints,
    RequestPruningPointProof,
    PruningPointProof,
    Ready,
    BlockWithTrustedDataV4,
    TrustedData,
    RequestIbdChainBlockLocator,
    IbdChainBlockLocator,
    RequestAntipast,
    RequestNextPruningPointAndItsAnticoneBlocks,
}

impl From<&KashdMessagePayload> for KashdMessagePayloadType {
    fn from(payload: &KashdMessagePayload) -> Self {
        match payload {
            KashdMessagePayload::Addresses(_) => KashdMessagePayloadType::Addresses,
            KashdMessagePayload::Block(_) => KashdMessagePayloadType::Block,
            KashdMessagePayload::Transaction(_) => KashdMessagePayloadType::Transaction,
            KashdMessagePayload::BlockLocator(_) => KashdMessagePayloadType::BlockLocator,
            KashdMessagePayload::RequestAddresses(_) => KashdMessagePayloadType::RequestAddresses,
            KashdMessagePayload::RequestRelayBlocks(_) => KashdMessagePayloadType::RequestRelayBlocks,
            KashdMessagePayload::RequestTransactions(_) => KashdMessagePayloadType::RequestTransactions,
            KashdMessagePayload::IbdBlock(_) => KashdMessagePayloadType::IbdBlock,
            KashdMessagePayload::InvRelayBlock(_) => KashdMessagePayloadType::InvRelayBlock,
            KashdMessagePayload::InvTransactions(_) => KashdMessagePayloadType::InvTransactions,
            KashdMessagePayload::Ping(_) => KashdMessagePayloadType::Ping,
            KashdMessagePayload::Pong(_) => KashdMessagePayloadType::Pong,
            KashdMessagePayload::Verack(_) => KashdMessagePayloadType::Verack,
            KashdMessagePayload::Version(_) => KashdMessagePayloadType::Version,
            KashdMessagePayload::TransactionNotFound(_) => KashdMessagePayloadType::TransactionNotFound,
            KashdMessagePayload::Reject(_) => KashdMessagePayloadType::Reject,
            KashdMessagePayload::PruningPointUtxoSetChunk(_) => KashdMessagePayloadType::PruningPointUtxoSetChunk,
            KashdMessagePayload::RequestIbdBlocks(_) => KashdMessagePayloadType::RequestIbdBlocks,
            KashdMessagePayload::UnexpectedPruningPoint(_) => KashdMessagePayloadType::UnexpectedPruningPoint,
            KashdMessagePayload::IbdBlockLocator(_) => KashdMessagePayloadType::IbdBlockLocator,
            KashdMessagePayload::IbdBlockLocatorHighestHash(_) => KashdMessagePayloadType::IbdBlockLocatorHighestHash,
            KashdMessagePayload::RequestNextPruningPointUtxoSetChunk(_) => {
                KashdMessagePayloadType::RequestNextPruningPointUtxoSetChunk
            }
            KashdMessagePayload::DonePruningPointUtxoSetChunks(_) => KashdMessagePayloadType::DonePruningPointUtxoSetChunks,
            KashdMessagePayload::IbdBlockLocatorHighestHashNotFound(_) => {
                KashdMessagePayloadType::IbdBlockLocatorHighestHashNotFound
            }
            KashdMessagePayload::BlockWithTrustedData(_) => KashdMessagePayloadType::BlockWithTrustedData,
            KashdMessagePayload::DoneBlocksWithTrustedData(_) => KashdMessagePayloadType::DoneBlocksWithTrustedData,
            KashdMessagePayload::RequestPruningPointAndItsAnticone(_) => KashdMessagePayloadType::RequestPruningPointAndItsAnticone,
            KashdMessagePayload::BlockHeaders(_) => KashdMessagePayloadType::BlockHeaders,
            KashdMessagePayload::RequestNextHeaders(_) => KashdMessagePayloadType::RequestNextHeaders,
            KashdMessagePayload::DoneHeaders(_) => KashdMessagePayloadType::DoneHeaders,
            KashdMessagePayload::RequestPruningPointUtxoSet(_) => KashdMessagePayloadType::RequestPruningPointUtxoSet,
            KashdMessagePayload::RequestHeaders(_) => KashdMessagePayloadType::RequestHeaders,
            KashdMessagePayload::RequestBlockLocator(_) => KashdMessagePayloadType::RequestBlockLocator,
            KashdMessagePayload::PruningPoints(_) => KashdMessagePayloadType::PruningPoints,
            KashdMessagePayload::RequestPruningPointProof(_) => KashdMessagePayloadType::RequestPruningPointProof,
            KashdMessagePayload::PruningPointProof(_) => KashdMessagePayloadType::PruningPointProof,
            KashdMessagePayload::Ready(_) => KashdMessagePayloadType::Ready,
            KashdMessagePayload::BlockWithTrustedDataV4(_) => KashdMessagePayloadType::BlockWithTrustedDataV4,
            KashdMessagePayload::TrustedData(_) => KashdMessagePayloadType::TrustedData,
            KashdMessagePayload::RequestIbdChainBlockLocator(_) => KashdMessagePayloadType::RequestIbdChainBlockLocator,
            KashdMessagePayload::IbdChainBlockLocator(_) => KashdMessagePayloadType::IbdChainBlockLocator,
            KashdMessagePayload::RequestAntipast(_) => KashdMessagePayloadType::RequestAntipast,
            KashdMessagePayload::RequestNextPruningPointAndItsAnticoneBlocks(_) => {
                KashdMessagePayloadType::RequestNextPruningPointAndItsAnticoneBlocks
            }
        }
    }
}
