#!/usr/bin/env bash
cargo build
cp target/debug/librust_for_java.so ../java_demo/src/main/resources/ 
