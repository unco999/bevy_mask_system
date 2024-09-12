#![recursion_limit = "256"]
#![feature(specialization)]
#![feature(trait_alias)]
#![feature(generic_const_exprs)]
#![feature(const_type_id)]
#![feature(associated_type_defaults)]
#![feature(unboxed_closures)]
#![feature(unsize)]
#![feature(const_trait_impl)]
#[warn(incomplete_features)]
#[warn(non_camel_case_types)]
use std::{default, iter::Sum, marker::PhantomData, ops::{BitAnd, Sub}, process::Output, usize};
use std::ops::Add;
pub use typenum::{op, Add1, Bit, Integer, ToInt, UInt, UTerm, Unsigned, Zero, B0, B1, U0};

pub type U1 = UInt<UTerm, B1>; // 1
pub type U2 = UInt<U1, B0>;    // 2
pub type U3 = UInt<U1, B1>;    // 3
pub type U4 = UInt<U2, B0>;    // 4
pub type U5 = UInt<U2, B1>;    // 5
pub type U6 = UInt<U3, B0>;    // 6
pub type U7 = UInt<U3, B1>;    // 7
pub type U8 = UInt<U4, B0>;    // 8
pub type U9 = UInt<U4, B1>;    // 9
pub type U10 = UInt<U5, B0>;   // 10
pub type U11 = UInt<U5, B1>;   // 11
pub type U12 = UInt<U6, B0>;   // 12
pub type U13 = UInt<U6, B1>;   // 13
pub type U14 = UInt<U7, B0>;   // 14
pub type U15 = UInt<U7, B1>;   // 15
pub type U16 = UInt<U8, B0>;   // 16
pub type U17 = UInt<U8, B1>;   // 17
pub type U18 = UInt<U9, B0>;   // 18
pub type U19 = UInt<U9, B1>;   // 19
pub type U20 = UInt<U10, B0>;  // 20
pub type U21 = UInt<U10, B1>;  // 21
pub type U22 = UInt<U11, B0>;  // 22
pub type U23 = UInt<U11, B1>;  // 23
pub type U24 = UInt<U12, B0>;  // 24
pub type U25 = UInt<U12, B1>;  // 25
pub type U26 = UInt<U13, B0>;  // 26
pub type U27 = UInt<U13, B1>;  // 27
pub type U28 = UInt<U14, B0>;  // 28
pub type U29 = UInt<U14, B1>;  // 29
pub type U30 = UInt<U15, B0>;  // 30
pub type U31 = UInt<U15, B1>;  // 31
pub type U32 = UInt<U16, B0>;  // 32
pub type U33 = UInt<U16, B1>;  // 33
pub type U34 = UInt<U17, B0>;  // 34
pub type U35 = UInt<U17, B1>;  // 35
pub type U36 = UInt<U18, B0>;  // 36
pub type U37 = UInt<U18, B1>;  // 37
pub type U38 = UInt<U19, B0>;  // 38
pub type U39 = UInt<U19, B1>;  // 39
pub type U40 = UInt<U20, B0>;  // 40
pub type U41 = UInt<U20, B1>;  // 41
pub type U42 = UInt<U21, B0>;  // 42
pub type U43 = UInt<U21, B1>;  // 43
pub type U44 = UInt<U22, B0>;  // 44
pub type U45 = UInt<U22, B1>;  // 45
pub type U46 = UInt<U23, B0>;  // 46
pub type U47 = UInt<U23, B1>;  // 47
pub type U48 = UInt<U24, B0>;  // 48
pub type U49 = UInt<U24, B1>;  // 49
pub type U50 = UInt<U25, B0>;  // 50
pub type U51 = UInt<U25, B1>;  // 51
pub type U52 = UInt<U26, B0>;  // 52
pub type U53 = UInt<U26, B1>;  // 53
pub type U54 = UInt<U27, B0>;  // 54
pub type U55 = UInt<U27, B1>;  // 55
pub type U56 = UInt<U28, B0>;  // 56
pub type U57 = UInt<U28, B1>;  // 57
pub type U58 = UInt<U29, B0>;  // 58
pub type U59 = UInt<U29, B1>;  // 59
pub type U60 = UInt<U30, B0>;  // 60
pub type U61 = UInt<U30, B1>;  // 61
pub type U62 = UInt<U31, B0>;  // 62
pub type U63 = UInt<U31, B1>;  // 63
pub type U64 = UInt<U32, B0>;  // 64
pub type U65 = UInt<U32, B1>;  // 65
pub type U66 = UInt<U33, B0>;  // 66
pub type U67 = UInt<U33, B1>;  // 67
pub type U68 = UInt<U34, B0>;  // 68
pub type U69 = UInt<U34, B1>;  // 69
pub type U70 = UInt<U35, B0>;  // 70
pub type U71 = UInt<U35, B1>;  // 71
pub type U72 = UInt<U36, B0>;  // 72
pub type U73 = UInt<U36, B1>;  // 73
pub type U74 = UInt<U37, B0>;  // 74
pub type U75 = UInt<U37, B1>;  // 75
pub type U76 = UInt<U38, B0>;  // 76
pub type U77 = UInt<U38, B1>;  // 77
pub type U78 = UInt<U39, B0>;  // 78
pub type U79 = UInt<U39, B1>;  // 79
pub type U80 = UInt<U40, B0>;  // 80
pub type U81 = UInt<U40, B1>;  // 81
pub type U82 = UInt<U41, B0>;  // 82
pub type U83 = UInt<U41, B1>;  // 83
pub type U84 = UInt<U42, B0>;  // 84
pub type U85 = UInt<U42, B1>;  // 85
pub type U86 = UInt<U43, B0>;  // 86
pub type U87 = UInt<U43, B1>;  // 87
pub type U88 = UInt<U44, B0>;  // 88
pub type U89 = UInt<U44, B1>;  // 89
pub type U90 = UInt<U45, B0>;  // 90
pub type U91 = UInt<U45, B1>;  // 91
pub type U92 = UInt<U46, B0>;  // 92
pub type U93 = UInt<U46, B1>;  // 93
pub type U94 = UInt<U47, B0>;  // 94
pub type U95 = UInt<U47, B1>;  // 95
pub type U96 = UInt<U48, B0>;  // 96
pub type U97 = UInt<U48, B1>;  // 97
pub type U98 = UInt<U49, B0>;  // 98
pub type U99 = UInt<U49, B1>;  // 99
pub type U100 = UInt<U50, B0>; // 100
pub type U101 = UInt<U50, B1>; // 101
pub type U102 = UInt<U51, B0>; // 102
pub type U103 = UInt<U51, B1>; // 103
pub type U104 = UInt<U52, B0>; // 104
pub type U105 = UInt<U52, B1>; // 105
pub type U106 = UInt<U53, B0>; // 106
pub type U107 = UInt<U53, B1>; // 107
pub type U108 = UInt<U54, B0>; // 108
pub type U109 = UInt<U54, B1>; // 109
pub type U110 = UInt<U55, B0>; // 110
pub type U111 = UInt<U55, B1>; // 111
pub type U112 = UInt<U56, B0>; // 112
pub type U113 = UInt<U56, B1>; // 113
pub type U114 = UInt<U57, B0>; // 114
pub type U115 = UInt<U57, B1>; // 115
pub type U116 = UInt<U58, B0>; // 116
pub type U117 = UInt<U58, B1>; // 117
pub type U118 = UInt<U59, B0>; // 118
pub type U119 = UInt<U59, B1>; // 119
pub type U120 = UInt<U60, B0>; // 120
pub type U121 = UInt<U60, B1>; // 121
pub type U122 = UInt<U61, B0>; // 122
pub type U123 = UInt<U61, B1>; // 123
pub type U124 = UInt<U62, B0>; // 124
pub type U125 = UInt<U62, B1>; // 125
pub type U126 = UInt<U63, B0>; // 126
pub type U127 = UInt<U63, B1>; // 127
pub type U128 = UInt<U64, B0>; // 128



