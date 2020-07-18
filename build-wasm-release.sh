#!/usr/bin/env bash

wasm-pack build light-letter-web -t no-modules --release
wasm-pack build light-letter-theme-ivy-leaf -t no-modules --release
