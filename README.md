# pulseaudio-mic-state

Get and prints the status of the microphone with PulseAudio :microphone:

If an id or name is not specified, it will return the state of the default
source.

You can specify `--muted` or `--unmuted` to print a different text for the
respective states.

## Build

```bash
git clone https://github.com/joshuachp/pulseaudio-mic-state.git
cd pulseaudio-mic-state
cargo build --release
./target/release/pulseaudio-mic-state
```

## Copyright & Licensing

All parts of these binding libraries are fully open-source and free to use.

All files in this source code repository, except as noted below, are licensed
under the MIT license or the Apache license, Version 2.0, at your option. You
can find copies of these licenses either in the LICENSE-MIT and LICENSE-APACHE
files, or alternatively here and here respectively.
