#!/usr/bin/env bash

cd light-letter-web
wasm-pack build
wasm2js -O4 -Os -o pkg/light_letter_web_bg.wasm.js pkg/light_letter_web_bg.wasm
sed -i.bak 's/"__wbindgen_malloc"/FUNCTION_TABLE, "__wbindgen_malloc"/' pkg/light_letter_web_bg.wasm.js
echo "export var __wbindgen_export_2 = { get(index) { return retasmFunc.FUNCTION_TABLE[index] } };" >> pkg/light_letter_web_bg.wasm.js
sed -i.bak 's/wasm.__wbindgen_start();//' pkg/light_letter_web.js
mv pkg/light_letter_web_bg.wasm pkg/light_letter_web_original.wasm
npm run build
cd ..
cp light-letter-web/dist/light_letter_web.js light-letter/src/static/

cd light-letter-theme-ivy-leaf
wasm-pack build
wasm2js -O4 -Os -o pkg/light_letter_theme_ivy_leaf_bg.wasm.js pkg/light_letter_theme_ivy_leaf_bg.wasm
sed -i.bak 's/"__wbindgen_malloc"/FUNCTION_TABLE, "__wbindgen_malloc"/' pkg/light_letter_theme_ivy_leaf_bg.wasm.js
echo "export var __wbindgen_export_2 = { get(index) { return retasmFunc.FUNCTION_TABLE[index] } };" >> pkg/light_letter_theme_ivy_leaf_bg.wasm.js
sed -i.bak 's/wasm.__wbindgen_start();//' pkg/light_letter_theme_ivy_leaf.js
mv pkg/light_letter_theme_ivy_leaf_bg.wasm pkg/light_letter_theme_ivy_leaf_original.wasm
npm run build
cd ..
