use crate::protowire::{kashd_request, KashdRequest, KashdResponse};

impl From<kashd_request::Payload> for KashdRequest {
    fn from(item: kashd_request::Payload) -> Self {
        KashdRequest { id: 0, payload: Some(item) }
    }
}

impl AsRef<KashdRequest> for KashdRequest {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsRef<KashdResponse> for KashdResponse {
    fn as_ref(&self) -> &Self {
        self
    }
}

pub mod kashd_request_convert {
    use crate::protowire::*;
    use kash_rpc_core::{RpcError, RpcResult};

    impl_into_kashd_request!(Shutdown);
    impl_into_kashd_request!(SubmitBlock);
    impl_into_kashd_request!(GetBlockTemplate);
    impl_into_kashd_request!(GetBlock);
    impl_into_kashd_request!(GetInfo);

    impl_into_kashd_request!(GetCurrentNetwork);
    impl_into_kashd_request!(GetPeerAddresses);
    impl_into_kashd_request!(GetSink);
    impl_into_kashd_request!(GetMempoolEntry);
    impl_into_kashd_request!(GetMempoolEntries);
    impl_into_kashd_request!(GetConnectedPeerInfo);
    impl_into_kashd_request!(AddPeer);
    impl_into_kashd_request!(SubmitTransaction);
    impl_into_kashd_request!(GetSubnetwork);
    impl_into_kashd_request!(GetVirtualChainFromBlock);
    impl_into_kashd_request!(GetBlocks);
    impl_into_kashd_request!(GetBlockCount);
    impl_into_kashd_request!(GetBlockDagInfo);
    impl_into_kashd_request!(ResolveFinalityConflict);
    impl_into_kashd_request!(GetHeaders);
    impl_into_kashd_request!(GetUtxosByAddresses);
    impl_into_kashd_request!(GetBalanceByAddress);
    impl_into_kashd_request!(GetBalancesByAddresses);
    impl_into_kashd_request!(GetSinkBlueScore);
    impl_into_kashd_request!(Ban);
    impl_into_kashd_request!(Unban);
    impl_into_kashd_request!(EstimateNetworkHashesPerSecond);
    impl_into_kashd_request!(GetMempoolEntriesByAddresses);
    impl_into_kashd_request!(GetCoinSupply);
    impl_into_kashd_request!(Ping);
    impl_into_kashd_request!(GetMetrics);
    impl_into_kashd_request!(GetServerInfo);
    impl_into_kashd_request!(GetSyncStatus);
    impl_into_kashd_request!(GetDaaScoreTimestampEstimate);

    impl_into_kashd_request!(NotifyBlockAdded);
    impl_into_kashd_request!(NotifyNewBlockTemplate);
    impl_into_kashd_request!(NotifyUtxosChanged);
    impl_into_kashd_request!(NotifyPruningPointUtxoSetOverride);
    impl_into_kashd_request!(NotifyFinalityConflict);
    impl_into_kashd_request!(NotifyVirtualDaaScoreChanged);
    impl_into_kashd_request!(NotifyVirtualChainChanged);
    impl_into_kashd_request!(NotifySinkBlueScoreChanged);

    macro_rules! impl_into_kashd_request {
        ($name:tt) => {
            paste::paste! {
                impl_into_kashd_request_ex!(kash_rpc_core::[<$name Request>],[<$name RequestMessage>],[<$name Request>]);
            }
        };
    }

    use impl_into_kashd_request;

