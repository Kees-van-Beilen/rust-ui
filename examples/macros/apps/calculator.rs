// The Calculator example.
// This example shows how to make a minimal calculator app
// You might notice that this example is missing some function that a normal pocket calculator has like:
// - a division operator
// - a clear button
// - a button to switch signs
// Try adding these features yourself
#![feature(more_qualified_paths,default_field_values)]
use rust_ui::prelude::*;


// we first create some data types, these represent the mathematical operations you may perform
enum Op {
    Add,
    Sub,
    Mul,
}

impl Op {
    // a utility function we call later
    pub fn from_char(c:char)->Option<Self> {
        match c {
            'x'=>Some(Self::Mul),
            '+'=>Some(Self::Add),
            '-'=>Some(Self::Sub),
            _=>None
        }
    }
}

// This is the main state of the application
enum CalculatorState {
    // Here we display the first number wer are entering
    FirstNumber(String),
    // Here we have received the mathematical operation we will be performing
    // and we display the second number we are entering. 
    // We still store the first number, that way we can calculate the result when the '=' button is pressed.
    SecondNumber{
        first_number:String,
        op:Op,
        second_number:String,
    },
    // here we display the result after the '=' button has been pressed
    Result(String)
}

impl CalculatorState {
    // this is a utility function.
    // These function are handy because we wouldn't want to write big logic heavy code inside of our ui code
    pub fn text(&self)->&str{
        match self {
            CalculatorState::FirstNumber(num) => &num,
            CalculatorState::SecondNumber { first_number:_, op:_, second_number } => &second_number,
            CalculatorState::Result(num) => &num,
        }
    }
}

// define the color of the buttons
const GREY:Color = Color::oklch(0.38, 0.0, 356.82);
const PINK:Color = Color::oklch(0.61, 0.16, 356.82);


// now we start defining our ui components
// first the calculator buttons
#[ui]
struct CalcButtonView {
    // these buttons have a character, which is either a number or 'x', '+', '-', '=', '.'
    // this text is set in the root view for every button instance.
    // We also use this to dictate the logic of what should happen if this button is pressed
    face_text:char,
    // if the button is pressed, we would like to change the state of the application.
    // because this application state is owned by the root view, we use a "binding" to bind
    // the application state to this view.
    #[binding] state:CalculatorState,
    // now we define how to render our view
    body:_ = view!{
        // the core of this button view is a text view that displays the `face_text`
        Text(face_text)
            .frame(Frame::no_preference()) // normally text wants to be as small as possible, but with this frame modifier we indicate that we have no preferred size, which almost always means that it will take all available space
            .background {
                // we now give this frame (that contains the text) a nice background depending on the `face_text`
                ColorView(if face_text.is_ascii_digit() {GREY}else{PINK})
            }
            .on_tap || { 
                // in this on_tap modifier, we can define the logic of what happens when we press this button
                // first we get a mutable reference to the application state
                // now be careful, even if we don't modify the variable it wil still rerender the RootView, because we called `get_mut`
                let borrow = &mut *state.get_mut();
                match borrow {
                    // in the case we pressed '.' we want to add a decimal separator iff the number doesn't already have one
                    CalculatorState::SecondNumber { first_number:_, op:_, second_number:num }
                    | CalculatorState::FirstNumber(num) if *face_text == '.' && !num.contains('.') => num.push('.'),
                    // if we pressed a number and the current number displayed is '0' we want to replace it and not append it.
                    // otherwise we would get something like "02" instead of "2"
                    CalculatorState::SecondNumber { first_number:_, op:_, second_number:num }
                    | CalculatorState::FirstNumber(num) if num == "0" => {
                        num.clear();
                        num.push(*face_text);
                    },
                    // if we pressed a number, we want to add it
                    CalculatorState::SecondNumber { first_number:_, op:_, second_number:num }
                    | CalculatorState::FirstNumber(num) if face_text.is_ascii_digit() => num.push(*face_text),
                    // if we press a number whilst a result is displayed, we first remove the result
                    CalculatorState::Result(_) if face_text.is_ascii_digit() => *borrow = CalculatorState::FirstNumber(face_text.to_string()),
                    // if we press the '=' button we calculate the result
                    CalculatorState::SecondNumber { first_number, op, second_number } if *face_text == '=' => {
                        let a:f64 = first_number.parse().unwrap();
                        let b:f64 = second_number.parse().unwrap();
                        let c = match op {
                            Op::Add => a+b,
                            Op::Sub => a-b,
                            Op::Mul => a*b,
                        };
                        // display the result and format it to show to digits after the decimal
                        *borrow = CalculatorState::Result(format!("{:.2}",c));
                    }
                    // if al else fails we check if face_text is an operator and advance the app state accordingly
                    CalculatorState::Result(num)
                    |CalculatorState::FirstNumber(num) => if let Some(op) = Op::from_char(*face_text) {
                        *borrow = CalculatorState::SecondNumber { first_number: num.clone(), op, second_number: "0".to_string() }
                    }
                    // if this button doesn't make sense to do anything at the current application state, then don't do anything
                    _=>{}
                };
            }
            
    }
}


// This is the root view, which we decorate with the main attribute
// that marks it the entrypoint of the application
#[ui(main)]
struct RootView {
    // This is the main application state
    // as this variable should be updated over the lifetime of the application
    // we mark it as a #[state] which we can later *bind* to our button views 
    #[state] calc:CalculatorState = CalculatorState::FirstNumber("0".to_string()),
    // Now we describe how our root view should be rendered
    body:_ = view!{
        VStack { // A VStack lays out its children vertically, in this case with a gap of 8px between them
            spacing: Some(8.0),
            // the first child is a spacer with a height of 200
            // this is such that our calculator is in the safe area of most mobile devices
            Spacer()
                .frame(Frame::no_preference().height(200.0))
            // the text that shows our current number
            Text(calc.get().text())
                .with_font_size(21.0)
                .align(TextAlignment::Trailing)
            // Now come the rows of buttons, these rows use HStacks to layout the views horizontally with a spacing of 8
            HStack {
                spacing:Some(8.0),
                CalcButtonView { face_text:'7', state:bind!(calc) }
                CalcButtonView { face_text:'8', state:bind!(calc) }
                CalcButtonView { face_text:'9', state:bind!(calc) }
                CalcButtonView { face_text:'x', state:bind!(calc) }
            }
            HStack {
                spacing:Some(8.0),
                CalcButtonView { face_text:'4',state:bind!(calc) }
                CalcButtonView { face_text:'5',state:bind!(calc) }
                CalcButtonView { face_text:'6',state:bind!(calc) }
                CalcButtonView { face_text:'-',state:bind!(calc) }
            }
            HStack {
                spacing:Some(8.0),
                CalcButtonView { face_text:'1', state:bind!(calc) }
                CalcButtonView { face_text:'2', state:bind!(calc) }
                CalcButtonView { face_text:'3', state:bind!(calc) }
                CalcButtonView { face_text:'+', state:bind!(calc) }
            }
             HStack {
                spacing:Some(8.0),
                CalcButtonView { face_text:'0', state:bind!(calc) }
                CalcButtonView { face_text:'.', state:bind!(calc) }
                CalcButtonView { face_text:'=', state:bind!(calc) }
            }
        }
    }
}