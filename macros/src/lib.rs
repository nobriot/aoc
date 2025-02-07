extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

use quote::{format_ident, quote, ToTokens};
use syn::{parse::Parse, parse::ParseStream, parse_macro_input, Token};

/// Macro that transforms a range of numbers into importing a day module and running it
/// to solve the problem
///
/// For example:
///
/// ```rust
/// use aoc_macros::solve_days;
/// solve_days!(1, 2, args)
/// ```
///
/// Will be expanded to:
///
/// ```rust
/// // Day 1
/// if args.all || args.days.contains(&1) {
///     println!("Solving Day {}.", 1);
///     let results = days::day01::solve();
///     println!("Part 1 result: {:?}", results.0);
///     println!("Part 2 result: {:?}", results.1);
/// }
/// // Day 2
/// if args.all || args.days.contains(&2) {
///     println!("Solving Day {}.", 2);
///     let results = days::day02::solve();
///     println!("Part 1 result: {:?}", results.0);
///     println!("Part 2 result: {:?}", results.1);
/// }
/// ```
#[proc_macro]
pub fn solve_days(attr: TokenStream) -> TokenStream {
    //println!("attr: {:?}", attr);
    let a: DaysRangeAndArgs = parse_macro_input!(attr as DaysRangeAndArgs);
    quote!(#a).into()

    // let start: usize = 1;
    // let stop = 2;
    // let args = format_ident!("{}", "args");
    //println!("item: {:?}", item);
    //     let mut expanded = quote! {};
    //
    //     // Now just loop from day start to stop
    //     for i in start..=stop {
    //         let module_name = if i < 10 {
    //             format_ident!("day0{}", i)
    //         } else {
    //             format_ident!("day{}", i)
    //         };
    //         expanded.extend(quote! {
    //             // Day #i
    //             if #args.all || #args.days.contains(&#i) {
    //             println!("Solving Day {}.", #i);
    //             let results = days::#module_name::solve();
    //             println!("Part 1 result: {:?}", results.0);
    //             println!("Part 2 result: {:?}", results.1);
    //             }
    //
    //         });
    //     }
    //
    //     TokenStream::from(expanded)
}

/// Struct for the solve_days! macro, representing the input:
/// (start, stop, args)
/// Start in an integer, in the range 1..=25
/// Stop in an integer, in the range 2..=25, larger or equal to start
/// args is an identifier, representing the arguments to the program
///
struct DaysRangeAndArgs {
    start: usize,
    stop: usize,
    args: syn::Ident,
}

impl Parse for DaysRangeAndArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let start_lit: syn::LitInt = input.parse()?;
        input.parse::<Token![,]>()?;
        let stop_lit: syn::LitInt = input.parse()?;
        input.parse::<Token![,]>()?;
        let args = input.parse::<syn::Ident>()?;

        let start = start_lit.base10_parse::<usize>()?;
        let stop = stop_lit.base10_parse::<usize>()?;
        Ok(Self { start, stop, args })
    }
}

impl ToTokens for DaysRangeAndArgs {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let start = self.start;
        let stop = self.stop;
        let args = &self.args;

        // Now just loop from day start to stop
        for i in start..=stop {
            let module_name = if i < 10 {
                format_ident!("day0{}", i)
            } else {
                format_ident!("day{}", i)
            };
            tokens.extend(quote! {
                // Day #i
                if #args.all || #args.days.contains(&#i) {
                println!("Solving Day {}.", #i);
                let results = days::#module_name::solve();
                println!("Part 1 result: {:?}", results.0);
                println!("Part 2 result: {:?}", results.1);
                }

            });
        }
    }
}
