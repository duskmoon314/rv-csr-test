# RV-CSR-TEST

一个简单的用户态中断机制的测试，尚有问题

## 依赖

- riscv crate
  - 我基于官方库做了一些修改，地址 [duskmoon314/riscv](https://github.com/duskmoon314/riscv) extN 分支
  - 使用 git submodule 放于 `./riscv`
- qemu
  - 我基于 stable-5.x 版本做了一些修改，地址 [duskmoon314/qemu](https://github.com/duskmoon314/qemu) riscv-N-stable-5.0 分支
  - 我的使用方式：
    - 在 `qemu/build/riscv` 中执行 `../../configure --target-list="riscv64-softmmu" && make -j8`
    - 本仓库相关路径也是基于此配置的
