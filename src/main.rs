mod game;

use std::{collections::HashMap, io::stdin};

use game::{Game, Ship};

fn read_line(line: &mut String) {
    line.clear();
    match stdin().read_line(line) {
        Ok(_) => *line = line.trim().to_owned(),
        Err(_) => line.clear(),
    }
}

fn clear() {
    print!("{}[2J", 27 as char);
}

fn main() {
    let mut game = Game::new();
    let mut input = String::new();
    clear();
    let mut ships = HashMap::from([
        (Ship::PatrolBoat, false),
        (Ship::Submarine, false),
        (Ship::Destroyer, false),
        (Ship::Battleship, false),
        (Ship::Carrier, false),
    ]);
    while game.is_placing() {
        println!("Commands: [done], [exit], [random], [(P|S|D|B|C),x1,y1,x2,y2]");
        game.print();
        read_line(&mut input);
        let input2 = input.to_uppercase();
        let input3 = input2.split(' ').collect::<Vec<&str>>();
        if let ["DONE"] = input3[..] {
            if ships.values().all(|v| *v) {
                game.play();
            } else {
                println!("You need to place all your boat before playing.");
            }
        } else if let ["EXIT"] = input3[..] {
            println!("GOOD BYE");
            return;
        } else if let [ship, x1, y1, x2, y2] = input3[..] {
            let Ok(ship) = Ship::try_from(ship) else {
                println!("Command format error");
                continue;
            };
            let Ok(x1) = x1.parse::<usize>() else {
                println!("Command format error");
                continue;
            };
            let Ok(y1) = y1.parse::<usize>() else {
                println!("Command format error");
                continue;
            };
            let Ok(x2) = x2.parse::<usize>() else {
                println!("Command format error");
                continue;
            };
            let Ok(y2) = y2.parse::<usize>() else {
                println!("Command format error");
                continue;
            };
            if game.place(ship, x1, y1, x2, y2).is_some() {
                *ships.get_mut(&ship).expect("Must contains all ship") = true;
                clear();
            } else {
                println!("Command format error or slot used");
            }
        } else if let ["RANDOM"] = input3[..] {
            game.random(&mut ships);
            clear();
        } else {
            println!("Command not found");
        }
    }
    while game.is_playing() {
        println!("Commands: [exit], [x,y]");
        game.print();
        read_line(&mut input);
        let input2 = input.to_uppercase();
        let input3 = input2.split(' ').collect::<Vec<&str>>();
        if let ["EXIT"] = input3[..] {
            println!("GOOD BYE");
            return;
        } else if let [x, y] = input3[..] {
            let Ok(x) = x.parse::<usize>() else {
                println!("Command format error");
                continue;
            };
            let Ok(y) = y.parse::<usize>() else {
                println!("Command format error");
                continue;
            };
            match game.hit(x, y) {
                game::End::Error => println!("Command format error or slot used"),
                game::End::Continue => clear(),
                game::End::Win => {
                    println!("You Win");
                    break;
                }
                game::End::Lose => {
                    println!("You Lose");
                    break;
                }
            }
        } else {
            println!("Command not found");
        }
    }
}
