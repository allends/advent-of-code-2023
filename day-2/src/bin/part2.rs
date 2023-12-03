use std::collections::HashMap;



fn main() {
    let input = include_str!("./input1.txt");

    let lines = input.lines();

    let mut total = 0;

    for line in lines {
        let mut iter = line.split(':');
        let _game_info = iter.next().unwrap();
        let game_content = iter.next().unwrap();

        let rounds = game_content.split(';');

        let mut min_colors: HashMap<&str, i32> = HashMap::new();

        for round in rounds {
            let sightings = round.split(',');

            for sighting in sightings {
                let sighting = sighting.trim();
                let mut parts = sighting.split(' ');

                let number = parts.next().unwrap().parse::<i32>().unwrap_or(0);
                let color = parts.next().unwrap();

                let min_color = min_colors.get(color).unwrap_or(&0);

                min_colors.insert(color, number.max(*min_color));
            }
        }

        println!("min_colors: {:?}", min_colors);

        let mut power = 1;

        for (_, min_color) in min_colors {
            power *= min_color;
        }

        total += power;

    }

    println!("total: {}", total);
}
