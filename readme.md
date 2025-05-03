
<p align="center">
    <picture>
    <source media="(prefers-color-scheme: dark)" srcset="./assets/logo-dark.svg">
    <img  alt="Text changing depending on mode. Light: 'So light!' Dark: 'So dark!'" src="./assets/logo-light.svg">
    </picture>
</p>
A truly native mobile focused UI-framework for iOS and android. Many current ui-frameworks in Rust do nothing more than rendering to a window's graphics context (like what games do). Instead Rust-ui uses the native ui-system of the current platform. This allows niche integrations, better accessibility support, interoperability with native ui components (like a tab/side bar) and much more!
---

# features
## platforms
| | iOS | macOS | android |
|-|-----|-------|---------|
|build|✅|✅|:construction:|
|target|aarch64-apple-ios <br> aarch64-apple-ios-sim <br>x86_64-apple-ios | x86_64-apple-darwin <br> aarch64-apple-darwin | t.b.d.

<details>
<table>
<tr><td>✅</td><td>complete 100%</td></tr>
<tr><td>:construction:</td><td>Planned</td></tr>
</table>
<summary>
<b>legend</b>
</summary>
</details>


# Getting started
Create a new rust project using `cargo init` (or your preferred initialization method). Next add the rust-ui package `cargo add kz-rust-ui`, and then copy the hello world example to the `main.rs` file. Your project is ready, now you can build an run. Enjoy!
> [!IMPORTANT]
> The package is called `kz-rust-ui` for the time being (this will change in the future). **However within Rust it is named `rust-ui`**. That means that in your cargo.toml you'll see a line like `kz-rust-ui = "0.1"` but in your rust code you have something along the lines of `use rust-ui::prelude::*;` 
## building / crosscompilation
Depending on platform the build process might look different. In all cases a simple `cargo run` will work if your targeting your own device.

### macOS
A basic macos executable can be obtained using `cargo build`. You may also specify a target architecture.
<table>
<tr><td>`--target x86_64-apple-darwin`</td><td>binary for intel macs</td></tr>
<tr><td>`--target aarch64-apple-darwin`</td><td>binary for apple silicon (M1/M2/M3 etc)</td></tr>
</table>