    macro_rules! impl_into_kashd_request_ex {
        // ($($core_struct:ident)::+, $($protowire_struct:ident)::+, $($variant:ident)::+) => {
        ($core_struct:path, $protowire_struct:ident, $variant:ident) => {
            // ----------------------------------------------------------------------------
            // rpc_core to protowire
            // ----------------------------------------------------------------------------

            impl From<&$core_struct> for kashd_request::Payload {
                fn from(item: &$core_struct) -> Self {
                    Self::$variant(item.into())
                }
            }

            impl From<&$core_struct> for KashdRequest {
                fn from(item: &$core_struct) -> Self {
                    Self { id: 0, payload: Some(item.into()) }
                }
            }

            impl From<$core_struct> for kashd_request::Payload {
                fn from(item: $core_struct) -> Self {
                    Self::$variant((&item).into())
                }
            }

            impl From<$core_struct> for KashdRequest {
                fn from(item: $core_struct) -> Self {
                    Self { id: 0, payload: Some((&item).into()) }
                }
            }

            // ----------------------------------------------------------------------------
            // protowire to rpc_core
            // ----------------------------------------------------------------------------

            impl TryFrom<&kashd_request::Payload> for $core_struct {
                type Error = RpcError;
                fn try_from(item: &kashd_request::Payload) -> RpcResult<Self> {
                    if let kashd_request::Payload::$variant(request) = item {
                        request.try_into()
                    } else {
                        Err(RpcError::MissingRpcFieldError("Payload".to_string(), stringify!($variant).to_string()))
                    }
                }
            }

            impl TryFrom<&KashdRequest> for $core_struct {
                type Error = RpcError;
                fn try_from(item: &KashdRequest) -> RpcResult<Self> {
                    item.payload
                        .as_ref()
                        .ok_or(RpcError::MissingRpcFieldError("KashRequest".to_string(), "Payload".to_string()))?
                        .try_into()
                }
            }

            impl From<$protowire_struct> for KashdRequest {
                fn from(item: $protowire_struct) -> Self {
                    Self { id: 0, payload: Some(kashd_request::Payload::$variant(item)) }
                }
            }

            impl From<$protowire_struct> for kashd_request::Payload {
                fn from(item: $protowire_struct) -> Self {
                    kashd_request::Payload::$variant(item)
                }
            }
        };
    }
    use impl_into_kashd_request_ex;
}

pub mod kashd_response_convert {
    use crate::protowire::*;
    use kash_rpc_core::{RpcError, RpcResult};

    impl_into_kashd_response!(Shutdown);
    impl_into_kashd_response!(SubmitBlock);
    impl_into_kashd_response!(GetBlockTemplate);
    impl_into_kashd_response!(GetBlock);
    impl_into_kashd_response!(GetInfo);
    impl_into_kashd_response!(GetCurrentNetwork);

    impl_into_kashd_response!(GetPeerAddresses);
    impl_into_kashd_response!(GetSink);
    impl_into_kashd_response!(GetMempoolEntry);
    impl_into_kashd_response!(GetMempoolEntries);
    impl_into_kashd_response!(GetConnectedPeerInfo);
    impl_into_kashd_response!(AddPeer);
    impl_into_kashd_response!(SubmitTransaction);
    impl_into_kashd_response!(GetSubnetwork);
    impl_into_kashd_response!(GetVirtualChainFromBlock);
    impl_into_kashd_response!(GetBlocks);
    impl_into_kashd_response!(GetBlockCount);
    impl_into_kashd_response!(GetBlockDagInfo);
    impl_into_kashd_response!(ResolveFinalityConflict);
    impl_into_kashd_response!(GetHeaders);
    impl_into_kashd_response!(GetUtxosByAddresses);
    impl_into_kashd_response!(GetBalanceByAddress);
    impl_into_kashd_response!(GetBalancesByAddresses);
    impl_into_kashd_response!(GetSinkBlueScore);
    impl_into_kashd_response!(Ban);
    impl_into_kashd_response!(Unban);
    impl_into_kashd_response!(EstimateNetworkHashesPerSecond);
    impl_into_kashd_response!(GetMempoolEntriesByAddresses);
    impl_into_kashd_response!(GetCoinSupply);
    impl_into_kashd_response!(Ping);
    impl_into_kashd_response!(GetMetrics);
    impl_into_kashd_response!(GetServerInfo);
    impl_into_kashd_response!(GetSyncStatus);
    impl_into_kashd_response!(GetDaaScoreTimestampEstimate);

    impl_into_kashd_notify_response!(NotifyBlockAdded);
    impl_into_kashd_notify_response!(NotifyNewBlockTemplate);
    impl_into_kashd_notify_response!(NotifyUtxosChanged);
    impl_into_kashd_notify_response!(NotifyPruningPointUtxoSetOverride);
    impl_into_kashd_notify_response!(NotifyFinalityConflict);
    impl_into_kashd_notify_response!(NotifyVirtualDaaScoreChanged);
    impl_into_kashd_notify_response!(NotifyVirtualChainChanged);
    impl_into_kashd_notify_response!(NotifySinkBlueScoreChanged);

    impl_into_kashd_notify_response!(NotifyUtxosChanged, StopNotifyingUtxosChanged);
    impl_into_kashd_notify_response!(NotifyPruningPointUtxoSetOverride, StopNotifyingPruningPointUtxoSetOverride);

    macro_rules! impl_into_kashd_response {
        ($name:tt) => {
            paste::paste! {
                impl_into_kashd_response_ex!(kash_rpc_core::[<$name Response>],[<$name ResponseMessage>],[<$name Response>]);
            }
        };
        ($core_name:tt, $protowire_name:tt) => {
            paste::paste! {
                impl_into_kashd_response_base!(kash_rpc_core::[<$core_name Response>],[<$protowire_name ResponseMessage>],[<$protowire_name Response>]);
            }
        };
    }
    use impl_into_kashd_response;

