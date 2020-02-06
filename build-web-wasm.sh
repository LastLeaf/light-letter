#!/usr/bin/env bash

cd light-letter-web
wasm-pack build
wasm2js -Os -o pkg/light_letter_web_bg.wasm.js pkg/light_letter_web_bg.wasm
rm pkg/light_letter_web_bg.wasm
npm run build
cd ..
cp light-letter-web/dist/light_letter_web.js light-letter/src/static/
