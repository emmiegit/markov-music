## markov-music
A music player that determines what song to next play based on a Markov chain. This way, based on how you normally listen to music, skipping songs you don't feel like hearing and repeating ones you liked, this program can 'learn' what you like and be able to generate new sequences of songs. The ideal is for it to generate an infinite playlist of songs that strike a balance between too random and too repetitive.

You should copy the provided sample configuration file to `~/.config/markov-music/config.toml`. The most important field to modify is the path to your music directory. This directory should be the root for all the music in your collection, and a file called `.markov_music_data` will be created inside that stores the last state of the player, including the built-up Markov chain.

The music player's backend is based on `libmpv`.

Licensed under the GPL, version 2 or later.

### Compilation
Run `cargo build --release` in the top directory of the repository.

### Usage
```
USAGE:
    markov-music [FLAGS] [OPTIONS]

FLAGS:
    -h, --help        Prints help information
        --no-color    Disables colors even on terminals that support them
    -V, --version     Prints version information

OPTIONS:
    -c, --config <FILE>    Use a specific configuration file instead of the
                           default
```

