extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;
use syn::{self, Data};
use mask_system_lib::{*};

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
