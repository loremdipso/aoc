#!/usr/bin/bash

# watchexec -w ./src/ 'clear && cargo run'
watchexec -w . 'clear && RUST_BACKTRACE=1 cargo run'
