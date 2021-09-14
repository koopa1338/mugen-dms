import init, { run_app } from '/js/mugen_ui.js';
async function main() {
  await init('/js/mugen_ui_bg.wasm');
  run_app();
}
main()
