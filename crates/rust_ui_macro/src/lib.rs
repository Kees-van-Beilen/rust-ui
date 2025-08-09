use std::str::FromStr;

use proc_macro::{Delimiter, Group, Ident, Punct, Spacing, Span, TokenStream, TokenTree};

use crate::derive_ui::{StructInfo, UIClassification};

mod derive_ui;


fn write_main(out:&mut TokenStream,struct_info:&StructInfo){
out.extend([
                TokenTree::Ident(Ident::new("fn", Span::call_site())),
                TokenTree::Ident(Ident::new("main", Span::call_site())),
                TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::new())),
                TokenTree::Group(Group::new(Delimiter::Brace, TokenStream::from_iter([
                    TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                    TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                    TokenTree::Ident(Ident::new("rust_ui", Span::call_site())),
                    TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                    TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                    TokenTree::Ident(Ident::new("native", Span::call_site())),
                    TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                    TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                    TokenTree::Ident(Ident::new("launch_application_with_view", Span::call_site())),
                    TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::from_iter([
                        TokenTree::Ident(struct_info.name.clone()),
                        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                        TokenTree::Ident(Ident::new("new", Span::call_site())),
                        TokenTree::Group(Group::new(Delimiter::Parenthesis, {
                            let mut s = TokenStream::from_str("<<").unwrap();
                            s.extend([
                                TokenTree::Ident(struct_info.name.clone()),
                            ]);
                            s.extend( TokenStream::from_str(" as ::rust_ui::PartialInitialisable>::PartialInit as ::std::default::Default>::default()").unwrap());
                            s
                        }))
                    ])))
                ])))
            ]);
}
#[proc_macro_attribute]
pub fn ui(attr:TokenStream,item:TokenStream)->TokenStream{
    let cls = UIClassification::from_ts(attr.clone());

    // println!("habibibibi");

    match cls {
        UIClassification::Main => {
            let mut out = TokenStream::new();
            let struct_info = derive_ui::get_struct_info(item.clone());
            // panic!("adfd {:?}",&struct_info);
            // dbg!(&struct_info);
            // out.extend(TokenStream::from_str("fn main(){ rust_ui::native::launch_application_with_view() }").unwrap());
            derive_ui::create_normalized_struct_mutable_view(&mut out, &struct_info);
            write_main(&mut out,&struct_info);
            out

        }
        UIClassification::View => {
            let mut out = TokenStream::new();
            let struct_info = derive_ui::get_struct_info(item.clone());
             derive_ui::create_normalized_struct_mutable_view(&mut out, &struct_info);
            out
        },
        UIClassification::PureView => todo!(),
    }
}
/// Usage:
/// 
/// ```
/// ui_decl!(
///     ui(main) struct RootView {
///         body = {
///         
///         }
///     }
/// )
/// ```
#[proc_macro]
pub fn ui_decl(ts:TokenStream)->TokenStream{

    let mut iter = ts.into_iter();

    match iter.next() {
        Some(TokenTree::Ident(i)) if i.to_string() == "ui" => {},
        _=>panic!("expected: `ui(main)` or `ui`")
    }

    let mut item = TokenStream::from_iter(iter.clone());

    let attr = match iter.next() {
        Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Parenthesis => {
            item = TokenStream::from_iter(iter);
            g.stream()
        }
        _=>TokenStream::new()
    };


    let cls = UIClassification::from_ts(attr);

    // println!("habibibibi");
    

    match cls {
        UIClassification::Main => {
            let mut out = TokenStream::new();
            let struct_info = derive_ui::get_struct_info(item.clone());
            // out.extend(item);
            // dbg!(&struct_info);
            // out.extend(TokenStream::from_str("fn main(){ rust_ui::native::launch_application_with_view() }").unwrap());
            derive_ui::create_normalized_struct_mutable_view(&mut out, &struct_info);
            write_main(&mut out,&struct_info);

            //creare a struct __StructName__PartialInit
            out

        }
        UIClassification::View => todo!(),
        UIClassification::PureView => todo!(),
    }
}