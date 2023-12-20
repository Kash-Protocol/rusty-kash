globalThis.WebSocket = require('websocket').w3cwebsocket; // W3C WebSocket module shim

const kash = require('./kash/kash_wasm');
const { parseArgs } = require("./utils");
const {
    RpcClient
} = kash;

kash.initConsolePanicHook();

const {
    networkId,
    encoding,
} = parseArgs();

(async () => {

    const rpc = new RpcClient("127.0.0.1", encoding, networkId);
    console.log(`Connecting to ${rpc.url}`)
    await rpc.connect();

    const info = await rpc.getBlockDagInfo();
    console.log("GetBlockDagInfo response:", info);

    await rpc.disconnect();
})();
