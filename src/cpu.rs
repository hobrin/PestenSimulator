
use crate::*;

pub trait Player {
    fn maketurn(&self, id: usize, cardsInHand: &Vec<u8>, playable: &Vec<u8>, onstack: u8, _instack: u8, cardsInOtherPlayers: &Vec<u8>, todraw: u8) -> u8;
}

pub struct ConsolePlayer {

}
impl ConsolePlayer {
    pub fn new() -> ConsolePlayer {
        ConsolePlayer{}
    }
}
impl Player for ConsolePlayer {
    fn maketurn(&self, id: usize, cardsInHand: &Vec<u8>, playable: &Vec<u8>, onstack: u8, _instack: u8, cardsInOtherPlayers: &Vec<u8>, todraw: u8) -> u8 {
        println!("\nYou have these cards:");
        for card in cardsInHand.iter() {
            println!("{}: {}", card, lib::get_name(*card));
        }
        println!("Please select one card (number):\t\t\t{:?}", cardsInOtherPlayers);
        loop {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input.");

            let name = input.trim();
            if let Ok(value) = name.parse::<u8>() {
                if value >= 54 || !cardsInHand.contains(&value) {
                    println!("Card not in hand.");
                    continue;
                }
                if !lib::can_card_stack(value, onstack) {
                    println!("Card not stackable.");
                    continue;
                }
                return value;
            } else {
                println!("Not a number. Please select the number of the card (in front of the card).");
                continue;
            }
        }
    }

}
pub struct RandomPlayer {

}
impl RandomPlayer {
    pub fn new() -> RandomPlayer {
        RandomPlayer{}
    }
}
impl Player for RandomPlayer {
    fn maketurn(&self, id: usize, cardsInHand: &Vec<u8>, playable: &Vec<u8>, onstack: u8, _instack: u8, cardsInOtherPlayers: &Vec<u8>, todraw: u8) -> u8 {
        playable[0]
    }
}

pub struct PeaceKeeperPlayer {

}
impl PeaceKeeperPlayer {
    pub fn new() -> PeaceKeeperPlayer {
        PeaceKeeperPlayer{}
    }
}
impl Player for PeaceKeeperPlayer {
    fn maketurn(&self, id: usize, cardsInHand: &Vec<u8>, playable: &Vec<u8>, onstack: u8, _instack: u8, cardsInOtherPlayers: &Vec<u8>, todraw: u8) -> u8 {
        let mut playable = playable.clone();
        let mut idx: i32 = 0;
        while (idx as usize) < playable.len() {
            let card = playable[idx as usize];
            if lib::get_card_nr(card) == 1 || lib::get_card_nr(card) == 13 {
                if playable.len() > 1 {
                    playable.remove(idx as usize);
                    idx-=1;
                }
            }
            idx+=1;
        }
        playable[0]
    }
}

pub struct BullyPlayer {

}
impl BullyPlayer {
    pub fn new() -> BullyPlayer {
        BullyPlayer{}
    }
}
impl Player for BullyPlayer {
    fn maketurn(&self, id: usize, cardsInHand: &Vec<u8>, playable: &Vec<u8>, onstack: u8, _instack: u8, cardsInOtherPlayers: &Vec<u8>, todraw: u8) -> u8 {
        for card in playable.iter() {
            if lib::get_card_nr(*card) == 1 || lib::get_card_nr(*card) == 13 {
                return *card;
            }
        }
        playable[0]
    }
}

pub struct CarefulBullyPlayer {
    maxBeforeBully: u8,
    retainJack: bool,
    throwJackDesperado: bool,
    throwSevenFirst: bool,
    retainSeven: bool,
}
impl CarefulBullyPlayer {
    pub fn new(maxBeforeBully: u8, retainJack: bool, throwJackDesperado: bool, throwSevenFirst: bool, retainSeven: bool) -> CarefulBullyPlayer {
        CarefulBullyPlayer{maxBeforeBully: maxBeforeBully, retainJack: retainJack, throwJackDesperado: throwJackDesperado, throwSevenFirst: throwSevenFirst, retainSeven: retainSeven}
    }
}
impl Player for CarefulBullyPlayer {
    fn maketurn(&self, id: usize, cardsInHand: &Vec<u8>, playable: &Vec<u8>, onstack: u8, _instack: u8, cardsInOtherPlayers: &Vec<u8>, todraw: u8) -> u8 {
        let mut playable = playable.clone();

        if todraw > 0 || cardsInOtherPlayers[(id+1) % cardsInOtherPlayers.len()] < self.maxBeforeBully { //TDDO: is bug with reversed order
            for card in playable.iter() { //throw cards as soon as possible.
                if lib::get_card_nr(*card) == 1 || lib::get_card_nr(*card) == 13 || (self.throwJackDesperado && todraw==0 && lib::get_card_nr(*card) == 10) {
                    return *card;
                }
            }
        } else { //retain cards.
            let mut idx: i32 = 0;
            while (idx as usize) < playable.len() {
                let card = playable[idx as usize];
                if lib::get_card_nr(card) == 1 || lib::get_card_nr(card) == 13 || (self.retainJack && lib::get_card_nr(card) == 10) || (self.retainSeven && lib::get_card_nr(card) == 6) {
                    if playable.len() > 1 {
                        if lib::get_card_nr(card) == 6 { //check to make sure it has card after this, so that it doesn't get stuck with a 7.
                            if playable.iter().filter(|&&followup| lib::can_card_stack(followup, card)).count() < 3 {
                                idx += 1;
                                continue;
                            }
                        }
                        playable.remove(idx as usize);
                        idx-=1;
                    }
                }
                idx+=1;
            }
        }
        if self.throwSevenFirst {
            if let Some(card) = playable.iter().find(|&&card| card==6 && cardsInHand.iter().any(|&followup| lib::can_card_stack(followup, card) && followup != card)) {
                return *card;
            }
        }

        playable[0]
    }
}
pub struct DetectivePlayer {
    cards_order: [u8; 54]
}
impl DetectivePlayer {
    pub fn new(cards_order: [u8; 54]) -> DetectivePlayer {
        DetectivePlayer{cards_order}
    }
}
impl Player for DetectivePlayer {
    fn maketurn(&self, id: usize, cardsInHand: &Vec<u8>, playable: &Vec<u8>, onstack: u8, _instack: u8, cardsInOtherPlayers: &Vec<u8>, todraw: u8) -> u8 {
        // playable.sort_by_key(|card| self.cards_order[*card as usize]);
        if todraw > 0 || cardsInOtherPlayers[(id+1) % cardsInOtherPlayers.len()] < 3 {
            for card in playable.iter() { //throw cards as soon as possible.
                if lib::get_card_nr(*card) == 1 || lib::get_card_nr(*card) == 13 {
                    return *card;
                }
            }
        }
        let mut min: u8 = 255;
        let mut min_card = 255;
        for &card in playable {
            if self.cards_order[card as usize] <= min {
                min = self.cards_order[card as usize];
                min_card = card;
            }
        }
        min_card
    }
}