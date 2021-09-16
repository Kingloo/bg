use bindings::Windows::Win32::Foundation::PWSTR;
use bindings::Windows::Win32::System::Com::{
	CoCreateInstance, CoInitializeEx, CLSCTX_ALL, COINIT_MULTITHREADED,
};
use bindings::Windows::Win32::UI::Shell::{
	DesktopWallpaper, IDesktopWallpaper, DESKTOP_SLIDESHOW_OPTIONS, DESKTOP_SLIDESHOW_STATE,
};
use rand::{thread_rng, Rng};
use std::{fs, path};

#[derive(Debug)]
struct Monitor {
	index: usize,
	monitor_id: String,
	wallpaper: String,
}

fn usage() -> Result<(), windows::Error> {
	println!(
		"usage:\n\
	commands: ls, get, set, slideshow\n\
	get: get N\n\
	set: set N {{path}}\n\
	set: set N random {{directory}}\n\
	slideshow: slideshow"
	);
	Ok(())
}

fn get_monitors(idw: &IDesktopWallpaper) -> Result<Vec<Monitor>, windows::Error> {
	let mut monitors = Vec::new();

	unsafe {
		let monitor_count = IDesktopWallpaper::GetMonitorDevicePathCount(idw)?;
		for i in 0..monitor_count {
			let monitor_id = IDesktopWallpaper::GetMonitorDevicePathAt(idw, i)?;
			let wallpaper = IDesktopWallpaper::GetWallpaper(idw, monitor_id)?;
			monitors.push(Monitor {
				index: i as usize,
				monitor_id: string_from_pwstr(monitor_id),
				wallpaper: string_from_pwstr(wallpaper),
			})
		}
	}

	Ok(monitors)
}

fn print_monitor(monitor: &Monitor) -> Result<(), windows::Error> {
	println!("{}", monitor.index);
	println!("  {}", monitor.monitor_id);
	println!("  {}", monitor.wallpaper);
	println!("");
	Ok(())
}

fn print_monitors(monitors: &Vec<Monitor>) -> Result<(), windows::Error> {
	for i in 0..monitors.len() {
		print_monitor(&monitors[i])?;
	}
	Ok(())
}

fn set_wallpaper(idw: &IDesktopWallpaper, monitor: &Monitor, path: &String) -> Result<(), windows::Error> {
	if path::Path::new(path).is_file() {
		unsafe {
			IDesktopWallpaper::SetWallpaper(idw, monitor.monitor_id.clone(), path.as_str())?;
		}
	} else {
		println!("file does not exist: {}", path);
	}
	Ok(())
}

fn set_random_wallpaper(idw: &IDesktopWallpaper, monitor: &Monitor, dir: &String) -> Result<(), windows::Error> {
	if path::Path::new(dir).is_file() {
		println!("not a directory: {}", dir);
		return Ok(());
	}

	if let Some(random_image) = get_random_image(&dir) {
		println!("setting monitor {} to {}", monitor.index, random_image);
		return set_wallpaper(idw, monitor, &random_image);
	} else {
		println!(
			"failed to set wallpaper: could not get random image from {}",
			dir
		);
		return Ok(());
	}
}

fn get_random_image(dir: &String) -> Option<String> {
	let files: Vec<fs::DirEntry> = fs::read_dir(dir)
		.unwrap()
		.filter(|file| file.is_ok())
		.map(|file| file.expect("failed to turn Ok<DirEntry> into DirEntry"))
		.filter(|file| has_valid_extension(file))
		.collect();

	if files.len() == 0 {
		return None;
	}

	if files.len() == 1 {
		return get_path_from_dir_entry(&files[0]);
	}

	let random_index = thread_rng().gen_range(0..files.len() - 1);

	get_path_from_dir_entry(&files[random_index])
}

fn has_valid_extension(entry: &fs::DirEntry) -> bool {
	if let Some(extension) = get_extension(entry) {
		return ["jpg", "jpeg", "png"].iter().any(|ext| ext == &extension);
	}
	false
}

fn get_path_from_dir_entry(entry: &fs::DirEntry) -> Option<String> {
	match entry.path().into_os_string().into_string() {
		Ok(path) => Some(path),
		Err(_) => None,
	}
}

fn get_extension(entry: &fs::DirEntry) -> Option<String> {
	match entry.path().extension() {
		Some(ext) => match ext.to_os_string().into_string() {
			Ok(s) => Some(s),
			Err(_) => None,
		},
		None => None,
	}
}

