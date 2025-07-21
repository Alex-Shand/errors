use std::collections::HashSet;

use proc::{
    ItemStruct, Path, Result,
    quote::{ToTokens, quote},
    syn::{
        Attribute, Error, Expr, Field, Fields, Ident, Type, Visibility,
        parse_quote,
    },
};

use super::BacktraceSource;

pub(super) struct Direct {
    attrs: Vec<Attribute>,
    vis: Visibility,
    ident: Ident,
    fields: Vec<Field>,
}

impl Direct {
    pub(super) fn new(
        crate_: &Path,
        wrapped_error: Option<Type>,
        display_args: &[Expr],
        context_vis: Option<Visibility>,
        backtrace: Option<BacktraceSource>,
        ItemStruct {
            mut attrs,
            vis,
            struct_token: _,
            ident,
            generics,
            fields,
            semi_token: _,
        }: ItemStruct,
    ) -> Result<Self> {
        if display_args.is_empty() {
            return Err(proc::meta::required_argument_error(
                "display",
                ident.span(),
            ));
        }
        if !generics.params.is_empty() {
            return Err(Error::new_spanned(
                generics,
                "errors::error doesn't support generic errors",
            ));
        }
        attrs.push(parse_quote!(#[derive(Debug, #crate_::snafu::Snafu)]));
        attrs.push(parse_quote!(#[snafu(display("{} ({})", ::std::format_args!(#(#display_args),*), self.location))]));
        attrs.push(parse_quote!(#[snafu(crate_root(#crate_::snafu))]));
        attrs.push(parse_quote!(#[snafu(context(suffix(Ctx)))]));
        if let Some(context_vis) = context_vis {
            attrs.push(parse_quote!(#[snafu(visibility(#context_vis))]));
        }

        let mut fields = match fields {
            Fields::Named(n) => FieldsData::new(n.named.into_iter()),
            Fields::Unit => FieldsData::empty(),
            Fields::Unnamed(u) => {
                return Err(Error::new_spanned(
                    u,
                    "errors::error cannot be applied to a tuple struct",
                ));
            }
        };

        fields.check_reserved_field("source")?;
        fields.check_reserved_field("backtrace")?;
        fields.check_reserved_field("location")?;

        fields.add(parse_quote! {
            #[snafu(implicit)]
            location: #crate_::snafu::Location
        });

        if let Some(wrapped_error) = wrapped_error {
            fields.add(parse_quote! {
                source: #wrapped_error
            });
        }

        if let Some(BacktraceSource::New) = backtrace {
            fields.add(parse_quote! {
                backtrace: #crate_::snafu::Backtrace
            });
        } else if let Some(BacktraceSource::Field(ident)) = backtrace {
            let mut found = false;
            for field in &mut fields.fields {
                if field.ident.as_ref().is_some_and(|i| *i == ident) {
                    field.attrs.push(parse_quote!(#[snafu(backtrace)]));
                    found = true;
                    break;
                }
            }
            if !found {
                return Err(Error::new_spanned(ident, "no such field"));
            }
        }

        Ok(Self {
            attrs,
            vis,
            ident,
            fields: fields.fields,
        })
    }
}

struct FieldsData {
    fields: Vec<Field>,
    names: HashSet<String>,
}

impl FieldsData {
    fn empty() -> Self {
        Self {
            fields: Vec::new(),
            names: HashSet::new(),
        }
    }

    fn new(fields: impl Iterator<Item = Field>) -> Self {
        let mut f = Vec::new();
        let mut names = HashSet::new();
        for field in fields {
            let _ = names.insert(
                field
                    .ident
                    .as_ref()
                    .expect("expected named fields")
                    .to_string(),
            );
            f.push(field);
        }
        Self { fields: f, names }
    }

    fn add(&mut self, field: Field) {
        let _ = self.names.insert(
            field
                .ident
                .as_ref()
                .expect("expected named fields")
                .to_string(),
        );
        self.fields.push(field);
    }

    fn check_reserved_field(&self, name: &str) -> Result<()> {
        if self.names.contains(name) {
            let field = self
                .fields
                .iter()
                .find(|f| {
                    f.ident.as_ref().expect("expected named fields") == name
                })
                .expect("expected the field to exist");
            return Err(Error::new_spanned(
                field,
                format!("field name {name} is reserved"),
            ));
        }
        Ok(())
    }
}

impl ToTokens for Direct {
    fn to_tokens(&self, tokens: &mut proc::TokenStream) {
        let Direct {
            attrs,
            vis,
            ident,
            fields,
        } = self;
        tokens.extend(quote! {
            #(#attrs)*
            #vis struct #ident {
                #(#fields),*
            }
        });
    }
}
