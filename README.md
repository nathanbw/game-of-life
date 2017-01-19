game-of-life
==============

This repo contains an implementation of [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life)
which I am writing to explore and learn the [rust programming language](https://www.rust-lang.org/). It is a work in
progress and exists completely for educational purposes. Have fun!

This program should compile cleanly on most Linux distributions with a recent rust compiler:
```
$ cd game-of-life
$ cargo run
```

To build/run on OS X/macOS, you'll need to install ncurses from homebrew:
```
$ brew tap homebrew/dupes
$ brew install ncurses
$ cd game-of-life
$ LIBRARY_PATH='/usr/local/opt/ncurses/lib' cargo run
```

The build steps above assume you have already installed rust, which you can do easily using
[rustup](https://www.rustup.rs/).