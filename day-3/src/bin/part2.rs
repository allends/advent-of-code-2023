#[derive(Debug, Clone)]
struct Number {
    value: i32,
    start: (u32, u32),
    end: (u32, u32),
}

#[derive(Debug)]
struct Symbol {
    value: char,
    position: (u32, u32),
}

fn is_surrounding(symbol: &Symbol, number: &Number) -> bool {
    let y_distance = i32::abs(number.start.0 as i32 - symbol.position.0 as i32);
    let x_distance_start = i32::abs(number.start.1 as i32 - symbol.position.1 as i32);
    let x_distance_end = i32::abs(number.end.1 as i32 - symbol.position.1 as i32);

    if y_distance <= 1 && (x_distance_start <= 1 || x_distance_end <= 1) {
        return true;
    }

    return false;
}

fn main() {
    let input = include_str!("./input.txt");

    let mut numbers = Vec::new();
    let mut symbols = Vec::new();

    for (row, line) in input.lines().enumerate() {
        let mut current_number = 0;
        for (col, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                current_number = current_number * 10 + c.to_digit(10).unwrap();
            } else {
                if current_number != 0 {
                    let number = Number {
                        value: current_number as i32,
                        start: (row as u32, col as u32 - current_number.to_string().len() as u32),
                        end: (row as u32, col as u32 - 1),
                    };
                    numbers.push(number);
                }
                current_number = 0;
            }

            if c != '.' && !c.is_digit(10) {
                let symbol = Symbol {
                    value: c,
                    position: (row as u32, col as u32),
                };
                symbols.push(symbol);
            }

        }
        if current_number != 0 {
            let number = Number {
                value: current_number as i32,
                start: (row as u32, line.len() as u32 - current_number.to_string().len() as u32),
                end: (row as u32, line.len() as u32 - 1),
            };
            numbers.push(number);
        }
    }

    let mut sum = 0;

    for symbol in &symbols {
        let mut intersections = Vec::new();
        for number in &numbers {
            if is_surrounding(symbol, &number) {
                intersections.push(number);
            }
            if intersections.len() > 2 {
                break;
            }
        }
        if intersections.len() == 2 {
            let gear_ratio = intersections[0].value * intersections[1].value;
            sum += gear_ratio;
        }
    }

    println!("Sum: {}", sum);
}