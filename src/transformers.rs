use proc_macro2::TokenStream;
use quote::quote;
use syn::visit_mut::VisitMut;

/// add println!() to the beginning of the fuzz_target! macro
pub struct ReportTransformer;

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

                mac.tokens = TokenStream::from(modified);
            }
        }
    }
}
