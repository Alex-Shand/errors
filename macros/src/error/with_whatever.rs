use proc::{
    ItemStruct, Path,
    quote::{ToTokens, format_ident, quote},
    syn::{Expr, Ident, Type, Visibility},
};

use super::BacktraceSource;

pub(super) struct WithWhatever {
    crate_: Path,
    wrapped_error: Option<Type>,
    display_args: Vec<Expr>,
    context_vis: Option<Visibility>,
    backtrace: Option<BacktraceSource>,
    item: ItemStruct,
}

impl WithWhatever {
    pub(super) fn new(
        crate_: Path,
        wrapped_error: Option<Type>,
        display_args: Vec<Expr>,
        context_vis: Option<Visibility>,
        backtrace: Option<BacktraceSource>,
        item: ItemStruct,
    ) -> Self {
        Self {
            crate_,
            wrapped_error,
            display_args,
            context_vis,
            backtrace,
            item,
        }
    }

    fn generate_details_names(&self) -> (Ident, Ident, Ident) {
        let root = self.item.ident.to_string();
        let root = root.trim_end_matches("Error");
        (
            format_ident!("{root}Details"),
            format_ident!("{root}DetailsCtx"),
            format_ident!("{root}Ctx"),
        )
    }
}

impl ToTokens for WithWhatever {
    fn to_tokens(&self, tokens: &mut proc::TokenStream) {
        let WithWhatever {
            crate_,
            wrapped_error,
            display_args,
            context_vis,
            backtrace,
            item:
                ItemStruct {
                    attrs,
                    vis,
                    struct_token: _,
                    ident,
                    generics,
                    fields,
                    semi_token: _,
                },
        } = self;
        let (details_name, details_context_name, public_context_name) =
            self.generate_details_names();
        let wrapped_error_arg =
            wrapped_error.as_ref().map(|w| quote!(wrapping = #w,));
        let context_vis_arg =
            context_vis.as_ref().map(|c| quote!(context = #c,));
        let backtrace_arg = backtrace.as_ref().map(|b| quote!(backtrace #b,));
        tokens.extend(quote! {
            #(#attrs)*
            #[#crate_::union(crate = #crate_, test_whatever)]
            #vis type #ident #generics = (#details_name,);
            #context_vis use self::#details_context_name as #public_context_name;

            #(#attrs)*
            #[#crate_::error(
                crate = #crate_,
                #wrapped_error_arg
                #context_vis_arg
                display(#(#display_args),*),
                #backtrace_arg
            )]
            #vis struct #details_name #generics #fields
        });
    }
}
