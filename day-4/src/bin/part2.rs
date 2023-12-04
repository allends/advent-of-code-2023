use std::collections::{HashMap, BTreeMap};

fn common_numbers(vec1: &Vec<u32>, vec2: &Vec<u32>) -> u32 {

    let mut vec1 = vec1.clone();
    let mut vec2 = vec2.clone();

    vec1.sort();
    vec2.sort();

    let mut index1 = 0;
    let mut index2 = 0;

    let mut matches = 0;

    loop {
        let number1 = vec1.get(index1);
        let number2 = vec2.get(index2);

        if number1.is_none() || number2.is_none() {
            return matches;
        }

        let number1 = number1.unwrap();
        let number2 = number2.unwrap();

        if number1 == number2 {
            matches += 1;
            index1 += 1;
            index2 += 1;
        }

        if number1 > number2 {
            index2 += 1;
        }

        if number2 > number1 {
            index1 += 1;
        }
    }
}

fn main() {
    let input = include_str!("./input.txt");

    let mut copies = BTreeMap::new();

    let lines: Vec<&str> = input.lines().collect();

    for index in 0..lines.len() {
        copies.insert(index, 1);
    }

    lines.iter().enumerate().for_each(|(index, line)| {
        let mut parts = line.split(':');
        let game_info = parts.next().unwrap();
        let number_info = parts.next().unwrap().trim();

        let card_copies = copies.get(&index).unwrap().clone();

        println!("{game_info} has {card_copies} copies");

        let mut numbers = number_info.split('|');
        let winning: Vec<u32> = numbers
            .next()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|x| {
                x.parse::<u32>().unwrap_or(0)
            })
            .collect();


        let pulled: Vec<u32> = numbers
            .next()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|x| {
                x.parse::<u32>().unwrap_or(0)
            })
            .collect();

        let matches = common_numbers(&winning, &pulled);

        for offset in 1..matches+1 {
            let updating_index = index + offset as usize;
            let existing_copies = copies.get(&updating_index);

            if existing_copies.is_none() {
                return;
            }

            let existing_copies = existing_copies.unwrap();

            // println!("updating game {updating_index} to {existing_copies} + {card_copies}");

            copies.insert(updating_index, existing_copies + card_copies);
        }
    });

    let mut total_copies = 0;

    copies.iter().for_each(|(_key, value)| {
        total_copies += value;
    });

    println!("{total_copies}");

}