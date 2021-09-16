use bindings::Windows::Win32::Foundation::PWSTR;
use bindings::Windows::Win32::System::Com::{
	CoCreateInstance, CoInitializeEx, CLSCTX_ALL, COINIT_MULTITHREADED,
};
use bindings::Windows::Win32::UI::Shell::{DesktopWallpaper, IDesktopWallpaper};
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
	commands: ls, get, set\n\
	get: get N\n\
	set: set N {{path}}"
	);
	Ok(())
}

fn get_monitors() -> Result<Vec<Monitor>, windows::Error> {
	let mut monitors = Vec::new();

	unsafe {
		CoInitializeEx(std::ptr::null_mut(), COINIT_MULTITHREADED)?;
		let idw: IDesktopWallpaper = CoCreateInstance(&DesktopWallpaper, None, CLSCTX_ALL)?;
		let monitor_count = IDesktopWallpaper::GetMonitorDevicePathCount(&idw)?;
		for i in 0..monitor_count {
			let monitor_id = IDesktopWallpaper::GetMonitorDevicePathAt(&idw, i)?;
			let wallpaper = IDesktopWallpaper::GetWallpaper(&idw, monitor_id)?;
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

fn set_wallpaper(monitor: &Monitor, path: &String) -> Result<(), windows::Error> {
	if path::Path::new(path).is_file() {
		unsafe {
			let idw: IDesktopWallpaper = CoCreateInstance(&DesktopWallpaper, None, CLSCTX_ALL)?;
			IDesktopWallpaper::SetWallpaper(&idw, monitor.monitor_id.clone(), path.as_str())?;
		}
	} else {
		println!("file does not exist: {}", path);
	}
	Ok(())
}

fn set_random_wallpaper(monitor: &Monitor, dir: &String) -> Result<(), windows::Error> {
	if path::Path::new(dir).is_file() {
		println!("not a directory: {}", dir);
		return Ok(());
	}

	if let Some(random_image) = get_random_image(&dir) {
		println!("setting monitor {} to {}", monitor.index, random_image);
		return set_wallpaper(monitor, &random_image);
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

fn set(monitors: &Vec<Monitor>, args: &Vec<String>) -> Result<(), windows::Error> {
	if args.len() < 3 {
		return usage();
	}

	return match args[2].parse::<usize>() {
		Ok(x) => {
			if args.len() >= 4 {
				if &args[3] == "random" {
					return set_random_wallpaper(&monitors[x], &args[4]);
				}

				if std::path::Path::new(&args[3]).exists() {
					return set_wallpaper(&monitors[x], &args[3]);
				}
			}

			return usage();
		}
		Err(_) => usage(),
	};
}

fn main() -> windows::Result<()> {
	let args: Vec<String> = std::env::args().collect();

	if args.len() < 2 {
		return usage();
	}

	let monitors = get_monitors()?;

	let command = args[1].as_str();

	if command == "ls" {
		return ls(&monitors);
	} else if command == "get" {
		return get(&monitors, &args);
	} else if command == "set" {
		return set(&monitors, &args);
	} else {
		return usage();
	}
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
