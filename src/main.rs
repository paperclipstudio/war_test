use rand::{self, seq::SliceRandom};

type Card = u8;

#[derive(Clone, Debug)]
struct Deck {
    cards: Vec<Card>,
    upgrade: bool,
    arrowed: bool,
}
#[derive(PartialEq)]
enum Result {
    Win,
    Lose,
    Tie,
}

impl Deck {
    fn new() -> Deck {
        Deck {
            cards: Vec::new(),
            upgrade: false,
            arrowed: false,
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
        self.cards.first()
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
            //println!("{:?}, {:?}", left.cards, right.cards);
            if left.tied(&right) {
                return Result::Tie;
            }

            let result = match (left.peek(), right.peek()) {
                (None, None) => return Result::Tie,
                (None, Some(_)) => return Result::Lose,
                (Some(_), None) => return Result::Win,
                (Some(left_value), Some(right_value)) => {
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
            match result {
                Result::Win => {
                    left.cycle();
                    right.discard();
                }
                Result::Lose => {
                    left.discard();
                    right.cycle();
                }
                Result::Tie => {
                    left.cycle();
                    right.cycle();
                }
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
    let starting = Deck::new().adds(vec![1, 2, 3]);
    let updated = starting.clone().upgrade();

    let complete = Deck::new().adds(vec![1, 2, 3, 5, 12, 100]).upgrade();
    let boss = Deck::new().adds(vec![5, 6, 7]);
    let boss_poison = boss.clone().arrow();
    let slime = Deck::new().adds(vec![1, 1, 2, 3]);

    let (win, tie) = starting.battle(&slime);
    println!("Starting vs Slime       {win}% {tie}% {}%", 100 - win - tie);
    let (win, tie) = updated.battle(&slime);
    println!("Upgrade vs Slime       {win}% {tie}% {}%", 100 - win - tie);

    let (win, tie) = starting.battle(&boss);
    println!("Starting vs Boss        {win}% {tie}% {}%", 100 - win - tie);
    let (win, tie) = complete.battle(&boss);
    println!("final vs Boss           {win}% {tie}% {}%", 100 - win - tie);
    let (win, tie) = complete.battle(&boss_poison);
    println!("final vs Boss Posion    {win}% {tie}% {}%", 100 - win - tie);
}
