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
    [(); {Content::tag_1_c}]:,
{
    const _marker:usize = 2;

    type Output = fn(
        Query<&element<{Content::tag_1_c}>>,
    );

    
    fn export() -> Self::Output {
        |query:Query<&element<{Content::tag_1_c}>>|{
            println!("WeaponExtractElements tag:{}",Content::tag_1_c); 
            println!("提取元素 这里的标签是:{}",Content::tag_1_c);
        }
    }
} 

impl<Content:MaskSystemContent> MaskSystem<WeaponDestructionOperation,Content> for Weapon
    where
    [(); {Content::tag_2_c}]:,
{
    const _marker:usize = 4;

    type Output = fn(
        Query<&element<{Content::tag_2_c}>>,
    );

    
    fn export() -> Self::Output {
        |query:Query<&element<{Content::tag_2_c}>>|{
            println!("WeaponDestructionOperation tag:{}",Content::tag_2_c); 
            println!("销毁武器 这里的标签是:{}",Content::tag_2_c);
        }
    }
} 



fn main() {

    /*用单一标签 WeaponExtractElements 提取系统函数 并使用绑定自定义上下文。**/
    /**"Extract system functions using a single label WeaponExtractElements and bind custom contexts." */
    let fns1 = F::<op!(WeaponExtractElements),Weapon,Content::<1,2,3,5,5,5,4>>::sign();
    
    /**用组合标签 WeaponExtractElements + WeaponDestructionOperation 提取WeaponExtractElements + WeaponDestructionOperation 并使用绑定自定义上下文。**/
    /**"Extract WeaponExtractElements + WeaponDestructionOperation using a combination of labels WeaponExtractElements + WeaponDestructionOperation and bind custom contexts." */
    let fns2 = F::<op!(WeaponExtractElements + WeaponDestructionOperation),Weapon,Content::<1,2,3,5,5,4,3>>::mask();
    
    /**提取所有系统函数 并使用绑定自定义上下文。**/
    /** "Extract all system functions and bind custom contexts."**/
    let fns3 = F::<op!(WeaponExtractElements + WeaponDestructionOperation),Weapon,Content::<1,2,3,5,5,4,3>>::all();

    App::new()
    .add_systems(Startup,fns3)
    .run();
}
