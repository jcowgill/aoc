#[derive(Clone, Debug, Default)]
struct BoardState {
    numbers: Vec<Option<u8>>,
    rows: [u8; 5],
    cols: [u8; 5],
}

impl BoardState {
    fn from_str(s: &str) -> BoardState {
        BoardState {
            numbers: s
                .split_ascii_whitespace()
                .map(|n| Some(n.parse().unwrap()))
                .collect(),
            ..Default::default()
        }
    }

    fn mark(&mut self, num: u8) -> Option<u32> {
        if let Some(index) = self.numbers.iter().position(|&n| n == Some(num)) {
            let row = index / 5;
            let col = index % 5;

            self.numbers[index] = None;
            self.rows[row] += 1;
            self.cols[col] += 1;

            if self.rows[row] == 5 || self.cols[col] == 5 {
                // Bingo!
                let unmarked_sum: u32 = self.numbers.iter().flatten().map(|&n| u32::from(n)).sum();

                // Clear numbers so this board will never complete again
                self.numbers.clear();

                return Some(unmarked_sum * u32::from(num));
            }
        }

        None
    }
}

fn parse_input(input: &str) -> (impl Iterator<Item = u8> + '_, Vec<BoardState>) {
    let mut paragraphs = input.split("\n\n");
    let numbers = paragraphs
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap());
    let boards: Vec<BoardState> = paragraphs.map(BoardState::from_str).collect();

    (numbers, boards)
}

pub fn star1(input: &str) -> String {
    let (numbers, mut boards) = parse_input(input);

    for num in numbers {
        for board in boards.iter_mut() {
            if let Some(score) = board.mark(num) {
                return score.to_string();
            }
        }
    }

    panic!("no winner after all numbers exhausted")
}

pub fn star2(input: &str) -> String {
    let (numbers, mut boards) = parse_input(input);
    let mut boards_left = boards.len();

    for num in numbers {
        for board in boards.iter_mut() {
            if let Some(score) = board.mark(num) {
                boards_left -= 1;
                if boards_left == 0 {
                    return score.to_string();
                }
            }
        }
    }

    panic!("no winner after all numbers exhausted")
}
