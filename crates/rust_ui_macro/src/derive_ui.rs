use std::{ cell::Cell, str::FromStr};

use proc_macro::{token_stream::IntoIter, Delimiter, Group, Ident, Punct, Spacing, Span, TokenStream, TokenTree};

pub(crate) enum UIClassification {
    Main,
    View,
    PureView,
}

impl UIClassification {
    pub fn from_ts(ts: TokenStream) -> Self {
        let mut iter = ts.into_iter();
        let s = match iter.next() {
            Some(TokenTree::Ident(ident)) if ident.to_string() == "main" => UIClassification::Main,
            None => UIClassification::View,
            e => panic!("Unexpected {:?}", e),
        };
        if let Some(next) = iter.next() {
            panic!("Unexpected {:?}", next);
        }
        s
    }
}
#[derive(Debug)]
pub struct StructInfo {
    pub name: Ident,
    pub fields: Vec<StructField>,
}
impl StructInfo {
    /// I.E. The view should never be rerendered
    pub fn is_pure(&self) -> bool {
        for field in self.fields.iter() {
            match field {
                StructField::State {
                    name: _,
                    initializer: _,
                    ty: _,
                } => return false,
                _ => continue,
            }
        }
        return true;
    }
}
#[derive(Debug)]
pub enum StructField {
    Body {
        initializer: TokenStream,
    },
    Field {
        name: Ident,
        ty: TokenStream,
        initializer: Option<TokenStream>,
    },
    Binding {
        name: Ident,
        ty: TokenStream,
    },
    State {
        name: Ident,
        initializer: Option<TokenStream>,
        ty: TokenStream,
    },
}

enum Decorator {
    None,
    Binding,
    State,
}

impl Decorator {
    pub fn from_ts(ts: TokenStream) -> Self {
        let mut iter = ts.into_iter();
        let s = match iter.next() {
            Some(TokenTree::Ident(ident)) => match ident.to_string().as_str() {
                "binding" => Self::Binding,
                "state" => Self::State,
                e => panic!("invalid field decorator {e}"),
            },
            e => panic!("Unexpected {:?}", e),
        };
        if let Some(next) = iter.next() {
            panic!("Unexpected {:?}", next);
        }
        s
    }
}

// pub fn

pub fn get_struct_info(item: TokenStream) -> StructInfo {
    //
    let mut iter = item.into_iter();
    let Some(TokenTree::Ident(span)) = iter.next() else {
        panic!("expected struct keyword")
    };
    assert!(span.to_string() == "struct");
    let Some(TokenTree::Ident(name)) = iter.next() else {
        panic!("expected name after struct")
    };

    let body = match iter.next() {
        Some(TokenTree::Punct(p)) if p.as_char() == '<' => {
            panic!("generics are not supported ui structs!")
        }
        Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Brace => g.stream(),
        _ => panic!("expected group"),
    };
    let mut fields = Vec::new();
    let mut fields_iter = body.into_iter();
    loop {
        let mut deco = Decorator::None;
        let field_name = match fields_iter.next() {
            Some(TokenTree::Ident(field_name)) => field_name,
            Some(TokenTree::Punct(p)) if p.as_char() == '#' => {
                //binding
                let deco_stream = match fields_iter.next() {
                    Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Bracket => g.stream(),
                    _ => panic!("invalid character"),
                };
                deco = Decorator::from_ts(deco_stream);

                let Some(TokenTree::Ident(ident)) = fields_iter.next() else {
                    panic!("expect field name")
                };

                ident
            }
            None => break,
            _ => panic!("invalid character"),
        };
        if field_name.to_string() != "body" {
            match fields_iter.next() {
                Some(TokenTree::Punct(p)) if p.as_char() == ':' => {}
                _ => panic!("expected type"),
            }
        }

        //next is the type decl. This would suck to parse. Luckily we can do a bit of skipping
        //we must treat <> as a group tho
        let mut type_stm = TokenStream::new();
        let mut init_stm = TokenStream::new();
        let mut generic_nest_depth = 0;
        let mut write_initializer = false;
        let mut writer = &mut type_stm;
        loop {
            let next = fields_iter.next();
            match next.clone() {
                None => break,
                Some(TokenTree::Punct(p)) => match p.as_char() {
                    //This break if using the turbofish syntax ::<> after =
                    ',' if generic_nest_depth == 0 || write_initializer => break,
                    '=' if generic_nest_depth == 0 && !write_initializer => {
                        //parse the initializer
                        write_initializer = true;
                        writer = &mut init_stm;
                        continue;
                    }
                    '<' => generic_nest_depth += 1,
                    '>' => generic_nest_depth -= 1,
                    _ => {}
                },
                _ => {}
            }
            if let Some(next) = next {
                writer.extend([next]);
            }
        }
        let field = match deco {
            Decorator::None if field_name.to_string() == "body" => {
                assert!(write_initializer);
                StructField::Body {
                    initializer: init_stm,
                }
            }
            Decorator::None => StructField::Field {
                name: field_name,
                ty: type_stm,
                initializer: write_initializer.then_some(init_stm),
            },
            Decorator::Binding => StructField::Binding {
                name: field_name,
                ty: type_stm,
            },
            Decorator::State => StructField::State {
                name: field_name,
                ty: type_stm,
                initializer: write_initializer.then_some(init_stm),
            },
        };
        fields.push(field);
    }

    StructInfo { name, fields }
}

