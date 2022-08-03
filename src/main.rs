use std::io;
use std::io::Write;

mod game;
mod input;
fn draw(stdout: &mut io::Stdout, field: &Vec<Vec<bool>>) {
    for row in field {
        for col in row {
            write!(stdout, "{} ", {
                if *col {
                    "X"
                } else {
                    "0"
                }
            })
            .unwrap();
        }
        write!(stdout, "\n").unwrap();
    }
    println!("-----------");
}

fn main() {
    println!("bidde nummer weite eingeben");
    let stdin = io::stdin();
    let width: usize;
    {
        let mut buffer = String::new();
        stdin.read_line(&mut buffer).unwrap();
        width = buffer.trim().parse::<usize>().unwrap_or(8);
    }
    println!("bidde nummer höhe eingeben");
    let height: usize;
    {
        let mut buffer = String::new();
        stdin.read_line(&mut buffer).unwrap();
        height = buffer.trim().parse::<usize>().unwrap_or(8);
    }
    #[cfg(debug_assertions)]
    println!("{} {}", height, width);
    let mut game: game::Game = game::Game::new(height, width);

    let mut stdout = io::stdout();

    let mut buffer: String;
    loop {
        draw(&mut stdout, &game.get_field());
        buffer = String::new();
        stdin.read_line(&mut buffer).unwrap();
        match buffer.trim() {
            "q" => break,
            "c" => flip(&stdin, &mut game),
            _ => {
                game.step();
            }
        }
    }
}

fn flip(stdin: &io::Stdin, game: &mut game::Game) {
    let mut buffer = String::new();
    stdin.read_line(&mut buffer).unwrap();
    let list = input::parse(buffer);
    for pair in list.iter() {
        if ((pair.x.end - pair.x.start) < 1000) && ((pair.y.end - pair.y.start) < 1000) {
            for x in pair.x.start..pair.x.end {
                for y in pair.y.start..pair.y.end {
                    game.flip(x, y);
                }
            }
        }
    }
}
