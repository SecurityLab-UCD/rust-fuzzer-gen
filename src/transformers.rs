use proc_macro2::TokenStream;
use quote::quote;
use syn::{visit_mut::VisitMut, Expr};

/// Convert the body of the fuzz_target! macro into a test function
fn to_test_fn(content: Expr) -> String {
    let test_fn = quote! {
        #[test]
        fn test_something() {
            let data = [];
            #content
        }
    };

    return test_fn.to_string();
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
                let body = &parsed_macro.body;
                let modified = quote! {
                    |#inputs| {
                        println!("{:?}", data);
                        #body
                    }
                };

                self.test_fns.push(to_test_fn(body.as_ref().clone()));
                mac.tokens = TokenStream::from(modified);
            }
        }
    }
}
