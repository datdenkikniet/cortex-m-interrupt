use quote::quote;
use std::collections::HashMap;
use syn::{parse::Parse, spanned::Spanned, Error, Ident, Path, Token};

//
// syntax
//
// register_interrupt!(MegaTimerToken,
//     Interrupt::TIM1_BRK -> hal::Brk<TIM1>,
//     Interrupt::TIM1_CC -> hal::Cc<TIM1>,
//     Interrupt::TIM1_TRG_COM_TIM11 -> hal::Trg<Tim1>,
//     Interrupt::TIM1_TRG_COM_TIM11 -> hal::Com<Tim1>,
//     Interrupt::TIM1_UP -> hal::Up<TIM1>,
// );
//

#[derive(Debug)]
struct Connection {
    interrupt_full_path: Path,
    hal_drivers: Vec<Path>,
}

#[derive(Debug)]
pub struct RegisterInterrupt {
    struct_name: Ident,
    interrupt_to_hal_driver: HashMap<Ident, Connection>,
}

impl Parse for RegisterInterrupt {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // Extract syntax
        let struct_name: Ident = input.parse()?;
        let _comma: Token![,] = input.parse()?;

        let mut interrupt_to_hal_driver = HashMap::new();

        loop {
            if input.is_empty() {
                break;
            }

            let irq_path: Path = input.parse()?;
            let _arrow: Token![->] = input.parse()?;
            let hal_driver: Path = input.parse()?;

            let irq_ident = irq_path.segments.last().unwrap().clone();
            interrupt_to_hal_driver
                .entry(irq_ident.ident)
                .or_insert(Connection {
                    interrupt_full_path: irq_path,
                    hal_drivers: vec![],
                })
                .hal_drivers
                .push(hal_driver);

            if input.is_empty() {
                break;
            }

            let _comma: Token![,] = input.parse()?;
        }

        // Error check

        // We need to get the interrupt enum's path and interrupt ident
        for (_, v) in &interrupt_to_hal_driver {
            if v.interrupt_full_path.segments.len() < 2 {
                return Err(Error::new(
                v.interrupt_full_path.span(),
                "Interrupt path is a single identifier, this marcro needs to know the path to the interrupt enum and the interrupts name, e.g. `hal::pac::Interrupt::Uart0`",
            ));
            }
        }

        // We need at least one driver
        let num_drivers: usize = interrupt_to_hal_driver
            .iter()
            .map(|v| v.1.hal_drivers.len())
            .sum();

        if num_drivers == 0 {
            return Err(Error::new(input.span(), "No interrupt drivers listed"));
        }

        Ok(Self {
            struct_name,
            interrupt_to_hal_driver,
        })
    }
}

impl RegisterInterrupt {
    pub fn codegen(&self) -> proc_macro2::TokenStream {
        let RegisterInterrupt {
            struct_name,
            interrupt_to_hal_driver,
        } = self;

        // Codegen const asserts for vector <-> driver connection

        let mut const_asserts = Vec::new();
        let mut interrupt_registrations = Vec::new();
        let mut handle_impls = Vec::new();

        for (interrupt_name, v) in interrupt_to_hal_driver {
            let hal_drivers = &v.hal_drivers;
            let interrupt_full_path = &v.interrupt_full_path;

            let mut interrupt_enum = interrupt_full_path.clone();
            interrupt_enum.segments.pop().unwrap().into_value().ident;
            let v = interrupt_enum.segments.pop().unwrap().into_value();
            interrupt_enum.segments.push_value(v);

            // Codegen interrupt to driver calls
            let on_interrupts: Vec<_> = hal_drivers
                .iter()
                .map(|driver| {
                    quote! {
                        <#driver as cortex_m_interrupt::InterruptRegistration<#interrupt_enum>>::on_interrupt();
                    }
                })
                .collect();

            interrupt_registrations.push(quote! {
                #[no_mangle]
                #[allow(non_snake_case)]
                unsafe extern "C" fn #interrupt_name() {
                    #(#on_interrupts)*
                }
            });

            let cs: Vec<_> = hal_drivers
            .iter()
            .map(|driver| {
                let ds = driver
                    .segments
                    .iter()
                    .map(|seg| format!("{}", seg.ident))
                    .collect::<Vec<String>>()
                    .join("::");
                let intn = interrupt_full_path
                    .segments
                    .iter()
                    .map(|seg| format!("{}", seg.ident))
                    .collect::<Vec<String>>()
                    .join("::");

                let panic_string =
                    format!("The driver `{ds}` does not request the provided interrupt `{intn}`");

                quote! {
                    const _: () = {
                        match <#driver as cortex_m_interrupt::InterruptRegistration<#interrupt_enum>>::VECTOR {
                            #interrupt_full_path => {}
                            _ => panic!(#panic_string),
                        }
                    };
                }
            })
            .collect();

            const_asserts.extend(cs);

            // Codegen trait impls for error checking
            let hi: Vec<_> = hal_drivers
                .iter()
                .map(|driver| {
                    quote! {
                        unsafe impl cortex_m_interrupt::InterruptToken<#driver> for #struct_name {}
                    }
                })
                .collect();

            handle_impls.extend(hi);
        }

        quote! {
            #(#const_asserts)*

            #(#interrupt_registrations)*

            #[derive(Debug, Copy, Clone)]
            pub struct #struct_name;

            #(#handle_impls)*
        }
    }
}
