use crate::input_string;
use std::cmp::Ordering;
use std::io::Read;

const CARDS_IN_PLAY: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];

const CARDS_IN_PLAY_JOKER_RULES: [char; 13] = [
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
];

pub fn run1() -> String {
    let input = input_string!();

    let mut hands = Vec::new();

    for line in input.lines() {
        let (hand, bid) = line.split_at(5);
        let play = Play::new(hand, bid.trim(), false);
        hands.push(play);
    }

    sort_hands(&mut hands, false);

    let mut sum = 0;
    for (i, hand) in hands.iter().enumerate() {
        sum += hand.bid * (i + 1) as u128;
    }

    sum.to_string()
}

pub fn run2() -> String {
    let input = input_string!();
    //let input = test_input!();
    let mut hands = Vec::new();

    for line in input.lines() {
        let (hand, bid) = line.split_at(5);
        let play = Play::new(hand, bid.trim(), true);
        hands.push(play);
    }

    sort_hands(&mut hands, true);

    for hand in hands.iter() {
        println!("Hand {:?} --> Type {:?}", hand.cards, hand.hand);
    }

    let mut sum = 0;
    for (i, hand) in hands.iter().enumerate() {
        sum += hand.bid * (i + 1) as u128;
    }

    sum.to_string()
}

#[derive(Debug)]
pub struct Play {
    hand: Hand,
    cards: String,
    bid: u128,
}
impl Play {
    fn new(cards: &str, bid: &str, rule: bool) -> Play {
        Play {
            hand: get_hand_type(cards, rule),
            cards: cards.to_string(),
            bid: bid.parse::<u128>().unwrap(),
        }
    }
}
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Hand {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

pub fn get_hand_type(hand: &str, rule: bool) -> Hand {
    let mut cards = [0; 13];
    for c in hand.chars() {
        match rule {
            false => {
                let card = match c {
                    'A' => 0,
                    'K' => 1,
                    'Q' => 2,
                    'J' => 3,
                    'T' => 4,
                    '9' => 5,
                    '8' => 6,
                    '7' => 7,
                    '6' => 8,
                    '5' => 9,
                    '4' => 10,
                    '3' => 11,
                    '2' => 12,
                    _ => panic!("Invalid card"),
                };
                cards[card] += 1;
            }
            true => {
                let card = match c {
                    'A' => 0,
                    'K' => 1,
                    'Q' => 2,
                    'T' => 3,
                    '9' => 4,
                    '8' => 5,
                    '7' => 6,
                    '6' => 7,
                    '5' => 8,
                    '4' => 9,
                    '3' => 10,
                    '2' => 11,
                    'J' => 12,

                    _ => panic!("Invalid card"),
                };
                cards[card] += 1;
            }
        }
    }

    match rule {
        false => get_hand_type_no_joker(&cards),
        true => get_hand_type_joker(&cards),
    }
}

fn get_hand_type_joker(cards: &[i32; 13]) -> Hand {
    let mut pairs = 0;
    let mut three = false;
    let mut four = false;
    let jokers = cards[12];
    for i in 0..12 {
        match cards[i] {
            4 => four = true,
            3 => three = true,
            2 => pairs += 1,
            _ => (),
        }
    }

    if jokers == 0 {
        return get_hand_type_no_joker(cards);
    }

    if jokers > 3 || four || (three && jokers == 2) || (jokers == 3 && pairs == 1) {
        return Hand::FiveOfAKind;
    }

    if (jokers == 3 && pairs == 0) || (pairs == 1 && jokers == 2) || (three && jokers == 1) {
        return Hand::FourOfAKind;
    }

    if (pairs == 1 && jokers == 1) || (jokers == 2 && pairs == 0) {
        return Hand::ThreeOfAKind;
    }

    if pairs == 2 {
        return Hand::FullHouse;
    }

    if jokers == 1 && pairs == 0 && !three {
        return Hand::OnePair;
    }

    panic!()
}

fn get_hand_type_no_joker(cards: &[i32; 13]) -> Hand {
    let mut pairs = 0;
    let mut three = false;
    let mut four = false;
    let mut five = false;
    for i in 0..13 {
        match cards[i] {
            5 => five = true,
            4 => four = true,
            3 => three = true,
            2 => pairs += 1,
            _ => (),
        }
    }

    if five {
        Hand::FiveOfAKind
    } else if four {
        Hand::FourOfAKind
    } else if three && pairs == 1 {
        Hand::FullHouse
    } else if three {
        Hand::ThreeOfAKind
    } else if pairs == 2 {
        Hand::TwoPairs
    } else if pairs == 1 {
        Hand::OnePair
    } else {
        Hand::HighCard
    }
}

fn sort_hands(p0: &mut Vec<Play>, rule: bool) {
    let rule_to_use = match rule {
        false => CARDS_IN_PLAY,
        true => CARDS_IN_PLAY_JOKER_RULES,
    };

    p0.sort_by(|a, b| {
        let hand_cmp = a.hand.cmp(&b.hand);
        if hand_cmp == Ordering::Equal {
            let mut a_cards = a.cards.chars();
            let mut b_cards = b.cards.chars();
            loop {
                let a_card = a_cards.next().unwrap();
                let b_card = b_cards.next().unwrap();
                let a_card_index = rule_to_use.iter().position(|&x| x == a_card).unwrap();
                let b_card_index = rule_to_use.iter().position(|&x| x == b_card).unwrap();
                let card_cmp = a_card_index.cmp(&b_card_index).reverse();
                if card_cmp != Ordering::Equal {
                    return card_cmp;
                }
            }
        } else {
            hand_cmp
        }
    });
}