pub type Tag_1_2 = U2;
pub type Tag_2_4 = U4;
pub type Tag_3_8 = U8;
pub type Tag_4_16 = U16;
pub type Tag_5_32 = U32;
pub type Tag_6_64 = U64;


// 递归终止条件：如果 T 实现了 PickTuple，直接使用这个实现


// trait Dec<T:OP> {
//     type D;
//     type Export;
//     type OutPut;
//     fn run()-> Self::OutPut;
// }

// impl<T:OP> Dec<T> for ONE{
//     type D = ();
//     type Export = ();
//     type OutPut = (<T as OP>::Output);
    
//     fn run() -> (<T as OP>::Output) {
//         println!("1");
//         (T::export())
//     }
// }

// impl<T:OP> Dec<T> for TWO{
//     type D = op!(TWO - ONE);
//     type Export = ();
//     type OutPut = (<T as OP>::Output,<T as OP>::Output);

//     fn run() -> Self::OutPut {
//         println!("2");
//         let a = <Self::D as Dec<T> >::run();
//         (T::export(),a)
//     }
// }

// impl<T:OP> Dec<T> for THREE{
//     type D = op!(THREE - ONE);
//     type Export = ();
//     type OutPut = (<T as OP>::Output,(<T as OP>::Output,<T as OP>::Output));

