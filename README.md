# RV-CSR-TEST

一个简单的用户态中断机制的测试程序。

## 依赖

- riscv crate
  - 我基于官方库做了一些修改，地址 [duskmoon314/riscv](https://github.com/duskmoon314/riscv) extN 分支
  - 使用 git submodule 放于 `./riscv`
- qemu
  - 我基于 stable-5.x 版本做了一些修改，地址 [duskmoon314/qemu](https://github.com/duskmoon314/qemu) riscv-N-stable-5.0 分支
  - 我的使用方式：
    - 在 qemu 文件夹的父文件夹中执行：

      ```sh
      mkdir qemu-build
      cd qemu-build
      ../qemu/configure --target-list="riscv64-softmmu"
      make -j8
      ```

      即 qemu-build 文件夹与 qemu 文件夹处于同一文件夹中。这样做是为了避免在 qemu 仓库中产生大量的编译输出文件。
    - 本仓库的 [justfile](./justfile) 中 qemu 相关路径也是基于此配置的。
