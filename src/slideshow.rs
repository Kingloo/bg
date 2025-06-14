use crate::monitor::Monitor;
use std::path::PathBuf;
use windows::Win32::UI::Shell::{
	DESKTOP_SLIDESHOW_OPTIONS, DESKTOP_SLIDESHOW_STATE, DSD_FORWARD, DSO_SHUFFLEIMAGES, DSS_ENABLED, DSS_SLIDESHOW, IDesktopWallpaper,
};
use windows::core::{PCWSTR, Result};

fn show_slideshow_details(idw: &IDesktopWallpaper, monitors: &[Monitor]) -> Result<()> {
	let mut slideshow_options: DESKTOP_SLIDESHOW_OPTIONS = DESKTOP_SLIDESHOW_OPTIONS(0);
	let mut tick: u32 = 0;
	let slideshow_options_ptr: *mut DESKTOP_SLIDESHOW_OPTIONS = &mut slideshow_options;
	let tick_ptr: *mut u32 = &mut tick;

	unsafe {
		IDesktopWallpaper::GetSlideshowOptions(idw, slideshow_options_ptr, tick_ptr)?;
	}

	println!("shuffle\t\t{}", is_slideshow_shuffle(slideshow_options));
	println!("duration\t{} mins", get_slideshow_tick_in_minutes(&tick));

	let slideshow_directory = match get_slideshow_directory(&monitors[0]) {
		Some(dir) => format!("{}", dir.display()),
		None => format!("failed to get slideshow directory for '{}'", &monitors[0].monitor_id_to_string()),
	};

	println!("directory\t{}", slideshow_directory);

	Ok(())
}

fn is_slideshow(state: DESKTOP_SLIDESHOW_STATE) -> bool {
	state == DESKTOP_SLIDESHOW_STATE(DSS_ENABLED.0 + DSS_SLIDESHOW.0)
}

fn is_slideshow_shuffle(options: DESKTOP_SLIDESHOW_OPTIONS) -> bool {
	options == DSO_SHUFFLEIMAGES
}

fn get_slideshow_tick_in_minutes(tick: &u32) -> f32 {
	(*tick as f32 / 1000f32) / 60f32
}

fn get_slideshow_directory(monitor: &Monitor) -> Option<PathBuf> {
	match monitor.wallpaper_to_pathbuf() {
		Some(path) => {
			if path.exists() && path.is_file() {
				path.parent().map(|parent| parent.to_path_buf())
			} else {
				None
			}
		}
		None => None,
	}
}

fn advance_slideshow(idw: &IDesktopWallpaper) -> Result<()> {
	unsafe {
		// monitor.monitor_id doesn't work here for second argument
		IDesktopWallpaper::AdvanceSlideshow(idw, PCWSTR::null(), DSD_FORWARD)
	}
}

pub fn slideshow(idw: &IDesktopWallpaper, monitors: &[Monitor], args: &[String]) -> Result<()> {
	let slideshow_state = unsafe { IDesktopWallpaper::GetStatus(idw)? };

	if is_slideshow(slideshow_state) {
		if args.len() > 1 && args[1] == "advance" {
			return advance_slideshow(idw);
		} else {
			return show_slideshow_details(idw, monitors);
		}
	} else {
		println!("not a slideshow")
	}

	Ok(())
}