//     fn run() -> Self::OutPut {
//         println!("3");
//         let a = <Self::D as Dec<T> >::run();
//         (T::export(),a)
//     }
// }

// struct  FP<T:Dec>(PhantomData<T>);
// type Result = <Two as TypeAdd<Three>>::Output;

//计算


pub trait IsMaskSystem {}


pub struct F<N:Unsigned,T:IsMaskSystem,Content:MaskSystemContent>(PhantomData<N>,PhantomData<T>,PhantomData<Content>);

impl<N:Unsigned+ CheckMaskSystem<O, Content>,O:IsMaskSystem + MaskSystem<N, Content>,Content:MaskSystemContent> F<N,O,Content> {
    pub fn all() ->  <<<N as CheckMaskSystem<O,Content>>::Recursion as PickTuple<O, N,Content>>::Output as Flat>::OutPut
    where 
        <N as CheckMaskSystem<O,Content>>::Recursion: PickTuple<O, N,Content>,
        <<N as CheckMaskSystem<O,Content>>::Recursion as PickTuple<O, N,Content>>::Output: Flat
    {
        let bit: <N as CheckMaskSystem<O,Content>>::Recursion = <N as CheckMaskSystem::<O,Content>>::find();
        let sys: <<N as CheckMaskSystem<O,Content>>::Recursion as PickTuple<O, N,Content>>::Output = PickTuple::<O, N,Content>::system(bit);
        let flat: <<<N as CheckMaskSystem<O,Content>>::Recursion as PickTuple<O, N,Content>>::Output as Flat>::OutPut = Flat::has_list(&sys);
        flat
    }

    pub fn sign()-><O as MaskSystem<N, Content>>::Output{
        <O as MaskSystem<N,Content>>::export()
    }

    pub fn mask()-><<<N as CheckMaskSystem<O, Content>>::CommbiRecursion as PickTuple<O, N, Content>>::Output as Flat>::OutPut
        where 
        <N as CheckMaskSystem<O, Content>>::CommbiRecursion: PickTuple<O, N, Content>,
        <<N as CheckMaskSystem<O, Content>>::CommbiRecursion as PickTuple<O, N, Content>>::Output: Flat
    {
        let bit = <N as CheckMaskSystem::<O,Content>>::combi();
        let sys = PickTuple::<O, N,Content>::system(bit);
        let flat = Flat::has_list(&sys);
        flat
    }
}

