#![feature(more_qualified_paths,default_field_values)]
// the let it poll app
use rust_ui::prelude::*;
use rust_ui::view::dyn_render::DynGroup;

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
                }
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