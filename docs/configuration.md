## Configuration

The app allows the user to customise the colour theme and keybindings. The user provided config file does not need to contain all settings, any that are missing will be pulled in from the default configuration. A complete config file will contain three different sections, `general`, `theme` and `actions`.

The app stores its configuration file in the following locations:
 - `~/.config/rid3` on linux

### Keybindings

Custom keybinds belong in the `actions` section. In the [default config](../default_config.toml) file there are five sub sections, General Actions, Main Screen Actions, Files Screen Actions, Frames Screen Actions and Popup Actions.

The Main Screen, Files Screen, Frames Screen and Popup Actions are independant of each other and can share keybindings. For example one action in each of these sub sections can have the same key assigned to it. However any keys assigned in the General sub section must not appear elsewhere or the config will be invalid.

### Theme

In the `theme` section colours can be specified various elements of the ui. Colours can be referred to by name (`Green`) or by a hexcode (`#25c525`). All the named colours are listed below:
 - `Reset`
 - `Black`
 - `Red`
 - `Green`
 - `Yellow`
 - `Blue`
 - `Magenta`
 - `Cyan`
 - `Gray`
 - `DarkGray`
 - `LightRed`
 - `LightGreen`
 - `LightYellow`
 - `LightBlue`
 - `LightMagenta`
 - `LightCyan`
 - `White`
 
