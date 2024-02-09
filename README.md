# yamg

## Description

I made this to practice my fast counting by pattern recognition. It will display a matrix and you will have to enter how many items there are in the matrix as fast as you can. I am also using this project to lean Rust, I'm not expecting anyone to see this repo.

## Installation

**Clone the repository**

`git clone https://github.com/richa3816/yamg.git && cd yamg`

**Build the binary**

`cargo build`

**Run the binary**

`cargo run`

## Keybinds

This program uses vim-like bindings and mode-style menus

**Normal mode**

| Key | Action            |
| --- | ----------------- |
| `q` | Quit the program  |
| `i` | Enter insert mode |

**Insert mode**

| Key     | Action            |
| ------- | ----------------- |
| `Esc`   | Enter normal mode |
| `<CR>`  | Submit answer     |
| `<C-BS> | Deletes last word |
| `<C-w>` | Deletes last word |

# TODO

- Matrix generation function

- Answer validation system

- Timing system

- Matrix size adjustment

- Matrix char adjustment

- Put into multiple files

- Create a builder that adds it to PATH
