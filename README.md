## Disable Keys

This program disables specific keys on the Windows operating system. It is useful for gaming or other scenarios where accidental key presses are disruptive.

### Download Here

You can download the latest release from the [Releases page](https://github.com/nous-/disable-keys/releases/latest).

These binaries are provided directly from github by compiling the code from this repo directly.

### Usage

By example:

- disable-keys.exe WIN_L WIN_R: Disables the left and right Windows keys.
- disable-keys.exe SLEEP f1: Disables the sleep and f1 key

Run `disable-keys.exe --list-keys` to get a list of key names

### To run yourself

1. [Install rust](https://learn.microsoft.com/en-us/windows/dev-environment/rust/setup)
1. Navigate to this cloned repo in the terminal
1. `cargo build --release`
1. Look in target folder for output. Ie `target/release/disable-keys.exe`