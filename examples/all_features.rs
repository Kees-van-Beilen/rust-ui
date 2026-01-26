#![feature(more_qualified_paths, default_field_values)]
use rust_ui::{PartialInitialisable, prelude::*, view::dyn_render::{DynGroup, DynInstance}, views::textfield::TextEditor};


macro_rules! create {
    ($name:ty) => {
        DynGroup::new(<$name>::new(<$name as PartialInitialisable>::PartialInit::default()))
    };
}


#[ui(main)]
pub struct RootView {

    views: Vec<Vec<(&'static str,&'static str,DynGroup)>> = vec![
        vec![
            ("Text","View",create!(ExampleText)),
            ("Text","foreground color",create!(ExampleTextColor)),
            ("Text","font size",create!(ExampleTextSize)),
            ("Text","font weight",create!(ExampleTextWeight)),
        ],

        vec![
            ("Button","View",create!(ExampleButton)),
            ("Button","foreground color",create!(ExampleButtonTextColor)),
            ("Button","font size",create!(ExampleButtonTextSize)),
            ("Button","font weight",create!(ExampleButtonTextWeight)),
        ],

        vec![
            ("Image","fit",create!(ExampleImageFit)),
            ("Image","fill",create!(ExampleImageFill)),
        ],
        vec![
            ("Color","View",create!(ExampleColorView)),
        ],
        vec![
            ("TextField","View",create!(ExampleInput)),
            ("TextField","foreground color",create!(ExampleInputTextColor)),
            ("TextField","font size",create!(ExampleInputTextSize)),
            ("TextField","font weight",create!(ExampleInputTextWeight)),
        ],
        vec![
            ("TextEditor","View",create!(ExampleTextEditor)),
        ],
        vec![
            ("ScrollView","(nested) Horizontal",create!(ExampleScrollViewHorizontal)),
            ("ScrollView","Vertical",create!(ExampleScrollViewVertical)),
        ],

        vec![
            ("Example","Counter",create!(ExampleCounter)),
            ("Example","Counter (custom button)",create!(ExampleCounterOnTap)),
        ],

        
    ],

    body:_ = view!{
        ScrollView {
            y: Some(ScrollBehavior::Scroll),
            VStack {
                spacing:Some(15.0*DPI),
                for category in views.iter() {
                    VStack {
                        spacing:Some(2.0*DPI),
                        for (name,sub,view) in category.iter() {
                            HStack {
                                spacing:Some(10.0*DPI),
                                VStack {
                                    spacing:Some(10.0*DPI),
                                    Spacer()
                                    Text(name)
                                        .title()
                                    Text(sub)
                                        .foreground_color(Color::oklch(0.79, 0.0, 332.47))
                                    Spacer()
                                    
                                }.frame(Frame::no_preference().width(140.0*DPI))
                                    .background{ColorView(Color::oklch(0.18, 0.0, 332.47))}
                                ColorView(Color::WHITE).frame(Frame::no_preference().width(2.0*DPI))
                                DynInstance(view).frame(Frame::no_preference().height(120.0*DPI))
                            }
                        }

                    }
                    
                }
                Spacer().frame(Frame::no_preference().height(200.0*DPI))
            }

        }.background{ColorView(Color::BLACK)}.foreground_color(Color::WHITE)
    }
}

#[ui] pub struct ExampleText {
    body:_ = view!{
        HStack {
            Spacer()
            Text("Hello world")
            Spacer()
        }
    }
}
#[ui] pub struct ExampleTextColor {
    body:_ = view!{
        HStack {
            Spacer()
            Text("Hello world")
                .foreground_color(Color::oklch(0.7294,0.1998, 332.47))
            Spacer()
        }
    }
}
#[ui] pub struct ExampleTextSize {
    body:_ = view!{
        HStack {
            Spacer()
            Text("Hello world")
                .with_font_size(31.0)
            Spacer()
        }
    }
}

#[ui] pub struct ExampleTextWeight {
    body:_ = view!{
        HStack {
            Spacer()
            Text("Hello world")
                .with_font_weight(FontWeight::Bold)
            Spacer()
        }
    }
}
#[ui] pub struct ExampleButton {
    body:_ = view!{
        HStack {
            Spacer()
            Button("Hello world") || {

            }
            Spacer()
        }
    }
}

#[ui] pub struct ExampleButtonTextColor {
    body:_ = view!{
        HStack {
            Spacer()
            Button("Hello world") || {

            }.foreground_color(Color::oklch(0.7294,0.1998, 332.47))
            Spacer()
        }
    }
}

