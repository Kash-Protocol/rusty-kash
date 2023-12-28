use super::method::{DropFn, Method, MethodTrait, RoutingPolicy};
use crate::{
    connection::Connection,
    connection_handler::ServerContext,
    error::{GrpcServerError, GrpcServerResult},
};
use kash_grpc_core::{
    ops::KashdPayloadOps,
    protowire::{KashdRequest, KashdResponse},
};
use std::fmt::Debug;
use std::{collections::HashMap, sync::Arc};

pub type KashdMethod = Method<ServerContext, Connection, KashdRequest, KashdResponse>;
pub type DynKashdMethod = Arc<dyn MethodTrait<ServerContext, Connection, KashdRequest, KashdResponse>>;
pub type KashdDropFn = DropFn<KashdRequest, KashdResponse>;
pub type KashdRoutingPolicy = RoutingPolicy<KashdRequest, KashdResponse>;

/// An interface providing methods implementations and a fallback "not implemented" method
/// actually returning a message with a "not implemented" error.
///
/// The interface can provide a method clone for every [`KashdPayloadOps`] variant for later
/// processing of related requests.
///
/// It is also possible to directly let the interface itself process a request by invoking
/// the `call()` method.
pub struct Interface {
    server_ctx: ServerContext,
    methods: HashMap<KashdPayloadOps, DynKashdMethod>,
    method_not_implemented: DynKashdMethod,
}

impl Interface {
    pub fn new(server_ctx: ServerContext) -> Self {
        let method_not_implemented = Arc::new(Method::new(|_, _, kashd_request: KashdRequest| {
            Box::pin(async move {
                match kashd_request.payload {
                    Some(ref request) => Ok(KashdResponse {
                        id: kashd_request.id,
                        payload: Some(KashdPayloadOps::from(request).to_error_response(GrpcServerError::MethodNotImplemented.into())),
                    }),
                    None => Err(GrpcServerError::InvalidRequestPayload),
                }
            })
        }));
        Self { server_ctx, methods: Default::default(), method_not_implemented }
    }

    pub fn method(&mut self, op: KashdPayloadOps, method: KashdMethod) {
        let method: DynKashdMethod = Arc::new(method);
        if self.methods.insert(op, method).is_some() {
            panic!("RPC method {op:?} is declared multiple times")
        }
    }

    pub fn replace_method(&mut self, op: KashdPayloadOps, method: KashdMethod) {
        let method: DynKashdMethod = Arc::new(method);
        let _ = self.methods.insert(op, method);
    }

    pub fn set_method_properties(&mut self, op: KashdPayloadOps, tasks: usize, queue_size: usize, routing_policy: KashdRoutingPolicy) {
        self.methods.entry(op).and_modify(|x| {
            let method: Method<ServerContext, Connection, KashdRequest, KashdResponse> =
                Method::with_properties(x.method_fn(), tasks, queue_size, routing_policy);
            let method: Arc<dyn MethodTrait<ServerContext, Connection, KashdRequest, KashdResponse>> = Arc::new(method);
            *x = method;
        });
    }

    pub async fn call(&self, op: &KashdPayloadOps, connection: Connection, request: KashdRequest) -> GrpcServerResult<KashdResponse> {
        self.methods.get(op).unwrap_or(&self.method_not_implemented).call(self.server_ctx.clone(), connection, request).await
    }

    pub fn get_method(&self, op: &KashdPayloadOps) -> DynKashdMethod {
        self.methods.get(op).unwrap_or(&self.method_not_implemented).clone()
    }
}

impl Debug for Interface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Interface").finish()
    }
}
