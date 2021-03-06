use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_macro_input::parse;
use syn::{ItemStruct, Result};

pub fn derive(input: proc_macro::TokenStream) -> Result<TokenStream> {
    let target = parse::<ItemStruct>(input)?;
    let (impl_generics, ty_generics, where_clause) = target.generics.split_for_impl();
    let struct_name = target.ident.clone();
    Ok(quote!(
        impl #impl_generics unhtml::FromText for #struct_name #ty_generics #where_clause {
           fn from_inner_text(select: unhtml::ElemIter) -> unhtml::Result<Self> {
                let first = select.next().ok_or(())?;
                let mut ret = String::new();
                for next_segment in first.text() {
                    ret += next_segment.trim();
                }
                Self::from_str(&ret).map_err(|err| (ret.to_owned(), stringify!(#struct_name).to_owned(), err.to_string()).into())
            }
            fn from_attr(select: unhtml::ElemIter, attr: &str) -> unhtml::Result<Self> {
                let first = select.next().ok_or(())?;
                let attr = first.value().attr(attr).ok_or((attr.to_owned(), first.html()))?;
                Self::from_str(attr.trim()).map_err(|err| (attr.trim().to_owned(), stringify!(#struct_name).to_owned(), err.to_string()).into())
            }
        }
    ))
}
