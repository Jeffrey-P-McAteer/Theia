
# Theia

> Theia is the Titan goddess of sight and the shining ether of the sky.
> Her name was derived from the Greek words thea "sight" and theiazô "prophesy".

Theia is a native binary which extends programs like `vlc` and `mpv` to provide
a system for subscribing to, viewing, and searching various backends which host videos.

Backends include:

 - Youtube (planned)
 - FTP server (planned)
 - HTTP Server with some form of AutoIndex (planned)
 - Local directory (planned)

# Use

## Windorks

Double-click `theia.exe`.

## Arch Linux

Execute `theia` for a GUI.

Pass one of the following arguments to do something you can add to your scripts:

 - `theia proto://hostname:port/path` Open a GUI with the given backend loaded
	- Protocols planned include `youtube://channel-name/video-name`, `ftp://host:port/directory/`, `file:///C:/some_videos/`.

 - `theia diff proto://hostname:port/path` Prints any new content since the last time the given source was checked.
 - `theia ls proto://hostname:port/path` Lists all content from the given backend (the path may be used to limit scope based on source; eg a single youtube channel).

## macOS

unimplemented but planned; need to learn if we'd need to package
as a .app directory or if a native binary like windorks would work.

# Dependencies

The _goal_ is to have a single, stand-alone binary. Unfortunately with media
processing being complex stuff and my development time being finite
some compromises have been made.

## Linux

 - `libvlccore.so.9`
 - `libsystemd.so.0` - I have no idea how that got in the binary and will have to investigate - your choice of init system should not affect a video player.

## Windorks

// TODO

## macOS

// TODO

# Project Status

Right now I'm playing around with all the GUI and video playback libraries offered
by the rust ecosystem. I'm thinking of using `vlc` as a great all-around backend
for playing content, `web-view` for the GUI, and I haven't thought about how
we'll be storing data. Likely have one config directory under the user's home
as well as a cache directory, probably going to use TOML for the config file.


# Development

Wanna put on a hard hat and help out?

Make sure you have `rustc` and the `cargo` toolchain [https://rustup.rs/](https://rustup.rs/),
download this repository of code somewhere on your system, and run:

```
cargo run --
```

## Cross-compiling

Because respectable folk develop on respectable OSes we
need to use a cross-compiler for Windorks.

```
rustup target add x86_64-pc-windows-gnu

# Arch linux only; Sorry. go trip over your package manager's 20-year-old documentation.
yay -S mingw-w64-gcc

# Add the following to your ~/.cargo/config, editing the path to x86_64-w64-mingw32-gcc
# for your system (this is the path used on Arch systems)
[target.x86_64-pc-windows-gnu]
linker = "/usr/bin/x86_64-w64-mingw32-gcc"

cargo build --target x86_64-pc-windows-gnu
# ^ Currently fails because we need `vlc.dll` on the system path
#   Adding this dependency is pretty painful so for now the
#   workaround is to compile the windows .exe on windows systems.

# TODO maybe abuse some container tech to get us good cross-compile-ability with Windorks?

```


