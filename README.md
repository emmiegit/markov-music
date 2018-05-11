## markov-music
A music player that determines what song to next play based on a Markov chain. This way, based on how you normally listen to music, skipping songs you don't feel like hearing and repeating ones you liked, this program can 'learn' what you like and be able to generate new sequences of songs. The ideal is for it to generate an infinite playlist of songs that strike a balance between too random and too repetitive.

You should copy the provided sample configuration file to `~/.config/markov-music/config.toml`. The player will listen to the mpd socket, determining how to modify the weights of the markov chain. It will automatically control the queue, adding new songs and removing old ones. If the daemon is disabled, it will continue to listen and build the chain, but not modify the queue.

The process will expose a UNIX domain socket at a configured location. This can be used to control
the daemon.

Licensed under the GPL, version 2 or later.

### Compilation
```
$ cargo build --release
```

### Usage
```
USAGE:
    markov-music [FLAGS] [OPTIONS]

FLAGS:
    -h, --help        Prints help information
    -V, --version     Prints version information

OPTIONS:
    -c, --config <FILE>    Use a specific configuration file instead of the
                           default
```

