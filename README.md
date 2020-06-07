# ManQL

Man pages Query Language.

## Install

### Via package manager (coming soon)

TODO: AUR package and via `cargo install`

### Building from source

```
git clone https://github.com/td0m/ManQL
cd ManQL
cargo build --release
```

#### FZF

You will also need to install fzf.

| Package Manager | Linux Distribution      | Command                    |
| --------------- | ----------------------- | -------------------------- |
| APK             | Alpine Linux            | `sudo apk add fzf`         |
| APT             | Debian 9+/Ubuntu 19.10+ | `sudo apt-get install fzf` |
| DNF             | Fedora                  | `sudo dnf install fzf`     |
| Nix             | NixOS                   | `nix-env -iA nixpkgs.fzf`  |
| Pacman          | Arch Linux              | `sudo pacman -S fzf`       |
| pkg             | FreeBSD                 | `pkg install fzf`          |
| pkg_add         | OpenBSD                 | `pkg_add fzf`              |
| Zypper          | openSUSE                | `sudo zypper install fzf`  |

#### ZSH

make sure you're in the `ManQL` directory and run:

```
echo "source $(pwd)/manql.plugin.zsh" >> ~/.zshrc
source ~/.zshrc
```

You're good to go! Go to the [Usage section](#usage) to see find out more about how to use it.

<!--
#### BASH
TODO: bash plugin.
-->

## Usage

### FZF
 1. Press <kbd>CTRL+G</kbd> to activate
 2. Search for a snippet or use <kbd>CTRL+K</kbd> and <kbd>CTRL+J</kbd> to navigate up/down. To select press <kbd>Enter</kbd>
 3. If there are any values you need to fill out, you'll get prompted again. If you'd like to input a custom value not on the list, type it on and press <kbd>ALT+Enter</kbd>.