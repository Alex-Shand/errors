use proc::{
    DeriveInput,
    quote::{ToTokens, quote},
};

pub(crate) struct FromRef {
    input: DeriveInput,
}

impl FromRef {
    pub(crate) fn new(input: DeriveInput) -> Self {
        Self { input }
    }
}

impl ToTokens for FromRef {
    fn to_tokens(&self, tokens: &mut proc::TokenStream) {
        let Self { input } = self;
        let name = &input.ident;
        tokens.extend(quote! {
            #input

            const _: () = {
                pub(crate) struct Ref<'a>(&'a #name);

                impl From<Ref<'_>> for #name {
                    fn from(value: Ref<'_>) -> Self {
                        value.0.clone()
                    }
                }

                impl #name {
                    pub(crate) fn fr(&self) -> Ref<'_> {
                        Ref(self)
                    }
                }
            };
        });
    }
}
