use crate::helpers::{usage, IntoString};
use crate::monitor::Monitor;
use windows::core::Result;

fn print_monitor(monitor: &Monitor) {
	println!("{}", monitor.index);
	println!("  {}", monitor.monitor_id.into_string());
	println!("  {}", monitor.wallpaper.into_string());
	println!("");
}

fn print_monitors(monitors: &Vec<Monitor>) {
	for i in 0..monitors.len() {
		print_monitor(&monitors[i]);
	}
}

pub fn ls(monitors: &Vec<Monitor>, args: &[String]) -> Result<()> {
	if args.len() > 1 {
		if let Ok(monitor_index) = args[1].parse::<usize>() {
			if monitor_index < monitors.len() {
				print_monitor(&monitors[monitor_index]);
			} else {
				println!("no monitor found at index {}", monitor_index);
			}
		} else {
			return usage()
		}
	} else {
		print_monitors(monitors);
	}

	Ok(())
}
