import init, { run_app } from '/assets/js/mugen_ui.js';
async function main() {
  await init('/assets/js/mugen_ui_bg.wasm');
  run_app();
}
main()
