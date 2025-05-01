use ncurses::{
    addstr, clear, endwin, getch, initscr, keypad, mv, noecho, refresh, setlocale, stdscr,
};
use std::{
    env,
    fs::{self, File},
    io::{Read, Write},
    process::exit,
};

const NUM_LINES_TO_SHOW: u8 = 25;
const DEBUG: bool = true;

fn display_lines(lines: &Vec<String>, start: usize, end: usize) {
    for line in &lines[start..end] {
        addstr(&line).unwrap();
        addstr("\n").unwrap();
    }
    refresh();
}

fn get_file_lines(file_name: &String) -> Vec<String> {
    let mut file = match File::open(&file_name) {
        Ok(f) => f,

        Err(e) => {
            println!("Failed to open file '{file_name}' with error: {e:?}");
            exit(1);
        }
    };

    let mut file_contents = String::new();
    let read_file = file.read_to_string(&mut file_contents);

    if read_file.is_err() {
        println!("Failed to read file '{file_name}' with err {read_file:?}");
        exit(1);
    }

    let lines: Vec<String> = file_contents.split("\n").map(|l| l.into()).collect();

    return lines;
}

fn up(mut start: usize, mut end: usize) -> (usize, usize) {
    if end > 1 {
        if end - start < NUM_LINES_TO_SHOW.into() {
            end -= 1;

            if start >= 1 {
                start -= 1;
            }
        }
    }

    (start, end)
}

fn down(mut start: usize, mut end: usize, lines: &Vec<String>) -> (usize, usize) {
    end = if end >= lines.len() - 1 {
        end
    } else {
        if end - start + 1 >= NUM_LINES_TO_SHOW.into() {
            start += 1;
        }

        end + 1
    };

    (start, end)
}

fn main() {
    let mut args = env::args();

    if args.len() < 2 {
        println!("Usage: ./ssel <file_name> [<cursor_start_y> <cursor_start_x>]");
        exit(1);
    }

    let file_name = args.nth(1).unwrap();

    let (mut cur_x, mut cur_y) = (0, 0);

    if let Some(posy) = args.next() {
        cur_y = posy.parse::<i32>().unwrap();
    }

    if let Some(posx) = args.next() {
        cur_x = posx.parse::<i32>().unwrap();
    }

    let (mut start, mut end) = (0, 1);

    let mut lines = get_file_lines(&file_name);

    setlocale(ncurses::LcCategory::all, "").unwrap();
    keypad(stdscr(), true);
    initscr();

    let mut file = if DEBUG {
        if fs::exists("./debug").expect("Failed to check if file exists") {
            fs::remove_file("./debug").expect("Failed to remove debug file");
        }

        Some(File::create("./debug").unwrap())
    } else {
        None
    };

    loop {
        clear();

        mv(cur_x, cur_y);

        noecho();

        display_lines(&lines, start, end);

        let char = getch();

        match char as u8 {
            b'j' | b's' => (start, end) = down(start, end, &lines),

            b'k' | b'w' => (start, end) = up(start, end),

            b'g' => {
                start = 0;
                end = 1;
            }

            b'G' => {
                start = (lines.len() - 1 - NUM_LINES_TO_SHOW as usize).max(0);
                end = lines.len() - 2;
            }

            // reload
            b'r' => {
                lines = get_file_lines(&file_name);
            }

            // Escape
            27 => {
                match getch() {
                    // [
                    91 => match getch() as u8 {
                        // Up Arrow
                        b'A' => (start, end) = up(start, end),

                        // Down Arrow
                        b'B' => (start, end) = down(start, end, &lines),

                        _ => {}
                    },

                    _ => {}
                }
            }

            // Quit
            b'q' => break,

            x => {
                if let Some(file) = &mut file {
                    writeln!(file, "Is a Char: {x}").expect("Failed to write to debug file");
                }
            }
        }
    }

    endwin();
}
