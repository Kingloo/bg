use bindings::Windows::Win32::Foundation::{PWSTR};
use bindings::Windows::Win32::System::Com::{
    CoCreateInstance, CoInitializeEx, CLSCTX_ALL, COINIT_MULTITHREADED,
};
use bindings::Windows::Win32::UI::Shell::{DesktopWallpaper, IDesktopWallpaper};

#[derive(Debug)]
struct Monitor {
    index: usize,
    monitor_id: String,
    wallpaper: String
}

fn usage() {
    println!("usage: blablabla");
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
                wallpaper: string_from_pwstr(wallpaper)
            })
        }
    }

    Ok(monitors)
}

fn print_monitor(monitor: &Monitor) {
    println!("{}", monitor.index);
    println!("\t{}", monitor.monitor_id);
    println!("\t{}", monitor.wallpaper);
}

fn print_monitors(monitors: &Vec<Monitor>) {
    for i in 0..monitors.len() {
        print_monitor(&monitors[i]);
    }
}

fn set_wallpaper(monitor: &Monitor, path: &String) -> Result<(), windows::Error> {
    unsafe {
        let idw: IDesktopWallpaper = CoCreateInstance(&DesktopWallpaper, None, CLSCTX_ALL)?;
        IDesktopWallpaper::SetWallpaper(&idw, monitor.monitor_id.clone(), path.as_str())?;
    }
    Ok(())
}

fn main() -> windows::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        usage();
        return Ok(());
    }

    let command = args[1].as_str();

    let monitors = get_monitors()?;

    if command == "ls" {
        print_monitors(&monitors);
    } else if command == "get" {
        match args[2].parse::<usize>() {
            Ok(x) => print_monitor(&monitors[x]),
            Err(_) => print_monitors(&monitors),
        }
    } else if command == "set" {
        if args.len() < 3 {
            usage();
            return Ok(());
        }
        match args[2].parse::<usize>() {
            Ok(x) => {
                if args.len() >= 4 && std::path::Path::new(&args[3]).exists() {
                    set_wallpaper(&monitors[x], &args[3])?;
                } else {
                    usage();
                    return Ok(());
                }
            },
            Err(_) => usage()
        }
    } else {
        usage();
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
