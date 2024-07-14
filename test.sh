#!/bin/bash

echo "test case 1: help"
echo ""
cargo run -q -- help
echo ""
echo "test case 2: git commit --help"
echo ""
cargo run -q -- git commit --help
echo ""
echo "test case 3: test bosh"
echo ""
cargo run -q -- test bosh
echo ""
