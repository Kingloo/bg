use windows::core::Result;
use windows::Win32::System::Com::{CoCreateInstance, CoInitializeEx, CLSCTX_LOCAL_SERVER, COINIT_MULTITHREADED};
use windows::Win32::UI::Shell::{DesktopWallpaper, IDesktopWallpaper};

mod helpers;
mod ls;
mod monitor;
mod set;
mod slideshow;

use helpers::{get_monitors, usage};
use ls::ls;
use set::set;
use slideshow::slideshow;

fn main() -> Result<()> {
	let args: Vec<String> = std::env::args().collect();

	if args.len() < 2 {
		return usage();
	}

	let idw: IDesktopWallpaper;

	unsafe {
		CoInitializeEx(std::ptr::null_mut(), COINIT_MULTITHREADED)?;
		idw = CoCreateInstance(&DesktopWallpaper, None, CLSCTX_LOCAL_SERVER)?;
	}

	let monitors = get_monitors(&idw)?;

	let command = args[1].as_str();
	let command_args = &args[1..];

	return match command {
		"ls" => ls(&monitors, &command_args),
		"set" => set(&idw, &monitors, &command_args),
		"slideshow" => slideshow(&idw, &monitors, &command_args),
		_ => usage(),
	};
}
