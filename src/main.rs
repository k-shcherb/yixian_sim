use serde::{Serialize, Deserialize};
use std::marker::PhantomData;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
struct Character<'a> {
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
    phantom: PhantomData<&'a i32>,
}

impl Character<'_> {
    fn from<'a>(health: i32) -> Character<'a> {
        Character{
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
            phantom: PhantomData
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
enum CharacterType {
    Player,
    Opponent,
}

#[derive(Serialize, Deserialize, Debug)]
struct Card<'a> {
    name: String,
    actions: Vec<CardAction<'a>>,
}

#[derive(Serialize, Deserialize, Debug)]
enum CardAction<'a> {
    Attack {hits: i32, damage: i32, player: &'a mut Character<'a>, opponent: &'a mut Character<'a>},
    Defend {amount: i32, player: &'a mut Character<'a>},
    ApplyFlaw {amount: i32, opponent: &'a mut Character<'a>},
    ApplyWeak {amount: i32, opponent: &'a mut Character<'a>},
    ApplyInjury {amount: i32, opponent: &'a mut Character<'a>},
    ApplyHexagram {amount: i32, player: &'a mut Character<'a>},
    Heal {amount: i32, player: &'a mut Character<'a>},
    AddMaxHp {amount: i32, player: &'a mut Character<'a>},
    
}

impl Card<'_> {
    fn attack(&self, hits: i32, damage: i32, player: &mut Character, opponent: &mut Character) {
        for i in 0..hits {
            let actual_damage = (damage + player.attack_up + player.sword_intent) as f32 * (if opponent.flaw > 0 {1.4} else {1.0}) * (if player.weak > 0 {0.6} else {1.0}) - 
                opponent.defense as f32;
            
            let actual_damage = actual_damage.floor();

            println!("{}", actual_damage);
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

    fn perform_actions(&self, actions: Vec<fn()>, player: &mut Character, opponent: &mut Character) {
        for i in actions {
            i();
        }
    }

}

fn main() {
    let mut char1 = Character::from(50);
    let mut char2 = Character::from(60);

    

    let mut card1 = Card{name: String::from("basic attack")}, vec!;

    let mut funcs: Vec<fn()> = Vec::new();

    card1.apply_flaw(2, &mut char2);
    card1.attack(1, 12, &mut char1, &mut char2);

    take_turn(&mut char1, &mut char2);
}

fn take_turn(c1: &mut Character, c2: &mut Character ) {
    println!("{} {}", c1.hp, c2.hp);

}