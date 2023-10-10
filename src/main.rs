use lib::get_card_nr;
use rand::seq::SliceRandom;
use rand::{Rng};
use std::io;
use std::time::Instant;
use std::sync::{Arc, Mutex};
use std::thread::{self, LocalKey};

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
    }

    let mut playerCards: Vec<Vec<u8>> = Vec::new();
    for _ in 0..players.len() { //we are not going to talk about this code
        playerCards.push(lib::draw_cards_for(7, &mut stack, Some(&mut playStack)));
    }



    let mut toDraw: u8 = 0;
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
            loop { //loop is there for 7 card
                for (i, _) in players.iter().enumerate() {
                    othersHave[i] = playerCards[i].len() as u8;
                }
                let lastCard: u8 = *playStack.last().unwrap();

                let playable: Vec<u8> = playerCards[i].iter().filter(|&&card| lib::can_card_stack(card, lastCard)).map(|c| *c).collect();
                let mut turn: u8;
                
                if playable.is_empty() { //if cannot place a card then draw one
                    playerCards[i].push(lib::draw_cards_for(1, &mut stack, Some(&mut playStack))[0]);
                    if verbose {
                        println!("{} draw a card because he can't play.", i);
                    }
                    if !lib::can_card_stack(*playerCards[i].last().unwrap(), lastCard) {
                        if verbose {
                            println!("and he couldn't he play the picked up card.");
                        }
                        continue 'player_turns; //next player is.
                    } else {
                        turn = *playerCards[i].last().unwrap();
                    }
                } else {
                    turn = p.maketurn(i, &playerCards[i], &playable, lastCard, stack.len() as u8, &othersHave, toDraw);
                }
    
                if verbose {
                    println!("{} placed card: {}: {}\n", i, turn, lib::get_name(turn));
                }
                playStack.push(turn);
    
                playerCards[i].retain(|&x| x != turn);

                if playerCards[i].len() == 0 {
                    return i as u8; //winner
                }
    
                if lib::get_card_nr(turn) == 1 {
                    toDraw += 2;
                } else if lib::get_card_nr(turn) == 13 {
                    toDraw += 5;
                } else if toDraw > 0 { //make the player pick up the cards.
                    playerCards[i].extend(&lib::draw_cards_for(toDraw, &mut stack, Some(&mut playStack)));
                    if verbose {
                        println!("{} picked up {} cards.", i, toDraw);
                    }
                    toDraw = 0;
                }
                if lib::get_card_nr(turn) == 0 { //Ace, flip the pace
                    reverse_order = !reverse_order;
                }
                if lib::get_card_nr(turn) == 6 { //7 Seven in heaven, the game unfolds, lay another card, see what it holds. -ChatGPT
                    continue;
                }
                if lib::get_card_nr(turn) == 7 { //8 wait
                    skip = 1;
                }
                break;
            }
        }
    }
}
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
}
