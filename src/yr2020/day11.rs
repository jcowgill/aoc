use nalgebra::DMatrix;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Seat {
    Floor,
    Empty,
    Occupied,
}

fn parse_input(input: &str) -> DMatrix<Seat> {
    DMatrix::from_row_iterator(
        input.lines().count(),
        input.lines().next().unwrap().len(),
        input.lines().flat_map(|l| {
            l.chars().map(|c| match c {
                '.' => Seat::Floor,
                'L' => Seat::Empty,
                '#' => Seat::Occupied,
                _ => panic!("invalid character {c}"),
            })
        }),
    )
}

fn run(
    input: &str,
    tolerance: usize,
    occupied_fn: impl Fn(&DMatrix<Seat>, usize, usize) -> usize,
) -> String {
    let mut a = parse_input(input);
    loop {
        let b = DMatrix::from_fn(a.nrows(), a.ncols(), |x, y| {
            let occupied = occupied_fn(&a, x, y);
            match a[(x, y)] {
                Seat::Occupied if occupied >= tolerance => Seat::Empty,
                Seat::Empty if occupied == 0 => Seat::Occupied,
                prev => prev,
            }
        });

        if a == b {
            break;
        }
        a = b;
    }
    a.into_iter()
        .filter(|&&s| s == Seat::Occupied)
        .count()
        .to_string()
}

pub fn star1(input: &str) -> String {
    run(input, 5, |m, x, y| {
        (x.max(1) - 1..=x + 1)
            .flat_map(|ox| (y.max(1) - 1..=y + 1).map(move |oy| m.get((ox, oy))))
            .filter(|&r| r == Some(&Seat::Occupied))
            .count()
    })
}

pub fn star2(input: &str) -> String {
    run(input, 5, |m, x, y| {
        (-1..=1)
            .flat_map(|ox| {
                (-1..=1).map(move |oy| {
                    if ox != 0 || oy != 0 {
                        for i in 1.. {
                            match m
                                .get(((x as i32 + ox * i) as usize, (y as i32 + oy * i) as usize))
                            {
                                Some(&Seat::Occupied) => return 1,
                                Some(&Seat::Empty) | None => break,
                                Some(&Seat::Floor) => (),
                            }
                        }
                    }
                    0
                })
            })
            .sum()
    })
}