// impl 武器锻造{
//     fn combi<T:Unsigned + PickTuple<武器锻造>>() -> <T as PickTuple<武器锻造>>::Output{
//         todo!()
//     }
// }


// impl<const const_t:usize> 武器锻造<const_t:usize> 
//     where
//     T:Unsigned + std::ops::Sub<typenum::UInt<typenum::UTerm, typenum::B1>> + CombiMask
// {
//     fn combi()-><op!(T - U1) as PickTuple<{T::USIZE}>>::Output where <T as Sub<U1>>::Output: PickTuple<{T::USIZE}>{
//         <op!(T - U1) as PickTuple<{T::USIZE}>>::system()
//     }
// }

trait CombiMask {}

impl CombiMask for U3 {}
impl CombiMask for U5 {}
impl CombiMask for U6 {}
impl CombiMask for U7 {}
impl CombiMask for U9 {}
impl CombiMask for U10 {}
impl CombiMask for U11 {}
impl CombiMask for U12 {}
impl CombiMask for U13 {}
impl CombiMask for U14 {}
impl CombiMask for U15 {}
impl CombiMask for U17 {}
impl CombiMask for U18 {}
impl CombiMask for U19 {}
impl CombiMask for U20 {}
impl CombiMask for U21 {}
impl CombiMask for U22 {}
impl CombiMask for U23 {}
impl CombiMask for U24 {}
impl CombiMask for U25 {}
impl CombiMask for U26 {}
impl CombiMask for U27 {}
impl CombiMask for U28 {}
impl CombiMask for U29 {}
impl CombiMask for U30 {}
impl CombiMask for U31 {}
impl CombiMask for U33 {}
impl CombiMask for U34 {}
impl CombiMask for U35 {}
impl CombiMask for U36 {}
impl CombiMask for U37 {}
impl CombiMask for U38 {}
impl CombiMask for U39 {}
impl CombiMask for U40 {}
impl CombiMask for U41 {}
impl CombiMask for U42 {}
impl CombiMask for U43 {}
impl CombiMask for U44 {}
impl CombiMask for U45 {}
impl CombiMask for U46 {}
impl CombiMask for U47 {}
impl CombiMask for U48 {}
impl CombiMask for U49 {}
impl CombiMask for U50 {}
impl CombiMask for U51 {}
impl CombiMask for U52 {}
impl CombiMask for U53 {}
impl CombiMask for U54 {}
impl CombiMask for U55 {}
impl CombiMask for U56 {}
impl CombiMask for U57 {}
impl CombiMask for U58 {}
impl CombiMask for U59 {}
impl CombiMask for U60 {}
impl CombiMask for U61 {}
impl CombiMask for U62 {}
impl CombiMask for U63 {}



// impl<T> 武器锻造<T>
//     where
//     T:Unsigned + PickTuple<{T::USIZE}>
// {
//     fn build()-><T as PickTuple<{T::USIZE}>>::Output{
//         <T as PickTuple<{T::USIZE}>>::system()
//     }
// }



// impl<Content:MaskSystemContent> MaskSystem<U1,Content> for 武器锻造{
//     const _marker:usize = 1;

//     type Output = fn(
//         Query<&元素<1>>,
//     );

//     fn export() -> Self::Output {
//         |query:Query<&元素<1>>|{
//             println!("提取元素<T> 这里的元素标签是 1"); 
//             println!("锻造武器");
//         }
//     }
// } 

// impl<T:Unsigned,Content:MaskSystemContent> MaskSystem<T,Content> for 武器锻造{
//     default const _marker:usize = 0;
    
//     default type Output = <武器锻造 as MaskSystem<T,Content>>::Output;
    
//     default fn export() -> Self::Output {
//         <武器锻造 as MaskSystem<T,Content>>::export()
//     }
// }
pub trait CheckMaskSystem<S,Content:MaskSystemContent>{
    type Recursion;
    type CombiExport;

