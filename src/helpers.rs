use crate::monitor::Monitor;
use windows::core::{PCWSTR, PWSTR, Result};
use windows::Win32::UI::Shell::IDesktopWallpaper;

pub fn usage() -> Result<()> {
	println!(
		"usage:\n\
	commands: ls, set, slideshow\n\
	ls: list wallpapers on all monitors\n\
	ls n: list wallpaper on monitor n\n\
	set n {{path}}: sets wallpaper of monitor n to path\n\
	set n random {{directory}}: sets wallpaper of monitor n to a random image from directory\n\
	slideshow advance: advance slideshow to next wallpaper"
	);
	Ok(())
}

pub fn get_monitors(idw: &IDesktopWallpaper) -> Result<Vec<Monitor>> {
	let mut monitors = Vec::new();

	let monitor_count = unsafe { IDesktopWallpaper::GetMonitorDevicePathCount(idw)? };

	for i in 0..monitor_count {
		let monitor_id = unsafe { IDesktopWallpaper::GetMonitorDevicePathAt(idw, i)? };
		let wallpaper = unsafe { IDesktopWallpaper::GetWallpaper(idw, PCWSTR(monitor_id.0))? };
		monitors.push(Monitor {
			index: i as usize,
			monitor_id: monitor_id,
			wallpaper: wallpaper,
		})
	}

	Ok(monitors)
}

pub trait IntoString {
	fn into_string(self) -> String;
}

impl IntoString for PWSTR {
	fn into_string(self) -> String {
		if self.is_null() {
			String::new()
		} else {
			let mut buffer = Vec::new();
			let mut pwz = self.0;

			unsafe {
				while *pwz != 0 {
					buffer.push(*pwz);
					pwz = pwz.add(1);
				}
			}

			String::from_utf16(&buffer).expect("string_from_pwstr")
		}
	}
}

// https://github.com/microsoft/windows-rs/pull/1091

pub trait IntoPWSTR {
	fn into_pwstr(self) -> PWSTR;
}

pub trait IntoPCWSTR {
	fn into_pcwstr(self) -> PCWSTR;
}

impl IntoPWSTR for String {
	fn into_pwstr(self) -> PWSTR {
		let mut encoded = self.encode_utf16().chain([0u16]).collect::<Vec<u16>>();
		PWSTR(encoded.as_mut_ptr())
	}
}

impl IntoPCWSTR for String {
	fn into_pcwstr(self) -> PCWSTR {
		let mut encoded = self.encode_utf16().chain([0u16]).collect::<Vec<u16>>();
		PCWSTR(encoded.as_mut_ptr())
	}
}

impl IntoPWSTR for &str {
	fn into_pwstr(self) -> PWSTR {
		let mut encoded = self.encode_utf16().chain([0u16]).collect::<Vec<u16>>();
		PWSTR(encoded.as_mut_ptr())
	}
}

impl IntoPCWSTR for &str {
	fn into_pcwstr(self) -> PCWSTR {
		let mut encoded = self.encode_utf16().chain([0u16]).collect::<Vec<u16>>();
		PCWSTR(encoded.as_mut_ptr())
	}
}
