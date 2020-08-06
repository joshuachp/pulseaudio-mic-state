# pulseaudio-mic-state 
Get and prints the status of the microphone with PulseAudio :microphone:

If an id or name is not specified, it will return the state of the default source.

You can specify `--muted` or `--unmuted` to print a different text for the
respective states.

## Build

```bash
git clone https://github.com/joshuachp/pulseaudio-mic-state.git
cd pulseaudio-mic-state
cargo build --release
./target/release/pulseaudio-mic-state
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