fn translate_rust_ui_closure(function_tokens:&mut TokenStream,function_args:&mut TokenStream,iter:&mut IntoIter)->TokenStream{
     while let Some(t) = iter.next() {
        match t {
            TokenTree::Punct(p) if p.as_char() == ':' => panic!("rust ui closures may not contain type annotation"),
            TokenTree::Punct(p) if p.as_char() == '|' => {
                function_tokens.extend([TokenTree::Punct(p)]);
                break;
            },
            t=> {
                function_args.extend([t.clone()]);
                function_tokens.extend([t]) 
            }
        }
    }
    let g = match iter.next() {
        Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Brace => g,
        e=>panic!("50 {:?} {}",e,function_tokens)
    };
    let outer_function = function_tokens.clone();
    function_tokens.extend([TokenTree::Group(g)]);
    return outer_function

}

fn translate_rust_ui_close_with_data(inner_function:TokenStream,outer_function:TokenStream,data_ref_unpack:&TokenStream,func_args_inner:TokenStream)->TokenTree{
    TokenTree::Group(Group::new(Delimiter::Brace, {
                            let mut s = TokenStream::from_str("let data = data.clone(); move").unwrap();
                            s.extend(outer_function);
                            s.extend([TokenTree::Group(Group::new(Delimiter::Brace, {
                                let mut s = TokenStream::from_str("let data_ref = data.borrow(); let signal = ::std::cell::Cell::new(false);let queue = ::rust_ui::view::state::BindingQueue::default(); let res = ").unwrap();
                                let mut sub = TokenStream::new();
                                sub.extend(data_ref_unpack.clone());
                                // sub.extend(TokenStream::from_str("let res = ").unwrap());

                                sub.extend([
                                    TokenTree::Group(Group::new(Delimiter::Parenthesis, {
                                        let mut s = TokenStream::from_str("move").unwrap();
                                        s.extend(inner_function);
                                        s
                                    })),
                                    TokenTree::Group(Group::new(Delimiter::Parenthesis, func_args_inner)),
                                ]);
                                // sub.extend(TokenStream::from_str("; if signal.take() {::rust_ui::view::mutable::MutableViewRerender::rerender(&data);} res").unwrap());
                                s.extend([TokenTree::Group(Group::new(Delimiter::Brace, sub))]);
                                s.extend(TokenStream::from_str(";queue.execute();::std::mem::drop(data_ref); if signal.take() {::rust_ui::view::mutable::MutableViewRerender::rerender(&data);} res").unwrap());

                                s
                                
                            }))]);
                            s
                        }))
}

