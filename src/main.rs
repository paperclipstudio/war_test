use rand::{self, seq::SliceRandom};

type Card = u8;

#[derive(Clone, Debug)]
struct Deck {
    name: &'static str,
    cards: Vec<Card>,
    upgrade: bool,
    arrowed: bool,
    mirror: bool,
}
#[derive(PartialEq)]
enum Result {
    Win,
    Lose,
    Tie,
}

impl Deck {
    fn new(name: &'static str) -> Deck {
        Deck {
            name,
            cards: Vec::new(),
            upgrade: false,
            arrowed: false,
            mirror: false,
        }
    }

    fn size(&self) -> usize {
        self.cards.len()
    }

    fn add(mut self, value: Card) -> Self {
        self.cards.push(value);
        self
    }

    fn adds(mut self, mut values: Vec<Card>) -> Self {
        self.cards.append(&mut values);
        self
    }

    fn upgrade(mut self) -> Deck {
        self.upgrade = true;
        self
    }

    fn mirror(mut self) -> Deck {
        self.mirror = true;
        self
    }

    fn arrow(mut self) -> Deck {
        self.arrowed = true;
        self
    }

    fn shuffle<T: rand::Rng>(mut self, rng: &mut T) -> Self {
        self.cards.shuffle(rng);
        self
    }

    fn apply(&mut self) {
        if self.arrowed {
            self.discard();
            self.arrowed = false;
        }
        if self.upgrade {
            self.cards = self.cards.iter().map(|f| f + 1).collect();
        }
    }

    fn peek(&self) -> Option<&Card> {
        self.cards.last()
    }

    fn cycle(&mut self) {
        if let Some(top) = self.cards.pop() {
            self.cards.insert(0, top)
        }
    }

    fn discard(&mut self) {
        self.cards.pop();
    }

    fn war(&self, other: &Deck) -> Result {
        let mut left = self.clone();
        let mut right = other.clone();
        left.apply();
        right.apply();
        let mut ties_in_row = 0;
        loop {
            if ties_in_row > left.size().max(right.size()) * 2 {
                return Result::Tie;
            }
            // println!("{:?}, {:?}", left.cards, right.cards);
            if left.tied(&right) {
                return Result::Tie;
            }

            let result = match (left.peek(), right.peek()) {
                (None, None) => return Result::Tie,
                (None, Some(_)) => return Result::Lose,
                (Some(_), None) => return Result::Win,
                (Some(left_value), Some(right_value)) => {
                    //println!("left {left_value} right {right_value}");
                    if left_value > right_value {
                        Result::Win
                    } else if left_value < right_value {
                        Result::Lose
                    } else {
                        ties_in_row += 1;
                        Result::Tie
                    }
                }
            };
            let mut rand = rand::rng();
            left = left.shuffle(&mut rand);
            right = right.shuffle(&mut rand);
            match result {
                Result::Win => {
                    left.cycle();
                    right.discard();
                }
                Result::Lose => {
                    left.discard();
                    right.cycle();
                }
                Result::Tie => match (left.mirror, right.mirror) {
                    (true, true) => panic!(),
                    (true, false) => {
                        let card = right.peek().unwrap().clone();
                        //println!("card{card}");
                        right.discard();
                        left = left.add(card);
                        left.cycle();
                        left.cycle();
                    }
                    (false, true) => {
                        let card = left.peek().unwrap().clone();
                        left.discard();
                        right = right.add(card);
                        right.cycle();
                        right.cycle();
                    }
                    (false, false) => {
                        left.cycle();
                        right.cycle();
                    }
                },
            }
        }
    }

    fn battle(&self, other: &Deck) -> (usize, usize) {
        let mut win = 0;
        let mut tie = 0;
        const TOTAL: usize = 1000;
        let mut rand = rand::rng();
        for _ in 0..TOTAL {
            let mut left = self.clone();
            let mut right = other.clone();
            left = left.shuffle(&mut rand);
            right = right.shuffle(&mut rand);
            match left.war(&right) {
                Result::Win => win += 1,
                Result::Lose => {}
                Result::Tie => tie += 1,
            }
        }
        (100 * win / TOTAL, 100 * tie / TOTAL)
    }

    fn tied(&self, other: &Deck) -> bool {
        if self.cards == other.cards {
            return true;
        }

        if self.cards.is_empty() && other.cards.is_empty() {
            return true;
        }

        if self.cards.is_empty() {
            return false;
        }

        if other.cards.is_empty() {
            return false;
        }

        let first_card = self.cards.first().unwrap();

        self.cards.iter().all(|x| first_card == x) && other.cards.iter().all(|x| first_card == x)
    }
}

fn main() {
    let start = vec![1, 2, 3u8];
    let sword = 5;
    let magic_wand = 12;
    let joker = 100;
    let player_states = [
        Deck::new("starting").adds(start.clone()),
        Deck::new("+sword").adds(start.clone()).add(sword),
        Deck::new("+upgrade").adds(start.clone()).upgrade(),
        Deck::new("+mirror").adds(start.clone()).mirror(),
        Deck::new("+sword+upgrade")
            .adds(start.clone())
            .add(sword)
            .upgrade(),
        Deck::new("complete")
            .adds(start.clone())
            .add(sword)
            .add(magic_wand)
            .add(joker)
            .upgrade(),
    ];

    let enemys = [
        Deck::new("Sleepy Giant").adds(vec![1, 1, 1, 1, 2, 2, 3, 4]),
        Deck::new("?????").adds(vec![1, 1, 1, 2, 2, 4]),
        Deck::new("Queen Bee").adds(vec![1, 3, 3, 4, 4, 6]),
        Deck::new("Jemery t Jellyfish").adds(vec![1, 2, 1, 2, 1, 4]),
        // Elf has joker, and on kill he drops the joker for you.
        Deck::new("Elf").adds(vec![100]),
        Deck::new("Elf-Arrow").adds(vec![100]).arrow(),
        Deck::new("Lizard King").adds(vec![5, 6, 7, 8, 9, 13]),
        Deck::new("Lizard King - Arrow").adds(vec![5, 6, 7]).arrow(),
    ];
    player_states[3].battle(&enemys[0]);

    println!("┏━━━━━━━━━━━━━━━━━━━━━━┯━━━━━━━━━━━━━━━━━━━━━━┯━━━━━━━┯━━━━━━━┯━━━━━━━┓");
    println!("┃ Player               │ Enemies              │  Win  │  Tie  │ Lose  ┃");
    println!("┠──────────────────────┼──────────────────────┼───────┼───────┼───────┃");

    player_states.iter().enumerate().for_each(|(i, hand)| {
        enemys.iter().for_each(|other| {
            let (win, tie) = hand.battle(other);
            println!(
                "┃ {:20} │ {:20} │  {win:3}% │  {tie:3}% │  {:3}% ┃",
                hand.name,
                other.name,
                100 - win - tie
            );
        });
        if i != player_states.len() - 1 {
            println!("┠──────────────────────┼──────────────────────┼───────┼───────┼───────┃");
        } else {
            println!("┗━━━━━━━━━━━━━━━━━━━━━━┷━━━━━━━━━━━━━━━━━━━━━━┷━━━━━━━┷━━━━━━━┷━━━━━━━┛");
        }
    });
}
