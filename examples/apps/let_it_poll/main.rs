#![feature(more_qualified_paths,default_field_values)]
use std::cell::Cell;

// the let it poll app
use rust_ui::prelude::*;
use rust_ui::view::dyn_render::DynGroup;
use rust_ui::view::mutable::MutableViewRerender;
use rust_ui::view::state::{PartialAnyBinding, PartialBinding};

pub enum GlobalState {
    WelcomeScreen,
    Overview
}

#[ui(main)]
struct RootView {
    #[state] global:GlobalState = GlobalState::WelcomeScreen,

    body:_ = view!{
        match *global.get() {
            GlobalState::WelcomeScreen => WelcomeScreen{global:bind!(global)}.boxed(),
            GlobalState::Overview => OverviewView {}.boxed()
        }
    }
    
}

struct Poll {
    identifier:usize,
    name:String,
    state:PollState
}

enum PollState {
    Draft,
    Running,
    Closed
}



#[ui]
struct OverviewView {
    #[state] show_create_poll_sheet:bool = false,
    #[state] polls: Vec<Poll>,

    body:_ = view!{
        VStack {
            HStack {
                Spacer()
                Button("New poll") || {
                    *show_create_poll_sheet.get_mut() = true;
                }.sheet(bind!(show_create_poll_sheet)) {
                    CreatePollView{shown:bind!(show_create_poll_sheet)}
                }
            }
        }
    }
}

#[ui]
struct CreatePollView {
    #[state] poll_name:String,
    #[state] field_names:Vec<(usize,String)>,
    #[binding] shown:bool,
    body:_ = view!{
        VStack {
            spacing:Some(10.0),
            HStack {
                Text("Create new poll")
                .title()
                Spacer()
            }
            HStack {
                spacing:Some(10.0),
                Text("title")
                TextField(bind!(poll_name))
            }
            PollOptionsView {
                fields:bind!(field_names)
            }
            HStack {
                Spacer()
                Button("cancel") || {
                    *shown.get_mut() = false;
                }
                Spacer()
                Button("create") || {
                    *shown.get_mut() = false;
                }
                Spacer()
            }
        }.margin(Margin::all(12.0))
    }
}

#[ui]
struct PollOptionsView {
    #[binding] fields:Vec<(usize,String)>,
    #[state] identity_counter:usize = 0,
    body:_ = view!{
        HStack {
            spacing:Some(10.0),
            ColorView(Color::WHITE).frame(Frame::no_preference().width(4.0))
            ScrollView {
                VStack {
                    Text("fields")
                    for (identity,field) in fields.iter() {
                        TextField(field)
                            .set_identity(identity)
                    }
                    Button("+ field") || {
                        let len = *identity_counter.get()+1;
                        fields.get_mut().push((*identity_counter.get(),format!("field {}",len)));
                        // let c = Cell::new(false);
                        // let mut new_state = identity_counter.to_partial_state().as_state(&c);
                        // *new_state.get_mut() += 1;
                        *identity_counter.get_mut() += 1;
                    }
                }.align(TextAlignment::Leading).frame(Frame::no_preference())
            }
            
        }
    }
}





#[ui]
struct WelcomeScreen {
    #[binding] global:GlobalState,
    body:_ = view!{
        HStack {
            Spacer()
            VStack {
                spacing:Some(10.0),
                Spacer()
                        ImageView("assets/demo/let_it_poll.png")
                        .frame(Frame::no_preference().height(100.0))
                        Text("Welcome to Let It Poll! In this app you can create whatever poll you want, and share them with friends. Excited? no? it is just an example app, calm down.")
                            .align(TextAlignment::Leading)
                        Button("Continue") || {
                            *global.get_mut() = GlobalState::Overview;
                        }
                Spacer()
                    
            }.frame(Frame::no_preference().width(300.0)).margin(Margin::all(12.0))
            Spacer()
        }
        
    }
}