    macro_rules! impl_into_kashd_response_base {
        ($core_struct:path, $protowire_struct:ident, $variant:ident) => {
            // ----------------------------------------------------------------------------
            // rpc_core to protowire
            // ----------------------------------------------------------------------------

            impl From<RpcResult<$core_struct>> for $protowire_struct {
                fn from(item: RpcResult<$core_struct>) -> Self {
                    item.as_ref().map_err(|x| (*x).clone()).into()
                }
            }

            impl From<RpcError> for $protowire_struct {
                fn from(item: RpcError) -> Self {
                    let x: RpcResult<&$core_struct> = Err(item);
                    x.into()
                }
            }

            impl From<$protowire_struct> for kashd_response::Payload {
                fn from(item: $protowire_struct) -> Self {
                    kashd_response::Payload::$variant(item)
                }
            }

            impl From<$protowire_struct> for KashdResponse {
                fn from(item: $protowire_struct) -> Self {
                    Self { id: 0, payload: Some(kashd_response::Payload::$variant(item)) }
                }
            }
        };
    }
    use impl_into_kashd_response_base;

    macro_rules! impl_into_kashd_response_ex {
        ($core_struct:path, $protowire_struct:ident, $variant:ident) => {
            // ----------------------------------------------------------------------------
            // rpc_core to protowire
            // ----------------------------------------------------------------------------

            impl From<RpcResult<&$core_struct>> for kashd_response::Payload {
                fn from(item: RpcResult<&$core_struct>) -> Self {
                    kashd_response::Payload::$variant(item.into())
                }
            }

            impl From<RpcResult<&$core_struct>> for KashdResponse {
                fn from(item: RpcResult<&$core_struct>) -> Self {
                    Self { id: 0, payload: Some(item.into()) }
                }
            }

            impl From<RpcResult<$core_struct>> for kashd_response::Payload {
                fn from(item: RpcResult<$core_struct>) -> Self {
                    kashd_response::Payload::$variant(item.into())
                }
            }

            impl From<RpcResult<$core_struct>> for KashdResponse {
                fn from(item: RpcResult<$core_struct>) -> Self {
                    Self { id: 0, payload: Some(item.into()) }
                }
            }

            impl_into_kashd_response_base!($core_struct, $protowire_struct, $variant);

            // ----------------------------------------------------------------------------
            // protowire to rpc_core
            // ----------------------------------------------------------------------------

            impl TryFrom<&kashd_response::Payload> for $core_struct {
                type Error = RpcError;
                fn try_from(item: &kashd_response::Payload) -> RpcResult<Self> {
                    if let kashd_response::Payload::$variant(response) = item {
                        response.try_into()
                    } else {
                        Err(RpcError::MissingRpcFieldError("Payload".to_string(), stringify!($variant).to_string()))
                    }
                }
            }

            impl TryFrom<&KashdResponse> for $core_struct {
                type Error = RpcError;
                fn try_from(item: &KashdResponse) -> RpcResult<Self> {
                    item.payload
                        .as_ref()
                        .ok_or(RpcError::MissingRpcFieldError("KashResponse".to_string(), "Payload".to_string()))?
                        .try_into()
                }
            }
        };
    }
    use impl_into_kashd_response_ex;

    macro_rules! impl_into_kashd_notify_response {
        ($name:tt) => {
            impl_into_kashd_response!($name);

            paste::paste! {
                impl_into_kashd_notify_response_ex!(kash_rpc_core::[<$name Response>],[<$name ResponseMessage>]);
            }
        };
        ($core_name:tt, $protowire_name:tt) => {
            impl_into_kashd_response!($core_name, $protowire_name);

            paste::paste! {
                impl_into_kashd_notify_response_ex!(kash_rpc_core::[<$core_name Response>],[<$protowire_name ResponseMessage>]);
            }
        };
    }
    use impl_into_kashd_notify_response;

    macro_rules! impl_into_kashd_notify_response_ex {
        ($($core_struct:ident)::+, $protowire_struct:ident) => {
            // ----------------------------------------------------------------------------
            // rpc_core to protowire
            // ----------------------------------------------------------------------------

            impl<T> From<Result<(), T>> for $protowire_struct
            where
                T: Into<RpcError>,
            {
                fn from(item: Result<(), T>) -> Self {
                    item
                        .map(|_| $($core_struct)::+{})
                        .map_err(|err| err.into()).into()
                }
            }

        };
    }
    use impl_into_kashd_notify_response_ex;
}
