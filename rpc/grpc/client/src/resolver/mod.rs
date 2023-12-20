use super::error::Result;
use core::fmt::Debug;
use kash_grpc_core::{
    ops::KashdPayloadOps,
    protowire::{KashdRequest, KashdResponse},
};
use std::{sync::Arc, time::Duration};
use tokio::sync::oneshot;

pub(crate) mod id;
pub(crate) mod matcher;
pub(crate) mod queue;

pub(crate) trait Resolver: Send + Sync + Debug {
    fn register_request(&self, op: KashdPayloadOps, request: &KashdRequest) -> KashdResponseReceiver;
    fn handle_response(&self, response: KashdResponse);
    fn remove_expired_requests(&self, timeout: Duration);
}

pub(crate) type DynResolver = Arc<dyn Resolver>;

pub(crate) type KashdResponseSender = oneshot::Sender<Result<KashdResponse>>;
pub(crate) type KashdResponseReceiver = oneshot::Receiver<Result<KashdResponse>>;
