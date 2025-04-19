use proc::{
    Path, Result,
    quote::{ToTokens, quote},
    syn::{Attribute, Error, Ident, ItemType, Type, TypeTuple, Visibility},
};

pub(crate) struct Union {
    crate_: Path,
    attrs: Vec<Attribute>,
    vis: Visibility,
    ident: Ident,
    variants: Vec<Variant>,
}

impl Union {
    pub(crate) fn new(
        crate_: Path,
        test_whatever: bool,
        ItemType {
            attrs,
            vis,
            type_token: _,
            ident,
            generics,
            eq_token: _,
            ty,
            semi_token: _,
        }: ItemType,
    ) -> Result<Self> {
        if !generics.params.is_empty() {
            return Err(Error::new_spanned(
                generics,
                "errors::union doesn't support generic errors",
            ));
        }
        let variants = match *ty {
            Type::Tuple(TypeTuple { elems, .. }) => elems.into_iter(),
            other => {
                return Err(Error::new_spanned(
                    other,
                    "errors::union must use a tuple type",
                ));
            }
        };
        let mut variants =
            variants.map(Variant::new).collect::<Result<Vec<_>>>()?;
        if test_whatever {
            variants.push(Variant::Whatever);
        }
        Ok(Self {
            crate_,
            attrs,
            vis,
            ident,
            variants,
        })
    }
}

impl ToTokens for Union {
    fn to_tokens(&self, tokens: &mut proc::TokenStream) {
        let Union {
            crate_,
            attrs,
            vis,
            ident,
            variants,
        } = self;
        tokens.extend(quote! {
            #(#attrs)*
            #[derive(Debug, #crate_::snafu::Snafu)]
            #[snafu(crate_root(#crate_::snafu))]
            #vis enum #ident {
                #(#variants),*
            }
        });
    }
}

enum Variant {
    Real { ident: Ident, ty: Type },
    Whatever,
}

impl Variant {
    fn new(ty: Type) -> Result<Self> {
        let err = "Tuple elements must be simple types";
        let path = match &ty {
            Type::Path(path) => path,
            other => return Err(Error::new_spanned(other, err)),
        };
        if path.qself.is_some() {
            return Err(Error::new_spanned(path, err));
        }
        let ident = path.path.require_ident()?.to_owned();
        Ok(Self::Real { ident, ty })
    }
}

impl ToTokens for Variant {
    fn to_tokens(&self, tokens: &mut proc::TokenStream) {
        tokens.extend(match self {
            Variant::Real { ident, ty } => quote! {
                #[allow(missing_docs)]
                #[snafu(transparent)]
                #ident {
                    #[snafu(backtrace)]
                    source: #ty
                }
            },
            Variant::Whatever => quote! {
                #[cfg(test)]
                #[snafu(whatever)]
                Whatever { message: String }
            },
        });
    }
}
