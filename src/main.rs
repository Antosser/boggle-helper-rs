#![allow(non_snake_case)]

use std::{collections::HashSet, io::BufRead};

fn printBoard(board: &[[char; 4]; 4]) {
    for y in 0..9 {
        for x in 0..9 {
            let isHorizontal = x % 2 == 0;
            let isVertical = y % 2 == 0;

            if isHorizontal && isVertical {
                print!("+");
            }
            if !isHorizontal && !isVertical {
                print!(" {} ", board[(x - 1) / 2][(y - 1) / 2]);
            }
            if isHorizontal && !isVertical {
                print!("|");
            }
            if !isHorizontal && isVertical {
                print!("---");
            }
        }
        println!();
    }
}

fn printVoid() {
    for _ in 0..50 {
        println!();
    }
}

fn findWords(
    words: &HashSet<String>,
    resultedWord: &String,
    path: &Vec<(i32, i32)>,
    board: &[[char; 4]; 4],
    foundWords: &mut HashSet<String>,
    printOnlyPositions: bool,
) {
    for x in 0..3 {
        for y in 0..3 {
            let mut resultedWord = resultedWord.clone();
            let mut path = path.clone();

            if x == 1 && y == 1 {
                continue;
            }
            let x = path.last().unwrap().0 + x - 1;
            let y = path.last().unwrap().1 + y - 1;
            if !(0..4).contains(&x) || !(0..4).contains(&y) {
                continue;
            }
            if path.contains(&(x, y)) {
                continue;
            }

            resultedWord.push(board[x as usize][y as usize]);
            path.push((x as i32, y as i32));

            if words.iter().any(|s| s.starts_with(resultedWord.as_str())) {
                if words.contains(resultedWord.as_str()) {
                    if printOnlyPositions {
                        let mut output = String::new();
                        for pos in &path {
                            output.push_str(format!("{} {} ", pos.0, pos.1).as_str());
                        }
                        output.pop();
                        println!("{}", output);
                    } else {
                        println!("{}", resultedWord);
                    }
                    foundWords.insert(resultedWord.clone());
                }

                findWords(
                    words,
                    &resultedWord,
                    &path,
                    board,
                    foundWords,
                    printOnlyPositions,
                );
            }
        }
    }
}

fn main() {
    let mut printOnlyPositions = false;
    {
        println!("Print only positions?");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "y" => printOnlyPositions = true,
            _ => printOnlyPositions = false,
        }
    }

    let mut board = [[' '; 4]; 4];

    if !printOnlyPositions {
        printBoard(&board);
    }

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input = input.trim().to_string();
    assert!(input.len() == 16);

    for i in 0..16 {
        board[i % 4][(i as f32 / 4f32) as usize] = input.chars().nth(i).unwrap();
    }

    if !printOnlyPositions {
        printVoid();
        println!("Board: ");
        printBoard(&board);

        println!("Importing words...");
    }
    let file = std::fs::File::open("words.txt").unwrap();
    let reader = std::io::BufReader::new(file);
    let mut words = HashSet::new();

    for line in reader.lines() {
        let word = line.unwrap().trim().to_string().to_lowercase();

        if !word.chars().all(|x| x.is_alphabetic()) || word.len() < 3 {
            continue;
        }

        words.insert(word);
    }
    if !printOnlyPositions {
        println!("Words imported");
    }

    let mut foundWords: HashSet<String> = HashSet::new();

    for x in 0..4 {
        for y in 0..4 {
            if !printOnlyPositions {
                println!("x: {x}, y: {y}");
            }
            findWords(
                &words,
                &String::from(board[x][y]),
                &vec![(x as i32, y as i32)],
                &board,
                &mut foundWords,
                printOnlyPositions,
            );
        }
    }

    if !printOnlyPositions {
        let mut foundWords = foundWords.iter().cloned().collect::<Vec<String>>();

        foundWords.sort_by(|a, b| a.len().cmp(&b.len()));

        println!("-----");
        for word in foundWords {
            println!("{}", word);
        }
    }
}
