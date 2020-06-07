# ManQL

Man pages Query Language.

## What is this?

Do you ever find yourself searching through man pages for a common use of a command?
No. I didn't think so either. You probably just search the solution on Google.
The only reason you might be using man pages is for detail argument information.

But searching for answers on Google isn't efficient. The process looks like this:
 * open a browser
 * open a new tab
 * search for a solution to your common bash problem
 * open a stack overflow post
 * copy the command
 * paste it into your terminal and replace it with your custom values.

What if you never had to leave the terminal? That's what ManQ aims to do.
 * Provides a fast searching UI to search for solutions/snippets
 * Gives you samples of valid values each field can take
 * Saves your values so you can reuse the snippet faster next time.

And much more, but enough talking. Here's a demo:

TODO: GIF

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