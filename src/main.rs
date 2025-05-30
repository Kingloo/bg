use windows::Win32::System::Com::{CLSCTX_LOCAL_SERVER, COINIT_MULTITHREADED, CoCreateInstance, CoInitializeEx};
use windows::Win32::UI::Shell::{DesktopWallpaper, IDesktopWallpaper};

mod ls;
mod monitor;
mod set;
mod slideshow;
mod usage;

use ls::ls;
use monitor::get_monitors;
use set::set;
use slideshow::slideshow;
use usage::usage;

fn main() -> windows::core::Result<()> {
	let args: Vec<String> = std::env::args().collect();

	if args.len() < 2 {
		return usage();
	}

	unsafe {
		CoInitializeEx(None, COINIT_MULTITHREADED).ok()?;
	}

	let idw: IDesktopWallpaper = unsafe { CoCreateInstance(&DesktopWallpaper, None, CLSCTX_LOCAL_SERVER)? };

	let monitors = get_monitors(&idw)?;

	let command = args[1].as_str();
	let command_args = &args[1..];

	match command {
		"ls" => ls(&monitors, command_args),
		"set" => set(&idw, &monitors, command_args),
		"slideshow" => slideshow(&idw, &monitors, command_args),
		_ => usage(),
	}
}
