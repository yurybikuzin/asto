#!/usr/bin/env bash
dir=$(dirname "$0")
cd "$dir/.."
# cargo run -p a100_back -- -w a100_back sax xml/2022-11-06/svd.xml xml/2022-11-06/smm.xml
sources=(
    xml/2022-10-09/svd.xml 
    xml/2022-10-09/smm.xml
    xml/2022-11-06/svd.xml 
    xml/2022-11-06/smm.xml
)
cargo run -p a100_back -- -w a100_back sax2 "${sources[@]}"


