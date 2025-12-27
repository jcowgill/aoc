use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::LazyLock;

/// Possible recorded actions
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
enum Action {
    /// Guard with given id begins shift
    BeginShift(u32),

    /// Current guard falls asleep
    FallAsleep,

    /// Current guard wakes up
    WakeUp,
}

impl FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Action, ()> {
        static GUARD_RE: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"^Guard #([0-9]*) begins shift$").unwrap());

        if s == "falls asleep" {
            Ok(Action::FallAsleep)
        } else if s == "wakes up" {
            Ok(Action::WakeUp)
        } else if let Some(caps) = GUARD_RE.captures(s) {
            Ok(Action::BeginShift(
                caps.get(1).unwrap().as_str().parse().map_err(|_| ())?,
            ))
        } else {
            Err(())
        }
    }
}

/// Record of actions with a timestamp
#[derive(Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
struct Record {
    date_hour: String,
    minute: u8,
    action: Action,
}

impl FromStr for Record {
    type Err = ();

    fn from_str(s: &str) -> Result<Record, ()> {
        static RE: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"^\s*\[([0-9-]* [0-9]*):([0-9]*)\]\s*(.*)$").unwrap());

        if let Some(caps) = RE.captures(s) {
            Ok(Record {
                date_hour: caps.get(1).unwrap().as_str().to_owned(),
                minute: caps.get(2).unwrap().as_str().parse().map_err(|_| ())?,
                action: caps.get(3).unwrap().as_str().parse().map_err(|_| ())?,
            })
        } else {
            Err(())
        }
    }
}

/// An iterator over wakeup events (with some extra metadata)
///  The parent iterator MUST return sorted items
struct WakeupIterator<'a, I> {
    parent: I,
    active_guard: Option<u32>,
    last_sleep: Option<&'a Record>,
}

impl<'a, I> WakeupIterator<'a, I> {
    pub fn new(parent: I) -> Self {
        WakeupIterator {
            parent,
            active_guard: None,
            last_sleep: None,
        }
    }
}

impl<'a, I> Iterator for WakeupIterator<'a, I>
where
    I: Iterator<Item = &'a Record>,
{
    /// (guard id, sleep minute, wakeup minute)
    type Item = (u32, u8, u8);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(record) = self.parent.next() {
            match record.action {
                Action::BeginShift(guard) => {
                    assert_eq!(self.last_sleep, None);
                    self.active_guard = Some(guard);
                    self.next()
                }
                Action::FallAsleep => {
                    assert_ne!(self.active_guard, None);
                    assert_eq!(self.last_sleep, None);
                    self.last_sleep = Some(record);
                    self.next()
                }
                Action::WakeUp => {
                    let last_sleep_rec = self.last_sleep.unwrap();

                    assert_eq!(record.date_hour, last_sleep_rec.date_hour);
                    assert!(record.minute > last_sleep_rec.minute);

                    self.last_sleep = None;
                    Some((
                        self.active_guard.unwrap(),
                        last_sleep_rec.minute,
                        record.minute,
                    ))
                }
            }
        } else {
            assert_eq!(self.last_sleep, None);
            None
        }
    }
}

/// Finds the guard id of the guard who slept the longest
fn longest_sleep<'a, I: Iterator<Item = &'a Record>>(records: I) -> u32 {
    let mut minutes_asleep = HashMap::new();

    for (guard, sleep, wakeup) in WakeupIterator::new(records) {
        *minutes_asleep.entry(guard).or_insert(0) += (wakeup - sleep) as u32;
    }

    *minutes_asleep.iter().max_by_key(|&(_, &v)| v).unwrap().0
}

/// Returns the frequency map which counts the frequency each guard is
/// asleep in each minute
fn minute_frequency_map<'a, I: Iterator<Item = &'a Record>>(
    records: I,
) -> HashMap<(u32, u8), usize> {
    let mut freq_map = HashMap::new();

    for (guard, sleep, wakeup) in WakeupIterator::new(records) {
        for minute in sleep..wakeup {
            *freq_map.entry((guard, minute)).or_insert(0) += 1;
        }
    }

    freq_map
}

/// Returns most frequent minute a guard sleeps in
fn most_frequent_minute<'a, I: Iterator<Item = &'a Record>>(records: I, freq_guard: u32) -> u8 {
    (minute_frequency_map(records)
        .iter()
        .filter(|&(&(guard, _), _)| guard == freq_guard)
        .max_by_key(|&(_, &v)| v)
        .unwrap()
        .0)
        .1
}

pub fn star1(input: &str) -> String {
    let mut records: Vec<Record> = input.lines().map(|line| line.parse().unwrap()).collect();
    records.sort();

    let guard = longest_sleep(records.iter());
    let minute = most_frequent_minute(records.iter(), guard);

    (guard * minute as u32).to_string()
}

pub fn star2(input: &str) -> String {
    let mut records: Vec<Record> = input.lines().map(|line| line.parse().unwrap()).collect();
    records.sort();

    let (guard, minute) = *minute_frequency_map(records.iter())
        .iter()
        .max_by_key(|&(_, &v)| v)
        .unwrap()
        .0;

    (guard * minute as u32).to_string()
}
