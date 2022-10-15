use syn::{parse::Parse, Error, TypePath};

struct TakeExceptionInput {
    exception: TypePath,
}

impl Parse for TakeExceptionInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let exception = input
            .parse()
            .map_err(|e| Error::new(e.span(), "Expected path to interrupt as first argument."))?;

        Ok(Self { exception })
    }
}
