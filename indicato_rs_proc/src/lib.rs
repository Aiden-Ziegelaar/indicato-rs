use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Apply)]
pub fn apply_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl Apply for #name {
            fn apply(&mut self, input: Self::Input) -> Self::Output {
                self.execute(input, &ExecutionContext::Apply)
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(Evaluate)]
pub fn evaluate_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl Evaluate for #name {
            fn evaluate(&mut self, input: Self::Input) -> Self::Output {
                self.execute(input, &ExecutionContext::Evaluate)
            }
        }
    };
    gen.into()
}
