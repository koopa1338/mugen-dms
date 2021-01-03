import init, { run_app } from '/ui/mugen_ui.js';
async function main() {
  await init('/ui/mugen_ui_bg.wasm');
  run_app();
}
main()
