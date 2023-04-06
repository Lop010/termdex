use crossterm::terminal::{ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use fuzzy_matcher::clangd::fuzzy_match;
use serde::{self, Deserialize};
use serde_json::{self, Map};
use std::env;
use std::io::Read;
use std::process::{Command, Output};
use std::ptr::null;
use std::{collections::HashMap, io::stdout};
use std::{default, io, thread};
use terminal_size::{terminal_size, Height, Width};

use crossterm::{
    cursor::MoveTo,
    event::{self, read, Event, KeyCode, KeyEvent},
    execute, Result,
};

use crate::hashmaps::stathashmap;

#[macro_use]
extern crate serde_derive;

mod hashmaps;

//  PokeAPI JSON Struct
#[derive(Deserialize)]
struct State {
    typing_enabled: bool,
    checkifyplusone: i32,
    keyY: i32,
    keyx: i32,
    inputstr: String,
    linestr: String,
    prevlinestr: String,
}

#[derive(Deserialize)]
pub struct Pokemon {
    stats: Vec<PokeAPIStats>,
    species: PokeAPISpecies,
    sprites: PokeAPISprites,
    id: i32,
    moves: Vec<PokeAPIMoves>,
}

#[derive(Deserialize)]
struct PokeAPIMoves {
    version_group_details: Vec<PokeAPIVerDet>,
    r#move: PokeAPIMove,
}

#[derive(Deserialize)]
struct PokeAPIVerDet {
    level_learned_at: i32,
    move_learn_method: PokeAPIMoveLearn,
    version_group: PokeAPIVerGroup,
}

#[derive(Deserialize)]
struct PokeAPIMove {
    name: String,
}

#[derive(Deserialize)]
struct PokeAPIMoveLearn {
    name: String,
}

#[derive(Deserialize)]
struct PokeAPIVerGroup {
    name: String,
}

#[derive(Deserialize)]
struct PokeAPISpecies {
    name: String,
}

#[derive(Deserialize)]
struct PokeAPISprites {
    front_default: String,
}

#[derive(Deserialize)]
struct PokeAPIStats {
    stat: PokeAPIStat,
    base_stat: i32,
}

#[derive(Deserialize)]
struct PokeAPIStat {
    name: String,
}
// \ PokeAPI JSON Structure

// / Cool Functions

pub fn get_pokemon_data(url: &str) -> Pokemon {
    let client = reqwest::blocking::Client::new();
    let res = client.get(url).send();

    let pokemon: Pokemon = res.unwrap().json().unwrap();

    return pokemon;
}

// Searches for a Pokemon in a hashmap using fuzzy matching on user input.
fn pokesearch(input: &str) -> String {
    let pokemonhashmap: HashMap<String, String> = hashmaps::pokehashmap();

    let input = input.to_lowercase();

    let mut finalvalue: String = "".to_string();
    let mut finalkey: String = "".to_string();
    for (key, value) in pokemonhashmap {
        if !(fuzzy_match(&input, &key) == None) {
            let score: i64 = fuzzy_match(&input, &key).unwrap();

            finalvalue = value;
            finalkey = key;
        }
    }

    return finalvalue + ", " + &finalkey;
}

pub fn read_line() -> Result<String> {
    let mut line = String::new();
    while let Event::Key(KeyEvent { code, .. }) = event::read()? {
        match code {
            KeyCode::Enter => {
                break;
            }

            KeyCode::Char(c) => {
                line.push(c);
            }
            _ => {}
        }
    }

    return Ok(line);
}

pub fn movecursorn(x: u16, y: u16) {
    execute!(stdout(), crossterm::cursor::MoveTo(x, y)).expect("Error Moving Cursor");
}

pub fn printat(x: u16, y: u16, msg: &str) {
    execute!(stdout(), crossterm::cursor::MoveTo(x, y)).expect("msg");
    print!("{}", msg)
}

pub fn cursorhide() {
    execute!(stdout(), crossterm::cursor::Hide).expect("msg");
    execute!(stdout(), crossterm::cursor::MoveTo(0, 0)).expect("msg");
}
pub fn cursorshow() {
    execute!(stdout(), crossterm::cursor::Show).expect("msg");
}

fn print_move_info(w: u16, h: u16, minush: u16, move_name: &str, move_info: &PokeAPIMoves) {
    // Clear previous move
    printat(w - 22, h - minush, &format!("{}", "                     "));

    printat(w - 20, h - minush, &format!("{}: ", move_name));
    
    // Learn Method
    // clear previous learn method
    printat(w - 22, h - minush - 1, &format!("{}", "                     "));
    if move_info.version_group_details[0].move_learn_method.name == "level-up" {
        printat(
            w - 20,
            h - minush-1,
            &format!("  Learn At: {}", move_info.version_group_details[0].level_learned_at),
        );
    } else {
        printat(
            w - 20,
            h - minush-1,
            &format!("  {}", move_info.version_group_details[0].move_learn_method.name),
        );
    }
}


pub fn mainmenu() {
    let size = terminal_size().expect("msg");

    let (Width(w), Height(h)) = size;

    movecursorn(w - 59, 0);
    println!("┌───────────────────────────────────────────Enter─Pokemon─┐");

    printat(w - 23, h - 21, "┌───Change─Page──◄ ►──┐");
    printat(w - 23, h - 20, "├───────Moves─1───────┤");
    printat(w - 23, h - 19, "│                     │");
    printat(w - 23, h - 18, "│                     │");
    printat(w - 23, h - 17, "│                     │");
    printat(w - 23, h - 16, "│                     │");
    printat(w - 23, h - 15, "│                     │");
    printat(w - 23, h - 14, "│                     │");
    printat(w - 23, h - 13, "│                     │");
    printat(w - 23, h - 12, "│                     │");
    printat(w - 23, h - 11, "│                     │");
    printat(w - 23, h - 10, "└─────────────────────┘");

    printat(0, h - 21, "┌────────Stats─────────┐");
    printat(0, h - 20, "│                      │");
    printat(0, h - 19, "│                      │");
    printat(0, h - 18, "│                      │");
    printat(0, h - 17, "│                      │");
    printat(0, h - 16, "│                      │");
    printat(0, h - 15, "│                      │");
    printat(0, h - 14, "│                      │");
    printat(0, h - 13, "│                      │");
    printat(0, h - 12, "│                      │");
    printat(0, h - 11, "│                      │");
    printat(0, h - 10, "│                      │");
     printat(0, h - 9, "│                      │");
     printat(0, h - 8, "│                      │");
     printat(0, h - 7, "│                      │");
     printat(0, h - 6, "│                      │");
     printat(0, h - 5, "│                      │");
     printat(0, h - 4, "│                      │");
     printat(0, h - 3, "└──────────────────────┘");

    // 7 Lines
    printat(w - 23, h - 9, "┌───────Species───────┐");
    printat(w - 23, h - 8, "│                     │");
    printat(w - 23, h - 7, "│ Name:               │");
    printat(w - 23, h - 6, "│                     │");
    printat(w - 23, h - 5, "│ ID:                 │");
    printat(w - 23, h - 4, "│                     │");
    printat(w - 23, h - 3, "└─────────────────────┘");
    // 7 Lines
}


/// App holds the state of the application
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut imgenable: bool = false;
    let mut pokemonmode = true;
    let mut movepage = 1;

    let mut prevpoke: String = String::new();

    let mut twooffset = 0;
    let mut oneoffset = 0;
    let mut threeoffset = 0;

    execute!(stdout(), EnterAlternateScreen).expect("msg");
    crossterm::terminal::SetTitle("TermDex");

    let size = terminal_size().expect("msg");

    let (Width(w), Height(h)) = size;

    let mut i = 0;

    // Add UI
    mainmenu();

    while i == 0 {
        movecursorn(0, 0);

        movecursorn(w - 57, 1);

        let mut input = read_line().expect("Oh No");
        if !(input == "q") {
            let res: String = pokesearch(&input);
            let list: Vec<&str> = res.split(",").collect();

            let url = format!(
                "{}{}",
                "https://pokeapi.co/api/v2/pokemon/",
                list[1].to_string().replace(" ", "")
            );

            if !(pokemonmode) {
                movecursorn(w - 59, 0);
                println!("┌─────────────────────────────────────────UI─Control─Mode─┐");
            }

            if input == "r" {
                pokemonmode = false
            }


            let pokemoninfo = get_pokemon_data(&url);
            let pokeinfo = &pokemoninfo;

            movecursorn(w - 59, 1);
            println!("│                                                         │");

            if pokemonmode {
                if input == "q" 
                {
                    i = 1 // Exits the loop if you input "q"
                } else {
                    //If not continue

                    prevpoke = input;
                    movecursorn(1, 10);

                    cursorhide();

                    printat(w - 15, h - 7, "             ");
                    printat(w - 15, h - 7, list[0]);

                    // Get info about pokemon from PokeAPI

                    let pokeid: &str = &format!("{}", pokemoninfo.id);

                    let mut hoff = 20;
                    for i in &pokemoninfo.stats {
                        printat(2, h - hoff, &"Name:                ");
                        printat(2, h - hoff + 1, "Base Stat:       ");

                        printat(2, h - hoff, &format!("Name: {}", stathashmap(&i.stat.name)));
                        printat(2, h - hoff + 1, &format!("Base Stat: {}", &i.base_stat));

                        hoff = hoff - 3;
                    }

                    let one;
                    let two;
                    let three = movepage * 3 + threeoffset;

                    one = three - 2 + oneoffset;
                    two = three - 1 + twooffset;

                    let pokeinfo = &pokemoninfo;

                    print_move_info(w, h, 19, &pokeinfo.moves[one].r#move.name, &pokeinfo.moves[one]);
                    print_move_info(w, h, 16, &pokeinfo.moves[two].r#move.name, &pokeinfo.moves[two]);
                    print_move_info(w, h, 13, &pokeinfo.moves[three].r#move.name, &pokeinfo.moves[three]);

                    let (n, m) = (1 - 1, 2 - 1);
                    printat(w - 15, h - 5, "    ");
                    printat(w - 15, h - 5, pokeid);

                    movecursorn(w - 57, 1);
                    cursorshow()
                }
            } else {
                let res: String = pokesearch(&prevpoke);
                let list: Vec<&str> = res.split(",").collect();

                let url = format!(
                    "{}{}",
                    "https://pokeapi.co/api/v2/pokemon/",
                    list[1].to_string().replace(" ", "")
                );

                movecursorn(w - 59, 0);
                println!("┌─────────────────────────────────────────UI─Control─Mode─┐");

                if input == "n" {
                    let pokemovelen = pokemoninfo.moves.len();

                    movepage = movepage + 1;

                    // Update Moves Display
                    let mut one;
                    let mut two;
                    let mut three = movepage * 3 + threeoffset;

                    one = three - 2;
                    two = three - 1;
                    movecursorn(w - 23, h - 20);
                    println!("├───────Moves─{}───────┤", movepage);

                    print_move_info(w, h, 18, &pokeinfo.moves[one].r#move.name, &pokeinfo.moves[one]);
                    print_move_info(w, h, 15, &pokeinfo.moves[two].r#move.name, &pokeinfo.moves[two]);
                    print_move_info(w, h, 12, &pokeinfo.moves[three].r#move.name, &pokeinfo.moves[three]);
                    //listmoves(pokeinfo, one, two, movepage);
                }

                if input == "p" {
                    let pokemoninfo = get_pokemon_data(&url);

                    // If movepage isnt 1 (PRevents it from going into - numbers)
                    if !(movepage == 1) {
                        movepage = movepage - 1;
                    }
                    movecursorn(w - 23, h - 20);
                    println!("├───────Moves─{}───────┤", movepage);

                    // Update Moves Diaplay
                    let mut one;
                    let mut two;
                    let mut three = movepage * 3 + threeoffset;

                    one = three - 2 + oneoffset;
                    two = three - 1 + twooffset;
                    print_move_info(w, h, 18, &pokeinfo.moves[one].r#move.name, &pokeinfo.moves[one]);
                    print_move_info(w, h, 15, &pokeinfo.moves[two].r#move.name, &pokeinfo.moves[two]);
                    print_move_info(w, h, 12, &pokeinfo.moves[three].r#move.name, &pokeinfo.moves[three]);
                }

                if input == "search" {
                    pokemonmode = true;
                    movepage = 1;
                    movecursorn(w - 59, 0);
                    mainmenu();
                }

                if input == "q" {
                    i = 1;
                }

            }
        } else {
            i = 1
        }
    }
    cursorshow();
    execute!(stdout(), LeaveAlternateScreen).expect("msg");
}
