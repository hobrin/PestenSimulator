<<<<<<< HEAD
<<<<<<< HEAD
use lib::get_card_nr;
use rand::seq::SliceRandom;
=======
>>>>>>> db1f889b036d83291c87ffda4840e9d97c634095
=======
>>>>>>> db1f889b036d83291c87ffda4840e9d97c634095
use rand::{Rng};
use std::io;
use std::time::Instant;
use std::sync::{Arc, Mutex};
use std::thread::{self, LocalKey};

<<<<<<< HEAD
<<<<<<< HEAD
mod lib;
mod cpu;



fn playGame(players: &Vec<Box<dyn cpu::Player>>, verbose: bool, randomStart: bool) -> u8{
    let mut stack: Vec<u8> = (0u8..54u8).collect();

    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
    stack.shuffle(&mut rng);

    let mut playStack: Vec<u8> = Vec::new();
    playStack.push(lib::draw_cards_for(1, &mut stack, None)[0]);
    if verbose {
        println!("First card is {}: {}\n", playStack[0], lib::get_name(playStack[0]));
=======
=======
>>>>>>> db1f889b036d83291c87ffda4840e9d97c634095
fn sum(cards: &Vec<bool>) -> u8 {
    let mut count: u8 = 0;
    for card in cards.iter() {
        if *card {count+=1;}
    }
    count
}

fn getCardNr(card: u8) -> u8 {
    if card < 52 {card%13} else {13}
}
fn getCardClass(card: u8) -> u8 {
    card / 13 //return 4 for joker
}
fn getName(card: u8) -> String {
    if card < 52 {
        let card_number = ["Ace", "2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King"]
            .get(usize::try_from(getCardNr(card)).unwrap())
            .unwrap();

        let card_class = ["Clubs", "Diamonds", "Hearts", "Spades"]
            .get(usize::try_from(getCardClass(card)).unwrap())
            .unwrap();
        format!("{} of {}", card_number, card_class)
    } else {
        format!("joker")
    }
}

fn canCardStack(card: u8, prevCard: u8) -> bool {
    card >= 52 || prevCard >= 52 || getCardClass(card) == getCardClass(prevCard) || getCardNr(card) == getCardNr(prevCard) || getCardNr(card) == 10
}

fn drawCardsFor(nToDraw: u8, fromStack: &mut Vec<u8>, playStack: Option<&mut Vec<u8>>) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    if (fromStack.len() as u8) < nToDraw {
        let playStack2 = playStack.unwrap();
        while playStack2.len()>1 {
            fromStack.push(playStack2.remove(0));
        }
    }
    for _i in 0..nToDraw {
        let idx: usize = rand::thread_rng().gen_range(0..fromStack.len());
        result.push(fromStack[idx]);
        fromStack.remove(idx);
    }
    result
}

trait Player {
    fn maketurn(&self, id: usize, cardsInHand: &Vec<u8>, onstack: u8, instack: u8, cardsInOtherPlayers: &Vec<u8>, todraw: u8) -> u8;
    }

struct ConsolePlayer {

}
impl ConsolePlayer {
    fn new() -> ConsolePlayer {
        ConsolePlayer{}
    }
}
impl Player for ConsolePlayer {
    fn maketurn(&self, _id: usize, cardsInHand: &Vec<u8>, onstack: u8, _instack: u8, cardsInOtherPlayers: &Vec<u8>, _todraw: u8) -> u8 {
        println!("\nYou have these cards:");
        for card in cardsInHand.iter() {
            println!("{}: {}", card, getName(*card));
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
                if !canCardStack(value, onstack) {
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
struct RandomPlayer {

}
impl RandomPlayer {
    fn new() -> RandomPlayer {
        RandomPlayer{}
    }
}
impl Player for RandomPlayer {
    fn maketurn(&self, _id: usize, cardsInHand: &Vec<u8>, onstack: u8, _instack: u8, _cardsInOtherPlayers: &Vec<u8>, _todraw: u8) -> u8 {
        let playable: Vec<u8> = cardsInHand.iter().cloned().filter(|&x| canCardStack(x, onstack)).collect();

        playable[rand::thread_rng().gen_range(0..playable.len()) as usize]
    }
}

struct PeaceKeeperPlayer {

}
impl PeaceKeeperPlayer {
    fn new() -> PeaceKeeperPlayer {
        PeaceKeeperPlayer{}
    }
}
impl Player for PeaceKeeperPlayer {
    fn maketurn(&self, _id: usize, cardsInHand: &Vec<u8>, onstack: u8, _instack: u8, _cardsInOtherPlayers: &Vec<u8>, _todraw: u8) -> u8 {
        let mut playable: Vec<u8> = cardsInHand.iter().cloned().filter(|&x| canCardStack(x, onstack)).collect();
        let mut idx: i32 = 0;
        while (idx as usize) < playable.len() {
            let card = playable[idx as usize];
            if getCardNr(card) == 1 || getCardNr(card) == 13 {
                if playable.len() > 1 {
                    playable.remove(idx as usize);
                    idx-=1;
                }
            }
            idx+=1;
        }
        playable[rand::thread_rng().gen_range(0..playable.len()) as usize]
    }
}

struct BullyPlayer {

}
impl BullyPlayer {
    fn new() -> BullyPlayer {
        BullyPlayer{}
    }
}
impl Player for BullyPlayer {
    fn maketurn(&self, _id: usize, cardsInHand: &Vec<u8>, onstack: u8, _instack: u8, _cardsInOtherPlayers: &Vec<u8>, _todraw: u8) -> u8 {
        let playable: Vec<u8> = cardsInHand.iter().cloned().filter(|&x| canCardStack(x, onstack)).collect();
        for card in playable.iter() {
            if getCardNr(*card) == 1 || getCardNr(*card) == 13 {
                return *card;
            }
        }
        playable[rand::thread_rng().gen_range(0..playable.len()) as usize]
    }
}

struct CarefulBullyPlayer {
    maxBeforeBully: u8,
    retainJack: bool,
    throwJackDesperado: bool,
    throwSevenFirst: bool,
    retainSeven: bool,
}
impl CarefulBullyPlayer {
    fn new(maxBeforeBully: u8, retainJack: bool, throwJackDesperado: bool, throwSevenFirst: bool, retainSeven: bool) -> CarefulBullyPlayer {
        CarefulBullyPlayer{maxBeforeBully: maxBeforeBully, retainJack: retainJack, throwJackDesperado: throwJackDesperado, throwSevenFirst: throwSevenFirst, retainSeven: retainSeven}
    }
}
impl Player for CarefulBullyPlayer {
    fn maketurn(&self, id: usize, cardsInHand: &Vec<u8>, onstack: u8, _instack: u8, cardsInOtherPlayers: &Vec<u8>, todraw: u8) -> u8 {
        let mut playable: Vec<u8> = cardsInHand.iter().cloned().filter(|&x| canCardStack(x, onstack)).collect();

        if todraw > 0 || cardsInOtherPlayers[(id+1) % cardsInOtherPlayers.len()] < self.maxBeforeBully {
            for card in playable.iter() { //throw cards as soon as possible.
                if getCardNr(*card) == 1 || getCardNr(*card) == 13 || (self.throwJackDesperado && todraw==0 && getCardNr(*card) == 10) {
                    return *card;
                }
            }
        } else { //retain cards.
            let mut idx: i32 = 0;
            while (idx as usize) < playable.len() {
                let card = playable[idx as usize];
                if getCardNr(card) == 1 || getCardNr(card) == 13 || (self.retainJack && getCardNr(card) == 10) || (self.retainSeven && getCardNr(card) == 6) {
                    if playable.len() > 1 {
                        if getCardNr(card) == 6 { //check to make sure it has card after this, so that it doesn't get stuck with a 7.
                            if playable.iter().filter(|&&followup| canCardStack(followup, card)).count() < 3 {
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
            if let Some(card) = playable.iter().find(|&&card| card==6 &&
                cardsInHand.iter().any(|&followup| canCardStack(followup, card) && followup != card)) {
                return *card;
            }
        }


        playable[rand::thread_rng().gen_range(0..playable.len()) as usize]
    }
}

fn playGame(players: &Vec<Box<dyn Player>>, verbose: bool, randomStart: bool) -> u8{
    let mut stack: Vec<u8> = (0u8..54u8).collect();

    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();

    let mut playStack: Vec<u8> = Vec::new();
    playStack.push(drawCardsFor(1, &mut stack, None)[0]);
    if verbose {
        println!("First card is {}: {}\n", playStack[0], getName(playStack[0]));
<<<<<<< HEAD
>>>>>>> db1f889b036d83291c87ffda4840e9d97c634095
=======
>>>>>>> db1f889b036d83291c87ffda4840e9d97c634095
    }

    let mut playerCards: Vec<Vec<u8>> = Vec::new();
    for _ in 0..players.len() { //we are not going to talk about this code
<<<<<<< HEAD
<<<<<<< HEAD
        playerCards.push(lib::draw_cards_for(7, &mut stack, Some(&mut playStack)));
=======
        playerCards.push(drawCardsFor(7, &mut stack, Some(&mut playStack)));
>>>>>>> db1f889b036d83291c87ffda4840e9d97c634095
=======
        playerCards.push(drawCardsFor(7, &mut stack, Some(&mut playStack)));
>>>>>>> db1f889b036d83291c87ffda4840e9d97c634095
    }



    let mut toDraw: u8 = 0;
<<<<<<< HEAD
<<<<<<< HEAD
    let mut reverse_order: bool = rng.gen_bool(0.5);
    let mut skip_rev: usize = 0;
    let mut skip: i32 = randomStart as i32 * rng.gen_range(0..players.len() as i32);
    let mut othersHave: Vec<u8> = (0..players.len() as u8).collect();
    loop {
        let iterator = players.iter().enumerate();
        'player_turns: for i in 0..players.len() { //each player takes a turn
            if skip_rev > 0 {
                skip_rev-=1;
                continue;
            }
            if reverse_order {
                skip_rev = n_players-2;
            }
            if skip>0 {
                skip-=1;
                continue;
            }
            let p = &players[i];
=======
=======
>>>>>>> db1f889b036d83291c87ffda4840e9d97c634095
    let mut skip: bool = randomStart && rng.gen_bool(0.5);
    let mut othersHave: Vec<u8> = (0..players.len() as u8).collect();
    loop {
        'player_turns: for (i, p) in players.iter().enumerate() { //each player takes a turn
            if skip {
                skip = false;
                continue;
            }
<<<<<<< HEAD
>>>>>>> db1f889b036d83291c87ffda4840e9d97c634095
=======
>>>>>>> db1f889b036d83291c87ffda4840e9d97c634095
            loop { //loop is there for 7 card
                for (i, _) in players.iter().enumerate() {
                    othersHave[i] = playerCards[i].len() as u8;
                }
                let lastCard: u8 = *playStack.last().unwrap();
<<<<<<< HEAD
<<<<<<< HEAD

                let playable: Vec<u8> = playerCards[i].iter().filter(|&&card| lib::can_card_stack(card, lastCard)).map(|c| *c).collect();
                let mut turn: u8;
                
                if playable.is_empty() { //if cannot place a card then draw one
                    playerCards[i].push(lib::draw_cards_for(1, &mut stack, Some(&mut playStack))[0]);
                    if verbose {
                        println!("{} draw a card because he can't play.", i);
                    }
                    if !lib::can_card_stack(*playerCards[i].last().unwrap(), lastCard) {
=======
=======
>>>>>>> db1f889b036d83291c87ffda4840e9d97c634095
                
                if !playerCards[i].iter().any(|x| canCardStack(*x, lastCard)) { //if cannot place a card then draw one
                    playerCards[i].push(drawCardsFor(1, &mut stack, Some(&mut playStack))[0]);
                    if verbose {
                        println!("{} draw a card because he can't play.", i);
                    }
                    if !canCardStack(*playerCards[i].last().unwrap(), lastCard) {
<<<<<<< HEAD
>>>>>>> db1f889b036d83291c87ffda4840e9d97c634095
=======
>>>>>>> db1f889b036d83291c87ffda4840e9d97c634095
                        if verbose {
                            println!("and he couldn't he play the picked up card.");
                        }
                        continue 'player_turns; //next player is.
<<<<<<< HEAD
<<<<<<< HEAD
                    } else {
                        turn = *playerCards[i].last().unwrap();
                    }
                } else {
                    turn = p.maketurn(i, &playerCards[i], &playable, lastCard, stack.len() as u8, &othersHave, toDraw);
                }
    
                if verbose {
                    println!("{} placed card: {}: {}\n", i, turn, lib::get_name(turn));
=======
=======
>>>>>>> db1f889b036d83291c87ffda4840e9d97c634095
                    }
                }
    
                let turn: u8 = p.maketurn(i, &playerCards[i], lastCard, stack.len() as u8, &othersHave, toDraw);
                if verbose {
                    println!("{} placed card: {}: {}\n", i, turn, getName(turn));
<<<<<<< HEAD
>>>>>>> db1f889b036d83291c87ffda4840e9d97c634095
=======
>>>>>>> db1f889b036d83291c87ffda4840e9d97c634095
                }
                playStack.push(turn);
    
                playerCards[i].retain(|&x| x != turn);

                if playerCards[i].len() == 0 {
                    return i as u8; //winner
                }
    
<<<<<<< HEAD
<<<<<<< HEAD
                if lib::get_card_nr(turn) == 1 {
                    toDraw += 2;
                } else if lib::get_card_nr(turn) == 13 {
                    toDraw += 5;
                } else if toDraw > 0 { //make the player pick up the cards.
                    playerCards[i].extend(&lib::draw_cards_for(toDraw, &mut stack, Some(&mut playStack)));
=======
=======
>>>>>>> db1f889b036d83291c87ffda4840e9d97c634095
                if getCardNr(turn) == 1 {
                    toDraw += 2;
                } else if getCardNr(turn) == 13 {
                    toDraw += 5;
                } else if toDraw > 0 { //make the player pick up the cards.
                    playerCards[i].extend(&drawCardsFor(toDraw, &mut stack, Some(&mut playStack)));
<<<<<<< HEAD
>>>>>>> db1f889b036d83291c87ffda4840e9d97c634095
=======
>>>>>>> db1f889b036d83291c87ffda4840e9d97c634095
                    if verbose {
                        println!("{} picked up {} cards.", i, toDraw);
                    }
                    toDraw = 0;
                }
<<<<<<< HEAD
<<<<<<< HEAD
                if lib::get_card_nr(turn) == 0 { //Ace, flip the pace
                    reverse_order = !reverse_order;
                }
                if lib::get_card_nr(turn) == 6 { //7 Seven in heaven, the game unfolds, lay another card, see what it holds. -ChatGPT
                    continue;
                }
                if lib::get_card_nr(turn) == 7 { //8 wait
                    skip = 1;
=======
=======
>>>>>>> db1f889b036d83291c87ffda4840e9d97c634095
                if getCardNr(turn) == 6 { //7 Seven in heaven, the game unfolds, lay another card, see what it holds. -ChatGPT
                    continue;
                }
                if getCardNr(turn) == 7 { //8 wait
                    skip = true;
<<<<<<< HEAD
>>>>>>> db1f889b036d83291c87ffda4840e9d97c634095
=======
>>>>>>> db1f889b036d83291c87ffda4840e9d97c634095
                }
                break;
            }
        }
    }
}
<<<<<<< HEAD
<<<<<<< HEAD
const n_players: usize = 2;
fn test_strategy(total_games: i32, cards_order: [u8; 54]) -> i32 {
    let before = Instant::now();
    let num_threads = 4;
    let wins = Arc::new(Mutex::new([0; n_players]));
    let iterations_per_thread = total_games / num_threads;
    let mut handles: Vec<_> = vec![];

    for _ in 0..num_threads {
        let wins = Arc::clone(&wins);

        let handle = thread::spawn(move || {
            let mut players: Vec<Box<dyn cpu::Player>> = Vec::new();
            // players.push(Box::new(cpu::DetectivePlayer::new(cards_order.clone())));
            // players.push(Box::new(cpu::RandomPlayer::new()));
            players.push(Box::new(cpu::CarefulBullyPlayer::new(3, true, false, true, false)));
            players.push(Box::new(cpu::CarefulBullyPlayer::new(3, true, false, true, false)));

            let mut wins_local = [0;n_players];

            //[339692, 203153, 226660, 230495]

=======
=======
>>>>>>> db1f889b036d83291c87ffda4840e9d97c634095

fn main() {
    println!("Pesten: with the rules: ");
    println!("- no special king move");
    println!("- jack can always be placed but not change the class");
    println!("- you can win by throwing a special card");

    
    let before = Instant::now();
    let num_threads = 8;
    let wins = Arc::new(Mutex::new([0; 2]));
    let iterations_per_thread = 1_000_000 / num_threads;
    let mut handles: Vec<_> = vec![];
    for _ in 0..num_threads {
        let wins = Arc::clone(&wins);
        let handle = thread::spawn(move || {
            let mut players: Vec<Box<dyn Player>> = Vec::new();
            players.push(Box::new(RandomPlayer::new()));
            players.push(Box::new(CarefulBullyPlayer::new(3, true, false, true, false)));
            // players.push(Box::new(CarefulBullyPlayer::new(3, true, false, true, false)));
    
            let mut wins_local = [0;2];
    
<<<<<<< HEAD
>>>>>>> db1f889b036d83291c87ffda4840e9d97c634095
=======
>>>>>>> db1f889b036d83291c87ffda4840e9d97c634095
            for _j in 0..iterations_per_thread {
                let win = playGame(&players, false, true);
                wins_local[win as usize] += 1;
            }

            let mut w = wins.lock().unwrap();
            for i in 0..players.len() {
                w[i] += wins_local[i];
            }
        });
        handles.push(handle);
    }
<<<<<<< HEAD
<<<<<<< HEAD

    for handle in handles { //wait for every thread to finish.
        handle.join().unwrap();
    }

    let wins = wins.lock().unwrap();
    println!("Executed in: {} seconds", before.elapsed().as_secs());
    println!("{:?}", wins);
    println!("winrate p1 increased: {}", (wins[0] as f32)/(total_games as f32 / n_players as f32));
    wins[0]
}

fn main() {
    println!("Pesten: with the rules: ");
    println!("- no special king move");
    println!("- jack can always be placed but not change the class");
    println!("- you can win by throwing a special card");

    // figure_out_cards_order();

    let cards_order: [u8; 54] = [20, 29, 44, 36, 43, 16, 10, 9, 15, 2, 45, 14, 40, 39, 50, 13, 18, 37, 26, 17, 5, 33, 34, 49, 12, 24, 31, 28, 11, 6, 46, 32, 30, 3, 0, 35, 53, 41, 1, 42, 19, 22, 7, 38, 21, 4, 25, 47, 27, 51, 8, 23, 48, 52];
    let winsP1: i32 = test_strategy(1_000_000, cards_order);
}

fn figure_out_cards_order() {
    let mut cards_order: [u8; 54] = [0; 54];
    for i in 0..54 {
        cards_order[i] = i as u8;
    }
    let mut rng = rand::thread_rng();
    cards_order.shuffle(&mut rng);

    let mut norm = 0;
    for i in 0..500 {
        let a = rng.gen_range(0..54);
        let b = rng.gen_range(0..54);
        let val = cards_order[a];
        cards_order[a] = cards_order[b];
        cards_order[b] = val;

        let wins = test_strategy(100_000, cards_order);
        if wins > norm {
            norm = wins;
        } else {
            let val = cards_order[a];
            cards_order[a] = cards_order[b];
            cards_order[b] = val;
            norm = (norm*9 + wins) / 10;
        }
    }
    for i in 0..54 {
        for j in 0..54 {
            if cards_order[j] == i {
                println!("{}: {}", i, lib::get_name(j as u8));
            }
        }
    }
    println!("{:?}", cards_order);
=======
=======
>>>>>>> db1f889b036d83291c87ffda4840e9d97c634095
    for handle in handles { //wait for every thread to finish.
        handle.join().unwrap();
    }
    let wins = wins.lock().unwrap();



    // for _i in 0..1_000_000 {
    //     let win = playGame(&players, false, false);
    //     wins[win as usize] += 1;
    // }

    println!("Executed in: {} seconds", before.elapsed().as_secs());

    println!("{:?}", wins);
    println!("winrate p1: {}", (wins[0] as f32)/1_000_000f32*2f32-1f32)
<<<<<<< HEAD
>>>>>>> db1f889b036d83291c87ffda4840e9d97c634095
=======
>>>>>>> db1f889b036d83291c87ffda4840e9d97c634095
}
