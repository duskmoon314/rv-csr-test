QEMU := "../../uintr/qemu-build/riscv64-softmmu/qemu-system-riscv64"
BIOS_BIN := "./opensbi_fw_dynamic.bin"
SERIAL_FLAGS := "-serial /dev/pts/80 -serial /dev/null -serial /dev/null -serial tcp::23334,server,nowait -serial tcp:localhost:23334"
# SERIAL_FLAGS := "-serial /dev/pts/1 -serial /dev/null -serial /dev/null -serial /dev/null -serial /dev/null"
TARGET := "riscv64imac-unknown-none-elf"
MODE := "release"
OBJDUMP := "rust-objdump"
OBJCOPY := "rust-objcopy"
# add-symbol-file target/riscv64gc-unknown-none-elf/release/rv-csr-test
BUILD_PATH := "target/" + TARGET + "/" + MODE + "/"
KERNEL_ELF := BUILD_PATH + "rv-csr-test"
KERNEL_ASM := BUILD_PATH + "rv-csr-test.asm"
KERNEL_BIN := BUILD_PATH + "rv-csr-test.bin"
KERNEL_LRV_BIN := BUILD_PATH + "rcore-n.bin"
OUTPUT_BIN := "rv-csr-test.bin"

env:
	(rustup target list | grep "{{TARGET}} (installed)") || rustup target add {{TARGET}}
	cargo install cargo-binutils --vers ~0.2
	rustup component add rust-src
	rustup component add llvm-tools-preview

build:
    cp src/linker-qemu.ld src/linker.ld
    cargo build --features "board_qemu" --release
    {{OBJCOPY}} -O binary {{KERNEL_ELF}} {{KERNEL_BIN}}
    rm src/linker.ld

build_lrv:
    cp src/linker-lrv.ld src/linker.ld
    cargo build --features "board_lrv" --release
    {{OBJCOPY}} -O binary {{KERNEL_ELF}} {{KERNEL_BIN}}
    cp -f {{KERNEL_BIN}} {{KERNEL_LRV_BIN}}
    cp -f {{KERNEL_BIN}} {{OUTPUT_BIN}}
    rm src/linker.ld

clean:
    cargo clean

disasm: build
    {{OBJDUMP}} -d -h -S {{KERNEL_ELF}} > {{KERNEL_ASM}}

disasm_lrv: build_lrv
    {{OBJDUMP}} -d -h -S {{KERNEL_ELF}} > {{KERNEL_ASM}}

run: build
    {{QEMU}} -machine virt -smp 2 {{SERIAL_FLAGS}} -nographic -bios {{BIOS_BIN}} -kernel {{KERNEL_ELF}} -d int,guest_errors -D debug.log

debug: build disasm
    tmux new-session -d "{{QEMU}} -machine virt -smp 2 {{SERIAL_FLAGS}} -nographic -bios  {{BIOS_BIN}} -kernel {{KERNEL_ELF}} -s -S -d int -D debug.log" && tmux split-window -h "riscv64-unknown-elf-gdb -ex 'file {{KERNEL_ELF}}' -ex 'set arch riscv:rv64' -ex 'target remote localhost:1234'" && tmux -2 attach-session -d