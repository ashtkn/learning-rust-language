# `embedded_system`

## セットアップ

```sh
$ rustup target add thumbv7em-none-eabihf
$ cargo install cargo-binutils
$ rustup component add llvm-tools-preview
```

### 必要なソフトウェア

* OpenOCD
  * Macの場合，`brew install open-ocd`でインストールできる
* GNU Arm Embedded Toolchain Downloads
  * [ダウンロード](https://developer.arm.com/tools-and-software/open-source-software/developer-tools/gnu-toolchain/gnu-rm/downloads)
  * Macの場合，`/Applications/ARM/bin`に必要なバイナリが入っている
