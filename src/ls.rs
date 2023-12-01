use crate::helpers::{usage, IntoString};
use crate::monitor::Monitor;
use windows::core::Result;

fn print_monitor(monitor: &Monitor) {
	println!("{}", monitor.index);
	println!("\tid\t\t{}", monitor.monitor_id.into_string());
	println!("\twallpaper\t{}", monitor.wallpaper.into_string());
	if !monitor.is_attached {
		println!("\tis attached\tfalse")
	}
	println!();
}

fn print_monitors(monitors: &Vec<Monitor>) {
	for each in monitors {
		print_monitor(each)
	}
}

fn print_attached_monitors(monitors: &Vec<Monitor>) {
	let attached_monitors: Vec<&Monitor> = monitors.into_iter().filter(|monitor| monitor.is_attached).collect();
	for each in attached_monitors {
		print_monitor(each)
	}
}

pub fn ls(monitors: &Vec<Monitor>, args: &[String]) -> Result<()> {
	match args.len() {
		1 => {
			print_attached_monitors(monitors);
		},
		i if i > 1 => {
			if let Ok(monitor_index) = args[1].parse::<usize>() {
				if monitor_index < monitors.len() {
					print_monitor(&monitors[monitor_index]);
				} else {
					eprintln!("no monitor at index {}", monitor_index);
				}
			} else if args[1] == "all" {
				print_monitors(monitors);
			} else {
				return usage();
			}
		},
		_ => panic!("args passed to ls had a negative .len()")
	}

	Ok(())
}
