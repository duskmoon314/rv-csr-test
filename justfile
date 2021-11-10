QEMU := "../qemu-build/riscv64-softmmu/qemu-system-riscv64"
SERIAL_FLAGS := "-serial /dev/pts/4 -serial /dev/null -serial /dev/null -serial tcp::23334,server,nowait -serial tcp:localhost:23334"
# SERIAL_FLAGS := "-serial /dev/pts/1 -serial /dev/null -serial /dev/null -serial /dev/null -serial /dev/null"
TARGET := "riscv64imac-unknown-none-elf"
MODE := "release"
OBJDUMP := "riscv64-unknown-elf-objdump"
OBJCOPY := "riscv64-unknown-elf-objcopy"
# add-symbol-file target/riscv64gc-unknown-none-elf/release/rv-csr-test
BUILD_PATH := "target/" + TARGET + "/" + MODE + "/"
KERNEL_ELF := BUILD_PATH + "rv-csr-test"
KERNEL_ASM := BUILD_PATH + "rv-csr-test.asm"
KERNEL_BIN := BUILD_PATH + "rv-csr-test.bin"
KERNEL_LRV_BIN := BUILD_PATH + "rcore-n.bin"

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
    rm src/linker.ld

disasm: build
    {{OBJDUMP}} -S {{KERNEL_ELF}} > {{KERNEL_ASM}}

disasm_lrv: build_lrv
    {{OBJDUMP}} -S {{KERNEL_ELF}} > {{KERNEL_ASM}}

run: build
    {{QEMU}} -machine virt -smp 4 {{SERIAL_FLAGS}} -nographic -bios ./rustsbi-qemu.bin -device loader,file={{KERNEL_BIN}},addr=0x80200000 -d int -D debug.log

debug: build disasm
    tmux new-session -d "{{QEMU}} -machine virt -smp 4 {{SERIAL_FLAGS}} -nographic -bios ./rustsbi-qemu.bin -device loader,file={{KERNEL_BIN}},addr=0x80200000 -s -S -d int -D debug.log" && tmux split-window -h "riscv64-unknown-elf-gdb -ex 'file {{KERNEL_ELF}}' -ex 'set arch riscv:rv64' -ex 'target remote localhost:1234'" && tmux -2 attach-session -d