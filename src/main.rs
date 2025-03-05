use ncurses::{addstr, endwin, getch, initscr, ll::clear, mv, refresh, setlocale};
use std::{env, fs::File, io::Read, process::exit};

const NUM_LINES_TO_SHOW: u8 = 25;

fn display_lines(lines: &Vec<String>, start: usize, end: usize) {
    for line in &lines[start..end] {
        addstr(&line).unwrap();
        addstr("\n").unwrap();
    }
    refresh();
}

fn main() {
    let mut args = env::args();

    println!("args: {args:#?}");

    if args.len() < 2 {
        println!("Usage: ./idk <file_name> [<cursor_start_y> <cursor_start_x>]");
        exit(1);
    }

    let file_name = args.nth(1).unwrap();

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

    let (mut cur_x, mut cur_y) = (0, 0);

    if let Some(posy) = args.next() {
        cur_y = posy.parse::<i32>().unwrap();
    }

    if let Some(posx) = args.next() {
        cur_x = posx.parse::<i32>().unwrap();
    }

    let lines: Vec<String> = file_contents.split("\n").map(|l| l.into()).collect();
    let (mut start, mut end) = (0, 1);

    setlocale(ncurses::LcCategory::all, "").unwrap();
    initscr();

    loop {
        unsafe {
            clear();
        }

        mv(cur_x, cur_y);

        display_lines(&lines, start, end);

        match getch() as u8 {
            b'j' => {
                end = if end >= lines.len() - 1 {
                    end
                } else {
                    if end - start + 1 >= NUM_LINES_TO_SHOW.into() {
                        start += 1;
                    }

                    end + 1
                };
            }

            b'k' if end > 1 => {
                println!("start: {start}");

                if end - start < NUM_LINES_TO_SHOW.into() {
                    end -= 1;

                    if start >= 1 {
                        start -= 1;
                    }
                }
            }

            b'q' => break,

            _ => {}
        }
    }

    endwin();
}
