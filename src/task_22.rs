use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Copy, Clone)]
struct Player {
    mana: i32,
    hit_points: i32,
    armor: i32,
}

#[derive(Copy, Clone)]
struct Boss {
    hit_points: i32,
    damage: i32,
}

trait Character {
    fn is_dead(&self) -> bool;
}

impl Player {
    fn new(mana: i32, hit_points: i32) -> Self {
        Player {mana, hit_points, armor: 0}
    }

    fn cast_spell(&mut self, spell: Spell, turn: i32, boss: &mut Boss) -> Option<Effect> {
        self.mana -= spell.cost();

        spell.on_cast(self, boss);

        return spell.effect(turn);
    }
}

impl Boss {
    fn new(hit_points: i32, damage: i32) -> Self {
        Boss {hit_points, damage}
    }

    fn attack(&self, player: &mut Player) {
        player.hit_points -= i32::max(self.damage - player.armor, 1);
    }
}

impl Character for Player {
    fn is_dead(&self) -> bool {
        self.hit_points <= 0 || self.mana <= 0
    }
}

impl Character for Boss {
    fn is_dead(&self) -> bool {
        self.hit_points <= 0
    }
}

#[derive(Copy, Clone)]
struct Effect {
    spell: Spell,
    finishes_on: i32,
}

impl Effect {
    fn apply(&self, player: &mut Player, boss: &mut Boss, turn: i32) {
        self.spell.on_each_turn(player, boss, turn);
    }
}

#[derive(PartialEq, Copy, Clone)]
enum Spell {
    MagicMissle,
    Drain,
    Shield,
    Poison,
    Recharge,
    HardLevelSpell,
}

impl Spell {
    fn cost(&self) -> i32 {
        match self {
            Spell::MagicMissle => 53,
            Spell::Drain => 73,
            Spell::Shield => 113,
            Spell::Poison => 173,
            Spell::Recharge => 229,
            Spell::HardLevelSpell => 0,
        }
    }

    fn on_cast(&self, player: &mut Player, boss: &mut Boss) {
        match self {
            Spell::MagicMissle => boss.hit_points -= 4,
            Spell::Drain => {
                player.hit_points += 2;
                boss.hit_points -= 2;
            },
            Spell::Shield => player.armor += 7,
            _ => (),
        }
    }

    fn on_finished(&self, player: &mut Player, _: &mut Boss) {
        match self {
            Spell::Shield => player.armor -= 7,
            _ => (),
        }
    }

    fn on_each_turn(&self, player: &mut Player, boss: &mut Boss, turn: i32) {
        match self {
            Spell::Poison => boss.hit_points -= 3,
            Spell::Recharge => player.mana += 101,
            Spell::HardLevelSpell => {
                if turn % 2 == 0 {
                    player.hit_points -= 1;
                }
            },
            _ => (),
        }
    }

    fn effect(&self, turn: i32) -> Option<Effect> {
        match self {
            Spell::Shield => Some(Effect {
                spell: self.clone(),
                finishes_on: turn + 6
            }),
            Spell::Poison => Some(Effect {
                spell: self.clone(),
                finishes_on: turn + 6
            }),
            Spell::Recharge => Some(Effect {
                spell: self.clone(),
                finishes_on: turn + 5
            }),
            Spell::HardLevelSpell => Some(Effect {
                spell: self.clone(),
                finishes_on: turn - 1, // endless
            }),
            _ => None,
        }
    }
}

pub fn run() {
    let input = File::open("inputs/task_22").unwrap();
    let input = BufReader::new(input);

    let boss = input.lines()
        .filter_map(|l| l.ok())
        .map(|l: String| l.split(": ").map(|s| s.to_string()).collect())
        .map(|s: Vec<String>| s[1].parse::<i32>())
        .filter_map(|s| s.ok())
        .collect::<Vec<i32>>();

    let boss = Boss::new(boss[0], boss[1]);
    let player = Player::new(500, 50);

    let result = game(&player, &boss, &vec![], 0, None).unwrap();

    println!("Result: {}", result)
}

fn game(player: &Player, boss: &Boss, effects: &Vec<Effect>, turn: i32, best: Option<i32>) -> Option<i32> {
    let mut player = *player;
    let mut boss = *boss;
    for e in effects {
        e.apply(&mut player, &mut boss, turn);
        if player.is_dead() {
            return None;
        }
        if boss.is_dead() {
            return Some(0);
        }
    }

    if turn % 2 == 0 {
        let mut min = None;
        let effects = filter_out_effects(effects, turn, &mut player, &mut boss);

        if player.is_dead() {
            return None;
        }
        if boss.is_dead() {
            return Some(0);
        }

        for i in 0..5 {

            let mut p = player;
            let mut b = boss;

            let spell = match i {
                0 => Spell::Drain,
                1 => Spell::MagicMissle,
                2 => Spell::Poison,
                3 => Spell::Recharge,
                _ => Spell::Shield
            };
            let result = if has_spell(&spell, &effects) {
                None
            } else if min.is_some() && min.unwrap() - spell.cost() < 0 {
                None
            } else if best.is_some() && best.unwrap() - spell.cost() < 0 {
                None
            } else {
                let effect = p.cast_spell(spell, turn, &mut b);
                if p.is_dead() {
                    None
                } else if b.is_dead() {
                    Some(0)
                } else {
                    let ef = if effect.is_some() {
                        let mut r = effects.clone();
                        r.push(effect.unwrap());
                        r
                    } else {
                        effects.clone()
                    };
                    game(&p, &b, &ef, turn + 1, min.or_else(|| best).map(|m| m - spell.cost()))
                } 
            };
            match result {
                Some(m) => {
                    let cost = m + spell.cost();
                    if min.is_none() {
                        min = Some(cost);
                    } else if min.unwrap() > cost {
                        min = Some(cost);
                    }
                },
                _ => {},
            };
        }
        min
    } else {
        boss.attack(&mut player);

        if player.is_dead() {
            return None;
        }
        if boss.is_dead() {
            return Some(0);
        }

        let effects = filter_out_effects(effects, turn, &mut player, &mut boss);

        if player.is_dead() {
            return None;
        }
        if boss.is_dead() {
            return Some(0);
        }

        game(&player, &boss, &effects, turn + 1, best)
    }
}

fn filter_out_effects(effects: &Vec<Effect>, turn: i32, player: &mut Player, boss: &mut Boss) -> Vec<Effect> {
    let mut result = vec![];

    for e in effects {
        if e.finishes_on == turn {
            e.spell.on_finished(player, boss);
        } else {
            result.push(e.clone())
        }
    }

    result
}

fn has_spell(spell: &Spell, effects: &Vec<Effect>) -> bool {
    for e in effects {
        if e.spell == *spell {
            return true;
        }
    }
    return false;
}

pub fn run_e() {
    let input = File::open("inputs/task_22").unwrap();
    let input = BufReader::new(input);

    let boss = input.lines()
        .filter_map(|l| l.ok())
        .map(|l: String| l.split(": ").map(|s| s.to_string()).collect())
        .map(|s: Vec<String>| s[1].parse::<i32>())
        .filter_map(|s| s.ok())
        .collect::<Vec<i32>>();

    let boss = Boss::new(boss[0], boss[1]);
    let player = Player::new(500, 50);

    let result = game(&player, &boss, &vec![Spell::HardLevelSpell.effect(0).unwrap()], 0, None).unwrap();

    println!("Result: {}", result)
}