///returns true if there are child view present
fn translate_rust_ui_init_syntax_partial_init(writer:&mut TokenStream,input:TokenStream,data_ref_unpack:&TokenStream)->bool{
    // writer.extend([TokenTree::Group(Group::new(Delimiter::Brace, ))]);
    let mut fields = TokenStream::new();
    let mut children = TokenStream::new();

    let mut iter = input.into_iter();
    let mut parsing_fields = true;
    let mut started_children = false;

    while let Some(next) = iter.next() {
        let ident = match next  {
            TokenTree::Ident(i)=>i,
            TokenTree::Punct(p) if p.as_char() == '.' => {
                let attrib_name = match iter.next() {
                    Some(TokenTree::Ident(i))=>i,
                    Some(TokenTree::Punct(pt)) if pt.as_char() == '.' => {
                        //placer syntax
                        todo!("30");
                        fields.extend([
                            TokenTree::Punct(p),
                            TokenTree::Punct(pt),
                        ]);
                        while let Some(next) = iter.next() {
                            match next {
                                TokenTree::Punct(p) if p.as_char() == ',' => break,
                                token => fields.extend([token]),
                            }
                        }
                        fields.extend([TokenTree::Punct(Punct::new(',', Spacing::Alone))]);
                        
                        continue;
                    },
                    _=>panic!("14")
                };
                // let Some(TokenTree::Ident(attrib_name)) = iter.next() else {panic!("14")};
                match iter.next() {
                    Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Parenthesis => {
                        children.extend([
                            TokenTree::Punct(p),
                            TokenTree::Ident(attrib_name),
                            TokenTree::Group(g)
                        ]);
                    },
                    Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Brace => {
                        let mut body = TokenStream::new();
                        translate_rust_ui_init_syntax(&mut body, g.stream(), data_ref_unpack);
                        children.extend([
                            TokenTree::Punct(p),
                            TokenTree::Ident(attrib_name),
                            TokenTree::Group(Group::new(Delimiter::Parenthesis, body))
                        ]);
                    },
                    Some(TokenTree::Punct(p1)) if p1.as_char() == '|' => {
                        let mut inner_function = TokenStream::from_iter([TokenTree::Punct(p1)]);
                        let mut func_args_inner = TokenStream::new();
                        let outer_function =   translate_rust_ui_closure(&mut inner_function,&mut func_args_inner,&mut iter);
                        // let outer_function = inner_function.clone();
                        // println!("{}",&func_args_inner);
                        children.extend([
                            TokenTree::Punct(p),
                            TokenTree::Ident(attrib_name),
                            TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::from_iter([
                                translate_rust_ui_close_with_data(inner_function, outer_function, data_ref_unpack, func_args_inner),
                            ])))
                        ]);
                    }
                    _=> panic!("15")
                }
                continue;
            },
            //trailing callback modifier i.e.
            // Button("hello world") || {
            //    println!("pressed");
            // }
            TokenTree::Punct(p) if p.as_char() == '|' => {
                let mut inner_function = TokenStream::from_iter([TokenTree::Punct(p)]);
                let mut func_args_inner = TokenStream::new();
               
               
                let outer_function =  translate_rust_ui_closure(&mut inner_function,&mut func_args_inner,&mut iter);


                // let mut bracket_stream = TokenStream::new();
                children.extend([
                    TokenTree::Punct(Punct::new('.', Spacing::Alone)),
                    TokenTree::Ident(Ident::new("with_capture_callback", Span::call_site())),
                    TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::from_iter([
                        translate_rust_ui_close_with_data(inner_function, outer_function, data_ref_unpack, func_args_inner),
                    ])))
                    
                ]);
                // children.extend([TokenTree::Group(Group::new(Delimiter::Brace, bracket_stream))]);

                continue;
            }
            _=>panic!("16")
        };
        match iter.next() {
            Some(TokenTree::Group(g)) if g.delimiter() != Delimiter::Bracket => {
                if started_children {
                    children.extend([TokenTree::Punct(Punct::new(',', Spacing::Alone))]);
                }
                parsing_fields = false;
                started_children = true;
                //child
                translate_rust_ui_init_syntax_view(&mut children, ident, g,data_ref_unpack);
            }
            Some(TokenTree::Punct(p)) if p.as_char() == ':' && parsing_fields => {
                //fields
                fields.extend([
                    TokenTree::Ident(ident),
                    TokenTree::Punct(p),
                ]);
                while let Some(next) = iter.next() {
                    match next {
                        TokenTree::Punct(p) if p.as_char() == ',' => break,
                        token => fields.extend([token]),
                    }
                }
                fields.extend([TokenTree::Punct(Punct::new(',', Spacing::Alone))]);
            }
            _=>panic!("unexpected")
        }
    }

    if started_children {
        children.extend([TokenTree::Punct(Punct::new(',', Spacing::Alone))]);
        fields.extend(TokenStream::from_str("children: ::std::option::Option::Some"));
        fields.extend([TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::from_iter([
            TokenTree::Group(Group::new(Delimiter::Parenthesis, children)),
        ]))),
         TokenTree::Punct(Punct::new(',', Spacing::Alone))
        ]);
    }
    fields.extend(TokenStream::from_str("..Default::default()"));
    writer.extend([
        TokenTree::Group(Group::new(Delimiter::Brace, fields))
    ]);
    return started_children;

}

