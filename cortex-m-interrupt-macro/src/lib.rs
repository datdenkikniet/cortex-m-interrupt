use proc_macro2::Literal;
use quote::quote;
use syn::{parse::Parse, token::Comma, Expr, Lit, Path};

struct TakeInput {
    interrupt_path: Path,
    priority: Expr,
    irq_handle_path: Option<Path>,
}

impl Parse for TakeInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let interrupt_ident = input.parse()?;
        input.parse::<Comma>()?;
        let priority = input.parse()?;

        let irq_handle_path = input
            .parse::<Option<Comma>>()
            .map(|_| input.parse().ok())
            .ok()
            .flatten();

        Ok(Self {
            interrupt_path: interrupt_ident,
            priority,
            irq_handle_path,
        })
    }
}

fn build(input: proc_macro::TokenStream, use_rtic_prio: bool) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as TakeInput);

    let TakeInput {
        interrupt_path,
        priority,
        irq_handle_path: cortex_m_interrupt_path,
    } = input;

    let interrupt_export_name = Lit::new(Literal::string(
        &interrupt_path.segments.last().unwrap().ident.to_string(),
    ));

    let cortex_m_int_path = if let Some(cortex_m_int_path) = cortex_m_interrupt_path {
        quote! {
            #cortex_m_int_path
        }
    } else {
        quote! {
            cortex_m_interrupt
        }
    };

    let set_priority = if use_rtic_prio {
        quote! {
            let prio_bits = #cortex_m_int_path::determine_prio_bits(&mut nvic, #interrupt_path.number());
            let priority = #cortex_m_int_path::logical2hw(self.priority, prio_bits);
            nvic.set_priority(#interrupt_path, priority);
        }
    } else {
        quote! {
            nvic.set_priority(#interrupt_path, self.priority);
        }
    };

    quote! {
        {
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
                    use  #cortex_m_int_path::cortex_m::interrupt::InterruptNumber;

                    #cortex_m_int_path::cortex_m::peripheral::NVIC::mask(#interrupt_path);

                    unsafe {
                        HANDLER.write(f);
                    }

                    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::Release);

                    #cortex_m_int_path::cortex_m::interrupt::free(|_| {
                        unsafe {
                            let mut nvic: #cortex_m_int_path::cortex_m::peripheral::NVIC = core::mem::transmute(());
                            #set_priority
                            #cortex_m_int_path::cortex_m::peripheral::NVIC::unmask(#interrupt_path);
                        }
                    });
                }
            }

            Handle {
                priority: #priority,
            }
        }
    }.into()
}

#[proc_macro]
pub fn take(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    build(input, true)
}

#[proc_macro]
pub fn take_raw_prio(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    build(input, false)
}
