rustup toolchain install stable --component rust-src
rustup target add riscv32imc-unknown-none-elf # For ESP32-C3

cargo install cargo-generate espflash
