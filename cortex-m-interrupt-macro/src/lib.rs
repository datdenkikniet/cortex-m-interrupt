use proc_macro2::Literal;
use quote::quote;
use syn::{parse::Parse, token::Comma, Expr, Ident, Lit, Path};

#[proc_macro]
pub fn take(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as TakeInput);

    let TakeInput {
        pac_path,
        interrupt_ident,
        priority,
        irq_handle_path: cortex_m_interrupt_path,
    } = input;

    let interrupt_export_name = Lit::new(Literal::string(&interrupt_ident.to_string()));

    let cortex_m_int_path = if let Some(cortex_m_int_path) = cortex_m_interrupt_path {
        quote! {
            #cortex_m_int_path
        }
    } else {
        quote! {
            cortex_m_interrupt
        }
    };

    quote! {
        {
            const INTERRUPT: #pac_path::Interrupt = #pac_path::interrupt::#interrupt_ident;

            static mut HANDLER: core::mem::MaybeUninit<fn()> = core::mem::MaybeUninit::uninit();

            #[export_name = #interrupt_export_name]
            pub unsafe extern "C" fn isr() {
                (HANDLER.assume_init())();
            }

            pub struct Handle {
                priority: u8,
            }

            impl #cortex_m_int_path::IrqHandle for Handle {
                fn register(self, f: fn()) {
                    #cortex_m_int_path::cortex_m::peripheral::NVIC::mask(INTERRUPT);

                    unsafe {
                        HANDLER.write(f);
                    }

                    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::Release);

                    unsafe {
                        let mut nvic: #cortex_m_int_path::cortex_m::peripheral::NVIC = core::mem::transmute(());
                        #cortex_m_int_path::logical2hw(self.priority, #pac_path::NVIC_PRIO_BITS);
                        #cortex_m_int_path::cortex_m::peripheral::NVIC::unmask(INTERRUPT);
                    }
                }
            }

            Handle {
                priority: #priority,
            }
        }
    }
    .into()
}

struct TakeInput {
    pac_path: Path,
    interrupt_ident: Ident,
    priority: Expr,
    irq_handle_path: Option<Path>,
}

impl Parse for TakeInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let pac_path = input.parse()?;
        input.parse::<Comma>()?;
        let interrupt_ident = input.parse()?;
        input.parse::<Comma>()?;
        let priority = input.parse()?;

        let irq_handle_path = input
            .parse::<Option<Comma>>()
            .map(|_| input.parse().ok())
            .ok()
            .flatten();

        Ok(Self {
            pac_path,
            interrupt_ident,
            priority,
            irq_handle_path,
        })
    }
}