#[ui] pub struct ExampleButtonTextSize {
    body:_ = view!{
        HStack {
            Spacer()
            Button("Hello world") || {

            }.with_font_size(31.0)
            Spacer()
        }
    }
}

#[ui] pub struct ExampleButtonTextWeight {
    body:_ = view!{
        HStack {
            Spacer()
            Button("Hello world") || {

            }.with_font_weight(FontWeight::Bold)
            Spacer()
        }
    }
}

#[ui] pub struct ExampleImageFit {
    body:_ = view!{
        ImageView("assets/demo/cat.png").fit()
    }
}
#[ui] pub struct ExampleImageFill {
    body:_ = view!{
        ImageView("assets/demo/cat.png").fill()
    }
}

#[ui] pub struct ExampleColorView {
    body:_ = view!{
        ColorView(Color::oklch(0.79, 0.11, 170.47))
    }
}

#[ui] pub struct ExampleInput {
    #[state] text:String = "edit this text".to_string(),

    body:_ = view!{
        VStack {
            Text(format!("typed: {text}"))
            TextField(bind!(text)).frame(Frame::no_preference())
        }
    }
}

#[ui] pub struct ExampleInputTextColor {
    #[state] text:String = "edit this text".to_string(),

    body:_ = view!{
        VStack {
            Text(format!("typed: {text}"))
            TextField(bind!(text))
                .frame(Frame::no_preference())
                .foreground_color(Color::oklch(0.7294,0.1998, 332.47))
        }
    }
}


#[ui] pub struct ExampleInputTextSize {
    #[state] text:String = "edit this text".to_string(),

    body:_ = view!{
        VStack {
            Text(format!("typed: {text}"))
            TextField(bind!(text))
                .frame(Frame::no_preference())
                .with_font_size(31.0)
        }
    }
}
#[ui] pub struct ExampleInputTextWeight {
    #[state] text:String = "edit this text".to_string(),

    body:_ = view!{
        VStack {
            Text(format!("typed: {text}"))
            TextField(bind!(text))
                .frame(Frame::no_preference())
                .with_font_weight(FontWeight::Bold)
        }
    }
}

#[ui] pub struct ExampleTextEditor {
    #[state] text:String = "edit this text".to_string(),

    body:_ = view!{
        VStack {
            Text(format!("typed: {text}"))
            TextEditor(bind!(text))
                .set_identity(10)
                .frame(Frame::no_preference())
        }
    }
}

const COLORS: &[Color] = &[
    Color::oklch(0.42, 0.1, 23.29),
    Color::oklch(0.42, 0.07, 144.0),
    Color::oklch(0.42, 0.07, 250.94),
];

#[ui] pub struct ExampleScrollViewHorizontal  {
    body:_ = view!{
        ScrollView {
            y:Some(ScrollBehavior::Scroll),
            VStack {
                for (index,color) in COLORS.iter().enumerate() {
                    Text(index).margin(Margin::all(50.0))
                        .background{ColorView(color.clone())}

                }
                
            }
        }
    }
}


#[ui] pub struct ExampleScrollViewVertical {
    body:_ = view!{
        ScrollView {
            x:Some(ScrollBehavior::Scroll),
            HStack {
                for (index,color) in COLORS.iter().enumerate() {
                    Text(index).margin(Margin::all(50.0))
                        .background{ColorView(color.clone())}

                }
                
            }
        }
    }
}

#[ui] pub struct ExampleCounter {
    #[state] count:usize = 0,
    body:_ = view!{
        Button(format!("count: {count}")) || {
            *count.get_mut() += 1;
        }
    }
}

const BLUE: Color = Color::oklch(0.62, 0.12, 236.12);

//there is something wrong in this view model
#[ui] pub struct ExampleCounterOnTap {
    #[state] count:usize = 0,
    body:_ = view!{
        VStack {
            Spacer()
            HStack {
                Spacer()
                ImageView("assets/demo/cat.png")
                    .fill()

                // .margin(Margin::all(30.0))
                .background {ColorView(Color::WHITE)}
                .margin(Margin::all(4.0))
                .background {ColorView(BLUE)}
                .frame(Frame::new(60.0, 60.0))
                .foreground_color(BLUE)

                    .on_tap || {
                        *count.get_mut() += 1
                    }
                Spacer()
            }
            Spacer()
        }
        
    }
}







// #[ui] pub struct ExampleTextColor {
//     body:_ = view!{
//         HStack {
//             Spacer()
//             Text("Hello world")
//                 .foreground_color(Color::oklch(0.7294,0.1998, 332.47))
//             Spacer()
//         }
//     }
// }