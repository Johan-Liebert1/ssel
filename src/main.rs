use ncurses::{
    addstr, clear, endwin, getch, initscr, keypad, mv, noecho, refresh, setlocale, stdscr,
};
use std::{
    env,
    fs::{self, File},
    io::{Read, Write},
    process::exit,
    usize,
};

const NUM_LINES_TO_SHOW: u8 = 25;
const DEBUG: bool = false;

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

fn up(mut start: usize, mut end: usize, by: usize) -> (usize, usize) {
    if end > by {
        if end - start < NUM_LINES_TO_SHOW.into() {
            end -= by;

            if start >= by {
                start -= by;
            }
        }
    }

    (start, end)
}

fn down(mut start: usize, mut end: usize, lines: &Vec<String>, by: usize) -> (usize, usize) {
    end = if end >= lines.len() - by {
        end
    } else {
        if end - start + by >= NUM_LINES_TO_SHOW.into() {
            start += by;
        }

        end + by
    };

    (start, end)
}

fn main() {
    let mut args = env::args();

    if args.len() < 2 {
        println!("Usage: ./ssel <file_name> [-r,--random] [<cursor_start_y> <cursor_start_x>]");
        exit(1);
    }

    let file_name = args.nth(1).unwrap();

    let (mut cur_x, mut cur_y) = (0, 0);
    let mut random = false;

    loop {
        match args.next() {
            Some(arg) => match arg.as_str() {
                "-r" | "--random" => random = true,

                pos => {
                    if cur_y == 0 {
                        cur_y = pos.parse::<i32>().unwrap();
                    } else if cur_x == 0 {
                        cur_x = pos.parse::<i32>().unwrap();
                    }
                }
            },

            None => break,
        }
    }

    let (mut start, mut end) = (0, 1);

    let mut lines = get_file_lines(&file_name);

    // shuffle the array
    if random {
        for i in 0..lines.len() {
            let mut r = std::fs::File::open("/dev/urandom").unwrap();
            let mut buf = [0; 1];
            r.read_exact(&mut buf).unwrap();

            let j = (buf[0] as usize) % lines.len();

            let temp = lines[i].clone();
            lines[i] = lines[j].clone();
            lines[j] = temp;
        }
    }

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
            b'j' | b's' => (start, end) = down(start, end, &lines, 1),

            b'k' | b'w' => (start, end) = up(start, end, 1),

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

            // Ctrl + D
            4 => (start, end) = down(start, end, &lines, 10),

            // Ctrl + U
            21 => (start, end) = up(start, end, 10),

            // Escape
            27 => {
                match getch() {
                    // [
                    91 => match getch() as u8 {
                        // Up Arrow
                        b'A' => (start, end) = up(start, end, 1),

                        // Down Arrow
                        b'B' => (start, end) = down(start, end, &lines, 1),

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
