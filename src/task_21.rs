use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Debug, Clone)]
struct HitPoints(u16);


#[derive(Debug, Clone)]
enum Props {
    Damage(u16),
    Armor(u16),
}

impl Props {
    fn damage(&self) -> u16 {
        match self {
            Props::Damage(d) => *d,
            _ => 0
        }
    }
    fn armor(&self) -> u16 {
        match self {
            Props::Armor(a) => *a,
            _ => 0
        }
    }
}

#[derive(Debug, Clone)]
struct Character {
    hit_points: HitPoints,
    damage: Props,
    defence: Props,
}

impl Character {
    fn enemy(hit_ponts: u16, damage: u16, armor: u16) -> Self { Character { hit_points: HitPoints(hit_ponts), damage: Props::Damage(damage), defence: Props::Armor(armor) } }
    fn player(weapon: Option<&Props>, armor: Option<&Props>, rings: Vec<&Props>) -> Self {
        let mut damage = 0;
        let mut defence = 0;
        if weapon.is_some() {
            damage += weapon.unwrap().damage();
        }
        if armor.is_some() {
            defence += armor.unwrap().armor();
        }
        for r in rings {
            defence += r.armor();
            damage += r.damage();
        }
        Character { hit_points: HitPoints(100), damage: Props::Damage(damage), defence: Props::Armor(defence) }
    }
}

pub fn run() {
    let input = File::open("inputs/task_21").unwrap();
    let input = BufReader::new(input);

    let input: Vec<u16> = input.lines()
        .filter_map(|l| l.ok())
        .map(|l| l.split(": ").map(|s| s.to_string()).collect())
        .map(|l: Vec<String>| {
            l[1].parse::<u16>()
        })
        .filter_map(|l| l.ok())
        .collect();

    let enemy = Character::enemy(input[0], input[1], input[2]);

    let weapons = vec![
        (8, Props::Damage(4)),
        (10, Props::Damage(5)),
        (25, Props::Damage(6)),
        (40, Props::Damage(7)),
        (74, Props::Damage(8))
    ];
    let armor = vec![
        (13, Props::Armor(1)),
        (31, Props::Armor(2)),
        (53, Props::Armor(3)),
        (75, Props::Armor(4)),
        (102, Props::Armor(5))
    ];
    let rings = vec![
        (25, Props::Damage(1)),
        (50, Props::Damage(2)),
        (100, Props::Damage(3)),
        (20, Props::Armor(1)),
        (40, Props::Armor(2)),
        (80, Props::Armor(3))
    ];

    let mut min = std::u16::MAX;

    for w in 0..weapons.len() {
        for a in 0..armor.len() + 1 {
            for r1 in 0..rings.len() + 1 {
                for r2 in r1 + 1..rings.len() + 2 {
                    let mut cost = 0;

                    let w_s = weapons.get(w).map(|(c, i)| {
                        cost += c;
                        i
                    });
                    let a_s = armor.get(a).map(|(c, i)| {
                        cost += c;
                        i
                    });
                    let r1_s = rings.get(r1).map(|(c, i)| {
                        cost += c;
                        i
                    });
                    let r2_s = rings.get(r2).map(|(c, i)| {
                        cost += c;
                        i
                    });

                    let mut r = vec![];
                    if r1_s.is_some() {
                        r.push(r1_s.unwrap());
                    }
                    if r2_s.is_some() {
                        r.push(r2_s.unwrap());
                    }

                    let player = &mut Character::player(w_s, a_s, r);
                    if game(player, &mut enemy.clone()) && cost < min {
                        min = cost;
                    }
                }
            }
        }
    }

    println!("{}", min);
}

fn game(player: &Character, enemy: &Character) -> bool {
    let player_wins = turns_to_win(player, enemy);
    let enemy_wins = turns_to_win(enemy, player);
    return player_wins <= enemy_wins;
}

fn turns_to_win(first: &Character, second: &Character) -> u32 {
    let damage_done = if first.damage.damage() <= second.defence.armor() {
        1
    } else {
        first.damage.damage() - second.defence.armor()
    };
    (f32::from(second.hit_points.0) / f32::from(damage_done)).ceil() as u32
}

pub fn run_e() {
    let input = File::open("inputs/task_21").unwrap();
    let input = BufReader::new(input);


    let input: Vec<u16> = input.lines()
        .filter_map(|l| l.ok())
        .map(|l| l.split(": ").map(|s| s.to_string()).collect())
        .map(|l: Vec<String>| {
            l[1].parse::<u16>()
        })
        .filter_map(|l| l.ok())
        .collect();

    let enemy = Character::enemy(input[0], input[1], input[2]);

    let weapons = vec![
        (8, Props::Damage(4)),
        (10, Props::Damage(5)),
        (25, Props::Damage(6)),
        (40, Props::Damage(7)),
        (74, Props::Damage(8))
    ];
    let armor = vec![
        (13, Props::Armor(1)),
        (31, Props::Armor(2)),
        (53, Props::Armor(3)),
        (75, Props::Armor(4)),
        (102, Props::Armor(5))
    ];
    let rings = vec![
        (25, Props::Damage(1)),
        (50, Props::Damage(2)),
        (100, Props::Damage(3)),
        (20, Props::Armor(1)),
        (40, Props::Armor(2)),
        (80, Props::Armor(3))
    ];

    let mut max = 0;

    for w in 0..weapons.len() {
        for a in 0..armor.len() + 1 {
            for r1 in 0..rings.len() + 1 {
                for r2 in r1 + 1..rings.len() + 2 {
                    let mut cost = 0;

                    let w_s = weapons.get(w).map(|(c, i)| {
                        cost += *c;
                        i
                    });
                    let a_s = armor.get(a).map(|(c, i)| {
                        cost += *c;
                        i
                    });
                    let r1_s = rings.get(r1).map(|(c, i)| {
                        cost += *c;
                        i
                    });
                    let r2_s = rings.get(r2).map(|(c, i)| {
                        cost += *c;
                        i
                    });

                    let mut r = vec![];
                    if r1_s.is_some() {
                        r.push(r1_s.unwrap());
                    }
                    if r2_s.is_some() {
                        r.push(r2_s.unwrap());
                    }

                    let player = &mut Character::player(w_s, a_s, r.clone());
                    if !game(player, &mut enemy.clone()) && cost >= max {
                        max = cost;
                    }
                }
            }
        }
    }

    println!("{}", max);
}