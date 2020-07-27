# pulseaudio-mic

Get the status of the microphone with PulseAudio.

Returns `true` if the microphone is muted, otherwise `false`.

```bash
sh -c "[[ \"\$(pulseaudio-mic-state --index 1)\" == 'true' ]] && echo mut || echo not-mute"
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