    type Export = bool;

    type CommbiRecursion;

    fn find()->Self::Recursion;

    fn combi()->Self::CommbiRecursion;
}


// trait GetStatic<TraitType: ?Sized> {
//     fn has_trait() -> bool ;
// }

// default impl<TraitType: ?Sized, T> GetStatic<TraitType> for T
// {
//     fn has_trait() -> bool { false }
// }

// impl<TraitType: ?Sized, T> GetStatic<TraitType> for T where T: std::marker::Unsize<TraitType>
// {
//     fn has_trait() -> bool { true }
// }

trait ConstUsize<const const_t:usize> {
    const _marker:usize = const_t;
}

struct Proxy;
impl<const const_t:usize> ConstUsize<const_t> for Proxy{
    const _marker:usize = const_t;
}

pub const fn validate_mask<S:MaskSystem<UInt<N,B>,Content>, N: Unsigned, B: Bit,Content:MaskSystemContent>() -> usize {
    let marker = <S as MaskSystem<UInt<N, B>, Content>>::_marker;
    let b_value = <UInt<N,B>>::USIZE & marker;

    if(b_value > 0){
        1
    }else {
        0
    }
}

pub const fn validate_max_1<S:MaskSystem<UInt<N,B>,Content>,N: Unsigned, B: Bit,Content:MaskSystemContent>() -> usize {
    let marker = <S as MaskSystem<UInt<N, B>, Content>>::_marker;

    if(marker > 1){
        1
    }else{
        0
    }
}


impl<N,B,S,Content:MaskSystemContent> CheckMaskSystem<S,Content> for UInt<N,B>
    where 
    N:Unsigned,
    UInt<N,B>:Unsigned + Sub<U1>,
    < UInt<N,B> as Sub<U1>>::Output:CheckMaskSystem<S,Content> + Unsigned,
    S:MaskSystem<UInt<N,B>,Content> ,
    B: typenum::Bit,
    [(); { validate_max_1::<S,N,B,Content>()}]:,
    [(); { validate_mask::<S,N,B,Content>() }]:,
    //S as MaskSystem<UInt<N,B>,Content>>::_marker + B::U8 as usize
{
    type Recursion = (Self::Export, << UInt<N,B> as Sub<U1>>::Output as CheckMaskSystem<S,Content>>::Recursion);

    type Export = TupleExist< {validate_max_1::<S,N,B,Content>()} >;
    
    type CombiExport = TupleExist< {  validate_mask::<S,N,B,Content>() } >;

    type CommbiRecursion = (Self::CombiExport, << UInt<N,B> as Sub<U1>>::Output as CheckMaskSystem<S,Content>>::CommbiRecursion);

    fn find()->Self::Recursion {
        ( TupleExist::<{ validate_max_1::<S,N,B,Content>() } > ,<< UInt<N,B> as Sub<U1>>::Output as CheckMaskSystem<S,Content>>::find())
    }

    fn combi()->Self::CommbiRecursion {
        ( TupleExist::<{ validate_mask::<S,N,B,Content>() } > ,<< UInt<N,B> as Sub<U1>>::Output as CheckMaskSystem<S,Content>>::combi())
    }


}

impl<S,Content:MaskSystemContent> CheckMaskSystem<S,Content> for UTerm
{
    type Recursion = (TupleTail,);

    type Export = bool;

    type CombiExport = Self::Recursion;
    
    type CommbiRecursion = (TupleTail,);
    
    fn find()->Self::Recursion {
        (TupleTail,)
    }

    fn combi()->Self::CommbiRecursion {
        (TupleTail,)
    }
}



pub trait MaskSystemContent{
    const market:usize;
    const main_mask:usize;
    const fp_type:usize;
    const custom_val:usize;
}

pub struct Content<const market:usize,const main_mask:usize,const fp_type:usize,const custom_val:usize>;

