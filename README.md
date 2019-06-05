## Texte, a figlet-like exercise

The purpose of this small program is to prove to myself (and to others) that
I'm doing actual progress learning Rust.

It is a figlet-like utility that takes standard input and prints an ASCII banner like this:

```
  ____                   _   _
 / ___|   ___     ___   | | | |
| |      / _ \   / _ \  | | | |
| |___  | (_) | | (_) | | | |_|
 \____|  \___/   \___/  |_| (_)
```

I would be most pleased if anyone cares to read the code and comment it.

### Get the thing running

Assuming you're familiar with git and [Cargo](https://doc.rust-lang.org/cargo/).

```bash
git clone https://github.com/Keksoj/Texte.git
cd texte
cargo run
```

Since it takes from standard output, you can even pipe it like so:

```bash
echo "Dumbledore is gay!" | cargo run | lolcat -a
```

(install [lolcat](https://www.tecmint.com/lolcat-command-to-output-rainbow-of-colors-in-linux-terminal/) first, you won't regret it).
