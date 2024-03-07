# Eclipto-rs

Eclipto is a simple, fast and easy-to-use theme management tool for linux desktops running the i3 Windows Manager.
Switch between your most diverse system-global themes with a single command in an instant.  
As most of the linux users, i like to customize my system to my liking, but with time i had found myself wasting too
much time changing the theme of my system, so i decided to create a tool to do it for me.

## Installation

This tool is only relevant to you if you use:

- [i3](https://i3wm.org/) window manager
- [kitty](https://sw.kovidgoyal.net/kitty/) terminal
- [rofi](https://github.com/davatorium/rofi) dmenu replacement
- [polybar](https://github.com/d1msk1y/polybar-collection) bar (link to my polybar-collection as it has the ability to
  change bar themes on the fly)
- [nitrogen](https://github.com/l3ib/nitrogen) wallpaper manager

Eclipto is published on aur, so you can install it with your favorite aur helper.

```bash
yay -S eclipto-rs-1.0.0-x86_64.tar.gz
```

## Configuration

Eclipto uses a simple configuration file to manage themes, which is located
at `~/.config/eclipto/config.json`. The configuration file is a simple JSON file containing a list of themes and their
kitty, wallpaper, rofi, polybar and commands to run on theme change.

Here is an example configuration file:

```bash
{
	"themes": {
		"<your theme name>": {
			"kitty": "kitty config path(just the color scheme)",
			"wallpaper": "wallpaper path",
			"rofi": "rofi config path",
			"polybar": "polybar theme name(polybar-collection of mine)",
			"i3": "path to your i3 config file with overrides for the theme",
			"gtk3": "path to your gtk3 theme(a copy of .config/gtk-3.0/settings.ini)",
            "commans": ["commands", "to", "run", "on", "theme", "change"]
		},
		"<your another theme name>": {
#			...repeat for each theme
		}
	}
}
```

### i3

To configurate i3 config, you have to:

1. create a new file with the values you want to be overridden in the theme.
2. Then, you have to create a file in the same directory as the i3 config file with "overrides" in the name.
3. In the main config file, you have to include the overrides file at the bottom below the other settings.

```bash
# ~/.config/i3/config
# ...
include ~/.config/i3/overrides
```

4. Then add the path to the custom overrides file in the theme configuration.

```bash
# ~/.config/eclipto/config.json
"i3": "path to your i3 config file with overrides for the theme",
```

## Usage

Eclipto is a simple command line tool, just run it with the theme name as the first argument.

```bash
eclipto <theme name>
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.  
Also feel free to fork this repository and port it to other .