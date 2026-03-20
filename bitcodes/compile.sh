#!/bin/bash

set -e

INPUT_DIR="./c"
OUTPUT_DIR_BC="./bc"
OUTPUT_DIR_LL="./ll"

mkdir -p "$OUTPUT_DIR_BC"
mkdir -p "$OUTPUT_DIR_LL"

for file in "$INPUT_DIR"/*; do
  if [[ -f "$file" ]]; then
    filename=$(basename "$file")
    output="${filename%.*}.bc"

    clang -emit-llvm -c "$file" -O3 -I./c/lib/ -o "$OUTPUT_DIR_BC/$output"
    output="${filename%.*}.ll"
    clang -S -emit-llvm "$file" -O3 -I./c/lib/ -o "$OUTPUT_DIR_LL/$output"
  fi
done
