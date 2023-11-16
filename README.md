# pnged

## Introduction

PNG files are structured as a series of "chunks," each containing its own data. 
Each chunk is identified by a 4-character type code. While standard chunk types are defined for various purposes,
PNGme takes advantage of the flexibility by allowing you to insert custom chunks with your own data. 
By capitalizing chunk types in specific ways, you can even make these chunks be ignored by PNG decoders.


Read [PNG-Spec](http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html) For More.

## Getting Started
Before you start, ensure you have Rust and Cargo installed. You can install them by following the instructions at [rust-lang-org](https://www.rust-lang.org/).

Clone the pnged repository:
```sh
git clone https://github.com/ravenxd0/pnged.git
cd pnged
cargo build --release
```
## Usage 
To hide and retrieve secret messages using PNGme, follow these steps:
 - Encode a message into a PNG file using the `encode` command.
 - Decode a message from a PNG file using the `decode` command.
 - Remove a message from a PNG file using the `remove` command.
 - Print a list of PNG chunks using the `print` command.


## Commands
Encode
Encode a secret message into a PNG file:

```sh
pnged encode <path_to_png> <chunk_type> <message> [output_file]
```
Decode
Decode a secret message stored in a PNG file:

```sh
pnged decode <path_to_png> <chunk_type>
```
Remove
Remove a secret message from a PNG file:

```sh
pnged remove <path_to_png> <chunk_type>
```
Print
Print a list of PNG chunks that can potentially store messages (Non Critical,Private and Safe-to-Copy Chunk Type):

```sh
pnged print <path_to_png>
```

## Example
```sh
pnged encode dice.png ruSt "This is a Sceret Message"
pnged decode dice.png ruSt
pnged print dice.png
pnged remove dice.png
```


# Test Cases
Test Cases From [PNGme](https://picklenerd.github.io/pngme_book/introduction.html)