impl<const market:usize,const main_mask:usize,const fp_type:usize,const custom_val:usize> MaskSystemContent for Content<market,main_mask,fp_type,custom_val> {
    const market:usize = market;
    const main_mask:usize = main_mask;
    const fp_type:usize = fp_type;
    const custom_val:usize = custom_val;
}



pub trait MaskSystem<T:Unsigned,Content>
{
    const _marker:usize;

    type Output = ();
    
    fn export() -> Self::Output;
}

pub trait IsB1 { const VALUE: bool;}

impl IsB1 for B1 { const VALUE: bool = true;}

impl IsB1 for B0 { const VALUE: bool = false;}
pub trait IsB0 {}
impl IsB0 for B0 {}

pub trait PickTuple<T,Bit:Unsigned,Content:MaskSystemContent> {
    type Output;

    fn system(self) -> Self::Output;
}


#[macro_export]
macro_rules! impl_pick_tuples {
    ($start:ident, $end:ident) => {
        // 定义递归宏来生成每个 U 值的实现
        macro_rules! generate_impls {
            ($current:ident) => {
                if $current <= $end {
                    impl_pick_tuple!($current);
                    generate_impls!(next($current)); // 递归调用
                }
            };
        }

        // 启动生成过程
        generate_impls!($start);
    };
}

