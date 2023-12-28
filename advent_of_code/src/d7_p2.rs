use std::{fs, fmt::Write};

#[derive(Clone, Copy, Eq, Ord)]
struct Card {
    value: char
}

impl std::fmt::Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.value);
        return Ok(());
    }
}


#[derive(Clone, Copy, Eq)]
struct Hand {
    cards: (Card, Card, Card, Card, Card),
    bid: u64
}

impl std::fmt::Debug for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("<");
        self.cards.0.fmt(f);
        self.cards.1.fmt(f);
        self.cards.2.fmt(f);
        self.cards.3.fmt(f);
        self.cards.4.fmt(f);
        f.write_str("; ");
        f.write_str(self.bid.to_string().as_str());
        f.write_str(">");
        return Ok(());
    }
}

/*
    A, K, Q, J, T
*/



impl Card {
    pub fn from(c: char) -> Card {
        let possible_cards = vec!['A', 'K', 'Q', 'J', 'T', 'J'];
        let v = c as u16;
        if v <= '9' as u16 && v >= '0' as u16 {
            return Card { value: c };
        } else if possible_cards.contains(&c) {
            return Card { value: c };
        } else {
            panic!();
        }
    }

    pub fn tn(&self) -> u16 {
        return match self.value {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 1,
            'T' => 10,
            _ => self.value as u16 - '0' as u16
        };
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        return self.value == other.value;
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return self.tn().partial_cmp(&other.tn());
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        return self.cards.0 == other.cards.0
            && self.cards.1 == other.cards.1
            && self.cards.2 == other.cards.2
            && self.cards.3 == other.cards.3
            && self.cards.4 == other.cards.4;
    }
}

/*
- Five of a kind, where all five cards have the same label: AAAAA
- Four of a kind, where four cards have the same label and one card has a different label: AA8AA
- Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
- Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
- Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
- One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
*/

impl Hand {
    fn is_five_of_a_kind(&self) -> bool {
        return self.cards.0 == self.cards.1
            && self.cards.1 == self.cards.2 
            && self.cards.2 == self.cards.3
            && self.cards.3 == self.cards.4;
    }

    fn is_four_of_a_kind(&self) -> bool {
        let mut v_cards = vec![self.cards.0, self.cards.1, self.cards.2, self.cards.3, self.cards.4];
        // assert!(!self.is_five_of_a_kind());
        v_cards.sort();
        let mut b = true;
        for i in 0..3 {
            b &= v_cards[i] == v_cards[i+1];
        }
        if b { return true; }

        b = true;
        for i in 1..4 {
            b &= v_cards[i] == v_cards[i+1];
        }
        return b;
    }

    fn is_full_house(&self) -> bool {
        let mut v_cards = vec![self.cards.0, self.cards.1, self.cards.2, self.cards.3, self.cards.4];
        // assert!(!self.is_four_of_a_kind());
        v_cards.sort();
        let mut b = true;
        for i in 0..1 {
            b &= v_cards[i] == v_cards[i+1];
        }
        for i in 2..4 {
            b &= v_cards[i] == v_cards[i+1];
        }
        if b { return true; }

        b = true;
        for i in 0..2 {
            b &= v_cards[i] == v_cards[i+1];
        }
        for i in 3..4 {
            b &= v_cards[i] == v_cards[i+1];
        }
        return b;
    }

    fn is_three_of_a_kind(&self) -> bool {
        let mut v_cards = vec![self.cards.0, self.cards.1, self.cards.2, self.cards.3, self.cards.4];
        // assert!(!self.is_full_house());
        v_cards.sort();
        // println!("sorted {v_cards:?}");
        let mut b = true;
        for i in 0..2 {
            b &= v_cards[i] == v_cards[i+1];
        }
        if b { return true; }
        let mut b = true;
        for i in 1..3 {
            b &= v_cards[i] == v_cards[i+1];
        }
        if b { return true; }
        let mut b = true;
        for i in 2..4 {
            b &= v_cards[i] == v_cards[i+1];
        }
        return b;
    }

    fn is_two_pair(&self) -> bool {
        let mut v_cards = vec![self.cards.0, self.cards.1, self.cards.2, self.cards.3, self.cards.4];
        // assert!(!self.is_three_of_a_kind());
        v_cards.sort();
        let mut b = false;
        for i in 0..4 {
            if b && v_cards[i] == v_cards[i+1] { return true; }
            b |= v_cards[i] == v_cards[i+1];
        }
        return false;
    }

    fn is_one_pair(&self) -> bool {
        let mut v_cards = vec![self.cards.0, self.cards.1, self.cards.2, self.cards.3, self.cards.4];
        // assert!(!self.is_two_pair());
        v_cards.sort();
        for i in 0..4 {
            if v_cards[i] == v_cards[i+1] { return true; }
        }
        return false;
    }

