use proc_macro::TokenStream;
use quote::{format_ident, quote};

extern crate proc_macro;

#[proc_macro]
pub fn impl_or_matcher(_stream: TokenStream) -> TokenStream {
    let max = 32; // TODO: parse with unsynn

    let mut or_to_matcher_impls = Vec::new();

    for len in 1..=max {
        let numbered_idents = |initial: char| {
            (1..=len)
                .map(|i| format_ident!("{initial}{i}"))
                .collect::<Vec<_>>()
        };
        let t_idents = numbered_idents('T');
        let is_nothings = vec![quote! {IsNothing}; len];
        let units = vec![quote! {()}; len];

        or_to_matcher_impls.push(quote! {
            impl<#(#t_idents: TParse + 'static,)*> Or<(#(#t_idents,)*)> {
                pub fn matcher<Args, Out>(
                    self,
                    args: Args,
                ) -> Matcher<
                    (#(#t_idents,)*),
                    Args,
                    (#(fn(Box<#t_idents>, Args) -> Out,)*),
                    (#(#is_nothings,)*),
                > {
                    Matcher(args, (#(#units,)*), self)
                }
            }
        });
    }

    quote! {#(#or_to_matcher_impls)*}.into()
}
