# Boggle helper
Prints out every word in a boggle board relativly fast OR autoselects the words in an online game

## Installation
1. Download the executabe from the latest release.
2. Put a [dictionary](https://raw.githubusercontent.com/dwyl/english-words/master/words.txt) in the directory with the executable
3. Run it.
4. To the question if it should only print out positions type ...

### If you just need the words
5. ...no
6. Type in all 16 letters from the board in sequence.
7. All words will be printed out shortly.

### If you want the words automatically selected in an online game
5. ...y
6. Also download the python file from the project root.
7. Type in all 16 letters from the board in sequence.
8. Copy the output to positions.txt to where the python file is.
9. Get the python dependencies and run the python file.
10. In the online game press insert over the first and the last letter (top left and bottom right).
11. Program will select all the correct words

## How to compile yourself
- Python doesn't require compilation
1. Download rust (not the game)
2. Go to the directory with the Cargo.toml
3. Run `cargo build -r`
4. Executable is in `target/release/`
