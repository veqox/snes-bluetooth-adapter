# snes-bluetooth-adapter

> [!NOTE]
> This project is theoretical for now and information might be incomplete or incorrect.

## Prerequisites

- [rustup](https://rustup.rs/)
- [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [esp32-c3](https://www.espressif.com/en/products/socs/esp32-c3)
- snes

### Setup the toolchain

```bash
./scripts/setup_toolchain.sh
```

## Build

```bash
cargo build [-r]
```

## Flash

```bash
cargo run [-r]
```

## Emulation

The run scripts do not build the project before running the emulation.

### Docker

#### Prerequisites

- [docker](https://docs.docker.com/get-docker/)

#### Setup

```bash
./scripts/setup_qemu_docker.sh
```

#### Run

```bash
./scripts/run_qemu_docker.sh
```

### Local

#### Prerequisites

- [espressif-qemu](https://github.com/espressif/esp-toolchain-docs/blob/main/qemu/esp32c3/README.md#prerequisites)

#### Setup

```bash
./scripts/setup_qemu.sh
```

#### Run

```bash
./scripts/run_qemu.sh
```

## References

- [esp32c3-qemu](https://github.com/espressif/esp-toolchain-docs/blob/main/qemu/esp32c3/README.md)
- [esp32c3-qemu-docker](https://github.com/svenstaro/qemu-espressif-docker/blob/main/Dockerfile)
