use windows::Win32::Foundation::PWSTR;

#[derive(Debug)]
pub struct Monitor {
	pub index: usize,
	pub monitor_id: PWSTR,
	pub wallpaper: PWSTR,
}
