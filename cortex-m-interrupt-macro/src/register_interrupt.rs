use syn::{parse::Parse, Error, Ident, Path, Token};

#[derive(Debug)]
pub struct RegisterInterrupt {
    struct_name: Ident,
    interrupt_full_path: Path,
    interrupt_enum: Path,
    interrupt_name: Ident,
    hal_drivers: Vec<Path>,
}

impl Parse for RegisterInterrupt {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // Extract syntax
        let struct_name: Ident = input.parse()?;
        let _comma: Token![,] = input.parse()?;

        let mut irq: Path = input.parse()?;
        let _colon: Token![->] = input.parse()?;

        let mut hal_drivers = Vec::new();

        loop {
            let hal_driver: Path = input.parse()?;
            hal_drivers.push(hal_driver);

            if input.is_empty() {
                break;
            }

            let _comma: Token![,] = input.parse()?;
        }

        // Error check

        // We need to get the interrupt enum's path and interrupt ident
        if irq.segments.len() < 2 {
            return Err(Error::new(
                input.span(),
                "Interrupt path is a single identifier, this marcro needs to know the path to the interrupt enum and the interrupts name, e.g. `hal::pac::Interrupt::Uart0`",
            ));
        }

        let interrupt_full_path = irq.clone();
        let interrupt_name = irq.segments.pop().unwrap().into_value().ident;

        let v = irq.segments.pop().unwrap().into_value();
        irq.segments.push_value(v);

        // We need at least one driver
        if hal_drivers.is_empty() {
            return Err(Error::new(
                input.span(),
                "Expected path to event (interrupt or exception) as first argument.",
            ));
        }

        Ok(Self {
            struct_name,
            interrupt_full_path,
            interrupt_enum: irq,
            interrupt_name,
            hal_drivers,
        })
    }
}

impl RegisterInterrupt {
    pub fn codegen(&self) -> proc_macro2::TokenStream {
        let RegisterInterrupt {
            struct_name,
            interrupt_full_path,
            interrupt_enum,
            interrupt_name,
            hal_drivers,
        } = self;

        // Codegen const asserts for vector <-> driver connection
        let const_asserts: Vec<_> = hal_drivers
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
                    format!("The driver `{ds}` does not request the proveided interrupt `{intn}`");

                quote::quote! {
                    const _: () = {
                        match <#driver as cortex_m_interrupt::InterruptRegistration<#interrupt_enum>>::VECTOR {
                            #interrupt_full_path => {}
                            _ => panic!(#panic_string),
                        }
                    };
                }
            })
            .collect();

        // Codegen interrupt to driver calls
        let on_interrupts: Vec<_> = hal_drivers
            .iter()
            .map(|driver| {
                quote::quote! {
                    <#driver as cortex_m_interrupt::InterruptRegistration<#interrupt_enum>>::on_interrupt();
                }
            })
            .collect();

        // Codegen trait impls for error checking
        let handle_impls: Vec<_> = hal_drivers
            .iter()
            .map(|driver| {
                quote::quote! {
                    unsafe impl cortex_m_interrupt::InterruptToken<#driver> for #struct_name {}
                }
            })
            .collect();

        quote::quote! {
            #(#const_asserts)*

            #[no_mangle]
            #[allow(non_snake_case)]
            unsafe extern "C" fn #interrupt_name() {
                #(#on_interrupts)*
            }


            #[derive(Debug, Copy, Clone)]
            pub struct #struct_name;

            #(#handle_impls)*
        }
    }
}
