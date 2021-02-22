import init, { run_app } from "./pkg/playlist_randomizer.js";
async function main() {
    await init("/pkg/playlist_randomizer_bg.wasm");
    run_app();
}
main();