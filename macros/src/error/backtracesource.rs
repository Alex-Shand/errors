use proc::{
    quote::{ToTokens, quote},
    syn::{
        Ident, parenthesized,
        parse::{Parse, ParseStream},
        token::Paren,
    },
};

pub(crate) enum BacktraceSource {
    Field(Ident),
    New,
}

impl Parse for BacktraceSource {
    fn parse(input: ParseStream<'_>) -> proc::Result<Self> {
        if input.peek(Paren) {
            let content;
            let _ = parenthesized!(content in input);
            Ok(BacktraceSource::Field(content.parse()?))
        } else {
            Ok(BacktraceSource::New)
        }
    }
}

impl ToTokens for BacktraceSource {
    fn to_tokens(&self, tokens: &mut proc::TokenStream) {
        match self {
            BacktraceSource::Field(ident) => tokens.extend(quote!((#ident))),
            BacktraceSource::New => (),
        }
    }
}
