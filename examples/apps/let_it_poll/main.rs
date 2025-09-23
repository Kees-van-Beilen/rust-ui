#![feature(more_qualified_paths, default_field_values)]
use std::cell::Cell;

// the let it poll app
use rust_ui::prelude::*;
use rust_ui::view::dyn_render::DynGroup;
use rust_ui::view::mutable::MutableViewRerender;
use rust_ui::view::state::{Identifiable, PartialAnyBinding, PartialBinding};

mod create_poll_view;
use create_poll_view::CreatePollView;

pub enum GlobalState {
    WelcomeScreen,
    Overview,
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

#[derive(Clone, Default)]
pub struct Poll {
    identifier: usize,
    name: String,
    fields: Vec<String>,
}
impl Identifiable for Poll {
    type Value = Poll;

    fn identity(&self) -> usize {
        self.identifier
    }

    fn value(&self) -> &Self::Value {
        self
    }

    fn value_mut(&mut self) -> &mut Self::Value {
        self
    }
}

#[ui]
struct OverviewView {
    #[state] show_create_poll_sheet:bool = false,
    #[state] polls: Vec<Poll>,

    body:_ = view!{
        VStack {
            Spacer().frame(Frame::no_preference().height(50.0))
            HStack {
                Spacer()
                Button("New poll") || {
                    *show_create_poll_sheet.get_mut() = true;
                }.sheet(bind!(show_create_poll_sheet)) {
                    CreatePollView{
                        polls:bind!(polls),
                        shown:bind!(show_create_poll_sheet)
                    }
                }
            }
            ScrollView {
                y:Some(ScrollBehavior::Scroll),
                VStack {
                    spacing:Some(5.0),
                    for (identity,poll) in bind!(polls).iter() {
                        OverviewPollView{
                            poll:Some(poll.get().clone())
                        }.set_identity(identity)
                    }
                }
            }
        }
    }
}
#[ui]
struct OverviewPollView {
    poll:Poll = Poll::default(),
    body:_ = view!{
        HStack {
            Text(&poll.name)
                .title()
            Spacer()
            Text(">")
                .with_font_size(21.0)
        }.margin(Margin::all(5.0)).background(ColorView(Color::BLACK)).margin(Margin::all(5.0))
    }
}

// struct OverviewView {
//     show_create_poll_sheet: ::rust_ui::view::state::PartialState<bool> ,polls: ::rust_ui::view::state::PartialState<Vec<Poll>> ,view: ::std::option::Option<::std::rc::Rc<::std::cell::RefCell<::rust_ui::native::MutableView>>>,identity:usize
// }
// #[derive(Default)]
// struct OverviewViewPartialInit {
//     pub show_create_poll_sheet: ::std::option::Option<bool> ,pub polls:Vec<Poll>,
// }
// impl OverviewView {
//     pub fn new(initializer:impl OverviewViewInitializer) ->  ::std::rc::Rc< ::std::cell::RefCell<Self> >{
//         ::std::rc::Rc::new(::std::cell::RefCell::new(OverviewViewInitializer::init(initializer)))
//     }

//     }
// trait OverviewViewInitializer {
//     fn init(self) -> OverviewView;

//     }impl OverviewViewInitializer for OverviewViewPartialInit {
//     fn init(self) -> OverviewView {
//         OverviewView {
//             show_create_poll_sheet: ::std::option::Option::<bool>::unwrap_or_else(self.show_create_poll_sheet, | |false).into(),polls:self.polls.into(),view: ::std::option::Option::<::std::rc::Rc<::std::cell::RefCell<::rust_ui::native::MutableView>>>::None,identity:0usize
//         }
//     }

//     }
// impl ::rust_ui::PartialInitialisable for OverviewView {
//     type PartialInit = OverviewViewPartialInit;
// }
// impl ::rust_ui::view::mutable::MutableView for OverviewView {
//     #[allow(unused_macros)]
//     fn children(data: ::std::rc::Rc<::std::cell::RefCell<Self>>) -> impl ::rust_ui::layout::RenderObject+'static{
//         use::rust_ui::layout::RenderObject;
//         let data_ref = data.borrow();
//         let show_create_poll_sheet =  &data_ref.show_create_poll_sheet;
//         let polls =  &data_ref.polls;
//         macro_rules! bind {
//             ($state:expr) => {
//                 ::rust_ui::view::state::AsPartiBinding::as_partial_binding($state,data.clone())
//             };
//         }
//         ;
//         macro_rules! effect {
//             (some box$expr:expr) => {
//                 Some(Box::new(effect!($expr)))
//             };
//             (|$($arg:ident: $t:ty),+| $expr:block) => {
//                 {
//                     let data = data.clone();
//                     move|$($arg: $t),+|{
//                         let data_ref = data.borrow();
//                         let signal =  ::std::cell::Cell::new(false);
//                         let queue =  ::rust_ui::view::state::BindingQueue::default();
//                         let res = {
//                             let mut show_create_poll_sheet = data_ref.show_create_poll_sheet.as_state(&signal);
//                             let mut polls = data_ref.polls.as_state(&signal);
//                             (move |$($arg: $t),+| $expr)($($arg),+)
//                         };
//                         queue.execute();
//                         ::std::mem::drop(data_ref);
//                         if signal.take(){
//                             ::rust_ui::view::mutable::MutableViewRerender::rerender(&data);
//                         }res
//                     }
//                 }
//             }
//         }
//         {
//             VStack::new(<VStack<_>as ::rust_ui::PartialInitialisable>::PartialInit {
//                 children: ::std::option::Option::Some((HStack::new(<HStack<_>as ::rust_ui::PartialInitialisable>::PartialInit {
//                     children: ::std::option::Option::Some((Spacer::new(#[allow(unused_parens)]
//                     ()).set_identity(0),Button::new(#[allow(unused_parens)]
//                     ("New poll")).set_identity(0).with_capture_callback({
//                         let data = data.clone();
//                         move||{
//                             let data_ref = data.borrow();
//                             let signal =  ::std::cell::Cell::new(false);
//                             let queue =  ::rust_ui::view::state::BindingQueue::default();
//                             let res = {
//                                 let mut show_create_poll_sheet = data_ref.show_create_poll_sheet.as_state(&signal);
//                                 let mut polls = data_ref.polls.as_state(&signal);
//                                 (move||{
//                                     *show_create_poll_sheet.get_mut() = true;
//                                 })()
//                             };
//                             queue.execute();
//                             ::std::mem::drop(data_ref);
//                             if signal.take(){
//                                 ::rust_ui::view::mutable::MutableViewRerender::rerender(&data);
//                             }res
//                         }
//                     },0).sheet(::rust_ui::view::state::AsPartiBinding::as_partial_binding(show_create_poll_sheet,data.clone())).with_capture_callback({
//                         let data = data.clone();
//                         move||{
//                             use::rust_ui::layout::RenderObject;
//                             let data_ref = data.borrow();
//                             let show_create_poll_sheet =  &data_ref.show_create_poll_sheet;
//                             let polls =  &data_ref.polls;
//                             macro_rules! bind {
//                                 ($state:expr) => {
//                                     ::rust_ui::view::state::AsPartiBinding::as_partial_binding($state,data.clone())
//                                 };
//                             }
//                             ;
//                             CreatePollView::new(<CreatePollView as ::rust_ui::PartialInitialisable>::PartialInit {
//                                 on_create_poll:Some(Box::new(effect!(|e:usize|{
//                                     println!("sdffsd");
//                                 }))),shown: ::rust_ui::view::state::AsPartiBinding::as_partial_binding(show_create_poll_sheet,data.clone()), ..Default::default()
//                             }).set_identity(1)
//                         }
//                     },2),)), ..Default::default()
//                 }).set_identity(3),)), ..Default::default()
//             }).set_identity(4)
//         }
//     }
//     fn get_attached(&self) ->  &::std::option::Option<::std::rc::Rc<::std::cell::RefCell<::rust_ui::native::MutableView>>>{
//         &self.view
//     }
//     fn get_mut_attached(&mut self) ->  &mut ::std::option::Option<::std::rc::Rc<::std::cell::RefCell<::rust_ui::native::MutableView>>>{
//         &mut self.view
//     }
//     fn set_identity(&mut self,identity:usize){
//         self.identity = identity;
//     }
//     fn get_identity(&self) -> usize {
//         self.identity
//     }
//     fn clone_bindings(&self,into:&mut Self){
//         into.show_create_poll_sheet = self.show_create_poll_sheet.clone();
//         into.polls = self.polls.clone();
//     }

//     }

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
