
use rand::{Rng};

pub fn sum(cards: &Vec<bool>) -> u8 {
    let mut count: u8 = 0;
    for card in cards.iter() {
        if *card {count+=1;}
    }
    count
}

pub const card_nr: [u8; 54] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12,
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12,
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12,
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12,
    13, 13
];

pub fn get_card_nr(card: u8) -> u8 {
    card_nr[card as usize]
    // if card < 52 {
    //     card%13
    // } else {
    //     13
    // }
}
pub fn get_card_class(card: u8) -> u8 {
    card / 13 //return 4 for joker
}
pub fn get_name(card: u8) -> String {
    if card < 52 {
        let card_number = ["Ace", "2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King"]
            .get(usize::try_from(get_card_nr(card)).unwrap())
            .unwrap();

        let card_class = ["Clubs", "Diamonds", "Hearts", "Spades"]
            .get(usize::try_from(get_card_class(card)).unwrap())
            .unwrap();
        format!("{} of {}", card_number, card_class)
    } else {
        format!("joker")
    }
}

pub fn can_card_stack(card: u8, prev_card: u8) -> bool {
    card >= 52 || prev_card >= 52 || get_card_class(card) == get_card_class(prev_card) || get_card_nr(card) == get_card_nr(prev_card) || get_card_nr(card) == 10
}

pub fn draw_cards_for(n_to_draw: u8, fromStack: &mut Vec<u8>, play_stack: Option<&mut Vec<u8>>) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    if (fromStack.len() as u8) < n_to_draw {
        let playStack2 = play_stack.unwrap();
        while playStack2.len()>1 {
            fromStack.push(playStack2.remove(0));
        }
    }
    for _i in 0..n_to_draw {
        let idx: usize = rand::thread_rng().gen_range(0..fromStack.len());
        result.push(fromStack[idx]);
        fromStack.remove(idx);
    }
    result
}