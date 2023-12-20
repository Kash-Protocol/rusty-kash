# Kash WASM SDK

An integration wrapper around [`kash-wasm`](https://www.npmjs.com/package/kash-wasm) module that uses [`ws`](https://www.npmjs.com/package/ws) together with the  [`isomorphic-ws`](https://www.npmjs.com/package/isomorphic-ws) w3c adaptor for WebSocket communication.

## Usage

Kash module exports include all WASM32 bindings.
```javascript
const kash = require('kash');
```