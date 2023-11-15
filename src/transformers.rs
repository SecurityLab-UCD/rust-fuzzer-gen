use proc_macro2::{TokenStream, TokenTree};
use quote::quote;
use syn::{visit_mut::VisitMut, Expr, ExprBlock};

/// Convert the body of the fuzz_target! macro into a test function
fn to_test_fn(content_tokens: TokenStream, input_identifier: TokenStream) -> String {
    let test_fn = quote! {
        #[test]
        fn test_something() {
            let #input_identifier = [];
            #content_tokens
        }
    };

    return test_fn.to_string();
}

fn input_identifier_wo_type(inputs: TokenStream) -> TokenStream {
    let mut input_identifier = TokenStream::new();
    for token in inputs.into_iter() {
        match &token {
            TokenTree::Punct(punct) if punct.as_char() == ':' => break,
            _ => input_identifier.extend(Some(token)),
        }
    }
    input_identifier
}
/// add println!() to the beginning of the fuzz_target! macro
pub struct ReportTransformer {
    pub test_fns: Vec<String>,
}

impl VisitMut for ReportTransformer {
    fn visit_macro_mut(&mut self, mac: &mut syn::Macro) {
        // only visit fuzz_target! macros
        if mac.path.is_ident("fuzz_target") {
            if let Ok(parsed_macro) = syn::parse2::<syn::ExprClosure>(mac.tokens.clone()) {
                // Transform the closure
                let inputs = &parsed_macro.inputs;
                let body = parsed_macro.body.as_ref().clone();

                // extract the statements' tokens from the body block to avoid extra braces {}
                let body_tokens = if let Expr::Block(ExprBlock { block, .. }) = body {
                    block
                        .stmts
                        .into_iter()
                        .map(|stmt| quote! { #stmt })
                        .collect()
                } else {
                    // if it's not a block, then just use body as it is.
                    quote! { #body }
                };

                let input_tokens: TokenStream =
                    inputs.into_iter().map(|stmt| quote! {#stmt}).collect();
                let input_identifier = input_identifier_wo_type(input_tokens);

                let modified = quote! {
                    |#inputs| {
                        println!("{:?}", #input_identifier);
                        #body_tokens
                    }
                };

                self.test_fns
                    .push(to_test_fn(body_tokens, input_identifier));
                mac.tokens = TokenStream::from(modified);
            }
        }
    }
}
