use std::collections::VecDeque;

const NUMBER_OF_PLAYERS: u32 = 405;
const NUMBER_OF_MARBLES: u32 = 71700 * 100;

fn main() {
    let mut marbles = VecDeque::new();
    marbles.push_back(0);
    let mut scores = vec![0; NUMBER_OF_PLAYERS as usize];

    for next_marble in 1..=NUMBER_OF_MARBLES {
        if next_marble % 23 != 0 {
            marbles.rotate_left(1);
            marbles.rotate_left(1);
            marbles.push_front(next_marble);
        } else {
            marbles.rotate_right(7);
            scores[(next_marble % NUMBER_OF_PLAYERS) as usize] +=
                marbles.pop_front().unwrap() + next_marble;
        }
    }

    println!("{}", scores.iter().max().unwrap());
}
