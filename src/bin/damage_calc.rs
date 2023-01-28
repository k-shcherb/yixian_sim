use serde::{Serialize, Deserialize};
use rand::prelude::*;
use serde_json;
use std::alloc::handle_alloc_error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::error::Error;

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
enum Player {
    One,
    Two,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
struct Character {
    player_num: Player,
    max_hp: i32,
    hp: i32,
    qi: i32,
    defense: i32,
    hexagram: i32,
    injury: i32,
    flaw: i32,
    attack_up: i32,
    attack_down: i32,
    weak: i32,
    sword_intent: i32,
    cloud_count: i32,
    unrestrained_count: i32,
}

impl Character {
    fn from(health: i32, player_num: Player) -> Character {
        Character{
            player_num,
            max_hp: health,
            hp: health,
            qi: 0,
            defense: 0,
            hexagram: 0,
            injury: 0,
            flaw: 0,
            attack_up: 0,
            attack_down: 0,
            weak: 0,
            sword_intent: 0,
            cloud_count: 0,
            unrestrained_count: 0,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
enum CardAction{
    Attack {hits: i32, damage: i32, additional_effects: Vec<Status>},
    Defend { amount: i32, additional_effects: Vec<Status>},
    ApplyFlaw {amount: i32},
    ApplyWeak {amount: i32},
    ApplyInjury {amount: i32},
    ApplyHexagram {amount: i32},
    Heal{amount: i32},
    AddMaxHp{amount: i32},
    AddQi{amount: i32},
    AddIgnoreDef{amount: i32},
    AddSwordIntent{amount: i32, additional_effects: Vec<Status>}
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
enum Status {
    SmashDef,
    IgnoreDef,
    Chase,
    FiveElements,
    QiMultiplier,
    Continuous,
    OnInjury,
    CloudCount{multiplier: i32},
    NeedsCloud,
    NextTurn
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
enum Tag {
    Cloud,
    Fire,
    Water,
    Earth,
    Wood,
    Continuous
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Card {
    name: String,
    level: i32,
    cost: i32,
    actions: Vec<CardAction>,
    tags: Vec<Tag>
}

impl Card {
    fn attack(&self, hits: &i32, damage: &i32) -> i32 {
        let mut total_damage = 0;
        for _ in 0..hits.clone() {
            total_damage += damage;
        }
        total_damage
    }

    fn perform_actions(&self) {
        let mut total_dmg = 0;
        for action in &self.actions {
            match action {
                CardAction::Attack { hits, damage, additional_effects } => total_dmg += &self.attack(hits, damage),
                CardAction::Defend { amount, additional_effects } => todo!(),
                CardAction::ApplyFlaw { amount } => todo!(),
                CardAction::ApplyWeak { amount } => todo!(),
                CardAction::ApplyInjury { amount } => todo!(),
                CardAction::ApplyHexagram { amount } => todo!(),
                CardAction::Heal { amount } => todo!(),
                CardAction::AddMaxHp { amount } => todo!(),
                CardAction::AddQi { amount } => todo!(),
                CardAction::AddIgnoreDef { amount } => todo!(),
                CardAction::AddSwordIntent { amount, additional_effects } => todo!(),
            }
            println!("total damage: {:#?}", total_dmg);
        }
    }
}



fn main() {

    let mut you = Character::from(100, Player::One);
    let mut dummy = Character::from(100, Player::Two);

    let card2 = read_card_from_file("./card_jsons/cards.json").unwrap();
    card2[0].perform_actions();
}

fn read_card_from_file(filename: &str) -> Result<Vec<Card>, Box<dyn Error>> {
    let file = File::open(Path::new(filename))?;
    let reader = BufReader::new(file);

    let u = serde_json::from_reader(reader)?;

    Ok(u)
}

