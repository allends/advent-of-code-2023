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

fn scratch_off_score(raw: u32) -> u32 {

    if raw <= 1 {
        return raw;
    }

    (2 as u32).pow(raw - 1)
}

fn main() {
    let input = include_str!("./input.txt");

    let mut score = 0;

    input.lines().for_each(|line| {
        let mut parts = line.split(':');
        let _game_info = parts.next().unwrap();
        let number_info = parts.next().unwrap().trim();

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
        let computed_score = scratch_off_score(matches);

        score += computed_score;
    });

    println!("Score: {score}");

}