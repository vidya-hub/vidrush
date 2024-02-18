use console::Term;
use std::thread;
use std::time::Duration;
use std::vec;

fn main() {
    let stdout = Term::buffered_stdout();
    let term = Term::stdout();
    let _ = term.clear_screen();
    print_initial_stuff(&term);
    let height = 10;
    let width = 10;
    let mut x = 5;
    let mut y = 5;
    let mut timer_count = 3;

    let mut _game_grid: Vec<Vec<i32>> = vec![vec![0; width]; height];
    _game_grid[x][y] = 1;

    'timer_loop: loop {
        let _ = term.write_line(&format!("Game will start at {}", timer_count));
        thread::sleep(Duration::from_millis(2000));
        let _ = term.clear_last_lines(1);
        timer_count -= 1;
        if timer_count == 0 {
            let _ = term.clear_screen();
            break 'timer_loop;
        }
    }

    print_grid(&term, &_game_grid);

    'game_loop: loop {
        if let Ok(character) = stdout.read_char() {
            match character {
                'd' => {
                    // RIGHT
                    if y > 0 {
                        _game_grid[x][y] = 0;
                        let _ = term.clear_screen();
                        y -= 1;
                        _game_grid[x][y] = 1;
                        print_grid(&term, &_game_grid);
                    }
                }
                'w' => {
                    // UP
                    if x > 0 {
                        _game_grid[x][y] = 0;
                        let _ = term.clear_screen();
                        x -= 1;
                        _game_grid[x][y] = 1;
                        print_grid(&term, &_game_grid);
                    }
                }
                'a' => {
                    // LEFT
                    if y < height - 1 {
                        _game_grid[x][y] = 0;
                        let _ = term.clear_screen();
                        y += 1;
                        _game_grid[x][y] = 1;
                        print_grid(&term, &_game_grid);
                    }
                }
                's' => {
                    // DOWN
                    if x < width - 1 {
                        _game_grid[x][y] = 0;
                        let _ = term.clear_screen();
                        x += 1;
                        _game_grid[x][y] = 1;
                        print_grid(&term, &_game_grid);
                    }
                }
                'q' => {
                    let _ = term.clear_screen();
                    println!("Ok I am quitting!!!!!....");
                    break 'game_loop;
                }

                _ => {
                    println!("Entered Wrong key..... {} ", character);
                    continue;
                }
            }
        }
    }
}

fn print_grid(term: &Term, grid: &Vec<Vec<i32>>) {
    for row in grid {
        let data: Vec<_> = row.iter().rev().map(|r| r.to_string()).collect();
        let final_row = data.join(" ");
        let _ = term.write_line(&final_row);
    }
}

fn print_initial_stuff(term: &Term) {
    let _ = term.write_line("----------RUST VECTOR GAME---------");
    let _ = term.write_line("       W -> UP");
    let _ = term.write_line("       A -> LEFT");
    let _ = term.write_line("       S -> BOTTOM");
    let _ = term.write_line("       D -> RIGHT");
    let _ = term.write_line("       Q -> QUIT GAME");
    let _ = term.write_line("-----------------------------");
}
