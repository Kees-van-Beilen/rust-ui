<p align="center">
    <picture>
    <source media="(prefers-color-scheme: dark)" srcset="./assets/logo-dark.svg">
    <img  alt="Text changing depending on mode. Light: 'So light!' Dark: 'So dark!'" src="./assets/logo-light.svg">
    </picture>
</p>
A truly native mobile focused UI-framework for iOS and android. Many current ui-frameworks in Rust are nothing more than 

# Examples
## Hello World
One of the first programs you'll write in rust-ui, perfect for testing if everything worked. 
```rs
//main.rs
use rust_ui::prelude::*;

// thanks to the #[ui(main)] macro we dont
// have to include a main function.
#[ui(main)]
struct MainView {
    // define the body of this view
    body = {
        Text("Hello world")
    }
}
```

## Counter
```rs
//main.rs
use rust_ui::prelude::*;

#[ui(main)]
struct MainView {
    // We use #[state] to indicate that a
    // change in  this  variables  should
    // redraw our view.
    #[state] count: i32 = 0,
    body = {
        HStack {
            spacing: 8.0,
            Text(format!("You have clicked {count} times")),
            
            Button("Click me") || {
                printf!("clicked");
                count += 1;
            }
        }
    }
}
```
## Bindings
```rs 
//main.rs
use rust_ui::prelude::*;

#[ui]
struct IncrementorButton {
    // Bindings indicate that  if  this  value
    // changes, the owner of this value should
    // rerender its view.
    #[binding] value: i32,
    body = {
        Button("click me") || {
            value += 1;
        }
    }
}

#[ui(main)]
struct MainView {
    #[state] count: i32 = 0,
    body = {
        HStack {
            spacing: 8.0,
            Text(format!("You have clicked {count} times")),
            
            IncrementorButton {
                //we create a binding to our 
                //count variable
                value:binding!(count)
            }
        }
    }
}
```
## Hide/show
```rs 
//main.rs
use rust_ui::prelude::*;

#[ui(main)]
struct MainView {
    #[state] hidden: bool = true,
    body = {
        HStack {
            spacing: 8.0,
            if !hidden {
                Text("My secret text")
            }
            Button(if hidden {"show"}else{"hide"}) || {
                hidden = !hidden;
            }
        }
    }
}
```
## Lists
```rs 
//main.rs
use rust_ui::prelude::*;

#[ui(main)]
struct MainView {

    #[state] items: Vec<String> = vec![],
    #[state] text_input: String = String::new(),

    body = {
        VStack {
            spacing: 8.0,
            HStack {
                Input(binding!(text_input)),
                Button("add item") || {
                    items.push(text_input);
                    text_input.drain();
                }
            }
            
        }
    }
}
```


# The necessary evil
First why the weird macro syntax? This is done to abstract away some very verbose syntax. Besides that it is also there to make sure people don't shoot themselves in the foot. For instance the `body = {...}`, which compiles down to a function, makes sure you only do UI related initialization (you still can do some funky stuff but it is a lot harder now). 

Next why no main function? In this library you should have one view in the `main.rs` file with the tag `#[ui(main)]` as opposed to just the `#[ui]` tag. This automatically adds a main function that looks similar to this:
```rs
fn main(){
    rust_ui::native::launch_application_with_view(MyMainView::new())
}
```
So why not write this function yourself? You can, but again you might shoot yourself in the foot. The main function should initialize the application and register it to the os as fast as possible, doing any form of initialization is strongly discouraged, instead use a `.task {}` hook. Also your application might want to support more than just a main view. Think of a preview provider or home-screen widget. Whilst these aren't supported by the current project, these would be implement using `#ui(widget)` or similar