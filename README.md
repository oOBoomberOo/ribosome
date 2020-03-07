# Ribosome [![Crates.io](https://img.shields.io/crates/v/ribosome)](https://crates.io/crates/ribosome) [![Build Status](https://travis-ci.com/oOBoomberOo/ribosome.svg?branch=master)](https://travis-ci.com/oOBoomberOo/ribosome)

## About
Ribosome is a command-line application that convert `structure.nbt` file into `mcfunction` file to help making multi-block detection system easier.  

## Installation
1) Install [Rustup](https://www.rust-lang.org/tools/install) (A Rust compiler that needed to compile this program)
2) Open command line/terminal.
3) Run command: `cargo install ribosome`
4) The program is now installed and you can run it via: `ribosome --help` command.

## Usage
1) Run `ribosome` program with the configuration you need.
2) The program will output file like this:
```
scoreboard players set #structure.pass ffi.ribosome 1
execute if score #structure.pass ffi.ribosome matches 1 unless block ~0 ~0 ~0 minecraft:cobblestone_stairs run scoreboard players set #structure.pass ffi.ribosome 0
```
3) You can then run that function from anywhere in your datapack.  
In this example, `#structure.pass ffi.ribosome` will be set to 1 if the location you're running this command from; contain this structure. And will be 0 otherwise.

## Configuration
This can be found when running `ribosome --help` as well.  
Run the program with these arguments to enable these options.
- Air Block (`-v`, `--ignore-air`)  
To ignore any air blocks inside structure file. (effectively turn air blocks into structure voids)
- NBT Block (`-n`, `--ignore-nbt`)  
To ignore NBT data inside structure file completely.
- Block State (`-b`, `--ignore-block-state`)  
To ignore Block State inside structure file completely.
- Mode (`-m <mode>`, `--mode <mode>`)  
Select the origin of structure, default to `corner`.
  - `corner`, the origin of structure is in the north-west most corner of structure file.
  - `center`, the origin of structure is at exactly the center of structure in all axis.
  - `center_top`, similar to `center` but the y-axis will be the top most block instead.
  - `center_bottom`, similar to `center` but the y-axis will be the bottom most block instead.
- Target (`-t <name>`, `--target <name>`)  
Change the scoreboard name of the outputed file, default to `#structure.pass`.
- Objectuve (`-o <name>`, `--objective <name>`)  
Change the objective name of the outputed file, default to `ffi.ribosome`.
