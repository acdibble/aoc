use std::{env, fs, path::Path, result::Result, str, time::SystemTime};

#[derive(Debug, Eq, PartialEq)]
enum DamageKind {
    Fire,
    Radiation,
    Slashing,
    Cold,
    Bludgeoning,
}

impl Default for DamageKind {
    fn default() -> Self {
        Self::Fire
    }
}

impl str::FromStr for DamageKind {
    type Err = String;

    fn from_str(string: &str) -> Result<Self, <Self as str::FromStr>::Err> {
        use DamageKind::*;

        match string {
            "fire" => Ok(Fire),
            "radiation" => Ok(Radiation),
            "slashing" => Ok(Slashing),
            "cold" => Ok(Cold),
            "bludgeoning" => Ok(Bludgeoning),
            _ => Err(string.to_owned()),
        }
    }
}

#[derive(Default, Debug, Eq, PartialEq)]
struct Unit {
    id: usize,
    count: usize,
    hp: usize,
    damage_amount: usize,
    initiative: usize,
    damage_kind: DamageKind,
    weaknesses: Vec<DamageKind>,
    immunities: Vec<DamageKind>,
    target: Option<usize>,
    took_damage: bool,
}

impl Unit {
    fn new() -> Self {
        Default::default()
    }

    fn effective_power(&self) -> usize {
        self.count * self.damage_amount
    }

    fn damage_dealt(&self, other: &Self) -> usize {
        let base_damage = self.effective_power();

        if other.weaknesses.contains(&self.damage_kind) {
            base_damage * 2
        } else if other.immunities.contains(&self.damage_kind) {
            0
        } else {
            base_damage
        }
    }

    fn attack(&self, other: &mut Self) {
        let damage = self.damage_dealt(other);
        other.receive_damage(damage);
    }

    fn receive_damage(&mut self, amount: usize) {
        let units_lost = amount / self.hp;
        self.count = self.count.saturating_sub(units_lost);
        self.took_damage = units_lost != 0;
    }

    fn from_str(line: &str, id: usize) -> Result<Self, String> {
        let mut tokens = tokenize(line).peekable();

        let mut unit = Unit::new();

        unit.id = id;
        unit.count = tokens.next().unwrap().parse().unwrap();

        tokens.next(); // units
        tokens.next(); // each
        tokens.next(); // with

        unit.hp = tokens.next().unwrap().parse().unwrap();

        tokens.next(); // hit
        tokens.next(); // points

        if matches!(tokens.peek(), Some(&"(")) {
            tokens.next();

            loop {
                let buff = tokens.next().unwrap();
                tokens.next(); // to

                loop {
                    let kind = tokens.next().unwrap();

                    match buff {
                        "immune" => unit.immunities.push(kind.parse()?),
                        "weak" => unit.weaknesses.push(kind.parse()?),
                        _ => return Err(line.to_owned()),
                    }

                    if matches!(tokens.peek(), Some(&",")) {
                        tokens.next();
                    } else {
                        break;
                    }
                }

                if matches!(tokens.peek(), Some(&")")) {
                    tokens.next();
                    break;
                } else if matches!(tokens.peek(), Some(&";")) {
                    tokens.next();
                    continue;
                } else {
                    unreachable!()
                }
            }
        }

        tokens.next(); // with
        tokens.next(); // an
        tokens.next(); // attack
        tokens.next(); // that
        tokens.next(); // does

        unit.damage_amount = tokens.next().unwrap().parse().unwrap();
        unit.damage_kind = tokens.next().unwrap().parse()?;

        tokens.next(); // damage
        tokens.next(); // at
        tokens.next(); // initiative

        unit.initiative = tokens.next().unwrap().parse().unwrap();

        Ok(unit)
    }
}

fn tokenize(line: &str) -> impl Iterator<Item = &str> {
    line.split_ascii_whitespace().flat_map(|token| {
        if token.contains('(') {
            let (a, b) = token.split_at(1);
            [Some(a), Some(b)]
        } else if token.contains(',') || token.contains(')') || token.contains(';') {
            let (a, b) = token.split_at(token.len() - 1);
            [Some(a), Some(b)]
        } else {
            [Some(token), None]
        }
        .into_iter()
        .filter_map(|x| x)
    })
}

#[derive(Debug)]
struct Army(Vec<Unit>);

impl Army {
    fn prepare_for_selection(&mut self) {
        self.0.sort_by(|a, b| {
            match a.effective_power().cmp(&b.effective_power()) {
                std::cmp::Ordering::Equal => a.initiative.cmp(&b.initiative),
                other => other,
            }
            .reverse()
        })
    }