#[macro_export]
macro_rules! impl_pick_tuple {
    ($u:ident) => {

        impl<TAG, T,Content:MaskSystemContent> PickTuple<TAG, $u,Content> for (TupleExist::<0>, T)
        where
            T: PickTuple<TAG, <$u as Sub<U1>>::Output,Content>, // 递归约束
            <$u as Sub<typenum::UInt<UTerm, B1>>>::Output: typenum::Unsigned,
            $u: Sub<U1>,
        {
            type Output = <T as PickTuple<TAG,<$u as Sub<U1>>::Output,Content>>::Output;
            fn system(self) -> Self::Output {
                let (_, t) = self;
                <T as PickTuple<TAG, <$u as Sub<U1>>::Output,Content>>::system(t)
            }
        }

        impl<TAG, T,Content:MaskSystemContent> PickTuple<TAG, $u,Content> for (TupleExist::<1>, T)
        where
            T: PickTuple<TAG, <$u as Sub<U1>>::Output,Content>, // 递归约束
            <$u as Sub<typenum::UInt<UTerm, B1>>>::Output: typenum::Unsigned,
            $u: Sub<U1>,
            TAG: MaskSystem<$u,Content>,
        {
            type Output = (
                <TAG as MaskSystem<$u,Content>>::Output,
                <T as PickTuple<TAG, <$u as Sub<typenum::UInt<UTerm, B1>>>::Output,Content>>::Output,
            );
            fn system(self) -> Self::Output {
                let (_, t) = self;
                let data = <TAG as MaskSystem<$u,Content>>::export();
                (data, <T as PickTuple<TAG, <$u as Sub<U1>>::Output,Content>>::system(t))
            }
        }
    };
}
impl_pick_tuple!(U128);
impl_pick_tuple!(U127);
impl_pick_tuple!(U126);
impl_pick_tuple!(U125);
impl_pick_tuple!(U124);
impl_pick_tuple!(U123);
impl_pick_tuple!(U122);
impl_pick_tuple!(U121);
impl_pick_tuple!(U120);
impl_pick_tuple!(U119);
impl_pick_tuple!(U118);
impl_pick_tuple!(U117);
impl_pick_tuple!(U116);
impl_pick_tuple!(U115);
impl_pick_tuple!(U114);
impl_pick_tuple!(U113);
impl_pick_tuple!(U112);
impl_pick_tuple!(U111);
impl_pick_tuple!(U110);
impl_pick_tuple!(U109);
impl_pick_tuple!(U108);
impl_pick_tuple!(U107);
impl_pick_tuple!(U106);
impl_pick_tuple!(U105);
impl_pick_tuple!(U104);
impl_pick_tuple!(U103);
impl_pick_tuple!(U102);
impl_pick_tuple!(U101);
impl_pick_tuple!(U100);
impl_pick_tuple!(U99);
impl_pick_tuple!(U98);
impl_pick_tuple!(U97);
impl_pick_tuple!(U96);
impl_pick_tuple!(U95);
impl_pick_tuple!(U94);
impl_pick_tuple!(U93);
impl_pick_tuple!(U92);
impl_pick_tuple!(U91);
impl_pick_tuple!(U90);
impl_pick_tuple!(U89);
impl_pick_tuple!(U88);
impl_pick_tuple!(U87);
impl_pick_tuple!(U86);
impl_pick_tuple!(U85);
impl_pick_tuple!(U84);
impl_pick_tuple!(U83);
impl_pick_tuple!(U82);
impl_pick_tuple!(U81);
impl_pick_tuple!(U80);
impl_pick_tuple!(U79);
impl_pick_tuple!(U78);
impl_pick_tuple!(U77);
impl_pick_tuple!(U76);
impl_pick_tuple!(U75);
impl_pick_tuple!(U74);
impl_pick_tuple!(U73);
impl_pick_tuple!(U72);
impl_pick_tuple!(U71);
impl_pick_tuple!(U70);
impl_pick_tuple!(U69);
impl_pick_tuple!(U68);
impl_pick_tuple!(U67);
impl_pick_tuple!(U66);
impl_pick_tuple!(U65);
impl_pick_tuple!(U64);
impl_pick_tuple!(U63);
impl_pick_tuple!(U62);
impl_pick_tuple!(U61);
impl_pick_tuple!(U60);
impl_pick_tuple!(U59);
impl_pick_tuple!(U58);
impl_pick_tuple!(U57);
impl_pick_tuple!(U56);
impl_pick_tuple!(U55);
impl_pick_tuple!(U54);
impl_pick_tuple!(U53);
impl_pick_tuple!(U52);
impl_pick_tuple!(U51);
impl_pick_tuple!(U50);
impl_pick_tuple!(U49);
impl_pick_tuple!(U48);
impl_pick_tuple!(U47);
impl_pick_tuple!(U46);
impl_pick_tuple!(U45);
impl_pick_tuple!(U44);
impl_pick_tuple!(U43);
impl_pick_tuple!(U42);
impl_pick_tuple!(U41);
impl_pick_tuple!(U40);
impl_pick_tuple!(U39);
impl_pick_tuple!(U38);
impl_pick_tuple!(U37);
impl_pick_tuple!(U36);
impl_pick_tuple!(U35);
impl_pick_tuple!(U34);
impl_pick_tuple!(U33);
impl_pick_tuple!(U32);
impl_pick_tuple!(U31);
impl_pick_tuple!(U30);
impl_pick_tuple!(U29);
impl_pick_tuple!(U28);
impl_pick_tuple!(U27);
impl_pick_tuple!(U26);
impl_pick_tuple!(U25);
impl_pick_tuple!(U24);
impl_pick_tuple!(U23);
impl_pick_tuple!(U22);
impl_pick_tuple!(U21);
impl_pick_tuple!(U20);
impl_pick_tuple!(U19);
impl_pick_tuple!(U18);
impl_pick_tuple!(U17);
impl_pick_tuple!(U16);
impl_pick_tuple!(U15);
impl_pick_tuple!(U14);
impl_pick_tuple!(U13);
impl_pick_tuple!(U12);
impl_pick_tuple!(U11);
impl_pick_tuple!(U10);
impl_pick_tuple!(U9);
impl_pick_tuple!(U8);
impl_pick_tuple!(U7);
impl_pick_tuple!(U6);
impl_pick_tuple!(U5);
impl_pick_tuple!(U4);
impl_pick_tuple!(U3);
impl_pick_tuple!(U2);
impl_pick_tuple!(U1);