fn ls(monitors: &Vec<Monitor>) -> Result<(), windows::Error> {
	print_monitors(monitors)
}

fn get(monitors: &Vec<Monitor>, args: &Vec<String>) -> Result<(), windows::Error> {
	if args.len() < 3 {
		return usage();
	}

	return match args[2].parse::<usize>() {
		Ok(x) => print_monitor(&monitors[x]),
		Err(_) => usage(),
	};
}

fn set(idw: &IDesktopWallpaper, monitors: &Vec<Monitor>, args: &Vec<String>) -> Result<(), windows::Error> {
	if args.len() < 3 {
		return usage();
	}

	return match args[2].parse::<usize>() {
		Ok(x) => {
			if args.len() >= 4 {
				if &args[3] == "random" {
					return set_random_wallpaper(idw, &monitors[x], &args[4]);
				}

				if std::path::Path::new(&args[3]).exists() {
					return set_wallpaper(idw, &monitors[x], &args[3]);
				}
			}

			return usage();
		}
		Err(_) => usage(),
	};
}

fn slideshow(idw: &IDesktopWallpaper, monitors: &Vec<Monitor>) -> Result<(), windows::Error> {
	unsafe {
		let slideshow_state = IDesktopWallpaper::GetStatus(idw)?;
		let is_slideshow = is_slideshow(slideshow_state);
		println!("slideshow\t{}", is_slideshow);
		if is_slideshow {
			let mut slideshow_options: DESKTOP_SLIDESHOW_OPTIONS =
				DESKTOP_SLIDESHOW_OPTIONS::from(0);
			let mut tick: u32 = 0;
			let slideshow_options_ptr: *mut DESKTOP_SLIDESHOW_OPTIONS = &mut slideshow_options;
			let tick_ptr: *mut u32 = &mut tick;
			IDesktopWallpaper::GetSlideshowOptions(idw, slideshow_options_ptr, tick_ptr)?;
			println!("shuffle\t\t{}", is_slideshow_shuffle(slideshow_options));
			println!("duration\t{} mins", get_slideshow_tick_in_minutes(&tick));
			println!("directory\t{}", get_slideshow_directory(idw, &monitors[0])?);
		}
	}
	Ok(())
}

fn is_slideshow(state: DESKTOP_SLIDESHOW_STATE) -> bool {
	return state == DESKTOP_SLIDESHOW_STATE(3);
}

fn is_slideshow_shuffle(options: DESKTOP_SLIDESHOW_OPTIONS) -> bool {
	return options == DESKTOP_SLIDESHOW_OPTIONS(1);
}

fn get_slideshow_tick_in_minutes(tick: &u32) -> f32 {
	(tick.clone() as f32 / 1000f32) / 60f32
}

fn get_slideshow_directory(idw: &IDesktopWallpaper, monitor: &Monitor) -> Result<String, windows::Error> {
	unsafe {
		let wallpaper = IDesktopWallpaper::GetWallpaper(idw, monitor.monitor_id.clone())?;
		let wallpaper_string = string_from_pwstr(wallpaper);
		let full_path = path::Path::new(&wallpaper_string);
		if let Some(parent) = full_path.parent() {
			if parent.is_dir() {
				if let Some(s) = parent.to_str() {
					return Ok(s.to_string());
				}
			}
		}
	}

	Ok(String::default())
}

fn main() -> windows::Result<()> {
	let args: Vec<String> = std::env::args().collect();

	if args.len() < 2 {
		return usage();
	}

	let idw: IDesktopWallpaper;

	unsafe {
		CoInitializeEx(std::ptr::null_mut(), COINIT_MULTITHREADED)?;
		idw = CoCreateInstance(&DesktopWallpaper, None, CLSCTX_ALL)?;
	}

	let monitors = get_monitors(&idw)?;

	let command = args[1].as_str();

	if command == "ls" {
		ls(&monitors)?;
	} else if command == "get" {
		get(&monitors, &args)?;
	} else if command == "set" {
		set(&idw, &monitors, &args)?;
	} else if command == "slideshow" {
		slideshow(&idw, &monitors)?;
	} else {
		usage()?;
	}

	Ok(())
}

fn string_from_pwstr(source: PWSTR) -> String {
	if source.is_null() {
		String::new()
	} else {
		let mut buffer = Vec::new();
		let mut pwz = source.0;

		unsafe {
			while *pwz != 0 {
				buffer.push(*pwz);
				pwz = pwz.add(1);
			}
		}

		String::from_utf16(&buffer).expect("string_from_pwstr")
	}
}
