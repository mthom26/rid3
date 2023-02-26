# rid3
`rid3` is a terminal application for viewing and editing the id3 metadata on mp3 audio files. It supports customizable theming and keybindings.

## Usage
The application consists of three main states which can be selected using the '1', '2' and '3' keys by default.

The main state contains the current files you are working on and the frames on that file. From here you can add, delete and edit frames and rename files (most of your time will be spent in here).
The files state contains the contents of the current directory. From here you can browse your file system and add more mp3 files to the main state.
The frames state contains a list of id3 frames supported by the application. Here you can add new frames to files in the main state.

Pressing the 'h' key will bring up help text with relevant keybindings for each state. A detailed tutorial can be found [here](./docs/tutorial.md).

## Configuration
On app startup the application will look in the `~/.config/rid3` directory for a 'config.toml' file (this file will not be created automatically by the app). This file does not need to contain all possible settings, any missing values will be obtained from the default configuration.
