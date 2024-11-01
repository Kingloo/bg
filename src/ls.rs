use crate::monitor::Monitor;
use crate::usage::usage;
use windows::core::Result;

fn print_monitor(monitor: &Monitor) {
	println!("{}", monitor.index);
	println!("\tid\t\t{}", monitor.monitor_id_to_string());
	println!(
		"\twallpaper\t{}",
		match monitor.wallpaper_to_pathbuf() {
			Some(wallpaper) => wallpaper.display().to_string(),
			None => format!("failed to get wallpaper path for '{}'", monitor.monitor_id_to_string()),
		}
	);
	if !monitor.is_attached {
		println!("\tis attached\tfalse")
	}
	println!();
}

fn print_monitors(monitors: &[Monitor]) {
	monitors.iter().for_each(print_monitor)
}

fn print_attached_monitors(monitors: &[Monitor]) {
	monitors.iter().filter(|monitor| monitor.is_attached).for_each(print_monitor)
}

pub fn ls(monitors: &[Monitor], args: &[String]) -> Result<()> {
	match args.len() {
		1 => {
			print_attached_monitors(monitors);
		}
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
		}
		_ => panic!("args passed to ls had a negative .len()"),
	}

	Ok(())
}
