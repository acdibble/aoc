use std::time::SystemTime;

const DATA: &'static str = include_str!("../data.txt");

type Disk = Vec<Option<i32>>;

fn parse_disk_data() -> Disk {
    let mut id = 0;
    let mut is_file = true;

    let mut disk = Vec::new();

    for ch in DATA.trim().chars() {
        let amount = ch.to_digit(10).unwrap();

        let mut value = None;
        if is_file {
            value = Some(id);
            id += 1;
        }

        is_file = !is_file;

        for _ in 0..amount {
            disk.push(value);
        }
    }

    disk
}

fn calculate_checksum(disk: &Disk) -> u64 {
    disk.iter()
        .enumerate()
        .map(|(index, v)| v.unwrap_or(0) as u64 * index as u64)
        .sum()
}

fn part_one() -> u64 {
    let mut disk = parse_disk_data();

    let mut head = 0;
    let mut tail = disk.len() - 1;

    loop {
        while disk[head].is_some() {
            head += 1;
        }

        while disk[tail].is_none() {
            tail -= 1;
        }

        if head >= tail {
            break;
        }

        disk.swap(head, tail);
    }

    calculate_checksum(&disk)
}

fn part_two() -> u64 {
    let mut disk = parse_disk_data();

    let mut end = disk.len() - 1;

    while end > 1 {
        while disk[end].is_none() {
            end -= 1;
        }

        let mut start = end;

        while disk[start] == disk[end] {
            start -= 1;
            if start == 0 {
                break;
            }
        }

        start += 1;

        let len = end - start;

        let mut search_start = 1;
        let mut search_end = search_start + len;

        while search_end < start {
            if disk[search_start..=search_end].iter().all(Option::is_none) {
                for i in 0..=len {
                    disk.swap(start + i, search_start + i);
                }
                break;
            }

            search_start += 1;
            search_end += 1;
        }

        end = start - 1;
    }

    calculate_checksum(&disk)
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
