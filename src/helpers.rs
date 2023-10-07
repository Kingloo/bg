use windows::core::{Result, PCWSTR, PWSTR};

pub fn usage() -> Result<()> {
	println!(
		"USAGE\n\n\
	COMMANDS\t\t\tls, set, slideshow\n\n\
	ls\t\t\t\tlist wallpapers on attached monitors\n\
	ls n\t\t\t\tlist wallpaper on monitor at index n\n\
	ls all\t\t\t\tlist wallpapers on all monitors\n\n\
	set n {{path}}\t\t\tsets wallpaper of monitor n to path\n\
	set n random {{directory}}\tsets wallpaper of monitor n to a random image from directory\n\n\
	slideshow\t\t\tshow details of wallpaper slideshow\n\
	slideshow advance\t\tadvance slideshow to next wallpaper\n\
	"
	);
	Ok(())
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
