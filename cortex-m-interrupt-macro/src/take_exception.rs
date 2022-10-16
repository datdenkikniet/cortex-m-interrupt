use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::quote;
use syn::{
    parse::Parse, punctuated::Punctuated, spanned::Spanned, token::Colon2, Error, LitStr, Path,
    PathSegment, TypePath,
};

fn handle_generator(
    interrupt_export_name: LitStr,
    interrupt_path: Path,
    defs: TokenStream,
    pre_write: TokenStream,
    post_write: TokenStream,
    interrupt_impl: TokenStream,
    return_value: TokenStream,
) -> TokenStream {
    let segments = interrupt_path.segments.len();

    if segments <= 1 {
        abort!(
            interrupt_path,
            "You must specify the interrupt name with at least two path segments! Try using Interrupt::{} or interrupt::{} instead.",
            interrupt_path.segments.first().unwrap().ident.to_string(),
            interrupt_path.segments.first().unwrap().ident.to_string()
        );
    }

    let interrupt_path_type = interrupt_path.segments.iter().take(segments - 1);
    let path: Punctuated<PathSegment, Colon2> = interrupt_path_type.map(|s| s.clone()).collect();

    quote::quote! {
        {
            #defs

            static mut HANDLER: core::mem::MaybeUninit<fn()> = core::mem::MaybeUninit::uninit();

            #[export_name = #interrupt_export_name]
            pub unsafe extern "C" fn isr() {
                (HANDLER.assume_init())();
            }

            impl ::cortex_m_interrupt::EventHandle for Handle {
                fn register(self, f: fn()) {
                    #pre_write
                    unsafe {
                        HANDLER.write(f);
                    }

                    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::Release);

                    #post_write
                }
            }

            impl ::cortex_m_interrupt::IrqHandle for Handle {
                type Number = #path;

                fn interrupt(&self) -> Self::Number {
                    #interrupt_impl
                }
            }

            #return_value
        }
    }
}

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

        handle_generator(
            exception_name,
            exception_path.path.clone(),
            defs,
            pre_write,
            post_write,
            quote! {},
            return_value,
        )
        .into()
    }
}
