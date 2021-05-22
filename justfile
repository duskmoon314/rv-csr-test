QEMU := "../qemu/build/riscv/riscv64-softmmu/qemu-system-riscv64"
TARGET := "riscv64gc-unknown-none-elf"
MODE := "release"
OBJDUMP := "rust-objdump --arch-name=riscv64"
OBJCOPY := "rust-objcopy --binary-architecture=riscv64"
# add-symbol-file target/riscv64gc-unknown-none-elf/release/rv-csr-test


build:
    cargo build --release
    {{OBJCOPY}} target/{{TARGET}}/{{MODE}}/rv-csr-test --strip-all -O binary target/{{TARGET}}/{{MODE}}/rv-csr-test.bin

disasm:
    {{OBJDUMP}} -D target/{{TARGET}}/{{MODE}}/rv-csr-test > test.asm

run: build
    {{QEMU}} -machine virt -nographic -bios ./rustsbi-qemu.bin -device loader,file=target/{{TARGET}}/{{MODE}}/rv-csr-test.bin,addr=0x80200000 -d int -D debug.log

debug: build
    tmux new-session -d "{{QEMU}} -machine virt -nographic -bios ./rustsbi-qemu.bin -device loader,file=target/{{TARGET}}/{{MODE}}/rv-csr-test.bin,addr=0x80200000 -s -S -d int -D debug.log" && tmux split-window -h "riscv64-unknown-elf-gdb -ex 'file $(KERNEL_ELF)' -ex 'set arch riscv:rv64' -ex 'target remote localhost:1234'" && tmux -2 attach-session -d