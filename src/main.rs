mod item;
mod format;

use item::Item;
use format::Format;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;
use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    let mut typeConsts: HashMap<String, String> = HashMap::new();
    let item = Item::new();
    //println!("{}", item);

    let path = args.last().expect("Requires path");
    println!("{}", path);
    let re = load(path.clone());
    println!("{:?}", re);
}

fn load(path: String) -> io::Result<()> {
    println!("{}", path);
    let lines = read_from_file(path);
    let items = parse_items(lines);

    Ok(())
}

#[inline]
fn read_from_file(path: String) -> Vec<String> {
    let mut s = String::new();
    File::open(path).unwrap().read_to_string(&mut s);
    s.lines().map(|l| l.to_string())
        .map(|l| {println!("{}", l); l})
        .collect::<Vec<String>>()
}

fn parse_items(mut lines: Vec<String>) -> Vec<Item> { //TODO
    let mut i = lines.into_iter();
    Vec::new()
}