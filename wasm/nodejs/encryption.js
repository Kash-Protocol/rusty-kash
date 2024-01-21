const kash = require('./kash/kash_wasm');

kash.initConsolePanicHook();

(async () => {

    let encrypted = kash.encryptXChaCha20Poly1305("my message", "my_password");
    console.log("encrypted:", encrypted);
    let decrypted = kash.decryptXChaCha20Poly1305(encrypted, "my_password");
    console.log("decrypted:", decrypted);

})();
