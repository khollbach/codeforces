use std::{collections::HashMap, fmt, io, str::FromStr};

fn main() {
    let mut lines = io::stdin().lines().skip(1).map(Result::unwrap);
    while let Some(n) = lines.next() {
        let n: usize = n.parse().unwrap();
        let trump = match lines.next().unwrap().as_str() {
            s @ ("H" | "C" | "S" | "D") => s.chars().next().unwrap(),
            s => panic!("not a suit: {s:?}"),
        };
        let cards: Vec<Card> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        assert_eq!(cards.len(), n * 2);
        let ans = soln(&cards, trump);
        print_ans(ans);
    }
}

fn print_ans(ans: Option<Vec<(Card, Card)>>) {
    match ans {
        None => println!("IMPOSSIBLE"),
        Some(pairs) => {
            for (c1, c2) in pairs {
                println!("{c1} {c2}");
            }
        }
    }
}

impl FromStr for Card {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.is_ascii() {
            return Err("s must be ascii");
        }
        if s.len() != 2 {
            return Err("len must be 2");
        }

        let rank = s.chars().next().unwrap() as u8;
        let suit = s.chars().skip(1).next().unwrap();

        if !(b'2'..=b'9').contains(&rank) {
            return Err("rank must be in 2..=9");
        }

        let rank = rank as u8 - b'0';
        Ok(Self { rank, suit })
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}

#[derive(Clone, Copy)]
struct Card {
    suit: char,
    rank: u8,
}

/* idea
- for every non-trump suit, pair up its cards
    and output them in (less, greater) order
- (leaving up to one for each suit)

- if the number of trump cards is less than the
    number of unfinished suits: lose.
- pair up each unfinished card with a trump card

- pair up the remaining trumps with each other
*/

fn soln(cards: &[Card], trump: char) -> Option<Vec<(Card, Card)>> {
    assert_eq!(cards.len() % 2, 0);

    let mut trump_cards = Vec::<u8>::new();
    let mut normal_suits = HashMap::<char, Vec<u8>>::new();
    for c in cards {
        if c.suit == trump {
            trump_cards.push(c.rank);
        } else {
            normal_suits.entry(c.suit).or_default().push(c.rank);
        }
    }
    trump_cards.sort_unstable();
    for ranks in normal_suits.values_mut() {
        ranks.sort_unstable();
    }

    let mut unfinished_cards = Vec::<Card>::new();
    for (&suit, ranks) in &mut normal_suits {
        if ranks.len() % 2 == 1 {
            unfinished_cards.push(Card {
                suit,
                rank: ranks.pop().unwrap(),
            });
        }
    }
    if unfinished_cards.len() > trump_cards.len() {
        return None;
    }

    let mut out = vec![];
    for (suit, ranks) in normal_suits {
        assert_eq!(ranks.len() % 2, 0);
        for pair in ranks.chunks(2) {
            let c0 = Card {
                suit,
                rank: pair[0],
            };
            let c1 = Card {
                suit,
                rank: pair[1],
            };
            out.push((c0, c1));
        }
    }

    for c in unfinished_cards {
        let trump_card = Card {
            suit: trump,
            rank: trump_cards.pop().unwrap(),
        };
        out.push((c, trump_card));
    }

    assert_eq!(trump_cards.len() % 2, 0); // since we've paired off all the other cards.
    for pair in trump_cards.chunks(2) {
        let c0 = Card {
            suit: trump,
            rank: pair[0],
        };
        let c1 = Card {
            suit: trump,
            rank: pair[1],
        };
        out.push((c0, c1));
    }

    Some(out)
}
