/// Represents an individual program
///  Using a tuple struct so that values and positions don't mix
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Program(u8);

impl Program {
    fn from_char(prog: char) -> Option<Program> {
        if prog >= 'a' && prog <= 'z' {
            Some(Program(prog as u8 - 'a' as u8))
        } else {
            None
        }
    }

    fn to_char(self) -> Option<char> {
        if self.0 < 26 {
            Some(('a' as u8 + self.0) as char)
        } else {
            None
        }
    }
}

/// Enumeration of possible dance moves
#[derive(Copy, Clone, Debug)]
enum DanceMove {
    /// Right rotate list of numbers by given amount
    Spin(u8),

    /// Swap programs at given positions
    Exchange(u8, u8),

    /// Swap the programs given (at whatever position they are in)
    Partner(Program, Program)
}

/// Trait in common with both types of transformation
trait Transformation {
    /// Combines the given transformation with another of the same type
    fn combine(&self, other: &Self) -> Self;

    /// Performs the transformation on the given group of programs
    fn transform(&self, &[Program]) -> Vec<Program>;
}

/// A transposition transformation
///  This transformation rearranges the input based on the positions of its
///  elements. It never needs to know the value of each input element itself.
///
///  Transformation rule:
///   new[x] = old[trans[x]]
#[derive(Clone, Debug)]
struct Transposition(Vec<u8>);

impl Transposition {
    /// Returns the identity transposition with n elements
    fn identity(n: u8) -> Transposition {
        Transposition((0..n).collect())
    }

    /// Like transform, but works on any clonable type
    fn transform_any<T: Clone>(&self, input: &[T]) -> Vec<T> {
        assert_eq!(input.len(), self.0.len());
        (0..input.len()).map(|i| input[self.0[i] as usize].clone()).collect()
    }
}

impl Transformation for Transposition {
    fn combine(&self, other: &Transposition) -> Transposition {
        Transposition(self.transform_any(other.0.as_slice()))
    }

    fn transform(&self, input: &[Program]) -> Vec<Program> {
        self.transform_any(input)
    }
}

/// A substitution transformation
///  This transformation converts each input value into another value
///
///  Transformation rule:
///   new[x] = trans[old[x]]
#[derive(Clone, Debug)]
struct Substitution(Vec<Program>);

impl Substitution {
    /// Returns the identity substitution with n elements
    fn identity(n: u8) -> Substitution {
        Substitution((0..n).map(|v| Program(v)).collect())
    }
}

impl Transformation for Substitution {
    fn combine(&self, other: &Substitution) -> Substitution {
        Substitution(self.transform(other.0.as_slice()))
    }

    fn transform(&self, input: &[Program]) -> Vec<Program> {
        (0..input.len()).map(|i| self.0[input[i].0 as usize]).collect()
    }
}

/// Parses a single dance move
fn parse_move(input: &str) -> DanceMove {
    let (mtype, mdata_str) = input.split_at(1);
    let mdata: Vec<&str> = mdata_str.splitn(2, '/').collect();

    match mtype {
        "s" => DanceMove::Spin(mdata[0].parse().unwrap()),
        "x" => DanceMove::Exchange(mdata[0].parse().unwrap(), mdata[1].parse().unwrap()),
        "p" => DanceMove::Partner(
                Program::from_char(mdata[0].chars().next().unwrap()).unwrap(),
                Program::from_char(mdata[1].chars().next().unwrap()).unwrap()
               ),
        _   => panic!("invalid dance move type {}", mtype)
    }
}

/// Parses the input string returning (programs, moves list)
fn parse_input(input: &str) -> (u8, Vec<DanceMove>) {
    let (programs, moves_str) = match input.find('\n') {
        Some(pos) => {
            let (left, right) = input.split_at(pos);
            (left.parse().unwrap(), right.trim_left())
        },
        None => (16, input)
    };

    (programs, moves_str.split(',').map(parse_move).collect())
}

/// Derives the two types of transformation which represent a sequence of moves
fn derive_transformation(programs: u8, moves: &[DanceMove]) -> (Transposition, Substitution) {
    let mut transposition = Transposition::identity(programs);
    let mut substitution = Substitution::identity(programs);

    // Apply each move
    for dmove in moves.iter() {
        match *dmove {
            DanceMove::Spin(spin) => {
                // Rotate by reversing twice at different places
                //  From https://github.com/rust-lang/rust/pull/41670
                transposition.0.reverse();
                let (a, b) = transposition.0.split_at_mut(spin as usize);
                a.reverse();
                b.reverse();
            },
            DanceMove::Exchange(a, b) => {
                transposition.0.swap(a as usize, b as usize);
            },
            DanceMove::Partner(a, b) => {
                let a_pos = substitution.0.iter().position(|p| *p == a).unwrap();
                let b_pos = substitution.0.iter().position(|p| *p == b).unwrap();
                substitution.0.swap(a_pos, b_pos);
            }
        }
    }

    (transposition, substitution)
}

/// Repeats a transformation a given number of times
fn repeat_transformation<T: Transformation + Clone>(transformation: T, n: usize) -> T {
    assert!(n > 0);
    if n == 1 {
        // Base case
        transformation
    } else if n % 2 == 0 {
        // If n is even, combine with ourselves and repeat n / 2 times
        let double_transform = transformation.combine(&transformation);
        repeat_transformation(double_transform, n / 2)
    } else {
        // If n is odd, recurse to an even number and combine once
        let one_less_transform = repeat_transformation(transformation.clone(), n - 1);
        one_less_transform.combine(&transformation)
    }
}

/// Parses and executes a whole dance with the given number of repeats
fn whole_dance(input: &str, repeats: usize) -> String {
    let (programs, moves) = parse_input(input);
    let (transposition, substitution) = derive_transformation(programs, &moves);

    let repeat_trans = repeat_transformation(transposition, repeats);
    let repeat_subst = repeat_transformation(substitution, repeats);

    let result = repeat_subst.transform(&repeat_trans.transform(&Substitution::identity(programs).0));
    result.iter().map(|p| p.to_char().unwrap()).collect()
}


/// Executes a series of dance moves
pub fn star1(input: &str) -> String {
    whole_dance(input, 1)
}

/// Executes a series of dance moves 1 billion times
pub fn star2(input: &str) -> String {
    whole_dance(input, 1000000000)
}
