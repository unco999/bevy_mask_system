# BevyMaskSysmtem

# Bevy 系统架构探索库
这个库用于探索 Bevy 游戏引擎的系统架构方式，参考了一部分函数式编程，更多的是一种没有目的的常量掩码泛型实验。

cargo add bevy_mask_system
设计理念 https://www.bilibili.com/opus/973529984968687671
## 特性

- **细粒度操作**：我们可以将系统函数细分成更抽象的细粒度进行操作。
- **组合方式**：提供了一种方式将这些细粒度的系统函数组合起来。
- **自定义上下文**：可以绑定进自己的上下文进行 Bevy 查询标签的更改。

> **注意**：前提是你使用带常量的结构体。

## 使用示例

使用示例可以在 `example` 目录里面找到。

## 额外信息

- [使用夜间 Rust 模式](https://doc.rust-lang.org/nightly/)

## 示例
```rust
// examples/example.rs

#![recursion_limit = "256"]
#![feature(specialization)]
#![feature(trait_alias)]
#![feature(generic_const_exprs)]
#![feature(const_type_id)]
#![feature(associated_type_defaults)]
#![feature(unboxed_closures)]
#![feature(unsize)]
#![feature(const_trait_impl)]
use bevy_mask_system::MaskSys;
#[warn(incomplete_features)]

// use rust_mask_system::MaskSys;
use mask_system_lib::{*};
use bevy::{
    app::{App, Startup, Update}, ecs::{component::Components, query::{QueryData, QueryFilter}}, math::bool, prelude::{default, Commands, Component, Entity, IntoSystem, Query, QueryBuilder, With, Without}, reflect::DynamicTuple, ui::shader_flags::BORDER, utils::all_tuples, DefaultPlugins
};

#[derive(Component)]
pub struct element<const const_t:usize>;

type WeaponExtractElements = Tag_1_2;
type WeaponDestructionOperation = Tag_2_4;

#[derive(MaskSys)]
pub struct Weapon;

impl<Content:MaskSystemContent> MaskSystem<WeaponExtractElements,Content> for Weapon
where
    [(); {Content::custom_val}]:,
{
    const _marker:usize = 2;

    type Output = fn(
        Query<&element<{Content::custom_val}>>,
    );

    fn export() -> Self::Output {
        |query:Query<&element<{Content::custom_val}>>|{
            println!("WeaponExtractElements tag:{}",Content::custom_val); 
            println!("提取元素 这里的标签是:{}",Content::custom_val);
        }
    }
} 

impl<Content:MaskSystemContent> MaskSystem<WeaponDestructionOperation,Content> for Weapon
where
    [(); {Content::custom_val}]:,
{
    const _marker:usize = 4;

    type Output = fn(
        Query<&element<{Content::custom_val}>>,
    );

    fn export() -> Self::Output {
        |query:Query<&element<{Content::custom_val}>>|{
            println!("WeaponDestructionOperation tag:{}",Content::custom_val); 
            println!("销毁武器 这里的标签是:{}",Content::custom_val);
        }
    }
} 

fn main() {

    /*用单一标签 WeaponExtractElements 提取系统函数 并使用绑定自定义上下文。**/
    /**"Extract system functions using a single label WeaponExtractElements and bind custom contexts." */
    let fns1 = F::<op!(WeaponExtractElements),Weapon,Content::<1,2,3,5>>::sign();
    
    /**用组合标签 WeaponExtractElements + WeaponDestructionOperation 提取WeaponExtractElements + WeaponDestructionOperation 并使用绑定自定义上下文。**/
    /**"Extract WeaponExtractElements + WeaponDestructionOperation using a combination of labels WeaponExtractElements + WeaponDestructionOperation and bind custom contexts." */
    let fns2 = F::<op!(WeaponExtractElements + WeaponDestructionOperation),Weapon,Content::<1,2,3,5>>::mask();
    
    /**提取所有系统函数 并使用绑定自定义上下文。**/
    /** "Extract all system functions and bind custom contexts."**/
    let fns3 = F::<op!(WeaponExtractElements + WeaponDestructionOperation),Weapon,Content::<1,2,3,5>>::all();

    App::new()
    .add_systems(Startup,fns3)
    .run();
}

