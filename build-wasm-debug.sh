#!/usr/bin/env bash

wasm-pack build light-letter-backstage -t no-modules --dev
cargo build -p light-letter-backstage && cp target/debug/liblight_letter_backstage.* light-letter-backstage/pkg

wasm-pack build light-letter-theme-ivy-leaf -t no-modules --dev
cargo build -p light-letter-theme-ivy-leaf && cp target/debug/liblight_letter_theme_ivy_leaf.* light-letter-theme-ivy-leaf/pkg
