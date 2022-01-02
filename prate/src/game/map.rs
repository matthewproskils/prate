use crate::game::tile::Tile;
use std::{fs};

pub struct Map {
    pub tiles: Vec<Vec<Tile>>,
    pub width: usize,
    pub height: usize,
}

//split a string into a vector of chars and remove the last char
fn split_and_rem_last(y: usize, input: &str) -> Vec<Tile> {
    let mut output = input.chars().collect::<Vec<char>>();
    output = output.iter().filter(|&&x| x != '\r').map(|x| *x).collect::<Vec<char>>();

    let mut output2: Vec<Tile> = Vec::new();

    for (x, c) in output.iter().enumerate() {
      output2.push(Tile::new(*c, x as f64, y as f64, 1.0, 1.0));
    }

    output2
}

pub fn load_map(map_src: &str) -> Map {
    let map_raw =
        fs::read_to_string(format!("{}{}", "src/data/", map_src)).expect("Unable to read file");

    let map_split: Vec<&str> = map_raw.split('\n').collect();

    let mut map: Vec<Vec<Tile>> = Vec::new();

    for (i, s) in map_split.iter().enumerate(){
      map.push(split_and_rem_last(i, s))
    }

    let mw = map[0].len();
    let mh = map.len();

    return Map {
        tiles: map,
        width: mw,
        height: mh,
    };
}
