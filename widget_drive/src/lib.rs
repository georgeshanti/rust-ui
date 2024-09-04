extern crate proc_macro;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::Field;

#[proc_macro_derive(Widget, attributes(props))]
pub fn derive_widget(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    // Build the trait implementation
    impl_widget_macro(&ast)
}

fn impl_widget_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let struct_: &syn::DataStruct = match &ast.data {
        syn::Data::Struct(data) => data,
        _ => panic!("Usage of #[Modbus] on a non-struct type"),
    };
    println!("Hello");
    let v: Vec<&str> = struct_.fields
        .iter()
        .filter_map(|field| {
            for attr in field.attrs.iter() {
                // println!("attr: \"{attr}\"");
                // let x: TokenStream = attr..clone().into();
                // dbg!(attr.clone());

                //let _input = parse_macro_input!(x);
                for segment in attr.path().segments.iter() {
                    if segment.ident.to_string() == String::from("holding_register") {
                        let s = attr.into_token_stream();
                        println!("segment: \"{s}\"")
                        // return Some(RegisterMapping {
                        //     ident: field.clone().ident.unwrap().clone(),
                        //     address: 32,
                        //     count: 16,
                        // });
                    };
                }
            }

            return Some("");
        })
        .collect();
    let gen = quote! {
        impl Widget for #name {
    
            fn add_renderer(self: &mut Self, renderer: Renderer) {
                self.renderer = Some(renderer);
            }
    
            fn draw(self: &mut Self) -> Element {
                self.build().draw()
            }
        
            fn update_props(self: &mut Self, props: &dyn Any) {
                self.props = *props.downcast_ref::<AppProps>().expect("msg")
            }
    
            fn render(self: &mut Self) {
                match self.renderer {
                    Some(renderer) => renderer(self),
                    _ => (),
                }
            }
        }

        impl<'a> ProppedWidgetDefaults<AppProps> for App {
            fn get_props(self: &Self) -> AppProps {
                self.props
            }
        }
    };
    gen.into()
}
