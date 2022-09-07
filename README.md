# Paws 🐾

Generate an icon sets from emojis or images

## Usage

Create icons from emojis

```bash
  cargo run -- -e 🐾 -s 256 --sizes 64,128,256
  firefox icons.html
```

Create icons from image
  
```bash
  cargo run -- -i "./paws.png" -s 256 --sizes 64,128,256
  firefox icons.html
```

## Using a relase

``` bash
  chmod +x paws
  ./paws --emoji 🐾 --sizes 64,128,256
  firefox ./icons.html
```

## Manual

``` bash
  paws 0.1.0
  Generate an icon sets from emojis or images

  USAGE:
      paws [FLAGS] [OPTIONS]

  FLAGS:
      -e, --emoji      Create icons from emoji
      -h, --help       Prints help information
      -i, --image      Create icons from image
      -V, --version    Prints version information

  OPTIONS:
      -s, --size <size>          Size of the icon
      -o, --output <output>      Output directory
```


## Dev requirements

* [Rust and Cargo 🦀](https://doc.rust-lang.org/cargo/getting-started/installation.html) 