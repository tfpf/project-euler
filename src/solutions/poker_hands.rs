use std::io::BufRead;

#[derive(Debug)]
struct PokerHand {
    // Pairs in which the first number is the value and the second is the suit.
    hand: Vec<(u8, u8)>,
    score: u8,
}
impl PokerHand {
    const ROYAL_FLUSH: u8 = 0;
    const STRAIGHT_FLUSH: u8 = 1;
    const FOUR_OF_A_KIND: u8 = 2;
    const FULL_HOUSE: u8 = 3;
    const FLUSH: u8 = 4;
    const STRAIGHT: u8 = 5;
    const THREE_OF_A_KIND: u8 = 6;
    const TWO_PAIRS: u8 = 7;
    const ONE_PAIR: u8 = 8;
    const HIGH_CARD: u8 = 9;
}
impl PokerHand {
    fn new(hand: &[&str]) -> PokerHand {
        let hand = hand
            .iter()
            .map(|card| {
                let card = card.as_bytes();
                let value = match card[0] {
                    b'T' => 10,
                    b'J' => 11,
                    b'Q' => 12,
                    b'K' => 13,
                    b'A' => 14,
                    value => value - b'0',
                };
                let suit = card[1];
                (value, suit)
            })
            .collect();
        let mut poker_hand = PokerHand {
            hand: hand,
            score: 0,
        };
        poker_hand.calculate_score();
        poker_hand
    }
    fn calculate_score(&mut self) {
        self.hand.sort();
        let same_suit = self
            .hand
            .iter()
            .skip(1)
            .all(|card| card.1 == self.hand[0].1);
        let consecutive_values = self
            .hand
            .iter()
            .enumerate()
            .skip(1)
            .all(|(idx, card)| card.0 == self.hand[idx - 1].0 + 1);

        if same_suit && consecutive_values {
            if self.hand[0].0 == 10 {
                self.score = PokerHand::ROYAL_FLUSH;
            } else {
                self.score = PokerHand::STRAIGHT_FLUSH;
            }
            return;
        }

        let mut values_frequencies = [0; 15];
        let (twos, threes, fours) =
            self.hand
                .iter()
                .fold((0, 0, 0), |(twos, threes, fours), card| {
                    let value = card.0 as usize;
                    values_frequencies[value] += 1;
                    match values_frequencies[value] {
                        2 => (twos + 1, threes, fours),
                        3 => (twos - 1, threes + 1, fours),
                        4 => (twos, threes - 1, fours + 1),
                        _ => (twos, threes, fours),
                    }
                });

        if fours == 1 {
            self.score = PokerHand::FOUR_OF_A_KIND;
            return;
        }
        if twos == 1 && threes == 1 {
            self.score = PokerHand::FULL_HOUSE;
            return;
        }
        if same_suit {
            self.score = PokerHand::FLUSH;
            return;
        }
        if consecutive_values {
            self.score = PokerHand::STRAIGHT;
            return;
        }
        if threes == 1 {
            self.score = PokerHand::THREE_OF_A_KIND;
            return;
        }
        if twos == 2 {
            self.score = PokerHand::TWO_PAIRS;
            return;
        }
        if twos == 1 {
            self.score = PokerHand::ONE_PAIR;
            return;
        }
        self.score = PokerHand::HIGH_CARD;
    }
}

fn find_winner(p1_hand: &[&str], p2_hand: &[&str]) -> bool {
    let p1_hand = PokerHand::new(p1_hand);
    let p2_hand = PokerHand::new(p2_hand);
    println!("{:?}", p1_hand);
    false
}

pub fn solve() -> i64 {
    let fhandle = std::fs::File::open("res/poker_hands.txt").unwrap();
    let reader = std::io::BufReader::new(fhandle);
    let result = reader
        .lines()
        .filter(|line| {
            let split = line.as_ref().unwrap().split(" ").collect::<Vec<&str>>();
            find_winner(&split[..5], &split[5..])
        })
        .count();

    result as i64
}
