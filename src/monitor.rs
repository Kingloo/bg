use windows::core::{Result, PCWSTR, PWSTR};
use windows::Win32::UI::Shell::IDesktopWallpaper;

#[derive(Debug)]
pub struct Monitor {
	pub index: usize,
	pub monitor_id: PWSTR,
	pub wallpaper: PWSTR,
	pub is_attached: bool,
}

pub fn get_monitors(idw: &IDesktopWallpaper) -> Result<Vec<Monitor>> {
	let mut monitors = Vec::new();

	let monitor_count = unsafe { IDesktopWallpaper::GetMonitorDevicePathCount(idw)? };

	for i in 0..monitor_count {
		let monitor_id = unsafe { IDesktopWallpaper::GetMonitorDevicePathAt(idw, i)? };
		let wallpaper = unsafe { IDesktopWallpaper::GetWallpaper(idw, PCWSTR(monitor_id.0))? };
		let is_attached = unsafe { IDesktopWallpaper::GetMonitorRECT(idw, PCWSTR(monitor_id.0)).is_ok() };

		monitors.push(Monitor {
			index: i as usize,
			monitor_id,
			wallpaper,
			is_attached,
		})
	}

	Ok(monitors)
}
