use crate::monitor::Monitor;
use crate::usage::usage;
use rand::{thread_rng, Rng};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use windows::core::{Result, PCWSTR};
use windows::Win32::UI::Shell::IDesktopWallpaper;

fn set_wallpaper(idw: &IDesktopWallpaper, monitor: &Monitor, path: &Path) -> Result<()> {
	let full_path = format!("{}", path.display());
	unsafe { IDesktopWallpaper::SetWallpaper(idw, PCWSTR(monitor.monitor_id.0), PCWSTR(full_path.as_ptr() as *const u16)) }
}

fn set_random_wallpaper(idw: &IDesktopWallpaper, monitor: &Monitor, path: &Path) -> Result<()> {
	let random_image_path = get_random_image(path);
	set_wallpaper(idw, monitor, &random_image_path)
}

fn get_random_image(path: &Path) -> PathBuf {
	let files: Vec<PathBuf> = std::fs::read_dir(path)
		.unwrap()
		.filter_map(|entry| entry.ok())
		.map(|entry| entry.path()) // turn DirEntry into PathBuf
		.filter(|path| path.is_file())
		.filter(|file| is_valid_extension(file.extension()))
		.collect();

	match files.len() {
		0 => PathBuf::new(),
		1 => files[0].clone(),
		_ => {
			let random_index = thread_rng().gen_range(0..files.len() - 1);
			files[random_index].clone()
		}
	}
}

fn is_valid_extension(extension: Option<&OsStr>) -> bool {
	extension.is_some() && ["jpg", "jpeg", "png"].iter().any(|ext| ext == &extension.unwrap())
}

pub fn set(idw: &IDesktopWallpaper, monitors: &[Monitor], args: &[String]) -> Result<()> {
	if args.len() < 2 {
		return usage();
	}

	if let Ok(idx) = args[1].parse::<usize>() {
		if &args[2] == "random" {
			return set_random_wallpaper(idw, &monitors[idx], Path::new(&args[3]));
		}

		let path = Path::new(&args[2]);

		if path.exists() {
			return set_wallpaper(idw, &monitors[idx], path);
		}
	}

	usage()
}
