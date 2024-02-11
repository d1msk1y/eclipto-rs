# Eclipto-rs

Eclipto is a simple, fast and easy-to-use theme management tool for linux desktops running the i3 Windows Manager.

## Installation

This tool is only relevant to you if you use:

- [i3](https://i3wm.org/) window manager
- [kitty](https://sw.kovidgoyal.net/kitty/) terminal
- [rofi](https://github.com/davatorium/rofi) dmenu replacement
- [polybar](https://github.com/d1msk1y/polybar-collection) bar (link to my polybar-collection as it has the ability to
  change bar themes on the fly)
- [nitrogen](https://github.com/l3ib/nitrogen) wallpaper manager

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
            "commans": ["commands", "to", "run", "on", "theme", "change"]
		},
		"<your another theme name>": {
#			...repeat for each theme
		}
	}
}
```

## Usage

Eclipto is a simple command line tool, just run it with the theme name as the first argument.

```bash
eclipto <theme name>
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.  
Also feel free to fork this repository and port it to other .