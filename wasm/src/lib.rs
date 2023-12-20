/*!
# `rusty-kash WASM32 bindings`

[<img alt="github" src="https://img.shields.io/badge/github-kashnet/rusty--kash-8da0cb?style=for-the-badge&labelColor=555555&color=8da0cb&logo=github" height="20">](https://github.com/Kash-Protocol/rusty-kash/tree/master/wasm)
[<img alt="crates.io" src="https://img.shields.io/crates/v/kash-wasm.svg?maxAge=2592000&style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/kash-wasm)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-kash--wasm-56c2a5?maxAge=2592000&style=for-the-badge&logo=docs.rs" height="20">](https://docs.rs/kash-wasm)
<img alt="license" src="https://img.shields.io/crates/l/kash-wasm.svg?maxAge=2592000&color=6ac&style=for-the-badge&logoColor=fff" height="20">

<br>

Rusty-Kash WASM32 bindings offer direct integration of Rust code and Rusty-Kash
codebase within JavaScript environments such as Node.js and Web Browsers.

## Documentation

- [**integrating with Kash** guide](https://kash-mdbook.aspectron.com/)
- [**Rustdoc** documentation](https://docs.rs/kash-wasm/latest/kash-wasm)
- [**JSDoc** documentation](https://aspectron.com/docs/kash-wasm/)

Please note that while WASM directly binds JacaScript and Rust resources, their names on JavaScript side
are different from their name in Rust as they conform to the 'camelCase' convention in JavaScript and
to the 'snake_case' convention in Rust.

## Interfaces

The APIs are currently separated into the following groups (this will be expanded in the future):

- **Transaction API** — Bindings for primitives related to transactions.
This includes basic primitives related to consensus transactions, as well as
`MutableTransaction` and `VirtualTransaction` primitives usable for
transaction creation.

- **Wallet API** — API for async core wallet processing tasks.

- **RPC API** — [RPC interface bindings](rpc) for the Kash node using WebSocket connections.
Compatible with Rusty Kash as well as with the Golang node (kashd) via the `kash-wrpc-proxy`
WebSocket / gRPC proxy (located in `rpc/wrpc/proxy`).

## Using RPC

**NODEJS:** To use WASM RPC client in the Node.js environment, you need to introduce a W3C WebSocket object
before loading the WASM32 library. You can use any Node.js module that exposes a W3C-compatible
WebSocket implementation. Two of such modules are [WebSocket](https://www.npmjs.com/package/websocket)
(provides a custom implementation) and [isomorphic-ws](https://www.npmjs.com/package/isomorphic-ws)
(built on top of the ws WebSocket module).

You can use the following shims:

```js
// WebSocket
globalThis.WebSocket = require('websocket').w3cwebsocket;
// isomorphic-ws
globalThis.WebSocket = require('isomorphic-ws');
```

## Loading in a Web App

```html
<html>
    <head>
        <script type="module">
            import * as kash_wasm from './kash/kash-wasm.js';
            (async () => {
                const kash = await kash_wasm.default('./kash/kash-wasm_bg.wasm');
            })();
        </script>
    </head>
    <body></body>
</html>
```

## Loading in a Node.js App

```javascript
// W3C WebSocket module shim
globalThis.WebSocket = require('websocket').w3cwebsocket;

let {RpcClient,Encoding,init_console_panic_hook,defer} = require('./kash-rpc');
// init_console_panic_hook();

let rpc = new RpcClient(Encoding.Borsh,"ws://127.0.0.1:17110");

(async () => {
    await rpc.connect();

    let info = await rpc.getInfo();
    console.log(info);

    await rpc.disconnect();
})();
```

For more details, please follow the [**integrating with Kash**](https://kash-mdbook.aspectron.com/) guide.

*/

#![allow(unused_imports)]

pub mod utils;
pub use crate::utils::*;

pub use kash_addresses::{Address, Version as AddressVersion};
pub use kash_consensus_core::tx::{ScriptPublicKey, Transaction, TransactionInput, TransactionOutpoint, TransactionOutput};
pub use kash_pow::wasm::*;

pub mod rpc {
    //! Kash RPC interface
    //!

    pub mod messages {
        //! Kash RPC messages
        pub use kash_rpc_core::model::message::*;
    }
    pub use kash_rpc_core::api::rpc::RpcApi;
    pub use kash_wrpc_client::wasm::RpcClient;
}

pub use kash_consensus_wasm::*;

pub use kash_wallet_core::wasm::{tx::*, utils::*, utxo::*, wallet::*, xprivatekey::*, xpublickey::*};
