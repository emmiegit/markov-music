## markov-music
A music player that determines what song to next play based on a Markov chain. This way, based on how you normally listen to music, skipping songs you don't feel like hearing and repeating ones you liked, this program can 'learn' what you like and be able to generate new sequences of songs. The ideal is for it to generate an infinite playlist of songs that strike a balance between too random and too repetitive.

Through the use of the `Player` struct, it is possible to implement other player backends. Currently only mpv is supported through `libmpv`.

Licensed under the GPL, version 2 or later.

**Note:**
Don't expect this to work for a while. It may not even compile for you.

### Compilation
Run `cargo build --release` in the top directory of the repository.

### Usage
TODO

