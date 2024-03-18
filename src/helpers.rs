pub fn usage() -> windows::core::Result<()> {
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
