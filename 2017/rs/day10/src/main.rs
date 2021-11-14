use std::collections::VecDeque;
use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

fn u8_to_hex(value: u8) -> &'static str {
    match value {
        0 => "00",
        1 => "01",
        2 => "02",
        3 => "03",
        4 => "04",
        5 => "05",
        6 => "06",
        7 => "07",
        8 => "08",
        9 => "09",
        10 => "0a",
        11 => "0b",
        12 => "0c",
        13 => "0d",
        14 => "0e",
        15 => "0f",
        16 => "10",
        17 => "11",
        18 => "12",
        19 => "13",
        20 => "14",
        21 => "15",
        22 => "16",
        23 => "17",
        24 => "18",
        25 => "19",
        26 => "1a",
        27 => "1b",
        28 => "1c",
        29 => "1d",
        30 => "1e",
        31 => "1f",
        32 => "20",
        33 => "21",
        34 => "22",
        35 => "23",
        36 => "24",
        37 => "25",
        38 => "26",
        39 => "27",
        40 => "28",
        41 => "29",
        42 => "2a",
        43 => "2b",
        44 => "2c",
        45 => "2d",
        46 => "2e",
        47 => "2f",
        48 => "30",
        49 => "31",
        50 => "32",
        51 => "33",
        52 => "34",
        53 => "35",
        54 => "36",
        55 => "37",
        56 => "38",
        57 => "39",
        58 => "3a",
        59 => "3b",
        60 => "3c",
        61 => "3d",
        62 => "3e",
        63 => "3f",
        64 => "40",
        65 => "41",
        66 => "42",
        67 => "43",
        68 => "44",
        69 => "45",
        70 => "46",
        71 => "47",
        72 => "48",
        73 => "49",
        74 => "4a",
        75 => "4b",
        76 => "4c",
        77 => "4d",
        78 => "4e",
        79 => "4f",
        80 => "50",
        81 => "51",
        82 => "52",
        83 => "53",
        84 => "54",
        85 => "55",
        86 => "56",
        87 => "57",
        88 => "58",
        89 => "59",
        90 => "5a",
        91 => "5b",
        92 => "5c",
        93 => "5d",
        94 => "5e",
        95 => "5f",
        96 => "60",
        97 => "61",
        98 => "62",
        99 => "63",
        100 => "64",
        101 => "65",
        102 => "66",
        103 => "67",
        104 => "68",
        105 => "69",
        106 => "6a",
        107 => "6b",
        108 => "6c",
        109 => "6d",
        110 => "6e",
        111 => "6f",
        112 => "70",
        113 => "71",
        114 => "72",
        115 => "73",
        116 => "74",
        117 => "75",
        118 => "76",
        119 => "77",
        120 => "78",
        121 => "79",
        122 => "7a",
        123 => "7b",
        124 => "7c",
        125 => "7d",
        126 => "7e",
        127 => "7f",
        128 => "80",
        129 => "81",
        130 => "82",
        131 => "83",
        132 => "84",
        133 => "85",
        134 => "86",
        135 => "87",
        136 => "88",
        137 => "89",
        138 => "8a",
        139 => "8b",
        140 => "8c",
        141 => "8d",
        142 => "8e",
        143 => "8f",
        144 => "90",
        145 => "91",
        146 => "92",
        147 => "93",
        148 => "94",
        149 => "95",
        150 => "96",
        151 => "97",
        152 => "98",
        153 => "99",
        154 => "9a",
        155 => "9b",
        156 => "9c",
        157 => "9d",
        158 => "9e",
        159 => "9f",
        160 => "a0",
        161 => "a1",
        162 => "a2",
        163 => "a3",
        164 => "a4",
        165 => "a5",
        166 => "a6",
        167 => "a7",
        168 => "a8",
        169 => "a9",
        170 => "aa",
        171 => "ab",
        172 => "ac",
        173 => "ad",
        174 => "ae",
        175 => "af",
        176 => "b0",
        177 => "b1",
        178 => "b2",
        179 => "b3",
        180 => "b4",
        181 => "b5",
        182 => "b6",
        183 => "b7",
        184 => "b8",
        185 => "b9",
        186 => "ba",
        187 => "bb",
        188 => "bc",
        189 => "bd",
        190 => "be",
        191 => "bf",
        192 => "c0",
        193 => "c1",
        194 => "c2",
        195 => "c3",
        196 => "c4",
        197 => "c5",
        198 => "c6",
        199 => "c7",
        200 => "c8",
        201 => "c9",
        202 => "ca",
        203 => "cb",
        204 => "cc",
        205 => "cd",
        206 => "ce",
        207 => "cf",
        208 => "d0",
        209 => "d1",
        210 => "d2",
        211 => "d3",
        212 => "d4",
        213 => "d5",
        214 => "d6",
        215 => "d7",
        216 => "d8",
        217 => "d9",
        218 => "da",
        219 => "db",
        220 => "dc",
        221 => "dd",
        222 => "de",
        223 => "df",
        224 => "e0",
        225 => "e1",
        226 => "e2",
        227 => "e3",
        228 => "e4",
        229 => "e5",
        230 => "e6",
        231 => "e7",
        232 => "e8",
        233 => "e9",
        234 => "ea",
        235 => "eb",
        236 => "ec",
        237 => "ed",
        238 => "ee",
        239 => "ef",
        240 => "f0",
        241 => "f1",
        242 => "f2",
        243 => "f3",
        244 => "f4",
        245 => "f5",
        246 => "f6",
        247 => "f7",
        248 => "f8",
        249 => "f9",
        250 => "fa",
        251 => "fb",
        252 => "fc",
        253 => "fd",
        254 => "fe",
        255 => "ff",
    }
}

