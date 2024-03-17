use csv::StringRecord;
use rand::Rng;
use std::collections::HashMap;
use std::io::stdin;
use std::u8;
enum HandType {
    Hard,
    Soft,
    Pair,
}
struct Hand {
    player_cards: (u8, u8),
    dealer_card: u8,
    hand_type: HandType,
}
impl Hand {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let player_cards = (rng.gen_range(2..=11), rng.gen_range(2..=11));
        let dealer_card = rng.gen_range(2..=11);
        let hand_type: HandType;
        if player_cards.0 == player_cards.1 {
            hand_type = HandType::Pair;
        } else if player_cards.0 == 11 || player_cards.1 == 11 {
            hand_type = HandType::Soft;
        } else {
            hand_type = HandType::Hard;
        }
        Hand {
            player_cards,
            dealer_card,
            hand_type,
        }
    }
    fn evaluate(&self, strategy_table: &StrategyTable) -> char {
        let hand_value = &self.player_cards.0 + &self.player_cards.1;
        let table = match &self.hand_type {
            HandType::Hard => &strategy_table.hard,
            HandType::Soft => &strategy_table.soft,
            HandType::Pair => &strategy_table.pair,
        };
        let correct_decision = table
            .get(&(self.dealer_card, hand_value))
            .expect("Lookup failed");
        correct_decision.clone()
    }
}

// <(dealer_card, hand_value), decision>
struct StrategyTable {
    hard: HashMap<(u8, u8), char>,
    soft: HashMap<(u8, u8), char>,
    pair: HashMap<(u8, u8), char>,
}

impl StrategyTable {
    fn new() -> Self {
        let hard = load_strategy_table(include_str!("../resources/hard.csv"));
        let soft = load_strategy_table(include_str!("../resources/soft.csv"));
        let pair = load_strategy_table(include_str!("../resources/pair.csv"));
        StrategyTable { hard, soft, pair }
    }
}

fn main() {
    let strategy_table = StrategyTable::new();
    let mut streak = 0;
    loop {
        let win = game_loop(&strategy_table);
        if win {
            streak += 1;
        } else {
            streak = 0;
        }
        println!("Streak: [{}]", streak);
    }
}

fn game_loop(strategy_table: &StrategyTable) -> bool {
    println!();
    let hand = Hand::new();
    let correct_decision = hand.evaluate(strategy_table);
    draw_dealer_hand(&hand.dealer_card);
    draw_player_hand(&hand.player_cards);
    let mut buffer = String::new();
    stdin()
        .read_line(&mut buffer)
        .expect("Failed to parse input");
    if let Some(decision) = buffer.trim().chars().next() {
        if decision.to_ascii_uppercase() == correct_decision {
            println!("Correct!");
            return true;
        }
        println!("Incorrect!");
        println!("Correct decision is: {}", correct_decision);
        return false;
    }
    false
}

fn load_strategy_table(data: &str) -> HashMap<(u8, u8), char> {
    let mut csv_reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(data.as_bytes());
    let mut map: HashMap<(u8, u8), char> = HashMap::new();

    for (hand_value, row) in csv_reader.records().enumerate() {
        let record: StringRecord = row.expect("Failed to parse row");
        for (dealer_card, field) in record.iter().enumerate() {
            let character = match field {
                "X" => 'X',
                "H" => 'H',
                "S" => 'S',
                "D" => 'D',
                "P" => 'P',
                "R" => 'R',
                _ => panic!("Invalid character found in CSV"),
            };
            map.insert((dealer_card as u8, hand_value as u8), character);
        }
    }

    map
}

fn draw_dealer_hand(card_number: &u8) {
    let suits = ["♠", "♣", "♥", "♢"];
    let tens = ['K', 'Q', 'J'];

    let suit = suits[rand::thread_rng().gen_range(0..suits.len())];

    let card_char: char = match *card_number {
        11 => 'A',
        10 => tens[rand::thread_rng().gen_range(0..tens.len())],
        _ => (*card_number + b'0') as char,
    };

    println!("┌─────┌────────┐ ");
    println!("│{}    │        │ ", card_char);
    println!("│     │        │ ");
    println!("│   {} │        │ ", suit);
    println!("│     │        │ ");
    println!("│     │        │ ");
    println!("└─────└────────┘ ");
}

fn draw_player_hand(cards: &(u8, u8)) {
    let suits = ["♠", "♣", "♥", "♢"];
    let tens = ['K', 'Q', 'J'];

    let suit1 = suits[rand::thread_rng().gen_range(0..suits.len())];
    let suit2 = suits[rand::thread_rng().gen_range(0..suits.len())];

    let card_char0: char = match cards.0 {
        11 => 'A',
        10 => tens[rand::thread_rng().gen_range(0..tens.len())],
        _ => (cards.0 + b'0') as char,
    };

    let card_char1: char = match cards.1 {
        11 => 'A',
        10 => tens[rand::thread_rng().gen_range(0..tens.len())],
        _ => (cards.1 + b'0') as char,
    };

    println!("┌─────┌────────┐ ");
    println!("│{}    │{}       │ ", card_char0, card_char1);
    println!("│     │        │ ");
    println!("│   {} │   {}    │ ", suit1, suit2);
    println!("│     │        │ ");
    println!("│     │      {} │ ", card_char1);
    println!("└─────└────────┘ ");
}
