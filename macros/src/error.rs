use proc::{
    ItemStruct, Path, Result,
    quote::ToTokens,
    syn::{Expr, Type, Visibility},
};

pub(crate) use self::backtracesource::BacktraceSource;
use self::{direct::Direct, with_whatever::WithWhatever};

mod backtracesource;
mod direct;
mod with_whatever;

pub(crate) struct Error(ErrorImpl);

enum ErrorImpl {
    Direct(Direct),
    WithWhatever(WithWhatever),
}

impl Error {
    pub(crate) fn new(
        crate_: Path,
        wrapped_error: Option<Type>,
        display_args: Vec<Expr>,
        context_vis: Option<Visibility>,
        backtrace: Option<BacktraceSource>,
        test_whatever: bool,
        item: ItemStruct,
    ) -> Result<Self> {
        if test_whatever {
            Ok(Self(ErrorImpl::WithWhatever(WithWhatever::new(
                crate_,
                wrapped_error,
                display_args,
                context_vis,
                backtrace,
                item,
            ))))
        } else {
            Ok(Self(ErrorImpl::Direct(Direct::new(
                &crate_,
                wrapped_error,
                &display_args,
                context_vis,
                backtrace,
                item,
            )?)))
        }
    }
}

impl ToTokens for Error {
    fn to_tokens(&self, tokens: &mut proc::TokenStream) {
        match &self.0 {
            ErrorImpl::Direct(d) => d.to_tokens(tokens),
            ErrorImpl::WithWhatever(w) => w.to_tokens(tokens),
        };
    }
}
