use serde::{Serialize, Deserialize};
use rand::prelude::*;

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

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
enum Player {
    One,
    Two,
}

struct Game {
    players: Vec<Character>
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
enum CardAction {
    Attack {hits: i32, damage: i32, opponent: Player},
    Defend { amount: i32, player: Player},
    ApplyFlaw {amount: i32, opponent: Player},
    ApplyWeak {amount: i32, opponent: Player},
    ApplyInjury {amount: i32, opponent: Player},
    ApplyHexagram {amount: i32, player: Player},
    Heal{amount: i32, player: Player},
    AddMaxHp{amounts: i32, player: Player},
}

#[derive(Serialize, Deserialize, Debug)]
struct Card {
    name: String,
    actions: Vec<CardAction>,
}

impl Card {
    fn attack(&self, hits: &i32, damage: &i32, player: &mut Character, opponent: &mut Character) {
        for i in 0..hits.clone() {
            let actual_damage = (damage + player.attack_up + player.sword_intent) as f32 * (if opponent.flaw > 0 {1.4} else {1.0}) * (if player.weak > 0 {0.6} else {1.0});
            
            let actual_damage = actual_damage.floor();
            if opponent.defense > actual_damage as i32 {
                opponent.defense -= actual_damage as i32;
            }
            else {
                opponent.hp -= actual_damage as i32 - opponent.defense;
                opponent.defense = 0;
            }
            trigger_defenses();
        }
    }

    fn apply_flaw(&self, amount: i32, opponent: &mut Character) {
        opponent.flaw += amount;
    }

    fn apply_weak(&self, amount: i32, opponent: &mut Character) {
        opponent.weak += amount;
    }

    fn apply_injury(&self, amount: i32, opponent: &mut Character) {
        opponent.injury += amount;
    }

    fn block(&self, amount: i32, player: &mut Character) {
        player.defense += amount;
    }

    fn add_hexagram(&self, amount: i32, player: &mut Character) {
        player.hexagram += amount;
    }

    fn heal(&self, amount: i32, player: &mut Character) {
        if player.hp + amount > player.max_hp {
            player.hp = player.max_hp;
        }
        else {
            player.hp += amount;
        }
    }

    fn print_debug(&self, arg1: i32) {
        println!("{}", arg1);
    }

    fn perform_actions(&self, player: &mut Character, opponent: &mut Character) {
        let players = 
        for i in &self.actions {
            match i {
                CardAction::Attack { hits, damage, opponent} => self.attack(hits, damage, player, opponent),
                CardAction::Defend { amount, player } => todo!(),
                CardAction::ApplyFlaw { amount, opponent } => todo!(),
                CardAction::ApplyWeak { amount, opponent } => todo!(),
                CardAction::ApplyInjury { amount, opponent } => todo!(),
                CardAction::ApplyHexagram { amount, player } => todo!(),
                CardAction::Heal { amount, player } => todo!(),
                CardAction::AddMaxHp { amounts, player } => todo!(),
            }
        };
    }

}

fn trigger_defenses() {
    todo!();
}


fn main() {
    let mut char1 = Character::from(50, Player::One);
    let mut char2 = Character::from(60, Player::Two);

    let sample_card1 = Card {name: String::from("Multihit Attack"),
                                    actions: vec![
                                        CardAction::Attack{hits: 2, damage: 4, opponent: Player::Two}
                                    ]};
    let sample_card2 = Card {name: String::from("Attack and defend"),
                                    actions: vec![
                                        CardAction::Attack{hits: 1, damage: 5, opponent: Player::One},
                                        CardAction::Defend { amount: 4, player: Player::Two }
                                    ]};

    let mut game = Game { players: vec![char1, char2]};
    while &game.players[0].hp > &0 && &game.players[1].hp > &0 {
        for i in 0..2 {
            let dmg = rand::thread_rng().gen_range(0..10);
            match game.players[i].player_num {
                Player::One => game.players[1].hp -= dmg,
                Player::Two => game.players[0].hp -= dmg,
            }
            println!("player {:?} took {} damage", i, dmg);
        }
    }

    
    
}   