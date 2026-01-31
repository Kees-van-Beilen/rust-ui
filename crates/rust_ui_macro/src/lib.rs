use std::{
    fs::File,
    io::{BufReader, Read},
    str::FromStr,
};

use proc_macro::{Delimiter, Group, Ident, Punct, Spacing, Span, TokenStream, TokenTree};

use crate::derive_ui::{StructInfo, UIClassification};

mod derive_ui;

fn manifest_android_main_activity() -> Option<String> {
    // dbg!(std::env::var("CARGO_MANIFEST_PATH"));
    let mut reader = BufReader::new(File::open(std::env::var("CARGO_MANIFEST_PATH").ok()?).ok()?);
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer).unwrap(); //io error should be reflected in compile
    let value = toml::from_str::<toml::Value>(&buffer).ok()?;
    let activity = value
        .get("package")?
        .get("metadata")?
        .get("rust-ui")?
        .get("android")?
        .get("main_activity_class_entry_method")?
        .as_str()?;
    Some(activity.to_string())
    // package.metadata.rust-ui.android
    // main_activity_class = "com.example.myapplication.MainActivity"
}

fn write_main_with(
    out: &mut TokenStream,
    name: &str,
    after_name: TokenStream,
    arguments: &Vec<(Ident, TokenStream)>,
    struct_info: &StructInfo,
) {
    out.extend([
        TokenTree::Ident(Ident::new("fn", Span::call_site())),
        TokenTree::Ident(Ident::new(name, Span::call_site())),
    ]);
    out.extend(after_name);
    out.extend([
                TokenTree::Group(Group::new(Delimiter::Parenthesis, {
                    let mut a = TokenStream::new();
                    arguments.iter().map(|(name,ty)|{
                        let mut ts = TokenStream::from_iter([TokenTree::Ident(name.clone()),TokenTree::Punct(Punct::new(':', Spacing::Alone))]);
                        ts.extend(ty.clone().into_iter());
                        ts.extend([TokenTree::Punct(Punct::new(',', Spacing::Alone))]);
                        ts
                    }).for_each(|e|a.extend(e));
                    a
                })),
                TokenTree::Group(Group::new(Delimiter::Brace, {let mut s = TokenStream::from_iter([
                    TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                    TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                    TokenTree::Ident(Ident::new("rust_ui", Span::call_site())),
                    TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                    TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                    TokenTree::Ident(Ident::new("native", Span::call_site())),
                    TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                    TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                    TokenTree::Ident(Ident::new("launch_application_with_view", Span::call_site())), 
                ]);
                s.extend([TokenTree::Group(Group::new(Delimiter::Parenthesis, {
                        let mut s = TokenStream::from_iter([
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
                                })),
                                TokenTree::Punct(Punct::new(',', Spacing::Alone)),
                            ]);
                            arguments.iter().map(|(name,_)|{
                                let ts = TokenStream::from_iter([TokenTree::Ident(name.clone()),TokenTree::Punct(Punct::new(',', Spacing::Alone))]);
                                ts
                            }).for_each(|e|s.extend(e));
                            s
                        }
                    ))]);
                s}))
            ]);
}

fn write_main(out: &mut TokenStream, struct_info: &StructInfo) {
    if let Some(main_method) = manifest_android_main_activity() {
        out.extend(TokenStream::from_str(
            "#[cfg(target_os = \"android\")]\n#[unsafe(no_mangle)]\n",
        ));
        let export_main = format!("Java_{}", main_method.replace(".", "_"));
        //pub extern "system"
        out.extend(TokenStream::from_str("pub extern \"system\""));
        write_main_with(
            out,
            &export_main,
            TokenStream::from_str("<'local>").unwrap(),
            &vec![
                (
                    Ident::new("env", Span::call_site()),
                    TokenStream::from_str("::rust_ui::native::jni::JNIEnv<'local>").unwrap(),
                ),
                // instance: jni::objects::JObject<'local>,
                (
                    Ident::new("instance", Span::call_site()),
                    TokenStream::from_str("::rust_ui::native::jni::objects::JObject<'local>")
                        .unwrap(),
                ),
            ],
            struct_info,
        );
        // target_os = "android"
    } else {
        out.extend(TokenStream::from_str("#[cfg(target_os = \"android\")]\ncompile_error!(\"To compile for android add the following lines to your Cargo.toml:\\n[package.metadata.rust-ui.android]\\nmain_activity_class_entry_method = \\\"com.example.app.MainActivity.mainEntry\\\"\");"));
    }
    // dbg!();
    // dbg!(env!("CARGO_MANIFEST_DIR"));

    out.extend(TokenStream::from_str(
        "#[cfg(not(target_os = \"android\"))]",
    ));
    write_main_with(out, "main", TokenStream::new(), &vec![], struct_info);

    out.extend(TokenStream::from_str(
        "#[cfg(target_os = \"android\")] fn main(){/*boiler plate*/}",
    ));
}
#[proc_macro_attribute]
pub fn ui(attr: TokenStream, item: TokenStream) -> TokenStream {
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
            write_main(&mut out, &struct_info);
            out
        }
        UIClassification::View => {
            let mut out = TokenStream::new();
            let struct_info = derive_ui::get_struct_info(item.clone());
            derive_ui::create_normalized_struct_mutable_view(&mut out, &struct_info);
            out
        }
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
pub fn ui_decl(ts: TokenStream) -> TokenStream {
    let mut iter = ts.into_iter();

    match iter.next() {
        Some(TokenTree::Ident(i)) if i.to_string() == "ui" => {}
        _ => panic!("expected: `ui(main)` or `ui`"),
    }

    let mut item = TokenStream::from_iter(iter.clone());

    let attr = match iter.next() {
        Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Parenthesis => {
            item = TokenStream::from_iter(iter);
            g.stream()
        }
        _ => TokenStream::new(),
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
            write_main(&mut out, &struct_info);

            //creare a struct __StructName__PartialInit
            out
        }
        UIClassification::View => todo!(),
        UIClassification::PureView => todo!(),
    }
}
