# swayhide - A window swallower for sway

## Description

`swayhide` hides the currently active terminal (by moving it to the scratchpad),
then it executes the supplied command.
When the child process has finished, the terminal is moved back.
This is incredibly useful if your workflow includes opening graphical programs from the terminal,
as the "locked" terminal won't have to take up any space.

Ideally this would be done using wlroots instead of swayipc,
but I'm too lazy to figure out how that works.

This project will be deprecated when I find/build a wlroots alternative,
since that would make it wm-independent and thus more future proof.

This project is heavily inspired by [jamesofarrell/i3-swallow](https://github.com/jamesofarrell/i3-swallow)

## Usage

```sh
$ swayhide firefox
$ alias hide="swayhide"
$ hide zathura document.pdf
$ hide "imv image.jpg"
```

## Installation

```sh
$ cargo install swayhide
```

There's also an AUR package for ArchLinux users:

```sh
paru -S swayhide
```

## Building from source

```sh
$ git clone https://github.com/NomisIV/swayhide
$ cd swayhide
$ cargo install --path .
```

## TODO

- Shell completions (how to bundle completion files?)

Contributions are very welcome :)

## License

This project is licensed under GPL 3.0
