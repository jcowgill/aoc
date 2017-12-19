use std::str::FromStr;

/// A direction in the input grid
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West
}

impl Direction {
    /// Get direction clockwise of self
    fn clockwise(self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East  => Direction::South,
            Direction::South => Direction::West,
            Direction::West  => Direction::North
        }
    }

    /// Get direction anti-clockwise of self
    fn anticlockwise(self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::East  => Direction::North,
            Direction::South => Direction::East,
            Direction::West  => Direction::South
        }
    }
}

/// A point on the grid (x going east, y going south)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Point {
    x: isize,
    y: isize
}

impl Point {
    /// Move a point in the given direction
    fn move_direction(self, dir: Direction) -> Point {
        match dir {
            Direction::North => Point{ x: self.x,     y: self.y - 1 },
            Direction::East  => Point{ x: self.x + 1, y: self.y },
            Direction::South => Point{ x: self.x,     y: self.y + 1 },
            Direction::West  => Point{ x: self.x - 1, y: self.y }
        }
    }
}

/// Value of each cell in the grid
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CellValue {
    Blank,
    Vertical,
    Horizontal,
    Cross,
    Letter(u8)
}

/// An "infinite" grid used in this problem
#[derive(Clone, Debug)]
struct Grid {
    data: Vec<CellValue>,
    width: usize
}

impl Grid {
    /// Returns an empty grid
    fn new() -> Grid {
        Grid { data: Vec::new(), width: 0 }
    }

    /// Returns the value at a given point
    fn cell_value(&self, point: Point) -> CellValue {
        let pos = (point.y * self.width as isize + point.x) as usize;
        if point.x >= 0 && point.y >= 0 && point.x < self.width as isize && pos < self.data.len() {
            self.data[pos]
        } else {
            CellValue::Blank
        }
    }

    /// Resizes the grid filling new cells with Blank
    fn resize(&mut self, width: usize, height: usize) {
        if self.width == width {
            // If the width didn't change, we simply resize the grid data
            self.data.resize(width * height, CellValue::Blank);
        } else {
            // If the width has changed, we have to copy everything
            let mut new_data = Vec::with_capacity(width * height);
            for y in 0..height {
                for x in 0..width {
                    new_data.push(self.cell_value(Point { x: x as isize, y: y as isize }));
                }
            }

            self.data = new_data;
            self.width = width;
        }
    }
}

impl FromStr for Grid {
    type Err = char;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = Grid::new();
        for (y, line) in s.lines().enumerate() {
            // Add new line to grid
            let old_width = result.width;
            result.resize(old_width, y + 1);

            for (x, c) in line.chars().enumerate() {
                // Expand grid
                if x >= result.width { result.resize(x + 1, y + 1); }

                // Write character
                result.data[y * result.width + x] = match c {
                    ' '       => CellValue::Blank,
                    '|'       => CellValue::Vertical,
                    '-'       => CellValue::Horizontal,
                    '+'       => CellValue::Cross,
                    'A'...'Z' => CellValue::Letter(c as u8 - 'A' as u8),
                    _         => return Err(c)
                }
            }
        }

        Ok(result)
    }
}

/// Find the starting point
fn find_grid_start(grid: &Grid) -> Option<Point> {
    (0..grid.width)
        .map(|x| Point{ x: x as isize, y: 0 })
        .filter(|p| grid.cell_value(*p) == CellValue::Vertical)
        .next()
}

/// Traces the path in the grid given by the input string
///  Returns (letters, path length)`
fn trace_path(input: &str) -> (String, usize) {
    let grid: Grid = match input.parse() {
        Ok(grid) => grid,
        Err(c)   => panic!("invalid grid character: {}", c)
    };

    let mut pos = find_grid_start(&grid).unwrap();
    let mut dir = Direction::South;
    let mut letters = String::new();
    let mut steps = 0;

    loop {
        pos = pos.move_direction(dir);
        steps += 1;
        match grid.cell_value(pos) {
            CellValue::Blank => {
                // We went off the end of the path, do we're done now
                break;
            },
            CellValue::Vertical | CellValue::Horizontal => {
                // Keep going in same direction
            },
            CellValue::Cross => {
                // Test each possible direction
                if grid.cell_value(pos.move_direction(dir)) != CellValue::Blank {
                    // Don't change direction
                } else if grid.cell_value(pos.move_direction(dir.clockwise())) != CellValue::Blank {
                    dir = dir.clockwise();
                } else if grid.cell_value(pos.move_direction(dir.anticlockwise())) != CellValue::Blank {
                    dir = dir.anticlockwise()
                }
            },
            CellValue::Letter(l) => {
                // Append letter to letters list (and keep going)
                letters.push((l + 'A' as u8) as char)
            }
        }
    }

    (letters, steps)
}

/// Follow path and print letters
pub fn star1(input: &str) -> String {
    trace_path(input).0
}

/// Follow path and print path length
pub fn star2(input: &str) -> String {
    trace_path(input).1.to_string()
}
