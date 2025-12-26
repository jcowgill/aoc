use itertools::Itertools;

#[derive(Clone, Debug)]
struct Grid {
    data: Vec<u8>,
    width: usize,
}

impl Grid {
    fn parse(input: &str) -> Grid {
        Grid {
            data: input.bytes().filter(|&c| c == b'.' || c == b'@').collect(),
            width: input.lines().next().unwrap().len(),
        }
    }

    fn height(&self) -> usize {
        self.data.len() / self.width
    }

    fn get(&self, x: usize, y: usize) -> Option<u8> {
        self.data
            .get(y * self.width + x)
            .copied()
            .filter(|_| x < self.width)
    }

    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut u8> {
        self.data
            .get_mut(y * self.width + x)
            .filter(|_| x < self.width)
    }

    fn is_paper_p1(&self, xp1: usize, yp1: usize) -> bool {
        xp1 > 0 && yp1 > 0 && self.get(xp1 - 1, yp1 - 1) == Some(b'@')
    }

    fn surrounding(&self, x: usize, y: usize) -> u32 {
        (0..3)
            .cartesian_product(0..3)
            .map(|(ox, oy)| u32::from((ox, oy) != (1, 1) && self.is_paper_p1(x + ox, y + oy)))
            .sum()
    }

    fn removable(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..self.width)
            .cartesian_product(0..self.height())
            .filter(|&(x, y)| self.get(x, y) == Some(b'@') && self.surrounding(x, y) < 4)
    }
}

pub fn star1(input: &str) -> String {
    Grid::parse(input).removable().count().to_string()
}

pub fn star2(input: &str) -> String {
    let mut grid1 = Grid::parse(input);
    let mut grid2 = grid1.clone();
    let mut total = 0;
    let mut prev_total = -1;

    while total > prev_total {
        prev_total = total;

        for (x, y) in grid1.removable() {
            *grid2.get_mut(x, y).unwrap() = b'.';
            total += 1;
        }

        grid1.data.copy_from_slice(&grid2.data);
    }

    total.to_string()
}
