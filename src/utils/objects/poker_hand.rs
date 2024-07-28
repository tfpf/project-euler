/// A hand of poker.
#[derive(Eq, PartialEq)]
pub struct PokerHand {
    // Pairs in which the first number is the value of a card and the second is
    // its suit. The pairs shall be sorted in descending order of the value.
    hand: Vec<(u8, u8)>,
    // Used to rank this hand on an arbitrary scale. Higher is better.
    score: i32,
}

impl PokerHand {
    const HIGH_CARD: i32 = 0;
    const ONE_PAIR: i32 = 1 << 16;
    const TWO_PAIRS: i32 = 2 << 16;
    const THREE_OF_A_KIND: i32 = 3 << 16;
    const STRAIGHT: i32 = 4 << 16;
    const FLUSH: i32 = 5 << 16;
    const FULL_HOUSE: i32 = 6 << 16;
    const FOUR_OF_A_KIND: i32 = 7 << 16;
    const STRAIGHT_FLUSH: i32 = 8 << 16;
    const ROYAL_FLUSH: i32 = 9 << 16;
}

impl PokerHand {

    /// Construct a hand of poker. As an example, to represent 8 of clubs, 10
    /// of spades, king of clubs, 9 of hearts and 4 of spades, the input shall
    /// be `["8C", "TS", "KC", "9H", "4S"]`.
    pub fn new(hand: &[&str]) -> PokerHand {
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
        let mut poker_hand = PokerHand { hand, score: 0 };
        poker_hand.calculate_score();
        poker_hand
    }

    /// Assign an arbitrary score to the hand of poker.
    fn calculate_score(&mut self) {
        // Sorting the cards by descending order of value makes analysing them
        // easier.
        self.hand.sort_by(|a, b| b.cmp(a));
        let same_suit = self.hand.iter().skip(1).all(|card| card.1 == self.hand[0].1);
        let consecutive_values = self
            .hand
            .iter()
            .enumerate()
            .skip(1)
            .all(|(idx, card)| card.0 == self.hand[idx - 1].0 - 1);

        if same_suit && consecutive_values {
            if self.hand[0].0 == 14 {
                self.score = PokerHand::ROYAL_FLUSH;
            } else {
                self.score = PokerHand::STRAIGHT_FLUSH;
            }
            return;
        }

        // How many times does each value appear?
        let mut values_frequencies = [0; 15];
        for card in &self.hand {
            let value = card.0 as usize;
            values_frequencies[value] += 1;
        }

        // Suppose two hands have the same rank, four of a kind: 3 clubs,
        // 3 diamonds, 3 hearts and 3 spades, and 6 clubs, 6 diamonds, 6 hearts
        // and 6 spades. The second hand is the winner, because the value
        // forming four of a kind is higher. To take care of such
        // possibilities, assign a different weightage to each rank, depending
        // on the values forming that rank. This weightage shall be the extra
        // score added to the rank. (If we have two pairs, assign greater
        // weightage to the pair with the greater value.) If the scores are
        // tied, the greatest values in each hand are compared. Hence, if the
        // rank is formed by all cards (e.g. straight or flush), no extra score
        // has to be added. The weightage must be chosen carefully so that it
        // does not cause the score to overflow into the score of the next
        // higher rank.
        let mut score_weightage = (1, 1 << 8, 1 << 12);
        let (mut twos, mut threes, mut fours) = (0, 0, 0);
        let extra_score: i32 = values_frequencies
            .iter()
            .enumerate()
            .map(|(value, frequency)| match frequency {
                2 => {
                    twos += 1;
                    let extra_score = score_weightage.0 * value as i32;
                    // Increase the weightage for the next pair (if any).
                    score_weightage.0 <<= 4;
                    extra_score
                }
                3 => {
                    threes += 1;
                    score_weightage.1 * value as i32
                }
                4 => {
                    fours += 1;
                    score_weightage.2 * value as i32
                }
                _ => 0,
            })
            .sum();

        if fours == 1 {
            self.score = PokerHand::FOUR_OF_A_KIND + extra_score;
            return;
        }
        if twos == 1 && threes == 1 {
            self.score = PokerHand::FULL_HOUSE + extra_score;
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
            self.score = PokerHand::THREE_OF_A_KIND + extra_score;
            return;
        }
        if twos == 2 {
            self.score = PokerHand::TWO_PAIRS + extra_score;
            return;
        }
        if twos == 1 {
            self.score = PokerHand::ONE_PAIR + extra_score;
            return;
        }
        self.score = PokerHand::HIGH_CARD;
    }
}

impl Ord for PokerHand {
    fn cmp(&self, other: &PokerHand) -> std::cmp::Ordering {
        let compare = self.score.cmp(&other.score);
        if compare != std::cmp::Ordering::Equal {
            return compare;
        }
        for (scard, ocard) in self.hand.iter().zip(other.hand.iter()) {
            let compare = scard.0.cmp(&ocard.0);
            if compare != std::cmp::Ordering::Equal {
                return compare;
            }
        }
        std::cmp::Ordering::Equal
    }
}

impl PartialOrd for PokerHand {
    fn partial_cmp(&self, other: &PokerHand) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