fn translate_rust_ui_init_syntax_view(writer:&mut TokenStream,name:Ident,group:Group,data_ref_unpack:&TokenStream){
    match group.delimiter() {
        Delimiter::Parenthesis => {
            writer.extend([
                TokenTree::Ident(name.clone()),
                TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                TokenTree::Ident(Ident::new("new", Span::call_site())),
                TokenTree::Group(Group::new(Delimiter::Parenthesis, {
                    let mut s = TokenStream::from_str("#[allow(unused_parens)]").unwrap();
                    s.extend([TokenTree::Group(group),]);
                    s
                }))
            ]);
        },
        Delimiter::Brace => {
            writer.extend([
                TokenTree::Ident(name.clone()),
                TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                TokenTree::Ident(Ident::new("new", Span::call_site())),
                TokenTree::Group(Group::new(Delimiter::Parenthesis, {
                    let mut s = TokenStream::from_iter([
                        TokenTree::Punct(Punct::new('<', Spacing::Alone)),
                        TokenTree::Ident(name.clone()),
                    ]);
                    let mut temp_stream = TokenStream::new();
                    if translate_rust_ui_init_syntax_partial_init(&mut temp_stream,group.stream(),data_ref_unpack) {
                        s.extend(TokenStream::from_str("<_> as ::rust_ui::PartialInitialisable>::PartialInit"));
                    }else {
                        s.extend(TokenStream::from_str(" as ::rust_ui::PartialInitialisable>::PartialInit"));
                    }
                    s.extend(temp_stream);
                    s
                }))
            ]);
        },
        e=>panic!("unexpected {:?}",e)
    }
}


fn translate_rust_ui_init_syntax(writer:&mut TokenStream,input:TokenStream,data_ref_unpack:&TokenStream){
    //a top level item is either:
    // - a single view
    // - or a single view wrapped around {}
    let mut iter = input.into_iter();
    match iter.next() {
        Some(TokenTree::Ident(name)) => {
            // assert!(name.to_string().chars().next().unwrap().is_lowercase()
            // <name as ::rust_ui::PartialInitialisable>::PartialInit
            //next token must be: () or {}
            let Some(TokenTree::Group(group)) = iter.next() else {panic!("13")};
            translate_rust_ui_init_syntax_view(writer,name,group,data_ref_unpack);
            // writer.extend(iter);
            //we might have some . or || properties we need to handle
            while let Some(next) = iter.next() {
                match next {
                    TokenTree::Punct(p) if p.as_char() == '|' => {
                        let mut inner_function = TokenStream::from_iter([TokenTree::Punct(p)]);
                        let mut func_args_inner = TokenStream::new();
                    
                    
                        let outer_function =  translate_rust_ui_closure(&mut inner_function,&mut func_args_inner,&mut iter);


                        // let mut bracket_stream = TokenStream::new();
                        writer.extend([
                            TokenTree::Punct(Punct::new('.', Spacing::Alone)),
                            TokenTree::Ident(Ident::new("with_capture_callback", Span::call_site())),
                            TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::from_iter([
                                translate_rust_ui_close_with_data(inner_function, outer_function, data_ref_unpack, func_args_inner),
                            ])))
                            
                        ]);

                    },
                    TokenTree::Punct(p) if p.as_char() == '.' => {
                        let Some(TokenTree::Ident(ident)) = iter.next() else {continue};
                        match iter.next() {
                            Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Parenthesis => {
                                writer.extend([
                                    TokenTree::Punct(p),
                                    TokenTree::Ident(ident),
                                    TokenTree::Group(g)
                                ]);
                            }
                            Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Brace => {
                                let mut body = TokenStream::new();
                                translate_rust_ui_init_syntax(&mut body, g.stream(), data_ref_unpack);
                                writer.extend([
                                    TokenTree::Punct(p),
                                    TokenTree::Ident(ident),
                                    TokenTree::Group(Group::new(Delimiter::Parenthesis, body))
                                ]);
                            }
                            Some(TokenTree::Punct(p)) if p.as_char() == '|' => {
                                let mut inner_function = TokenStream::from_iter([TokenTree::Punct(p)]);
                                let mut func_args_inner = TokenStream::new();
                            
                            
                                let outer_function =  translate_rust_ui_closure(&mut inner_function,&mut func_args_inner,&mut iter);
                                writer.extend([
                                    TokenTree::Punct(Punct::new('.', Spacing::Alone)),
                                    TokenTree::Ident(ident),
                                    TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::from_iter([
                                        translate_rust_ui_close_with_data(inner_function, outer_function, data_ref_unpack, func_args_inner),
                                    ])))
                                    
                                ]);
                            }
                            _=>continue
                        }
                        
                    }
                    _=>{}
                }
            }
            
        },
        Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Brace => {
            let mut stream = TokenStream::new();
            translate_rust_ui_init_syntax(&mut stream,g.stream(),data_ref_unpack);
            // stream.extend(iter);

            writer.extend([
                TokenTree::Group(Group::new(Delimiter::Brace, stream))
            ]);
        },
        _ => panic!("unexpected"),
    }
}

