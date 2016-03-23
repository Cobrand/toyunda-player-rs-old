# toyunda-player

## Installation

### Requirements

[Rust 1.7](https://www.rust-lang.org/) is required

### Linux

#### Required packages

You must install libmpv, sdl2 and sdl2-ttf beforehand :

##### Archlinux

```bash
sudo pacman -S mpv sdl2 sdl2_ttf
```

#### Compiling target

```bash
git clone https://github.com/Cobrand/toyunda-player-rs
cd toyunda-player-rs
cargo build --release
```

Your executable in now in target/Release/

### Windows

download [libmpv](https://mpv.srsfckn.biz/) (latest build, "Dev archive"),
[sdl2](https://www.libsdl.org/download-2.0.php) (Windows -> SLD2-devel-mingw ),
[sdl2_ttf](https://www.libsdl.org/projects/SDL_ttf/) (Windows -> SDL2_ttf-devel-mingw)

And place x86_64-w64-mingw32/lib/ contents in C:\Program Files\Rust\\**lib**\rustlib\x86_64-pc-windows-gnu\lib

then

```bash
git clone https://github.com/Cobrand/toyunda-player-rs
cd toyunda-player-rs
cargo build --release
```

You then have to link these 3 libs with your executable whenever you want to share it.

### Mac

No.
