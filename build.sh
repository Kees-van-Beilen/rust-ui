# Build the java files inte dex classes
export ANDROID_HOME="~/Library/Android/sdk"
export ANDROID_NDK_ROOT="~/Library/Android/sdk/ndk/29.0.14206865"
# android build
alias d8="~/Library/Android/sdk/build-tools/35.0.0/d8"
android="~Library/Android/sdk/platforms/android-35/android.jar"
out="./scrates/rust_ui_core/src/native/android/"

javac ./crates/rust_ui_core/src/native/android/java/*.java --release 17 -cp $android

d8 ./crates/rust_ui_core/src/native/android/java/*.class --lib $android --output $out


