use std::ops::RangeInclusive;

type Field<'a> = (&'a str, RangeInclusive<u32>, RangeInclusive<u32>);
type Fields<'a> = Vec<Field<'a>>;
type Ticket = Vec<u32>;

fn parse_range(s: &str) -> RangeInclusive<u32> {
    let (a, b) = s.split_once('-').unwrap();
    a.trim().parse().unwrap()..=b.trim().parse().unwrap()
}

fn parse_ticket(s: &str) -> Ticket {
    s.split(',').map(|v| v.parse().unwrap()).collect()
}

fn parse(input: &str) -> (Fields, Ticket, Vec<Ticket>) {
    let paras: Vec<_> = input.split("\n\n").collect();
    (
        paras[0]
            .lines()
            .map(|l| {
                let (name, tail) = l.split_once(':').unwrap();
                let (a, b) = tail.split_once("or").unwrap();
                (name, parse_range(a), parse_range(b))
            })
            .collect(),
        parse_ticket(paras[1].lines().nth(1).unwrap()),
        paras[2].lines().skip(1).map(parse_ticket).collect(),
    )
}

fn check_value_field(value: u32, (_, a, b): &Field) -> bool {
    a.contains(&value) || b.contains(&value)
}

fn is_value_impossible(value: u32, fields: &Fields) -> bool {
    fields.iter().all(|f| !check_value_field(value, f))
}

pub fn star1(input: &str) -> String {
    let (fields, _, nearby) = parse(input);
    nearby
        .iter()
        .flat_map(|t| t.iter().filter(|&&v| is_value_impossible(v, &fields)))
        .sum::<u32>()
        .to_string()
}

fn is_field_legal_in_slot(field: &Field, nearby: &[Ticket], slot: usize) -> bool {
    nearby.iter().all(|t| check_value_field(t[slot], field))
}

pub fn star2(input: &str) -> String {
    let (fields, my_ticket, mut nearby) = parse(input);
    nearby.retain(|t| !t.iter().any(|&v| is_value_impossible(v, &fields)));

    // Determine which fields are legal in each slot
    let mut fields_in_slot: Vec<Vec<_>> = (0..fields.len())
        .map(|idx| {
            fields
                .iter()
                .enumerate()
                .filter(|&(_, f)| is_field_legal_in_slot(f, &nearby, idx))
                .map(|(i, _)| i)
                .collect()
        })
        .collect();

    let mut total = 1;
    while let Some(slot) = fields_in_slot.iter().position(|s| s.len() == 1) {
        // Assign the field in this slot
        let field_idx = fields_in_slot[slot][0];
        for fields in fields_in_slot.iter_mut() {
            fields.retain(|&f| f != field_idx);
        }

        if fields[field_idx].0.starts_with("departure") {
            total *= u64::from(my_ticket[slot]);
        }
    }

    // All fields must be assigned
    for fields in fields_in_slot {
        assert!(fields.is_empty());
    }

    total.to_string()
}
