fn main() {
	windows::build! {
		Windows::Win32::Foundation::{PWSTR},
		Windows::Win32::UI::Shell::{IDesktopWallpaper, DesktopWallpaper, DESKTOP_SLIDESHOW_STATE, DESKTOP_SLIDESHOW_OPTIONS},
		Windows::Win32::System::Com::{CoInitializeEx, CoCreateInstance}
	};
}
