use itertools::Itertools;

#[derive(Clone, Debug)]
struct BitReader<T> {
    /// The unread data buffer
    data: T,

    /// Cache of bits read from the data buffer, but not by the user
    cache: u64,

    /// Number of valid bits in the cache
    num_bits: u8,

    /// Total number of bytes which have been read into the cache
    cached_bytes: u32,
}

/// A single transmission packet and its children
#[derive(Clone, Debug, Eq, PartialEq)]
enum Packet {
    Literal {
        version: u8,
        value: u64,
    },
    Operator {
        version: u8,
        ty: u8,
        children: Vec<Packet>,
    },
}

impl<T: Iterator<Item = u8>> BitReader<T> {
    /// Constructs a new bit reader using the given iterator
    fn new(data: T) -> BitReader<T> {
        BitReader {
            data,
            cache: 0,
            num_bits: 0,
            cached_bytes: 0,
        }
    }

    /// Fills the cache bits with as much data as possible
    fn fill_cache(&mut self) {
        while self.num_bits <= 56 {
            if let Some(new_byte) = self.data.next() {
                self.cache = self.cache << 8 | new_byte as u64;
                self.num_bits += 8;
                self.cached_bytes += 1;
            } else {
                break;
            }
        }
    }

    /// Peeks some bits as an unsigned 32 bit integer without consuming them
    fn peek_u32(&mut self, bits: u8) -> Option<u32> {
        assert!(bits <= 32);

        if self.num_bits < bits {
            self.fill_cache();
            if self.num_bits < bits {
                panic!(
                    "buffer underflow reading {} bits (only have {})",
                    bits, self.num_bits
                );
            }
        }

        Some(((self.cache >> (self.num_bits - bits)) & ((1 << bits) - 1)) as u32)
    }

    /// Reads some bits as an unsigned 32 bit integer
    fn read_u32(&mut self, bits: u8) -> Option<u32> {
        if let Some(value) = self.peek_u32(bits) {
            self.num_bits -= bits;
            Some(value)
        } else {
            None
        }
    }

    /// Reads a single bit
    fn read_bit(&mut self) -> Option<bool> {
        self.read_u32(1).map(|v| v != 0)
    }

    /// Reads some bits as an unsigned 8 bit integer
    fn read_u8(&mut self, bits: u8) -> Option<u8> {
        assert!(bits <= 8);
        self.read_u32(bits).map(|v| v as u8)
    }

    /// Returns the number of bits read so far
    fn bits_read(&self) -> u32 {
        self.cached_bytes * 8 - self.num_bits as u32
    }
}

/// Constructs a new bit reader from a hexadecimal string
fn hex_reader(input: &str) -> BitReader<impl Iterator<Item = u8> + '_> {
    BitReader::new(
        input
            .trim()
            .chars()
            .map(|c| c.to_digit(16).unwrap() as u8)
            .tuples()
            .map(|(a, b)| (a << 4) + b),
    )
}

impl Packet {
    fn parse(reader: &mut BitReader<impl Iterator<Item = u8>>) -> Packet {
        let version = reader.read_u8(3).unwrap();
        let ty = reader.read_u8(3).unwrap();

        if ty == 4 {
            let mut value = 0;
            loop {
                let more_bits = reader.read_bit().unwrap();
                value = value << 4 | reader.read_u8(4).unwrap() as u64;
                if !more_bits {
                    break;
                }
            }

            Packet::Literal { version, value }
        } else {
            let mut children = Vec::new();

            if reader.read_bit().unwrap() {
                for _ in 0..reader.read_u32(11).unwrap() {
                    children.push(Packet::parse(reader));
                }
            } else {
                let end_bits = reader.read_u32(15).unwrap() + reader.bits_read();
                while reader.bits_read() < end_bits {
                    children.push(Packet::parse(reader));
                }
            }

            Packet::Operator {
                version,
                ty,
                children,
            }
        }
    }

    fn sum_versions(&self) -> u32 {
        match self {
            Packet::Literal { version, .. } => *version as u32,
            Packet::Operator {
                version, children, ..
            } => *version as u32 + children.iter().map(Packet::sum_versions).sum::<u32>(),
        }
    }

    fn evaluate(&self) -> u64 {
        match self {
            Packet::Literal { value, .. } => *value,
            Packet::Operator { ty, children, .. } => {
                let mut values = children.iter().map(Packet::evaluate);
                match ty {
                    0 => values.sum(),
                    1 => values.product(),
                    2 => values.min().unwrap_or(0),
                    3 => values.max().unwrap_or(0),
                    5 => values.tuples().all(|(a, b)| a > b) as u64,
                    6 => values.tuples().all(|(a, b)| a < b) as u64,
                    7 => values.all_equal() as u64,
                    _ => panic!("invalid packet"),
                }
            }
        }
    }
}

pub fn star1(input: &str) -> String {
    Packet::parse(&mut hex_reader(input))
        .sum_versions()
        .to_string()
}

pub fn star2(input: &str) -> String {
    Packet::parse(&mut hex_reader(input)).evaluate().to_string()
}