    fn replace_jokers(&self, f: fn(Hand) -> bool) -> bool {
        let mut v_cards = vec![self.cards.0, self.cards.1, self.cards.2, self.cards.3, self.cards.4];
        let mut joker_indices = vec![];
        for (i, card) in v_cards.iter().enumerate() {
            if card.value == 'J' { joker_indices.push(i); }
        }
        fn get_hands(v: &Vec<Hand>, i: usize) -> Vec<Hand> {
            let other_card_chars = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A'];
            let mut new_vec = vec![];
            for hand in v {
                for c in other_card_chars.iter() {
                    new_vec.push(
                        Hand {
                            cards: match i {
                                0 => (Card::from(*c), hand.cards.1, hand.cards.2, hand.cards.3, hand.cards.4),
                                1 => (hand.cards.0, Card::from(*c), hand.cards.2, hand.cards.3, hand.cards.4),
                                2 => (hand.cards.0, hand.cards.1, Card::from(*c), hand.cards.3, hand.cards.4),
                                3 => (hand.cards.0, hand.cards.1, hand.cards.2, Card::from(*c), hand.cards.4),
                                4 => (hand.cards.0, hand.cards.1, hand.cards.2, hand.cards.3, Card::from(*c)),
                                _ => panic!()
                            },
                            bid: hand.bid
                        }
                    )
                }
            }
            return new_vec;
        }

        let mut hands = vec![*self];
        for i in joker_indices {
            hands = get_hands(&hands, i);
        }

        for hand in hands {
            if f(hand) {
                return true;
            }
        }
        return false;
    }
}



impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.replace_jokers(|f| { f.is_five_of_a_kind() }) {
            // println!("self is a five of a kind! {:?}", self);
            return if !other.replace_jokers(|f| { f.is_five_of_a_kind() }) { 
                Some(std::cmp::Ordering::Greater)
            } else {
                self.cards.partial_cmp(&other.cards)
            }
        } else if other.replace_jokers(|f| { f.is_five_of_a_kind() }) {
            return Some(std::cmp::Ordering::Less);
        }

        if self.replace_jokers(|h| { h.is_four_of_a_kind() }) {
            // println!("self is a four of a kind! {:?}", self);
            return if !other.replace_jokers(|h| { h.is_four_of_a_kind() }) { 
                Some(std::cmp::Ordering::Greater)
            } else {
                self.cards.partial_cmp(&other.cards)
            }
        } else if other.replace_jokers(|h| { h.is_four_of_a_kind() }) {
            return Some(std::cmp::Ordering::Less);
        }

        if self.replace_jokers(|h| { h.is_full_house() }) {
            // println!("self is a full house! {:?}", self);
            return if !other.replace_jokers(|h| { h.is_full_house() }) { 
                Some(std::cmp::Ordering::Greater)
            } else {
                self.cards.partial_cmp(&other.cards)
            }
        } else if other.replace_jokers(|h| { h.is_full_house() }) {
            return Some(std::cmp::Ordering::Less);
        }

        if self.replace_jokers(|h| { h.is_three_of_a_kind() }) {
            // println!("self is a three of a kind! {:?}", self);
            return if !other.replace_jokers(|h| { h.is_three_of_a_kind() }) { 
                Some(std::cmp::Ordering::Greater)
            } else {
                self.cards.partial_cmp(&other.cards)
            }
        } else if other.replace_jokers(|h| { h.is_three_of_a_kind() }) {
            return Some(std::cmp::Ordering::Less);
        }

        if self.replace_jokers(|h| { h.is_two_pair() }) {
            // println!("self is a two pair! {:?}", self);
            return if !other.replace_jokers(|h| {h.is_two_pair()}) { 
                Some(std::cmp::Ordering::Greater)
            } else {
                self.cards.partial_cmp(&other.cards)
            }
        } else if other.replace_jokers(|h| {h.is_two_pair()}) {
            return Some(std::cmp::Ordering::Less);
        }

        if self.replace_jokers(|h| {h.is_one_pair()}) {
            // println!("self is a one pair! {:?}", self);
            return if !other.replace_jokers(|h| {h.is_one_pair()}) { 
                Some(std::cmp::Ordering::Greater)
            } else {
                self.cards.partial_cmp(&other.cards)
            }
        } else if other.replace_jokers(|h| {h.is_one_pair()}) {
            return Some(std::cmp::Ordering::Less);
        }

        // first card highest power
        return self.cards.partial_cmp(&other.cards);
    }    
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.partial_cmp(other).unwrap();
    }
}

pub fn solve() {
    let contents = fs::read_to_string("resources/d7.txt")
        .expect("Cannot open file to read");
    let mut lines = contents.lines();
    let mut hands: Vec<Hand> = vec![];
    for line in lines {
        let mut sp = line.split_ascii_whitespace();
        let mut cards: Vec<Card> = vec![];
        for c in sp.next().unwrap().chars() {
            cards.push(Card::from(c));
        }
        assert!(cards.len() == 5);
        hands.push(
            Hand { 
                cards: (cards[0], cards[1], cards[2], cards[3], cards[4]),
                bid: str::parse::<u64>(sp.next().unwrap()).unwrap()
            }
        );
    }
    hands.sort();
    for hand in hands.iter().enumerate() {
        println!("{:?}. {:?}", hand.0, hand.1);
    }
    let mut ans: u64 = 0;
    for i in 1..hands.len()+1 {
        ans += i as u64 * hands[i-1].bid;
    }
    println!("ans: {ans}");
}