You may also bundle the application using [cargo-bundle](https://crates.io/crates/cargo-bundle) and then code sign using [apple-codesign](https://gregoryszorc.com/docs/apple-codesign/0.17.0/apple_codesign.html) If done properly the application should be AppStore ready. _Building, bundling and signing does not require apple hardware_


### iOS
The easiest way to test a iOS build is to try building for the iOS simulator. iOS simulator has less requirements and doesn't check code signing. For iOS you have the following targets:

<table>
<tr><td>`--target aarch64-apple-ios`</td><td>binary for iPhones/iPads/APPL</td></tr>
<tr><td>`--target aarch64-apple-ios-sim`</td><td>binary for iOS simulator running on apple silicon (M1/M2/M3 etc)</td></tr>
<tr><td>`--target x86_64-apple-ios`</td><td>binary for iOS simulator running on intel</td></tr>
</table>

It is highly advised to build using [cargo-bundle](https://crates.io/crates/cargo-bundle) as these .app folders work immediately in iOS simulator. This requires just two commands:
```bash
cargo bundle --target "insert target here"
xcrun simctl install booted "path/to/created.app"
```
Note that this does require xcode to be installed with iOS build support. 
> [!NOTE]
> You may have to set the identifier in your Cargo.toml's `package.metadata.bundle`
#### On device testing (cross platform)
Building and then running on a iPhone can be done from any device. Currently it does require you to be enrolled in the apple developer program (unless you have a look at _Frankenstein with XCode_). First build the application using
```bash
cargo bundle --target aarch64-apple-ios
```
Next upp is code-signing, but first all apps running on device are required to be signed. (Self signing should work but that still has to be figured out). Furthermore your also required to ship a `.mobileprovision` certificate in the app bundle. This can be done by downloading the appropriate certificate from the apple developer website, and then adding `resources = ["embedded.mobileprovision"]` to the `package.metadata.bundle` section in your Cargo.toml. 

To codesign use the [apple-codesign](https://gregoryszorc.com/docs/apple-codesign/0.17.0/apple_codesign.html) utility. This utility program can be ran on all platforms. Please follow the instructions on there website on how to codesign a bundle as depending on your certificates the proccess might look different.


Next to deploy on device, you may any `ipa/app` installer. Commonly used is XCode (which can properly handle wireless installs) To install using XCode press ⇧⌘2 (shift+cmd+2) in XCode, this brings up th devices menu. Select your device and drag your `.app` bundle to app list. 

You may also use [libimobiledevice](https://libimobiledevice.org/) to install the application on your device. Simply run
```bash
ideviceinstaller install "path/to/bundle.app"
```



> [!TIP]
> You may need create a `.ipa` file this is just a zipped folder named `Payload` containing your .app bundle. If everything is correct your payload.ipa should look like: 
> ```
> payload.ipa •  •  •  • (zip archive)
> |
> |-Payload   •  •  •  • (directory)
> | |
> | |-YourApp.app   •  • (application bundle)
> ```

> [!CAUTION]
> Whilst running your app on a device you will not have access to stdin, stdout and stderr. That also means you will not get stack traces if your app crashes. _(We are working on proper crash logs with rust stack traces, but it isn't here yet)_. If you would like this information try using [libimobiledevice](https://libimobiledevice.org/) to mount a developer disk image to your device and then run the app. This however has become a bit more involved in newer versions of iOS. If you don't want to go through that trouble you may have a look at _Frankenstein with XCode_ as XCode automatically attached the proper ddi.

#### Frankenstein with XCode
If you're unable to boot the development disk on your target device, or if you're not enrolled in apple's developer program. It is possible launch your application using XCode. This process is a bit involved but here's a breakdown.
1. Create iOS project in xcode (using swift)
2. Delete all swift files in project
3. In _Info.plist_ or the info tab of your iOS app target remove the `Application Scene Manifest` key. Otherwise the ui will not properly initialize.
4. Create a pipeline to run `cargo bundle --target aarch64-apple-ios` on app build in XCode
5. Create a symbolic link to the binary in the `.app` folder (and any other resources that need to be bundled)
6. Make sure to rename the symbolic link to the application executable name (that's the name of the binary in the .app bundle generated by xcode) 
7. Drag the file into XCode and make sure it is copied to the final application bundle
8. Success, it might though not work on every build but every second build, in that case just build twice.





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
This example demonstrates how state and view redrawing works.
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
This example illustrates how to split up views, and pass bindings.
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
## Hide/sho
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
                    text_input.clear();
                }
                for item in items.iter() {
                    Text(item)
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


---

<table>
<tr>
<td>
<img src="https://nlnet.nl/logo/banner.svg" width="300">
</td>
<td>
<img src="https://nlnet.nl/image/logos/NGI0Core_tag.svg" width="300">
</td>
<td>
<img src="https://research-and-innovation.ec.europa.eu/themes/contrib/oe_theme/dist/ec/images/logo/positive/logo-ec--en.svg" width="300">
</td>
</tr>
</table>


This project was funded through the [NGI0 Commons Fund](https://nlnet.nl/commonsfund), a fund established by [NLnet](https://nlnet.nl/) with financial support from the European Commission's [Next Generation Internet](https://ngi.eu/) programme, under the aegis of [DG Communications Networks, Content and Technology](https://commission.europa.eu/about-european-commission/departments-and-executive-agencies/communications-networks-content-and-technology_en) under grant agreement No [101135429](https://cordis.europa.eu/project/id/101135429). Additional funding is made available by the [Swiss State Secretariat for Education, Research and Innovation](https://www.sbfi.admin.ch/sbfi/en/home.html) (SERI).
