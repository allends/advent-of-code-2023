fn is_valid(color: &str, number: i32) -> bool {
    match color {
        "red" => number <= 12,
        "green" => number <= 13,
        "blue" => number <= 14,
        _ => false,
    }
}

fn main() {
    let input = include_str!("./input1.txt");

    let lines = input.lines();

    let mut total = 0;

    for line in lines {
        let mut iter = line.split(':');
        let game_info = iter.next().unwrap();
        let game_content = iter.next().unwrap();

        let game_id = game_info.split(' ').nth(1).unwrap().parse::<i32>().unwrap_or(0);
        let rounds = game_content.split(';');

        let mut valid = true;

        for round in rounds {
            let sightings = round.split(',');

            for sighting in sightings {
                let sighting = sighting.trim();
                let mut parts = sighting.split(' ');

                let number = parts.next().unwrap().parse::<i32>().unwrap_or(0);
                let color = parts.next().unwrap();

                if !is_valid(color, number) {
                    valid = false;
                    break;
                }
            }
        }
        if valid {
            total += game_id;
        }
    }

    println!("total: {}", total);
}