pub fn create_normalized_struct_mutable_view(writer: &mut TokenStream, info: &StructInfo) {
    macro_rules! ident {
        ($x:tt) => {
            TokenTree::Ident(Ident::new($x, Span::call_site()))
        };
    }
    macro_rules! punct {
        ($x:tt) => {
            TokenTree::Punct(Punct::new($x, Spacing::Alone))
        };
    }

    let mut body = TokenStream::new();

    let mut partial_init_body = TokenStream::new();
    let mut init_fn_body = TokenStream::new();

    let mut children_fn_body_unwrapped = TokenStream::new();
    let mut children_fn_body_final_part = TokenStream::new();

    children_fn_body_unwrapped.extend(TokenStream::from_str("let data_ref = data.borrow();").unwrap());

    let mut data_ref_unpack = TokenStream::new();
    

    for field in info.fields.iter() {
        match field {
            StructField::Body { initializer } => {
                //
                let mut iter = initializer.clone().into_iter();
                let Some(TokenTree::Ident(ident)) = iter.next() else {panic!("20")};
                let Some(TokenTree::Punct(p)) = iter.next() else {panic!("21")};
                assert!(ident.to_string()=="view");
                assert!(p.as_char()=='!');
                translate_rust_ui_init_syntax(&mut children_fn_body_final_part, TokenStream::from_iter(iter),&data_ref_unpack);
            }
            StructField::Field {
                name,
                ty,
                initializer,
            } => {
                children_fn_body_unwrapped.extend([
                    ident!("let"),
                    TokenTree::Ident(name.clone()),
                    punct!('='),
                    ident!("data_ref"),
                    punct!('.'),
                    TokenTree::Ident(name.clone()),
                    punct!(';')
                ]);
                data_ref_unpack.extend([
                    ident!("let"),
                    TokenTree::Ident(name.clone()),
                    punct!('='),
                    ident!("data_ref"),
                    punct!('.'),
                    TokenTree::Ident(name.clone()),
                    punct!(';')
                ]);
                body.extend([TokenTree::Ident(name.clone()), punct!(':')]);
                body.extend(ty.clone());
                body.extend([punct!(',')]);
                partial_init_body.extend([TokenTree::Ident(name.clone()), punct!(':')]);

                if initializer.is_some() {
                    partial_init_body
                        .extend(TokenStream::from_str("::std::option::Option<").unwrap());
                    partial_init_body.extend(ty.clone());
                    partial_init_body.extend([punct!('>'), punct!(',')]);
                } else {
                    partial_init_body.extend(ty.clone());
                    partial_init_body.extend([punct!(',')]);
                }
                // ::std::option::Option::<i32>::unwrap_or_else(self, f)

                if let Some(i) = initializer {
                    init_fn_body.extend([TokenTree::Ident(name.clone()), punct!(':')]);
                    init_fn_body.extend(TokenStream::from_str("::std::option::Option::<").unwrap());
                    init_fn_body.extend(ty.clone());
                    init_fn_body.extend(TokenStream::from_str(">::unwrap_or_else").unwrap());
                    init_fn_body.extend([TokenTree::Group(Group::new(Delimiter::Parenthesis, {
                        let mut s = TokenStream::from_iter([
                            ident!("self"),
                            punct!('.'),
                            TokenTree::Ident(name.clone()),
                            punct!(','),
                            punct!('|'),
                            punct!('|'),
                        ]);
                        s.extend(i.clone());
                        s
                    }))]);
                } else {
                    init_fn_body.extend([
                        TokenTree::Ident(name.clone()),
                        punct!(':'),
                        ident!("self"),
                        punct!('.'),
                        TokenTree::Ident(name.clone()),
                    ]);
                }
                init_fn_body.extend([punct!(',')]);

            }
            StructField::Binding { name, ty } => {
                body.extend([TokenTree::Ident(name.clone()), punct!(':')]);
                body.extend(TokenStream::from_str("::rust_ui::view::state::PartialBinding<").unwrap());
                body.extend(ty.clone());
                body.extend([punct!('>'),punct!(',')]);

                partial_init_body.extend([TokenTree::Ident(name.clone()), punct!(':')]);
                partial_init_body.extend(TokenStream::from_str("::rust_ui::view::state::PartialBinding<").unwrap());
                partial_init_body.extend(ty.clone());
                partial_init_body.extend([punct!('>'),punct!(',')]);

                init_fn_body.extend([
                    TokenTree::Ident(name.clone()),
                    punct!(':'),
                    ident!("self"),
                    punct!('.'),
                    TokenTree::Ident(name.clone()),
                ]);
                init_fn_body.extend([punct!(',')]);

                children_fn_body_unwrapped.extend([
                    ident!("let"),
                    TokenTree::Ident(name.clone()),
                    punct!('='),
                    punct!('&'),
                    ident!("data_ref"),
                    punct!('.'),
                    TokenTree::Ident(name.clone()),
                    punct!(';')
                ]);

                data_ref_unpack.extend([
                    ident!("let"),
                    ident!("mut"),
                    TokenTree::Ident(name.clone()),
                    punct!('='),
                    // punct!('&'),
                    ident!("data_ref"),
                    punct!('.'),
                    TokenTree::Ident(name.clone()),
                    // punct!(';')
                ]);
                data_ref_unpack.extend(TokenStream::from_str(".as_binding(&queue);").unwrap());


            }
            StructField::State {
                name,
                initializer,
                ty,
            } => {
                body.extend([TokenTree::Ident(name.clone()), punct!(':')]);
                body.extend(TokenStream::from_str("::rust_ui::view::state::PartialState<").unwrap());
                body.extend(ty.clone());
                body.extend([punct!('>'),punct!(',')]);
                partial_init_body.extend([TokenTree::Ident(name.clone()), punct!(':')]);
                if initializer.is_some() {
                    partial_init_body
                        .extend(TokenStream::from_str("::std::option::Option<").unwrap());
                    partial_init_body.extend(ty.clone());
                    partial_init_body.extend([punct!('>'), punct!(',')]);
                } else {
                    partial_init_body.extend(ty.clone());
                    partial_init_body.extend([punct!(',')]);
                }
                // println!("encounted state lol");
                // println!("pappap: {}",partial_init_body.clone());

                if let Some(i) = initializer {
                    init_fn_body.extend([TokenTree::Ident(name.clone()), punct!(':')]);
                    init_fn_body.extend(TokenStream::from_str("::std::option::Option::<").unwrap());
                    init_fn_body.extend(ty.clone());
                    init_fn_body.extend(TokenStream::from_str(">::unwrap_or_else").unwrap());
                    init_fn_body.extend([TokenTree::Group(Group::new(Delimiter::Parenthesis, {
                        let mut s = TokenStream::from_iter([
                            ident!("self"),
                            punct!('.'),
                            TokenTree::Ident(name.clone()),
                            punct!(','),
                            punct!('|'),
                            punct!('|'),
                        ]);
                        s.extend(i.clone());
                        s
                    }))]);
                    init_fn_body.extend(TokenStream::from_str(".into()").unwrap());


                } else {
                    init_fn_body.extend([
                        TokenTree::Ident(name.clone()),
                        punct!(':'),
                        ident!("self"),
                        punct!('.'),
                        TokenTree::Ident(name.clone()),
                    ]);
                    init_fn_body.extend(TokenStream::from_str(".into()").unwrap());
                }
                init_fn_body.extend([punct!(',')]);
                children_fn_body_unwrapped.extend([
                    ident!("let"),
                    TokenTree::Ident(name.clone()),
                    punct!('='),
                    punct!('&'),
                    ident!("data_ref"),
                    punct!('.'),
                    TokenTree::Ident(name.clone()),
                    punct!(';'),

                ]);
                // children_fn_body_unwrapped.extend(TokenStream::from_str(".as_state(&signal);").unwrap());

                data_ref_unpack.extend([
                    ident!("let"),
                    ident!("mut"),
                    TokenTree::Ident(name.clone()),
                    punct!('='),
                    ident!("data_ref"),
                    punct!('.'),
                    TokenTree::Ident(name.clone()),

                ]);
                data_ref_unpack.extend(TokenStream::from_str(".as_state(&signal);").unwrap());

            }
        }
    }

    body.extend(TokenStream::from_str(
        "view: ::std::option::Option<::std::rc::Rc<::std::cell::RefCell<::rust_ui::native::MutableView>>>",
    ));
    init_fn_body.extend(TokenStream::from_str(
        "view: ::std::option::Option::<::std::rc::Rc<::std::cell::RefCell<::rust_ui::native::MutableView>>>::None",
    ));

    writer.extend([
        TokenTree::Ident(Ident::new("struct", Span::call_site())),
        TokenTree::Ident(info.name.clone()),
        TokenTree::Group(Group::new(Delimiter::Brace, body)),
    ]);
    let mut partial_name = info.name.to_string();
    partial_name.push_str("PartialInit");
    writer.extend([
        punct!('#'),
        TokenTree::Group(Group::new(Delimiter::Bracket, TokenStream::from_str("derive(Default)").unwrap())),
        TokenTree::Ident(Ident::new("struct", Span::call_site())),
        TokenTree::Ident(Ident::new(partial_name.as_str(), info.name.span())),
        TokenTree::Group(Group::new(Delimiter::Brace, partial_init_body)),
    ]);

    let mut partial_init_trait_name = info.name.to_string();
    partial_init_trait_name.push_str("Initializer");
    let partial_init_trait_ident =
        TokenTree::Ident(Ident::new(&partial_init_trait_name, info.name.span()));
    // let k = 10;
    // ToString::to_string(&k);

    writer.extend([
        ident!("impl"),
        TokenTree::Ident(info.name.clone()),
        TokenTree::Group(Group::new(
            Delimiter::Brace,
            TokenStream::from_iter([
                ident!("pub"),
                ident!("fn"),
                ident!("new"),
                TokenTree::Group(Group::new(
                    Delimiter::Parenthesis,
                    TokenStream::from_iter([
                        ident!("initializer"),
                        punct!(':'),
                        ident!("impl"),
                        partial_init_trait_ident.clone(),
                    ]),
                )),
                TokenTree::Punct(Punct::new('-', Spacing::Joint)),
                punct!('>'),
                TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                punct!(':'),
                ident!("std"),
                TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                punct!(':'),
                ident!("rc"),
                TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                punct!(':'),
                ident!("Rc"),
                punct!('<'),
                    TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                    punct!(':'),
                    ident!("std"),
                    TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                    punct!(':'),
                    ident!("cell"),
                    TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                    punct!(':'),
                    ident!("RefCell"),
                    punct!('<'),
                        ident!("Self"),
                    punct!('>'),

                punct!('>'),

                TokenTree::Group(Group::new(
                    Delimiter::Brace,
                    TokenStream::from_iter([
                        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                        punct!(':'),
                        ident!("std"),
                        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                        punct!(':'),
                        ident!("rc"),
                        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                        punct!(':'),
                        ident!("Rc"),
                        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                        punct!(':'),
                        ident!("new"),
                        TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::from_iter([
                            TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                            punct!(':'),
                            ident!("std"),
                            TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                            punct!(':'),
                            ident!("cell"),
                            TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                            punct!(':'),
                            ident!("RefCell"),
                            TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                            punct!(':'),
                            ident!("new"),
                            TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::from_iter([
                                 partial_init_trait_ident.clone(),
                        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                        punct!(':'),
                        ident!("init"),
                        TokenTree::Group(Group::new(
                            Delimiter::Parenthesis,
                            TokenStream::from_str("initializer").unwrap(),
                        )),
                            ])))
                        ])))

                        // partial_init_trait_ident.clone(),
                        // TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                        // punct!(':'),
                        // ident!("init"),
                        // TokenTree::Group(Group::new(
                        //     Delimiter::Parenthesis,
                        //     TokenStream::from_str("initializer").unwrap(),
                        // )),
                    ]),
                )),
            ]),
        )),
    ]);

    writer.extend([
        ident!("trait"),
        partial_init_trait_ident.clone(),
        TokenTree::Group(Group::new(
            Delimiter::Brace,
            TokenStream::from_iter([
                ident!("fn"),
                ident!("init"),
                TokenTree::Group(Group::new(
                    Delimiter::Parenthesis,
                    TokenStream::from_iter([ident!("self")]),
                )),
                TokenTree::Punct(Punct::new('-', Spacing::Joint)),
                punct!('>'),
                TokenTree::Ident(info.name.clone()),
                punct!(';')
            ]),
        )),
    ]);

    writer.extend([
        ident!("impl"),
        partial_init_trait_ident,
        ident!("for"),
        TokenTree::Ident(Ident::new(&partial_name, info.name.span())),
        TokenTree::Group(Group::new(
            Delimiter::Brace,
            TokenStream::from_iter([
                ident!("fn"),
                ident!("init"),
                TokenTree::Group(Group::new(
                    Delimiter::Parenthesis,
                    TokenStream::from_iter([ident!("self")]),
                )),
                TokenTree::Punct(Punct::new('-', Spacing::Joint)),
                punct!('>'),
                TokenTree::Ident(info.name.clone()),
                TokenTree::Group(Group::new(Delimiter::Brace, TokenStream::from_iter([
                    TokenTree::Ident(info.name.clone()),
                    TokenTree::Group(Group::new(Delimiter::Brace, init_fn_body))
                ]))),
            ]),
        )),
    ]);

    writer.extend([
        ident!("impl"),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        punct!(':'),
        ident!("rust_ui"),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        punct!(':'),
        ident!("PartialInitialisable"),
        ident!("for"),
        TokenTree::Ident(info.name.clone()),
        TokenTree::Group(Group::new(Delimiter::Brace, TokenStream::from_iter([
            ident!("type"),
            ident!("PartialInit"),
            punct!('='),
            TokenTree::Ident(Ident::new(&partial_name, info.name.span())),
            punct!(';'),

        ])))
    ]);
    //Inject the bind! macro. This method is type safe :) yeah
    children_fn_body_unwrapped.extend(TokenStream::from_str("macro_rules! bind {
    ($state:ident) => {
        ::rust_ui::view::state::PartialState::as_binding($state,data.clone())
    };
};").unwrap());

    children_fn_body_unwrapped.extend(children_fn_body_final_part);

    writer.extend([
        ident!("impl"),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        punct!(':'),
        ident!("rust_ui"),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        punct!(':'),
        ident!("view"),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        punct!(':'),
        ident!("mutable"),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        punct!(':'),
        ident!("MutableView"),
        ident!("for"),
        TokenTree::Ident(info.name.clone()),
        TokenTree::Group(Group::new(Delimiter::Brace, {
            let mut s  = TokenStream::new();
            s.extend(TokenStream::from_str("#[allow(unused_macros)] fn children(data: ::std::rc::Rc<::std::cell::RefCell<Self>>) -> impl ::rust_ui::layout::RenderObject + 'static").unwrap());
            s.extend([
                TokenTree::Group(Group::new(Delimiter::Brace, children_fn_body_unwrapped))
            ]);
            s.extend(TokenStream::from_str("fn get_attached(&self) -> &::std::option::Option<::std::rc::Rc<::std::cell::RefCell<::rust_ui::native::MutableView>>> {
        &self.view
    }

    fn get_mut_attached(&mut self) -> &mut ::std::option::Option<::std::rc::Rc<::std::cell::RefCell<::rust_ui::native::MutableView>>> {
         &mut self.view
    }"));
            s
        }))
    ]);
}



/*
struct Syntax:
- no generics allowed for now
- field initialisizer syntax

#[ui]
struct RootView {
    #[state] counter: i32 = 0
}

creates:


pub struct RootView {
    counter: i32,
    view: ...
}

#[derive(Default)]
pub struct RootViewPartialInit {
    counter:Option<i32>
}

impl RootView {
    pub type PartialInit = RootViewPartialInit

}


*/
