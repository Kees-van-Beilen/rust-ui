use std::cell::Cell;

use rust_ui::{prelude::*, view::state::NextIdentity,views::text::DebugText};

use crate::Poll;


#[ui]
pub struct CreatePollView {

    #[state] poll_name:String,
    #[state] field_names:Vec<(usize,String)>,
    #[binding] shown:bool,
    #[binding] polls:Vec<Poll>,

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
                Text("Title")
                Spacer().frame(Frame::no_preference().width(15.0))
                TextField(bind!(poll_name))
                    .set_identity(10000)
            }
            DebugText() |r_data| {
                        format!("{:#?}",r_data)
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
                    let name = poll_name.get().clone();
                    let fields = field_names.get().iter().map(|(_,a)|a.clone()).collect();
                    let identity = polls.get().next_identity();
                    polls.get_mut().push(Poll { identifier: identity, name, fields });
                    // on_create_poll(name,fields);
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
            ColorView(Color::WHITE).frame(Frame::no_preference().width(1.0))
            ScrollView {
                y:Some(ScrollBehavior::Scroll),
                VStack {
                    spacing:Some(6.0),
                    for (identity,field) in fields.iter() {
                        TextField(field)
                            .set_identity(identity)
                    }
                    Button("add field") || {
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