// 递归情况：处理两个元素的元组
impl<TAG,Bit:Unsigned,Content:MaskSystemContent> PickTuple<TAG, Bit,Content> for (TupleTail,)
{
    type Output = TupleTail;

    fn system(self) -> Self::Output {
        TupleTail
    }
}

#[derive(Debug)]
pub struct TupleEmpty;
#[derive(Debug)]
pub struct TupleTail;

#[derive(Debug)]
pub struct TupleExist<const const_t:usize>;

trait TupleElement {

}

impl TupleElement for TupleEmpty {}

impl TupleElement for TupleTail {}

impl<const const_t:usize> TupleElement for TupleExist<const_t> {}
pub trait TupleLen {
    type Length: Unsigned;
}

// 空元组的长度为 0（即 U0）
impl TupleLen for () {
    type Length = typenum::U0;
}

// 二元元组的长度为 1 加上 Tail 的长度
impl<Head, Tail> TupleLen for (Head, Tail)
where
    Tail: TupleLen,
    Tail::Length: Add<U1>,
    <Tail::Length as Add<U1>>::Output: Unsigned,
{
    type Length = <Tail::Length as Add<U1>>::Output;
}

pub trait Flat {
    type OutPut;

    fn has_list(&self)->Self::OutPut;

}

impl<T1: Clone> Flat for (T1,TupleTail) {
    type OutPut = (T1);

    fn has_list(&self) -> Self::OutPut {
        (self.0.clone())
    }
    
}

impl<T1: Clone, T2: Clone> Flat for (T1, (T2, TupleTail)) {
    type OutPut = (T1, T2);

    fn has_list(&self) -> Self::OutPut {
        (self.0.clone(), self.1.0.clone())
    }
    
}

impl<T1: Clone, T2: Clone,T3:Clone> Flat for (T1, (T2, (T3, TupleTail))) {
    type OutPut = (T1, T2, T3);

    fn has_list(&self) -> Self::OutPut {
        (self.0.clone(), self.1.0.clone(), self.1.1.0.clone())
    }
}

impl<T1: Clone, T2: Clone, T3: Clone,T4:Clone> Flat for (T1, (T2, (T3, (T4, TupleTail)))) {
    type OutPut = (T1, T2, T3, T4);

    fn has_list(&self) -> Self::OutPut {
        (self.0.clone(), self.1.0.clone(), self.1.1.0.clone(), self.1.1.1.0.clone())
    }
}

// 继续为更多嵌套元组实现 Flat trait

impl<T1: Clone, T2: Clone, T3: Clone, T4: Clone,T5:Clone> Flat for (T1, (T2, (T3, (T4, (T5, TupleTail))))) {
    type OutPut = (T1, T2, T3, T4, T5);

    fn has_list(&self) -> Self::OutPut {
        (self.0.clone(), self.1.0.clone(), self.1.1.0.clone(), self.1.1.1.0.clone(), self.1.1.1.1.0.clone())
    }
}

impl<T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone,T6:Clone> Flat for (T1, (T2, (T3, (T4, (T5, (T6, TupleTail)))))) {
    type OutPut = (T1, T2, T3, T4, T5, T6);

    fn has_list(&self) -> Self::OutPut {
        (self.0.clone(), self.1.0.clone(), self.1.1.0.clone(), self.1.1.1.0.clone(), self.1.1.1.1.0.clone(), self.1.1.1.1.1.0.clone())
    }
}

impl<T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone,T6:Clone,T7:Clone> Flat for (T1, (T2, (T3, (T4, (T5, (T6, (T7,TupleTail))))))) {
    type OutPut = (T1, T2, T3, T4, T5, T6,T7);

    fn has_list(&self) -> Self::OutPut {
        (self.0.clone(), self.1.0.clone(), self.1.1.0.clone(), self.1.1.1.0.clone(), self.1.1.1.1.0.clone(), self.1.1.1.1.1.0.clone(), self.1.1.1.1.1.1.0.clone())
    }
}
