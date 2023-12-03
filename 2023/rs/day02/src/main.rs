use std::time::SystemTime;

const DATA: &'static str = include_str!("../data.txt");

fn part_one() -> i32 {
    let mut sum = 0;

    'game: for line in DATA.trim().lines() {
        let mut it = line.split(": ");

        let game = it
            .next()
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .parse::<i32>()
            .unwrap();

        for rounds in it.next().unwrap().split("; ") {
            for round in rounds.split(", ") {
                println!("round: {round}");
                let mut it = round.split(" ");

                let count = it.next().unwrap().parse::<i32>().unwrap();
                let color = it.next().unwrap();

                match color {
                    "red" if count > 12 => continue 'game,
                    "green" if count > 13 => continue 'game,
                    "blue" if count > 14 => continue 'game,
                    _ => {}
                }
            }
        }

        sum += game
    }

    sum
}

fn part_two() -> i32 {
    let mut sum = 0;

    for line in DATA.trim().lines() {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for rounds in line.split(": ").last().unwrap().split("; ") {
            for round in rounds.split(", ") {
                println!("round: {round}");
                let mut it = round.split(" ");

                let count = it.next().unwrap().parse::<i32>().unwrap();
                let color = it.next().unwrap();

                match color {
                    "red" => red = red.max(count),
                    "green" => green = green.max(count),
                    "blue" => blue = blue.max(count),
                    _ => unreachable!(),
                }
            }
        }

        sum += red * green * blue;
    }

    sum
}

fn time_it<F, T>(fun: F) -> T
where
    F: Fn() -> T,
{
    let start = SystemTime::now();
    let result = fun();
    println!("Time elapsed: {} µs", start.elapsed().unwrap().as_micros());
    result
}

fn main() {
    time_it(|| println!("part 1: {}", part_one()));
    time_it(|| println!("part 2: {}", part_two()));
}
