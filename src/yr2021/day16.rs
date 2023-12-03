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
    fn peek_u32(&mut self, bits: u8) -> u32 {
        assert!(bits <= 32);

        if self.num_bits < bits {
            self.fill_cache();
            assert!(
                self.num_bits >= bits,
                "buffer underflow reading {} bits (only have {})",
                bits,
                self.num_bits
            );
        }

        ((self.cache >> (self.num_bits - bits)) & ((1 << bits) - 1)) as u32
    }

    /// Reads some bits as an unsigned 32 bit integer
    fn read_u32(&mut self, bits: u8) -> u32 {
        let value = self.peek_u32(bits);
        self.num_bits -= bits;
        value
    }

    /// Reads a single bit
    fn read_bit(&mut self) -> bool {
        self.read_u32(1) != 0
    }

    /// Reads some bits as an unsigned 8 bit integer
    fn read_u8(&mut self, bits: u8) -> u8 {
        assert!(bits <= 8);
        self.read_u32(bits) as u8
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
        let version = reader.read_u8(3);
        let ty = reader.read_u8(3);

        if ty == 4 {
            let mut value = 0;
            loop {
                let more_bits = reader.read_bit();
                value = value << 4 | reader.read_u8(4) as u64;
                if !more_bits {
                    break;
                }
            }

            Packet::Literal { version, value }
        } else {
            let mut children = Vec::new();

            if reader.read_bit() {
                for _ in 0..reader.read_u32(11) {
                    children.push(Packet::parse(reader));
                }
            } else {
                let end_bits = reader.read_u32(15) + reader.bits_read();
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

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    star_test!(example1a, star1, "D2FE28", "6");
    star_test!(example2, star1, "38006F45291200", "9");
    star_test!(example3, star1, "EE00D40C823060", "14");
    star_test!(example4, star1, "8A004A801A8002F478", "16");
    star_test!(example5, star1, "620080001611562C8802118E34", "12");
    star_test!(example6, star1, "C0015000016115A2E0802F182340", "23");
    star_test!(example7, star1, "A0016C880162017C3686B18A3D4780", "31");
    star_test!(me1, star1, ME, "989");

    star_test!(example1b, star2, "D2FE28", "2021");
    star_test!(a0, star2, "C200B40A82", "3");
    star_test!(a1, star2, "04005AC33890", "54");
    star_test!(a2, star2, "880086C3E88112", "7");
    star_test!(a3, star2, "CE00C43D881120", "9");
    star_test!(a4, star2, "D8005AC2A8F0", "1");
    star_test!(a5, star2, "F600BC2D8F", "0");
    star_test!(a6, star2, "9C005AC2F8F0", "0");
    star_test!(a7, star2, "9C0141080250320F1802104A08", "1");
    star_test!(me2, star2, ME, "7936430475134");

    const ME: &str = indoc! {"
        220D790065B2745FF004672D99A34E5B33439D96CEC80373C0068663101A98C406A5E7395DC1804678BF25A4093BFBDB886CA6E11FDE6D93D16A100325E5597A118F6640600ACF7274E6A5829B00526C167F9C089F15973C4002AA4B22E800FDCFD72B9351359601300424B8C9A00BCBC8EE069802D2D0B945002AB2D7D583E3F00016B05E0E9802BA00B4F29CD4E961491CCB44C6008E80273C393C333F92020134B003530004221347F83A200D47F89913A66FB6620016E24A007853BE5E944297AB64E66D6669FCEA0112AE06009CAA57006A0200EC258FB0440010A8A716A321009DE200D44C8E31F00010887B146188803317A3FC5F30056C0150004321244E88C000874468A91D2291802B25EB875802B28D13550030056C0169FB5B7ECE2C6B2EF3296D6FD5F54858015B8D730BB24E32569049009BF801980803B05A3B41F1007625C1C821256D7C848025DE0040E5016717247E18001BAC37930E9FA6AE3B358B5D4A7A6EA200D4E463EA364EDE9F852FF1B9C8731869300BE684649F6446E584E61DE61CD4021998DB4C334E72B78BA49C126722B4E009C6295F879002093EF32A64C018ECDFAF605989D4BA7B396D9B0C200C9F0017C98C72FD2C8932B7EE0EA6ADB0F1006C8010E89B15A2A90021713610C202004263E46D82AC06498017C6E007901542C04F9A0128880449A8014403AA38014C030B08012C0269A8018E007A801620058003C64009810010722EC8010ECFFF9AAC32373F6583007A48CA587E55367227A40118C2AC004AE79FE77E28C007F4E42500D10096779D728EB1066B57F698C802139708B004A5C5E5C44C01698D490E800B584F09C8049593A6C66C017100721647E8E0200CC6985F11E634EA6008CB207002593785497652008065992443E7872714
    "};
}
