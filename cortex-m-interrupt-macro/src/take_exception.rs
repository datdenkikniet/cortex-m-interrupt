use proc_macro_error::abort;
use quote::quote;
use syn::{parse::Parse, spanned::Spanned, Error, LitStr, TypePath};

fn is_allowed(name: &str) -> bool {
    match name {
        "HardFault" | "DefaultHandler" => false,
        _ => true,
    }
}

pub(crate) struct TakeException {
    exception_path: TypePath,
}

impl Parse for TakeException {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let exception = input
            .parse()
            .map_err(|e| Error::new(e.span(), "Expected path to interrupt as first argument."))?;

        Ok(Self {
            exception_path: exception,
        })
    }
}

impl TakeException {
    pub fn build(&self) -> proc_macro::TokenStream {
        let Self { exception_path } = self;

        let exception_name = if let Some(last_seg) = &exception_path.path.segments.last() {
            &last_seg.ident
        } else {
            abort!(exception_path, "Could not find last segment of type path.");
        };

        if !super::is_exception(&exception_name.to_string()) {
            abort!(
                exception_name,
                "`{}` is an interrupt that is served by the NVIC, not an exception. You may want to use `take` instead",
                exception_name
            )
        }

        if !is_allowed(&exception_name.to_string()) {
            abort!(
                exception_name,
                "`{}` is not an exception that can be used with `take_exception`",
                exception_name
            );
        }

        let exception_name = LitStr::new(&exception_name.to_string(), exception_path.span());

        let defs = quote! {
            struct Handle;
        };

        let pre_write = quote! {};
        let post_write = quote! {};
        let return_value = quote! {
            Handle
        };

        super::handle_generator(exception_name, defs, pre_write, post_write, return_value).into()
    }
}
