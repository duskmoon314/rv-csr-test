QEMU := "../qemu-build/riscv64-softmmu/qemu-system-riscv64"
TARGET := "riscv64imac-unknown-none-elf"
MODE := "debug"
OBJDUMP := "riscv64-unknown-elf-objdump"
OBJCOPY := "riscv64-unknown-elf-objcopy"
# add-symbol-file target/riscv64gc-unknown-none-elf/release/rv-csr-test
BUILD_PATH := "target/" + TARGET + "/" + MODE + "/"
KERNEL_ELF := BUILD_PATH + "rv-csr-test"
KERNEL_ASM := BUILD_PATH + "rv-csr-test.asm"
KERNEL_BIN := BUILD_PATH + "rv-csr-test.bin"
OS_BIN := BUILD_PATH + "os.bin"

build:
    cargo build
    {{OBJCOPY}} -O binary {{KERNEL_ELF}} {{KERNEL_BIN}}
    cp -f {{KERNEL_BIN}} {{OS_BIN}}

disasm: build
    {{OBJDUMP}} -D -S {{KERNEL_ELF}} > {{KERNEL_ASM}}

run: build
    {{QEMU}} -machine virt -nographic -bios ./rustsbi-qemu.bin -device loader,file={{KERNEL_BIN}},addr=0x80200000 -d int -D debug.log

debug: build disasm
    tmux new-session -d "{{QEMU}} -machine virt -nographic -bios ./rustsbi-qemu.bin -device loader,file={{KERNEL_BIN}},addr=0x80200000 -s -S -d int -D debug.log" && tmux split-window -h "riscv64-unknown-elf-gdb -ex 'file {{KERNEL_ELF}}' -ex 'set arch riscv:rv64' -ex 'target remote localhost:1234'" && tmux -2 attach-session -d