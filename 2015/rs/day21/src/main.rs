use std::env;
use std::fs;
use std::path::Path;

#[derive(Clone, Copy, Debug)]
struct Person(i32, i32, i32);

impl Person {
    fn attack(&self, other: &mut Self) {
        if self.is_alive() && other.is_alive() {
            other.0 -= self.1 - other.2
        }
    }

    fn is_alive(&self) -> bool {
        self.0 > 0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Item(&'static str, i32, i32, i32);

impl Item {
    fn cost(&self) -> i32 {
        self.1
    }

    fn damage(&self) -> i32 {
        self.2
    }

    fn armor(&self) -> i32 {
        self.3
    }
}

const WEAPONS: [Item; 5] = [
    Item("Dagger", 8, 4, 0),
    Item("Shortsword", 10, 5, 0),
    Item("Warhammer", 25, 6, 0),
    Item("Longsword", 40, 7, 0),
    Item("Greataxe", 74, 8, 0),
];

const ARMORS: [Item; 5] = [
    Item("Leather", 13, 0, 1),
    Item("Chainmail", 31, 0, 2),
    Item("Splintmail", 53, 0, 3),
    Item("Bandedmail", 75, 0, 4),
    Item("Platemail", 102, 0, 5),
];

const RINGS: [Item; 6] = [
    Item("Damage +1", 25, 1, 0),
    Item("Damage +2", 50, 2, 0),
    Item("Damage +3", 100, 3, 0),
    Item("Defense +1", 20, 0, 1),
    Item("Defense +2", 40, 0, 2),
    Item("Defense +3", 80, 0, 3),
];

#[derive(Debug)]
struct Loadout {
    weapon: Item,
    armor: Option<Item>,
    ring1: Option<Item>,
    ring2: Option<Item>,
}

impl Loadout {
    fn cost(&self) -> i32 {
        self.weapon.cost()
            + self.armor.map(|i| i.cost()).unwrap_or(0)
            + self.ring1.map(|i| i.cost()).unwrap_or(0)
            + self.ring2.map(|i| i.cost()).unwrap_or(0)
    }

    fn damage(&self) -> i32 {
        self.weapon.damage()
            + self.ring1.map(|i| i.damage()).unwrap_or(0)
            + self.ring2.map(|i| i.damage()).unwrap_or(0)
    }

    fn armor(&self) -> i32 {
        self.armor.map(|i| i.armor()).unwrap_or(0)
            + self.ring1.map(|i| i.armor()).unwrap_or(0)
            + self.ring2.map(|i| i.armor()).unwrap_or(0)
    }
}

fn generate_ring_combos(loadouts: &mut Vec<Loadout>, weapon: Item, armor: Option<Item>) {
    for ring1 in RINGS {
        loadouts.push(Loadout {
            weapon,
            armor,
            ring1: Some(ring1),
            ring2: None,
        });

        for ring2 in RINGS {
            if ring1 != ring2 {
                loadouts.push(Loadout {
                    weapon,
                    armor,
                    ring1: Some(ring1),
                    ring2: Some(ring2),
                });
            }
        }
    }
}

fn generate_loadouts() -> Vec<Loadout> {
    let mut loadouts = Vec::new();

    for weapon in WEAPONS {
        loadouts.push(Loadout {
            weapon,
            armor: None,
            ring1: None,
            ring2: None,
        });

        for armor in ARMORS {
            loadouts.push(Loadout {
                weapon,
                armor: Some(armor),
                ring1: None,
                ring2: None,
            });

            generate_ring_combos(&mut loadouts, weapon, Some(armor));
        }

        generate_ring_combos(&mut loadouts, weapon, None);
    }

    loadouts
}

fn parse_boss(input: &String) -> Person {
    let mut lines_it = input.lines();
    let line = lines_it.next().unwrap();
    let mut it = line.split_ascii_whitespace();
    it.next();
    it.next();
    let hp = it.next().unwrap().parse().unwrap();

    let line = lines_it.next().unwrap();
    let mut it = line.split_ascii_whitespace();
    it.next();
    let damage = it.next().unwrap().parse().unwrap();

    let line = lines_it.next().unwrap();
    let mut it = line.split_ascii_whitespace();
    it.next();
    let armor = it.next().unwrap().parse().unwrap();

    Person(hp, damage, armor)
}

fn part_one(loadouts: &Vec<Loadout>, boss: Person) -> i32 {
    let mut price = std::i32::MAX;

    for loadout in loadouts {
        let mut me = Person(100, loadout.damage(), loadout.armor());
        let mut boss = boss;

        while me.is_alive() && boss.is_alive() {
            me.attack(&mut boss);
            boss.attack(&mut me);
        }

        if me.is_alive() {
            price = std::cmp::min(price, loadout.cost());
        }
    }

    price
}

fn part_two(loadouts: &Vec<Loadout>, boss: Person) -> i32 {
    let mut price = 0;

    for loadout in loadouts {
        let mut me = Person(100, loadout.damage(), loadout.armor());
        let mut boss = boss;

        while me.is_alive() && boss.is_alive() {
            me.attack(&mut boss);
            boss.attack(&mut me);
        }

        if boss.is_alive() {
            price = std::cmp::max(price, loadout.cost());
        }
    }

    price
}

fn main() -> std::io::Result<()> {
    let file_path = env::current_dir()?.join(Path::new("data.txt"));
    let input = fs::read_to_string(file_path)?;

    let boss = parse_boss(&input);
    let loadouts = generate_loadouts();

    println!("part 1: {}", part_one(&loadouts, boss));
    println!("part 2: {}", part_two(&loadouts, boss));

    Ok(())
}
