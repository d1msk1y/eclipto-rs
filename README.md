# Eclipto-rs

Eclipto is a simple, fast and not-that-easy-to-use (what all linux users love) theme management tool for linux desktops running the i3 Windows Manager.
Switch between your most diverse system-global themes with a single command in an instant.  
As most of the linux users, i like to customize my system to my liking, but with time i had found myself wasting too
much time changing the theme of my system, so i decided to create a tool to do it for me.

So now, I can switch between these two themes in just one command!
![we23png](https://github.com/user-attachments/assets/23e44c93-2546-4e04-82dd-a6e73e10d284)

If you use different components, see if there are any fork of the tool with components that you use. But if you think you're too cool you can even create your own fork of it ;)


## Installation

This tool is a perfect for you if you use:

- [i3](https://i3wm.org/) window manager
- [kitty](https://sw.kovidgoyal.net/kitty/) terminal
- [rofi](https://github.com/davatorium/rofi) dmenu replacement
- [polybar](https://github.com/d1msk1y/polybar-collection) bar (link to my polybar-collection as it has the ability to
  change bar themes on the fly)
- [nitrogen](https://github.com/l3ib/nitrogen) wallpaper manager

Eclipto is published on aur, so you can install it with your favorite aur helper. Otherwise you can [build it with rust](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html#building-and-running-a-cargo-project)

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

## Contributing - Important if you use different components!
I know such specific stack of i3 + polybar + kitty etc looks like a joke. You lucky if you have the exact same stuck, however if you use different components, you can easily adjust the tool to your needs. This CLI tool is even simple than you think. The config.json is basically a list of parameters with bash commands tied to them. Simply go to a module in the code, find the issued bash command and adjust it.

For example:
Lets say you happen to use a different alternative to rofi. 
1. Go to the theme_setter/mod.rs and find the usage of the rofi parameter that you have in your config.json. [https://github.com/d1msk1y/eclipto-rs/blob/2a740159b86410fbdd30ca03e42854936f110952/src/theme_setter/mod.rs#L19](https://github.com/d1msk1y/eclipto-rs/blob/2a740159b86410fbdd30ca03e42854936f110952/src/theme_setter/mod.rs#L37)
2. As you can see its a simple write command that takes the path of your desired rofi theme and writes it into currently used rofi theme. So to use it with an alternative to rofi, you simply have to adjust where to write your desired theme https://github.com/d1msk1y/eclipto-rs/blob/2a740159b86410fbdd30ca03e42854936f110952/src/theme_setter/mod.rs#L39

Sounds easy to me! XD 

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.  
Also feel free to fork this repository and port it to other.
