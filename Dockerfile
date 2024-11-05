FROM docker.io/archlinux:base-devel AS git

RUN pacman -Sy --noconfirm git
RUN git clone https://github.com/espressif/qemu.git

FROM docker.io/archlinux:base-devel AS build

COPY --from=git /qemu /qemu

RUN pacman -Sy --noconfirm git python ninja libgcrypt sdl2 libslirp pixman

WORKDIR /qemu
RUN sed -z -i "s/\(.*dependency('libgcrypt'.*method: '\)config-tool\('.*\)/\1pkg-config\2/g" -- meson.build
RUN ./configure --target-list=riscv32-softmmu \
    --enable-gcrypt \
    --enable-slirp \
    --enable-debug --enable-sanitizers \
    --enable-sdl \
    --disable-strip --disable-user \
    --disable-capstone --disable-vnc \
    --disable-gtk
RUN ninja -C build
RUN ninja -C build install

WORKDIR /
RUN rm -rf qemu
RUN pacman -Scc --noconfirm

ENTRYPOINT ["/usr/local/bin/qemu-system-riscv32"]
