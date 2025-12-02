use darling::FromMeta;
use proc_macro::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{FnArg, ReturnType};

#[proc_macro_attribute]
pub fn parser(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input: syn::ItemFn = match syn::parse(input) {
        Ok(input) => input,
        Err(e) => return e.to_compile_error().into(),
    };

    let ReturnType::Type(_, return_type) = &input.sig.output else {
        return syn::Error::new(input.sig.output.span(), "expected a return type")
            .to_compile_error()
            .into();
    };

    let name = &input.sig.ident;

    quote! {
        impl From<&PuzzleInput> for #return_type {
            fn from(input: &PuzzleInput) -> Self {
                #name(input).into()
            }
        }

        #input
    }
    .into()
}

#[derive(FromMeta)]
#[darling(derive_syn_parse)]
struct SolutionArgs {
    day: usize,
    part: usize,
}

#[proc_macro_attribute]
pub fn solution(args: TokenStream, input: TokenStream) -> TokenStream {
    let args: SolutionArgs = match syn::parse(args) {
        Ok(args) => args,
        Err(e) => return e.to_compile_error().into(),
    };

    let input: syn::ItemFn = match syn::parse(input) {
        Ok(input) => input,
        Err(e) => return e.to_compile_error().into(),
    };

    let name = &input.sig.ident;

    if input.sig.inputs.len() != 1 {
        return syn::Error::new(input.sig.span(), "expected exactly one input")
            .to_compile_error()
            .into();
    }

    let FnArg::Typed(function_param) = &input.sig.inputs[0] else {
        return syn::Error::new(
            input.sig.inputs[0].span(),
            "expected a function parameter, not a self receiver",
        )
        .to_compile_error()
        .into();
    };

    let arg_type = &function_param.ty;

    let day = args.day;
    let part = args.part;

    quote! {
        impl Solver<#day, #part> for PuzzleInput {
            type Input = #arg_type;
            fn solve(&self, input: Self::Input) -> Option<impl std::fmt::Display + std::fmt::Debug> {
                Some(#name(input))
            }
        }

        #input
    }
    .into()
}
