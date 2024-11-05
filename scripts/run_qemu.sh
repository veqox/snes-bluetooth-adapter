#!/bin/bash

NAME=$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[].targets[] | select( .kind | map(. == "bin") | any ) | .name')
TARGET=riscv32imc-unknown-none-elf
ELF_PATH=target/$TARGET/release/$NAME
BIN_PATH=target/$TARGET/release/$NAME.bin
LOG_PATH=qemu-$NAME.log

espflash save-image --chip esp32c3 --merge ${ELF_PATH} ${BIN_PATH} || exit

./qemu/build/qemu-system-riscv32 -nographic -icount 3 -machine esp32c3 -drive file=${BIN_PATH},if=mtd,format=raw -serial file:$LOG_PATH || exit