    fn prepare_for_attack(&mut self) {
        self.0
            .sort_by(|a, b| a.initiative.cmp(&b.initiative).reverse())
    }

    fn iter(&self) -> impl Iterator<Item = &Unit> {
        self.0.iter()
    }

    fn iter_mut(&mut self) -> impl Iterator<Item = &mut Unit> {
        self.0.iter_mut()
    }

    fn remove_dead_units(&mut self) {
        self.0.retain(|unit| unit.count != 0);
    }

    fn undamaged(&self) -> bool {
        self.iter().all(|unit| !unit.took_damage)
    }

    fn viable(&self) -> bool {
        self.0.len() != 0
    }

    fn get_mut(&mut self, id: usize) -> Option<&mut Unit> {
        for unit in &mut self.0 {
            if unit.id == id {
                return Some(unit);
            }
        }

        None
    }

    fn find_targets(&mut self, other: &Army) {
        let mut taken = Vec::new();
        for attacker in self.iter_mut() {
            attacker.target = None;
            attacker.took_damage = false;
            let mut selected = None;

            for defender in other.iter() {
                let should_attack;
                if taken.contains(&defender.id) {
                    continue;
                }

                let damage = attacker.damage_dealt(defender);

                if damage == 0 {
                    continue;
                }

                if let Some(unit) = selected {
                    use std::cmp::Ordering::*;
                    let result = match attacker
                        .damage_dealt(defender)
                        .cmp(&attacker.damage_dealt(unit))
                    {
                        Equal => match defender.effective_power().cmp(&unit.effective_power()) {
                            Equal => match defender.initiative.cmp(&unit.initiative) {
                                Equal => unreachable!(),
                                other => other,
                            },
                            other => other,
                        },
                        other => other,
                    };

                    should_attack = matches!(result, Greater);
                } else {
                    should_attack = true;
                }

                if should_attack {
                    selected = Some(defender)
                }
            }

            if let Some(target) = selected {
                attacker.target = Some(target.id);
                taken.push(target.id)
            }
        }
    }
}

fn parse_army(lines: &mut str::Lines) -> Vec<Unit> {
    lines.next();

    lines
        .enumerate()
        .map_while(|(index, line)| {
            if line == "" {
                return None;
            }

            Unit::from_str(line, index + 1).ok()
        })
        .collect()
}

macro_rules! get_pair {
    ($increment:ident, $attacker:ident, $defenders:ident) => {{
        $increment += 1;
        if let Some(target) = $attacker.target {
            Some(($attacker, $defenders.get_mut(target)))
        } else {
            None
        }
    }};
}

fn do_battle(input: &str, boost: usize) -> (&'static str, usize) {
    let mut lines = input.lines();
    let mut immune_system = Army(parse_army(&mut lines));

    for unit in immune_system.iter_mut() {
        unit.damage_amount += boost;
    }

    let mut infection = Army(parse_army(&mut lines));

    while immune_system.viable() && infection.viable() {
        infection.prepare_for_selection();
        immune_system.prepare_for_selection();

        infection.find_targets(&immune_system);
        immune_system.find_targets(&infection);

        infection.prepare_for_attack();
        immune_system.prepare_for_attack();

        let mut infection_index = 0;
        let mut immune_index = 0;

        loop {
            let pair = match (
                infection.0.get(infection_index),
                immune_system.0.get(immune_index),
            ) {
                (Some(a), Some(b)) => {
                    if a.initiative > b.initiative {
                        get_pair!(infection_index, a, immune_system)
                    } else {
                        get_pair!(immune_index, b, infection)
                    }
                }
                (Some(a), None) => {
                    get_pair!(infection_index, a, immune_system)
                }
                (None, Some(b)) => {
                    get_pair!(immune_index, b, infection)
                }
                (None, None) => break,
            };

            if let Some((attacker, mut defender)) = pair {
                attacker.attack(defender.as_deref_mut().unwrap());
            }
        }

        immune_system.remove_dead_units();
        infection.remove_dead_units();

        if immune_system.undamaged() && infection.undamaged() {
            break;
        }
    }

    let (victor, army) = if infection.viable() && immune_system.viable() {
        ("stalemate", vec![])
    } else if infection.viable() {
        ("infection", infection.0)
    } else {
        ("reindeer", immune_system.0)
    };

    (
        victor,
        army.into_iter().fold(0, |acc, unit| acc + unit.count),
    )
}

fn part_one(input: &str) -> usize {
    do_battle(input, 0).1
}

fn part_two(input: &str) -> usize {
    let mut boost = 0;
    loop {
        println!("{}", boost);
        match do_battle(input, boost) {
            ("reindeer", units) => break units,
            _ => boost += 1,
        }
    }
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
