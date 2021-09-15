fn main() {
    windows::build! {
        Windows::Win32::Foundation::{PWSTR},
        Windows::Win32::UI::Shell::{IDesktopWallpaper, DesktopWallpaper},
        Windows::Win32::System::Com::{CoInitializeEx, CoCreateInstance, CLSCTX}
    };
}