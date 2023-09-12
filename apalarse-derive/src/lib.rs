extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Fields, ItemStruct};

#[proc_macro_attribute]
pub fn tla_state(_attrs: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);
    let name = &input.ident;

    let fields = match &input.fields {
        Fields::Named(named) => &named.named,
        _ => panic!("Only named fields are supported."),
    };

    let field_names: Vec<_> = fields.iter().map(|f| &f.ident).collect();
    let field_types: Vec<_> = fields.iter().map(|f| &f.ty).collect();
    let field_vis: Vec<_> = fields.iter().map(|f| &f.vis).collect();
    let field_attrs: Vec<_> = fields.iter().map(|f| &f.attrs).collect();

    let expanded = quote! {
        #[derive(Clone, Debug, Deserialize)]
        struct #name {
            #(
                #(#field_attrs)*
                #field_vis #field_names: Symbol<#field_types>,
            )*
        }

        impl #name {
            fn variable(cx: &mut Context) -> Self {
                Self {
                    #(#field_names: Symbol::new(cx.var_with_name(stringify!(#field_names))),)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn tla_record(_attrs: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);
    let name = &input.ident;

    let fields = match &input.fields {
        Fields::Named(named) => &named.named,
        _ => panic!("Only named fields are supported."),
    };

    let field_names: Vec<_> = fields.iter().map(|f| &f.ident).collect();
    let field_types: Vec<_> = fields.iter().map(|f| &f.ty).collect();
    let field_vis: Vec<_> = fields.iter().map(|f| &f.vis).collect();
    let field_attrs: Vec<_> = fields.iter().map(|f| &f.attrs).collect();

    // #(
    //     fn #field_names(&self) -> RecordField<Self, <#field_types as Expr>::Output> {
    //         RecordField::new(self.clone(), stringify!(#field_names))
    //     }
    // )

    let expanded = quote! {
        #[derive(Clone, Debug, Deserialize)]
        struct #name {
            #(
                #(#field_attrs)*
                #field_vis #field_names: #field_types,
            )*
        }

        impl #name {

        }
    };

    TokenStream::from(expanded)
}
