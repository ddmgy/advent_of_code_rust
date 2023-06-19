#[derive(Debug)]
enum Error {
    ParseCharacter,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseCharacter => write!(f, "unable to parse character information")?,
        }

        Ok(())
    }
}

impl std::error::Error for Error {}

#[derive(Debug)]
struct Character {
    hp: u32,
    damage: u32,
    armor: u32,
}

impl std::str::FromStr for Character {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values: Result<Vec<u32>, _> = s
            .trim()
            .lines()
            .map(|line| line.split_whitespace().last().unwrap().parse())
            .collect();

        match values {
            Ok(values) => Ok(Self {
                hp: values[0],
                damage: values[1],
                armor: values[2],
            }),
            Err(_) => Err(Error::ParseCharacter),
        }
    }
}

fn does_player_win(player: &Character, boss: &Character) -> bool {
    let a = ((player.hp as f32) / std::cmp::max(1, boss.damage.saturating_sub(player.armor)) as f32).ceil();
    let b = ((boss.hp as f32) / std::cmp::max(1, player.damage.saturating_sub(boss.armor)) as f32).ceil();

    a >= b
}

#[derive(Debug)]
struct Item {
    cost: u32,
    damage: u32,
    armor: u32,
}

const WEAPONS: &[Item] = &[
    Item { cost: 8, damage: 4, armor: 0 }, // Dagger
    Item { cost: 10, damage: 5, armor: 0 }, // Shortsword
    Item { cost: 25, damage: 6, armor: 0 }, // Warhammer
    Item { cost: 40, damage: 7, armor: 0 }, // Longsword
    Item { cost: 74, damage: 8, armor: 0 }, // Greataxe
];

const ARMORS: &[Item] = &[
    Item { cost: 0, damage: 0, armor: 0 }, // No Armor
    Item { cost: 13, damage: 0, armor: 1 }, // Leather
    Item { cost: 31, damage: 0, armor: 2 }, // Chainmail
    Item { cost: 53, damage: 0, armor: 3 }, // Splintmail
    Item { cost: 75, damage: 0, armor: 4 }, // Bandedmail
    Item { cost: 102, damage: 0, armor: 5 }, // Platemail
];

const RINGS: &[Item] = &[
    Item { cost: 0, damage: 0, armor: 0 }, // No Ring 1
    Item { cost: 0, damage: 0, armor: 0 }, // No Ring 2
    Item { cost: 25, damage: 1, armor: 0 }, // Damage +1
    Item { cost: 50, damage: 2, armor: 0 }, // Damage +2
    Item { cost: 100, damage: 3, armor: 0 }, // Damage +3
    Item { cost: 20, damage: 0, armor: 1 }, // Defense +1
    Item { cost: 40, damage: 0, armor: 2 }, // Defense +2
    Item { cost: 80, damage: 0, armor: 3 }, // Defense +3
];

fn day21<F1, F2>(input: &str, initial: u32, determine_actual_winner: F1, cmp: F2) -> Option<u32>
where
    F1: Fn(bool) -> bool,
    F2: Fn(u32, u32) -> u32,
{
    match input.trim().parse::<Character>() {
        Ok(boss) => {
            let mut ret_cost = initial;

            for wi in 0..WEAPONS.len() {
                for ai in 0..ARMORS.len() {
                    for ri1 in 0..RINGS.len() - 1 {
                        for ri2 in ri1 + 1..RINGS.len() {
                            let weapon = &WEAPONS[wi];
                            let armor = &ARMORS[ai];
                            let ring1 = &RINGS[ri1];
                            let ring2 = &RINGS[ri2];

                            let cost = weapon.cost + armor.cost + ring1.cost + ring2.cost;
                            let damage = weapon.damage + armor.damage + ring1.damage + ring2.damage;
                            let armor = weapon.armor + armor.armor + ring1.armor + ring2.armor;

                            let player = Character {
                                hp: 100,
                                damage,
                                armor,
                            };

                            if determine_actual_winner(does_player_win(&player, &boss)) {
                                ret_cost = cmp(ret_cost, cost)
                            }
                        }
                    }
                }
            }

            Some(ret_cost)
        },
        Err(e) => {
            eprintln!("error: {}", e);
            None
        }
    }
}

#[crate::aoc(year = 2015, day = 21, part = "A")]
fn day21a(input: &str) -> Option<u32> {
    day21(
        input,
        u32::MAX,
        |winner| winner,
        std::cmp::min,
    )
}

#[crate::aoc(year = 2015, day = 21, part = "B")]
fn day21b(input: &str) -> Option<u32> {
    day21(
        input,
        u32::MIN,
        |winner| !winner,
        std::cmp::max,
    )
}

#[cfg(test)]
mod tests_y2015_day21 {
    use super::*;

    const PLAYER_STATS: &'static str = "Hit Points: 8\nDamage: 5\nArmor: 5";
    const BOSS_STATS: &'static str = "Hit Points: 12\nDamage: 7\nArmor: 2";

    #[test]
    #[allow(non_snake_case)]
    fn partA() -> Result<(), Error> {
        let player: Character = PLAYER_STATS.parse()?;
        let boss: Character = BOSS_STATS.parse()?;
        assert!(does_player_win(&player, &boss));

        Ok(())
    }

    #[test]
    #[allow(non_snake_case)]
    fn partB() {
    }
}
