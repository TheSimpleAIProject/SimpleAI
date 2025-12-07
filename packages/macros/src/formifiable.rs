use proc_macro2::{Span, TokenStream};
use quote::*;
use syn::{parse2, ItemStruct, LitStr};

pub fn macro_impl(item: TokenStream) -> TokenStream {
    let struct_item: ItemStruct = parse2(item).expect("No valid struct!");
    let struct_ident = struct_item.ident;
    let mut inputs = TokenStream::new();
    for field in struct_item.fields {
        let field_ident = field.ident.unwrap();
        let field_name = LitStr::new(&field_ident.to_string(), Span::call_site());
        quote! {
            section {
                class: "FormifyBox",
                h1 {
                    class: "FormifyText",
                    #field_name
                }
                input {
                    class: "FormifyInput",
                    name: {#field_name},
                    value: {self.#field_ident.clone()},
                    required: true,
                    r#type: "text",
                }
            }
        }
        .to_tokens(&mut inputs);
    }
    quote! {
        simple_ai_macros::icon! {
            formify_creation: <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M13.0001 10.9999L22.0002 10.9997L22.0002 12.9997L13.0001 12.9999L13.0001 21.9998L11.0001 21.9998L11.0001 12.9999L2.00004 13.0001L2 11.0001L11.0001 10.9999L11 2.00025L13 2.00024L13.0001 10.9999Z"></path></svg>,
            formify_edit: <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M15.7279 9.57627L14.3137 8.16206L5 17.4758V18.89H6.41421L15.7279 9.57627ZM17.1421 8.16206L18.5563 6.74785L17.1421 5.33363L15.7279 6.74785L17.1421 8.16206ZM7.24264 20.89H3V16.6473L16.435 3.21231C16.8256 2.82179 17.4587 2.82179 17.8492 3.21231L20.6777 6.04074C21.0682 6.43126 21.0682 7.06443 20.6777 7.45495L7.24264 20.89Z"></path></svg>
        }
        use dioxus::prelude::*;
        impl #struct_ident {
            pub fn rsx_creation_form(&mut self, oncreate: fn(dioxus::html::events::FormEvent)) -> Element {
                rsx! {
                    form {
                        class: "FormifyForm FormifyCreation",
                        onsubmit: oncreate,

                        div {
                            class: "FormifyInputs",
                            #inputs
                        }

                        button {
                            class: "FormifyButton",
                            r#type: "submit",
                            FormifyCreationIcon {}
                        }
                    }
                }
            }

            pub fn rsx_edit_form(&mut self, onedit: fn(dioxus::html::events::FormEvent)) -> Element {
                rsx! {
                    form {
                        class: "FormifyForm FormifyEdit",
                        onsubmit: onedit,

                        div {
                            class: "FormifyInputs",
                            #inputs
                        }

                        button {
                            class: "FormifyButton",
                            r#type: "submit",
                            FormifyEditIcon {}
                        }
                    }
                }

            }
        }
    }
}
