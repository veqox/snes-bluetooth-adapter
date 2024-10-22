#!/bin/bash

git clone https://github.com/espressif/qemu.git
pushd qemu
sed -z -i "s/\(.*dependency('libgcrypt'.*method: '\)config-tool\('.*\)/\1pkg-config\2/g" -- meson.build
./configure --target-list=riscv32-softmmu \
    --enable-gcrypt \
    --enable-slirp \
    --enable-debug --enable-sanitizers \
    --enable-sdl \
    --disable-strip --disable-user \
    --disable-capstone --disable-vnc \
    --disable-gtk
ninja -C build
