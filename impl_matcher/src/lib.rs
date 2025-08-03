use proc_macro::TokenStream;
use proc_macro2::Literal;
use quote::{format_ident, quote};

extern crate proc_macro;

#[proc_macro]
pub fn impl_or_matcher(_stream: TokenStream) -> TokenStream {
    let max = 16; // TODO: parse with unsynn

    let mut or_to_matcher_impls = Vec::new();
    let mut or_tparse_impls = Vec::new();
    let mut do_match_impls = Vec::new();
    let mut add_matcher_impls = Vec::new();

    for len in 1..=max {
        let numbered_idents = |initial: char| {
            (1..=len)
                .map(|i| format_ident!("{initial}{i}"))
                .collect::<Vec<_>>()
        };
        let parser_idents = numbered_idents('P');

        or_tparse_impls.push(quote! {
            impl<#(#parser_idents: TParse + 'static,)*> TParse for Or<(#(#parser_idents,)*)> {
                fn tparse(input: &str) -> Option<(Self, usize)>
                where
                    Self: Sized,
                {
                    #(
                        if let Some((data, offset)) = #parser_idents::tparse(input) {
                            return Some((Self(Box::new(data), PhantomData), offset));
                        }
                    )*
                    None
                }
            }
        });

        let is_nothings = vec![quote! {IsNothing}; len];
        let units = vec![quote! {()}; len];

        or_to_matcher_impls.push(quote! {
            impl<#(#parser_idents: TParse + 'static,)*> Or<(#(#parser_idents,)*)> {
                pub fn matcher<Args, Out>(
                    self,
                    args: Args,
                ) -> Matcher<
                    (#(#parser_idents,)*),
                    Args,
                    (#(fn(Box<#parser_idents>, Args) -> Out,)*),
                    (#(#is_nothings,)*),
                > {
                    Matcher(args, (#(#units,)*), self)
                }
            }
        });

        let is_presents = vec![quote! {IsPresent}; len];
        let nums = (0..len).map(Literal::usize_unsuffixed).collect::<Vec<_>>();

        do_match_impls.push(quote! {
            impl<#(#parser_idents: 'static,)* Args, Out>
                Matcher<
                    (#(#parser_idents,)*),
                    Args,
                    (#(fn(Box<#parser_idents>, Args) -> Out,)*),
                    (#(#is_presents,)*),
                >
            {
                pub fn do_match(self) -> Out {
                    let mut dyn_parser = self.2.0;
                    #(
                        match dyn_parser.downcast::<#parser_idents>() {
                            Ok(parser) => return self.1.#nums(parser, self.0),
                            Err(e) => dyn_parser = e,
                        }
                    )*
                    unreachable!()
                }
            }
        });

        for i in 0..len {
            let mut map_types_before = numbered_idents('M');
            map_types_before[i] = format_ident!("IsNothing");

            let mut map_generics = map_types_before.clone();
            map_generics.remove(i);

            let mut map_types_after = map_types_before.clone();
            map_types_after[i] = format_ident!("IsPresent");

            let parser_ident = &parser_idents[i];

            let map = (0..len)
                .map(|num| {
                    if num == i {
                        quote! {f}
                    } else {
                        let num = Literal::usize_unsuffixed(num);
                        quote! {self.1.#num}
                    }
                })
                .collect::<Vec<_>>();

            let i = Literal::usize_unsuffixed(i);

            add_matcher_impls.push(quote! {
                impl<#(#parser_idents: 'static,)* Args, Out, #(#map_generics: MapType,)*>
                    AddMatcher<#i> for
                    Matcher<
                        (#(#parser_idents,)*),
                        Args,
                        (#(fn(Box<#parser_idents>, Args) -> Out,)*),
                        (#(#map_types_before,)*),
                    >
                {
                    type Matcher = fn(Box<#parser_ident>, Args) -> Out;
                    type Output = Matcher<
                        (#(#parser_idents,)*),
                        Args,
                        (#(fn(Box<#parser_idents>, Args) -> Out,)*),
                        (#(#map_types_after,)*),
                    >;
                    fn add_matcher(self, f: Self::Matcher) -> Self::Output {
                        Matcher(self.0, (#(#map,)*), self.2)
                    }
                }
            });
        }
    }

    quote! {
        #(
            #or_tparse_impls
            #or_to_matcher_impls
            #do_match_impls
        )*
        #(
            #add_matcher_impls
        )*
    }
    .into()
}
