# BevyMaskSysmtem

# Bevy 系统架构探索库

这个库用于探索 Bevy 游戏引擎的系统架构方式，参考了一部分函数式编程，更多的是一种没有目的的常量掩码泛型实验。

## 特性

- **细粒度操作**：我们可以将系统函数细分成更抽象的细粒度进行操作。
- **组合方式**：提供了一种方式将这些细粒度的系统函数组合起来。
- **自定义上下文**：可以绑定进自己的上下文进行 Bevy 查询标签的更改。

> **注意**：前提是你使用带常量的结构体。

## 使用示例

使用示例可以在 `example` 目录里面找到。

## 额外信息

- [使用夜间 Rust 模式](https://doc.rust-lang.org/nightly/)
