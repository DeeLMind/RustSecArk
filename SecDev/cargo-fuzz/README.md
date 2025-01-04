# Cargo Fuzz

[cargo fuzz](https://github.com/rust-fuzz/cargo-fuzz) 是 Rust 中用于模糊测试（fuzz testing）的工具，它通过提供一个自动化的模糊测试框架来帮助你找到代码中的潜在漏洞。模糊测试的目标是通过向程序输入随机数据来触发异常行为，比如崩溃、内存泄漏或未定义的行为。

## 帮助文档

* 设置rust版本需要nightly

> https://releases.rs/

```sh
rustup install nightly-2023-03-31
```

* 在目标工程中初始化

```sh
cargo fuzz init
```

* 编写fuzz代码

```sh
fuzz_targets/*.rs
```

* 运行fuzz

```sh
cargo fuzz run fuzz_target
```