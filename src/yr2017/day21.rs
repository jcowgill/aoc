use std::collections::HashMap;
use std::ops::{Index, IndexMut};
use std::str::FromStr;

use crate::cartesian_product;

/// Monochrome image
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Image {
    /// Image width
    width: usize,

    /// Pixel data
    data: Vec<bool>
}

impl Image {
    /// Constructs an image from a raw vector in row major order
    ///  Size of vector must be a multiple of width
    fn from_raw_data(width: usize, data: Vec<bool>) -> Image {
        assert!(data.len() % width == 0);
        Image { width: width, data: data }
    }

    /// Constructs an image from a coordinate generator
    ///  coord_map maps (x, y) points to values in the image
    fn from_generator<F>(width: usize, height: usize, coord_map: F) -> Image
        where F: Fn(usize, usize) -> bool {

        Self::from_raw_data(width,
                            cartesian_product(0..height, 0..width)
                            .map(|(y, x)| coord_map(x, y))
                            .collect())
    }

    /// Returns the height of the image
    fn height(&self) -> usize {
        return self.data.len() / self.width;
    }

    /// Transforms this image into a new image based on the given function
    ///  width = new image width
    ///  height = new image height
    ///  coord_transform = function is given NEW coordinates and returns OLD
    ///                    coordinates to get pixel data from
    fn transform<F>(&self, width: usize, height: usize, coord_transform: F) -> Image
        where F: Fn(usize, usize) -> (usize, usize) {

        Self::from_generator(width, height, |x, y| self[coord_transform(x, y)])
    }

    /// Like transform, but keeps the same size
    ///  coord_transform is provided (new x, new y, width, height)
    fn transform_noscale<F>(&self, coord_transform: F) -> Image
        where F: Fn(usize, usize, usize, usize) -> (usize, usize) {

        let height = self.height();
        self.transform(self.width, height, |x, y| coord_transform(x, y, self.width, height))
    }

    /// Returns the given extract of an image
    fn subimage(&self, x: usize, y: usize, width: usize, height: usize) -> Image {
        self.transform(width, height, |newx, newy| (x + newx, y + newy))
    }
}

// Image indexing by x,y coordinates
impl Index<(usize, usize)> for Image {
    type Output = bool;
    fn index(&self, index: (usize, usize)) -> &bool {
        assert!(index.0 < self.width);
        &self.data[index.1 * self.width + index.0]
    }
}

impl IndexMut<(usize, usize)> for Image {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut bool {
        assert!(index.0 < self.width);
        &mut self.data[index.1 * self.width + index.0]
    }
}

// Parses an image consisting of ".#/" characters
//  All whitespace is ignored, all image rows must be same width
impl FromStr for Image {
    type Err = ();
    fn from_str(s: &str) -> Result<Image, ()> {
        /// Pushes a row to the data vector, returning the size pushed
        fn push_image_row(data: &mut Vec<bool>, row: &str) -> Result<usize, ()> {
            let prev_len = data.len();
            data.extend(row.chars()
                .filter(|c| !c.is_whitespace())
                .map(|c| match c { '#' => Ok(true), '.' => Ok(false), _ => Err(()) })
                .collect::<Result<Vec<bool>, ()>>()?);

            Ok(data.len() - prev_len)
        }

        let mut data = Vec::new();
        let mut rows = s.split('/');

        // Extract width from first row
        let width = match rows.next() {
            Some(row) => push_image_row(&mut data, row)?,
            None      => 0
        };

        // Push all other rows
        for row in rows {
            let pushed = push_image_row(&mut data, row)?;
            assert_eq!(width, pushed);
        };

        Ok(Image::from_raw_data(width, data))
    }
}

/// Parses the input list of rules
fn parse_rules<'a, I: Iterator<Item=&'a str>>(lines: I) -> HashMap<Image, Image> {
    let mut rules = HashMap::new();
    for line in lines {
        // Parse line
        let line_parts: Vec<Image> = line.split("=>").map(|s| s.parse().unwrap()).collect();
        assert_eq!(line_parts.len(), 2);

        // Insert all linear transformations of left side into the ruleset
        //  This is the dihedral group of order 8 (a = rotate 90, b = flip horiz)
        let lhs_list = vec![
            line_parts[0].clone(),                                                 // e Identity
            line_parts[0].transform_noscale(|x, y, _, h| (y,         h - x - 1)),  // a Rotate 90
            line_parts[0].transform_noscale(|x, y, w, h| (w - x - 1, h - y - 1)),  // aa Rotate 180
            line_parts[0].transform_noscale(|x, y, w, _| (w - y - 1, x        )),  // aaa Rotate 270
            line_parts[0].transform_noscale(|x, y, w, _| (w - x - 1, y        )),  // b Flip horizontal
            line_parts[0].transform_noscale(|x, y, w, h| (w - y - 1, h - x - 1)),  // ab
            line_parts[0].transform_noscale(|x, y, _, h| (x,         h - y - 1)),  // aab Flip vertical
            line_parts[0].transform_noscale(|x, y, _, _| (y,         x        )),  // aaab
        ];

        for lhs in lhs_list {
            rules.insert(lhs, line_parts[1].clone());
        }
    };

    rules
}

/// Performs one fractal iteration over the image
///  image = starting image
///  rules = ALL the rules (inc rotations etc)
fn fractal_iterate(image: Image, rules: &HashMap<Image, Image>) -> Image {
    assert!(image.width == image.height());
    assert!(image.width % 2 == 0 || image.width % 3 == 0);

    // Get list of blocks which we're going to expand into
    let block_size = if image.width % 2 == 0 { 2 } else { 3 };
    let block_count = image.width / block_size;
    let matched_blocks: Vec<&Image> =
        cartesian_product(0..block_count, 0..block_count)
        .map(|(by, bx)| rules.get(
                &image.subimage(bx * block_size, by * block_size, block_size, block_size))
                .unwrap())
        .collect();

    // Splice blocks together
    let new_block_size = block_size + 1;
    Image::from_generator(
        block_count * new_block_size,
        block_count * new_block_size,
        |x, y| matched_blocks[(x / new_block_size) + (y / new_block_size) * block_count]
                             [(x % new_block_size, y % new_block_size)])
}

/// Find number of enabled bits after n iterations
fn star_common(input: &str, default: usize) -> String {
    // Read number of iterations from the first line
    let mut lines = input.lines().peekable();
    let iterations = match lines.peek().unwrap().parse::<usize>() {
        Ok(v)  => { lines.next(); v },
        Err(_) => default
    };

    let rules = parse_rules(lines);
    let initial_image: Image = ".#./..#/###".parse().unwrap();

    // Perform fractal iteration 5 times
    let result = (0..iterations).fold(initial_image, |prev, _| fractal_iterate(prev, &rules));

    // Return number of set bits in the final image
    result.data.iter().filter(|&p| *p).count().to_string()
}

pub fn star1(input: &str) -> String { star_common(input, 5) }
pub fn star2(input: &str) -> String { star_common(input, 18) }
