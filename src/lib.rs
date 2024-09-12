extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;
use syn::{self, Data};
pub use mask_system_lib::MaskSystem;
pub use mask_system_lib::CheckMaskSystem;
pub use mask_system_lib::MaskSystemContent;
pub use mask_system_lib::Content;
pub use mask_system_lib::Tag_1_2;
pub use mask_system_lib::Tag_2_4;
pub use mask_system_lib::Tag_3_8;
pub use mask_system_lib::Tag_4_16;
pub use mask_system_lib::Tag_5_32;
pub use mask_system_lib::Tag_6_64;
pub use mask_system_lib::op;
pub use mask_system_lib::Unsigned;
pub use mask_system_lib::validate_mask;
pub use mask_system_lib::validate_max_1;
pub use mask_system_lib::F;
pub use mask_system_lib::Flat;
pub use mask_system_lib::PickTuple;

#[proc_macro_derive(MaskSys)]
pub fn MaskSys(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let id = ast.ident;

    quote! {
        impl<T:Unsigned,Content:MaskSystemContent> MaskSystem<T,Content> for #id{
              default const _marker:usize = 0;

              default type Output = <#id as MaskSystem<T,Content>>::Output;

              default fn export() -> Self::Output {
                  <#id as MaskSystem<T,Content>>::export()
              }
        }

        impl IsMaskSystem for #id{}
    }
    .into()
}