struct KnotHash {
    elements: VecDeque<u8>,
    stack: Vec<u8>,
    skip_amount: usize,
    front: usize,
    element_count: usize,
}

impl KnotHash {
    fn new() -> Self {
        let elements = (0..=u8::MAX).collect::<VecDeque<u8>>();
        Self {
            element_count: elements.len(),
            stack: vec![],
            skip_amount: 0,
            front: 0,
            elements,
        }
    }

    fn run(&mut self, inputs: &Vec<u8>) -> i32 {
        for &length in inputs {
            for _ in 0..length {
                self.stack.push(self.elements.pop_front().unwrap());
            }

            while let Some(el) = self.stack.pop() {
                self.elements.push_back(el);
            }

            for _ in 0..self.skip_amount {
                let temp = self.elements.pop_front().unwrap();
                self.elements.push_back(temp);
            }

            self.front = (self.front + self.skip_amount + length as usize) % self.element_count;
            self.skip_amount = (self.skip_amount + 1) % self.element_count;
        }

        let true_front = (self.element_count - self.front) as usize;

        *self.elements.get(true_front).unwrap() as i32
            * *self.elements.get(true_front + 1).unwrap() as i32
    }

    fn digest(mut self) -> String {
        for _ in 0..self.element_count - self.front {
            let temp = self.elements.pop_front().unwrap();
            self.elements.push_back(temp);
        }

        self.elements
            .make_contiguous()
            .chunks(16)
            .map(|numbers| {
                let mut value = 0;
                for &num in numbers {
                    value ^= num;
                }

                u8_to_hex(value)
            })
            .collect()
    }
}

fn part_one(input: &str) -> i32 {
    let inputs: Vec<u8> = input
        .split(',')
        .map(|value| value.parse().unwrap())
        .collect();

    KnotHash::new().run(&inputs)
}

fn part_two(input: &str) -> String {
    let inputs: Vec<u8> = input
        .as_bytes()
        .iter()
        .map(|b| *b)
        .chain([17, 31, 73, 47, 23])
        .collect();

    let mut knot_hash = KnotHash::new();

    for _ in 0..64 {
        knot_hash.run(&inputs);
    }

    knot_hash.digest()
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

fn main() -> std::io::Result<()> {
    let file_path = env::current_dir()?.join(Path::new("data.txt"));
    let input = fs::read_to_string(file_path)?;

    time_it(|| println!("part 1: {}", part_one(&input)));
    time_it(|| println!("part 2: {}", part_two(&input)));

    Ok(())
}
