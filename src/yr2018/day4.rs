use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

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
        lazy_static! {
            static ref GUARD_RE: Regex = Regex::new(r"^Guard #([0-9]*) begins shift$").unwrap();
        }

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
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^\s*\[([0-9-]* [0-9]*):([0-9]*)\]\s*(.*)$").unwrap();
        }

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

    *minutes_asleep.iter().max_by_key(|(_, &v)| v).unwrap().0
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
        .max_by_key(|(_, &v)| v)
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
        .max_by_key(|(_, &v)| v)
        .unwrap()
        .0;

    (guard * minute as u32).to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    star_test!(example1a, star1, IN1, "240");
    star_test!(me1, star1, ME, "8950");

    star_test!(example1b, star2, IN1, "4455");
    star_test!(me2, star2, ME, "78452");

    const IN1: &str = indoc! {"
        [1518-11-01 00:00] Guard #10 begins shift
        [1518-11-01 00:05] falls asleep
        [1518-11-01 00:25] wakes up
        [1518-11-01 00:30] falls asleep
        [1518-11-01 00:55] wakes up
        [1518-11-01 23:58] Guard #99 begins shift
        [1518-11-02 00:40] falls asleep
        [1518-11-02 00:50] wakes up
        [1518-11-03 00:05] Guard #10 begins shift
        [1518-11-03 00:24] falls asleep
        [1518-11-03 00:29] wakes up
        [1518-11-04 00:02] Guard #99 begins shift
        [1518-11-04 00:36] falls asleep
        [1518-11-04 00:46] wakes up
        [1518-11-05 00:03] Guard #99 begins shift
        [1518-11-05 00:45] falls asleep
        [1518-11-05 00:55] wakes up
    "};

    const ME: &str = indoc! {"
        [1518-06-02 23:58] Guard #179 begins shift
        [1518-09-18 00:43] wakes up
        [1518-06-06 00:10] falls asleep
        [1518-05-12 00:52] wakes up
        [1518-07-02 00:39] wakes up
        [1518-07-21 23:50] Guard #2917 begins shift
        [1518-09-22 00:47] wakes up
        [1518-09-09 00:31] wakes up
        [1518-07-21 00:15] wakes up
        [1518-07-30 00:43] wakes up
        [1518-10-08 00:52] wakes up
        [1518-07-26 23:57] Guard #1051 begins shift
        [1518-07-17 00:00] Guard #2273 begins shift
        [1518-08-18 00:24] falls asleep
        [1518-09-08 00:02] falls asleep
        [1518-08-19 00:00] Guard #2917 begins shift
        [1518-08-21 00:01] Guard #251 begins shift
        [1518-08-08 00:39] falls asleep
        [1518-09-08 23:57] Guard #2273 begins shift
        [1518-08-17 23:46] Guard #2971 begins shift
        [1518-06-30 00:04] falls asleep
        [1518-07-10 00:00] Guard #1783 begins shift
        [1518-10-30 00:03] Guard #2657 begins shift
        [1518-10-20 00:45] wakes up
        [1518-11-06 00:13] wakes up
        [1518-04-12 00:02] Guard #2273 begins shift
        [1518-08-13 00:57] wakes up
        [1518-07-03 00:48] falls asleep
        [1518-11-07 00:33] wakes up
        [1518-08-30 00:30] wakes up
        [1518-05-15 23:56] Guard #1811 begins shift
        [1518-07-09 00:26] wakes up
        [1518-07-05 23:58] Guard #1051 begins shift
        [1518-06-10 00:47] falls asleep
        [1518-06-29 00:40] falls asleep
        [1518-07-25 00:52] wakes up
        [1518-08-01 00:27] falls asleep
        [1518-06-15 00:53] wakes up
        [1518-05-24 00:15] wakes up
        [1518-06-15 00:50] falls asleep
        [1518-07-15 00:25] wakes up
        [1518-08-22 00:20] falls asleep
        [1518-04-26 00:18] falls asleep
        [1518-10-10 00:43] falls asleep
        [1518-07-20 23:51] Guard #2467 begins shift
        [1518-06-17 00:02] Guard #3467 begins shift
        [1518-09-06 00:01] Guard #3319 begins shift
        [1518-10-21 00:02] Guard #751 begins shift
        [1518-07-11 00:09] falls asleep
        [1518-06-02 00:11] falls asleep
        [1518-05-23 00:57] wakes up
        [1518-06-26 00:22] falls asleep
        [1518-11-21 00:07] falls asleep
        [1518-09-11 00:07] falls asleep
        [1518-07-17 00:35] wakes up
        [1518-06-27 00:45] wakes up
        [1518-08-06 00:21] falls asleep
        [1518-08-11 00:51] falls asleep
        [1518-11-02 00:19] falls asleep
        [1518-07-01 00:51] wakes up
        [1518-10-05 00:45] wakes up
        [1518-05-17 00:21] falls asleep
        [1518-04-18 23:56] Guard #643 begins shift
        [1518-11-22 00:44] wakes up
        [1518-07-23 00:50] wakes up
        [1518-11-17 23:56] Guard #1783 begins shift
        [1518-07-22 00:46] wakes up
        [1518-06-29 00:25] falls asleep
        [1518-11-13 00:00] Guard #179 begins shift
        [1518-11-10 23:51] Guard #499 begins shift
        [1518-10-05 00:00] Guard #751 begins shift
        [1518-11-11 00:53] wakes up
        [1518-09-17 00:26] wakes up
        [1518-06-21 00:49] wakes up
        [1518-09-12 00:06] falls asleep
        [1518-04-18 00:25] falls asleep
        [1518-09-28 00:59] wakes up
        [1518-04-13 00:09] falls asleep
        [1518-06-01 00:36] wakes up
        [1518-07-23 23:56] Guard #643 begins shift
        [1518-09-09 00:54] wakes up
        [1518-11-06 00:19] falls asleep
        [1518-06-27 00:21] falls asleep
        [1518-11-17 00:46] wakes up
        [1518-07-07 00:47] falls asleep
        [1518-11-01 00:02] Guard #457 begins shift
        [1518-08-22 00:37] wakes up
        [1518-08-29 00:56] wakes up
        [1518-10-12 00:04] Guard #3167 begins shift
        [1518-04-13 00:41] falls asleep
        [1518-07-15 00:32] falls asleep
        [1518-06-11 00:47] wakes up
        [1518-05-23 00:41] falls asleep
        [1518-09-16 23:46] Guard #2657 begins shift
        [1518-08-29 23:50] Guard #251 begins shift
        [1518-10-06 23:51] Guard #2657 begins shift
        [1518-11-18 00:56] falls asleep
        [1518-05-24 00:43] wakes up
        [1518-11-18 00:59] wakes up
        [1518-04-18 00:13] wakes up
        [1518-06-19 00:01] Guard #499 begins shift
        [1518-05-31 00:40] wakes up
        [1518-10-15 00:56] falls asleep
        [1518-08-03 00:30] wakes up
        [1518-04-12 00:55] falls asleep
        [1518-10-23 00:00] Guard #2971 begins shift
        [1518-06-26 00:52] wakes up
        [1518-07-15 00:00] Guard #1051 begins shift
        [1518-07-08 00:26] falls asleep
        [1518-08-25 00:46] wakes up
        [1518-06-09 00:45] wakes up
        [1518-04-25 00:46] wakes up
        [1518-10-02 00:28] falls asleep
        [1518-04-23 23:57] Guard #1051 begins shift
        [1518-06-18 00:03] Guard #1783 begins shift
        [1518-05-04 00:04] Guard #2657 begins shift
        [1518-09-25 00:15] falls asleep
        [1518-06-01 00:00] Guard #643 begins shift
        [1518-05-06 00:54] falls asleep
        [1518-09-16 00:01] Guard #3319 begins shift
        [1518-11-16 00:00] Guard #3467 begins shift
        [1518-10-22 00:31] wakes up
        [1518-09-07 00:00] Guard #2273 begins shift
        [1518-06-22 23:47] Guard #643 begins shift
        [1518-09-11 00:45] wakes up
        [1518-05-06 00:01] Guard #751 begins shift
        [1518-10-15 00:21] falls asleep
        [1518-04-10 00:59] wakes up
        [1518-09-04 00:30] falls asleep
        [1518-08-20 00:11] falls asleep
        [1518-11-12 00:25] falls asleep
        [1518-10-25 23:57] Guard #911 begins shift
        [1518-10-01 00:46] falls asleep
        [1518-07-08 00:52] falls asleep
        [1518-09-01 00:27] falls asleep
        [1518-07-19 00:30] wakes up
        [1518-09-25 00:57] falls asleep
        [1518-05-21 00:41] falls asleep
        [1518-10-26 00:39] wakes up
        [1518-06-24 23:50] Guard #3319 begins shift
        [1518-08-18 00:51] wakes up
        [1518-08-25 23:56] Guard #1783 begins shift
        [1518-09-29 00:00] Guard #179 begins shift
        [1518-09-10 00:48] wakes up
        [1518-06-14 00:10] falls asleep
        [1518-05-20 00:38] falls asleep
        [1518-06-02 00:56] wakes up
        [1518-10-06 00:43] falls asleep
        [1518-05-16 00:48] wakes up
        [1518-04-15 00:00] Guard #179 begins shift
        [1518-09-24 23:59] Guard #2467 begins shift
        [1518-09-21 00:49] wakes up
        [1518-09-21 00:19] falls asleep
        [1518-07-17 00:51] wakes up
        [1518-04-27 00:41] wakes up
        [1518-05-09 00:39] falls asleep
        [1518-08-12 00:49] falls asleep
        [1518-09-15 00:37] falls asleep
        [1518-05-03 00:16] falls asleep
        [1518-08-16 23:49] Guard #2477 begins shift
        [1518-11-10 00:45] wakes up
        [1518-07-26 00:32] falls asleep
        [1518-08-06 00:01] Guard #499 begins shift
        [1518-07-30 00:26] falls asleep
        [1518-09-14 00:01] Guard #643 begins shift
        [1518-09-29 00:59] wakes up
        [1518-04-18 00:48] wakes up
        [1518-09-29 00:08] falls asleep
        [1518-10-29 00:46] falls asleep
        [1518-07-19 00:55] wakes up
        [1518-07-06 00:38] falls asleep
        [1518-09-05 00:22] falls asleep
        [1518-04-13 00:26] wakes up
        [1518-11-22 23:46] Guard #251 begins shift
        [1518-04-06 00:55] wakes up
        [1518-08-12 00:06] falls asleep
        [1518-04-28 00:01] Guard #2971 begins shift
        [1518-04-28 00:43] falls asleep
        [1518-05-09 00:33] falls asleep
        [1518-09-16 00:59] wakes up
        [1518-08-21 00:16] falls asleep
        [1518-09-02 00:10] falls asleep
        [1518-08-19 00:23] falls asleep
        [1518-05-16 00:19] falls asleep
        [1518-06-23 00:39] wakes up
        [1518-04-18 00:57] wakes up
        [1518-09-22 00:17] wakes up
        [1518-06-15 00:04] falls asleep
        [1518-10-13 00:00] Guard #1811 begins shift
        [1518-10-18 00:02] Guard #751 begins shift
        [1518-10-26 00:24] falls asleep
        [1518-05-10 00:53] falls asleep
        [1518-06-22 00:01] Guard #3319 begins shift
        [1518-11-13 00:56] falls asleep
        [1518-07-14 00:16] falls asleep
        [1518-05-21 00:57] wakes up
        [1518-10-06 00:50] wakes up
        [1518-06-21 00:36] falls asleep
        [1518-04-08 00:47] wakes up
        [1518-09-06 00:10] falls asleep
        [1518-10-02 00:58] wakes up
        [1518-05-28 00:03] falls asleep
        [1518-09-15 00:58] wakes up
        [1518-07-28 00:02] Guard #2477 begins shift
        [1518-10-02 00:35] wakes up
        [1518-08-19 00:33] wakes up
        [1518-07-14 00:27] wakes up
        [1518-08-31 00:33] falls asleep
        [1518-05-06 00:58] wakes up
        [1518-05-24 00:10] falls asleep
        [1518-05-20 00:01] Guard #643 begins shift
        [1518-04-12 00:32] falls asleep
        [1518-09-16 00:17] falls asleep
        [1518-05-27 00:16] falls asleep
        [1518-11-17 00:15] falls asleep
        [1518-08-28 00:08] wakes up
        [1518-08-29 00:13] falls asleep
        [1518-09-20 00:23] falls asleep
        [1518-05-25 00:46] falls asleep
        [1518-05-26 00:59] wakes up
        [1518-07-28 00:16] falls asleep
        [1518-08-28 00:36] wakes up
        [1518-11-16 00:58] wakes up
        [1518-10-06 00:59] wakes up
        [1518-05-13 00:40] falls asleep
        [1518-09-29 00:35] wakes up
        [1518-11-06 00:04] falls asleep
        [1518-04-09 23:57] Guard #619 begins shift
        [1518-05-09 23:58] Guard #2467 begins shift
        [1518-09-17 23:59] Guard #619 begins shift
        [1518-11-08 00:51] wakes up
        [1518-05-29 00:37] wakes up
        [1518-05-01 00:44] falls asleep
        [1518-04-14 00:03] falls asleep
        [1518-10-09 00:59] wakes up
        [1518-09-14 23:59] Guard #911 begins shift
        [1518-07-21 00:38] falls asleep
        [1518-08-11 00:00] Guard #2273 begins shift
        [1518-05-17 00:57] wakes up
        [1518-04-24 00:54] wakes up
        [1518-11-07 00:39] falls asleep
        [1518-04-15 00:39] falls asleep
        [1518-10-28 00:57] wakes up
        [1518-05-24 23:59] Guard #3319 begins shift
        [1518-11-07 00:58] wakes up
        [1518-10-28 00:08] falls asleep
        [1518-08-27 23:53] Guard #2273 begins shift
        [1518-05-14 00:02] Guard #251 begins shift
        [1518-09-01 00:00] Guard #751 begins shift
        [1518-09-20 00:08] falls asleep
        [1518-09-23 00:05] falls asleep
        [1518-04-09 00:23] falls asleep
        [1518-09-25 00:58] wakes up
        [1518-07-21 00:59] wakes up
        [1518-08-08 00:40] wakes up
        [1518-05-13 00:04] Guard #1783 begins shift
        [1518-08-23 00:48] wakes up
        [1518-08-26 00:46] falls asleep
        [1518-10-13 00:12] falls asleep
        [1518-05-29 00:25] falls asleep
        [1518-09-18 00:16] falls asleep
        [1518-07-16 00:10] falls asleep
        [1518-06-10 00:52] wakes up
        [1518-06-29 00:30] wakes up
        [1518-08-06 23:48] Guard #2477 begins shift
        [1518-11-18 00:44] falls asleep
        [1518-11-19 00:16] wakes up
        [1518-11-07 00:12] falls asleep
        [1518-05-19 00:46] wakes up
        [1518-11-18 00:49] wakes up
        [1518-10-21 00:38] falls asleep
        [1518-04-20 00:42] falls asleep
        [1518-09-21 00:30] wakes up
        [1518-07-20 00:25] falls asleep
        [1518-07-12 00:18] falls asleep
        [1518-07-02 00:50] falls asleep
        [1518-05-31 00:02] falls asleep
        [1518-06-05 00:51] wakes up
        [1518-10-28 23:58] Guard #2477 begins shift
        [1518-07-14 00:59] wakes up
        [1518-10-30 00:34] wakes up
        [1518-10-19 00:00] Guard #1783 begins shift
        [1518-07-13 00:00] Guard #751 begins shift
        [1518-07-21 00:57] falls asleep
        [1518-10-01 00:42] wakes up
        [1518-08-29 00:04] Guard #643 begins shift
        [1518-08-01 23:58] Guard #457 begins shift
        [1518-11-08 00:02] Guard #179 begins shift
        [1518-11-06 00:36] wakes up
        [1518-10-03 00:57] wakes up
        [1518-11-15 00:29] falls asleep
        [1518-05-05 00:44] falls asleep
        [1518-08-02 00:42] falls asleep
        [1518-10-16 00:42] wakes up
        [1518-05-05 00:01] Guard #1811 begins shift
        [1518-10-27 00:21] wakes up
        [1518-05-09 00:34] wakes up
        [1518-05-01 00:03] Guard #1783 begins shift
        [1518-09-08 00:50] wakes up
        [1518-11-01 23:56] Guard #2477 begins shift
        [1518-07-03 23:59] Guard #1811 begins shift
        [1518-05-23 23:58] Guard #3319 begins shift
        [1518-11-09 00:41] falls asleep
        [1518-07-12 00:59] wakes up
        [1518-06-30 00:54] wakes up
        [1518-09-16 00:20] wakes up
        [1518-11-22 00:09] falls asleep
        [1518-08-12 00:55] wakes up
        [1518-07-09 00:44] wakes up
        [1518-06-27 23:51] Guard #1051 begins shift
        [1518-09-25 23:59] Guard #1051 begins shift
        [1518-07-05 00:58] wakes up
        [1518-09-27 00:02] Guard #617 begins shift
        [1518-11-11 00:05] falls asleep
        [1518-07-28 00:56] wakes up
        [1518-08-25 00:33] falls asleep
        [1518-05-23 00:50] wakes up
        [1518-07-07 00:35] wakes up
        [1518-08-22 00:45] wakes up
        [1518-10-22 00:23] wakes up
        [1518-08-11 00:10] falls asleep
        [1518-05-30 00:47] falls asleep
        [1518-05-28 23:57] Guard #2467 begins shift
        [1518-10-22 00:15] falls asleep
        [1518-04-10 00:29] falls asleep
        [1518-04-11 00:00] Guard #1783 begins shift
        [1518-07-31 00:53] wakes up
        [1518-11-20 00:02] falls asleep
        [1518-05-15 00:48] falls asleep
        [1518-10-06 00:44] wakes up
        [1518-08-24 00:33] wakes up
        [1518-06-03 00:44] wakes up
        [1518-06-16 00:51] falls asleep
        [1518-07-04 23:58] Guard #179 begins shift
        [1518-11-20 00:53] wakes up
        [1518-09-24 00:03] Guard #179 begins shift
        [1518-05-30 00:04] Guard #2917 begins shift
        [1518-10-02 23:56] Guard #643 begins shift
        [1518-07-23 00:42] falls asleep
        [1518-06-09 23:56] Guard #251 begins shift
        [1518-07-13 00:46] falls asleep
        [1518-09-23 00:38] falls asleep
        [1518-04-16 00:01] falls asleep
        [1518-11-15 00:41] wakes up
        [1518-08-04 00:02] Guard #2971 begins shift
        [1518-08-03 00:02] Guard #2477 begins shift
        [1518-09-02 00:45] wakes up
        [1518-11-05 23:53] Guard #2467 begins shift
        [1518-08-16 00:59] wakes up
        [1518-06-18 00:49] wakes up
        [1518-09-05 00:01] Guard #499 begins shift
        [1518-07-13 00:54] wakes up
        [1518-04-27 00:57] falls asleep
        [1518-09-29 00:41] falls asleep
        [1518-05-16 00:32] wakes up
        [1518-05-15 00:02] falls asleep
        [1518-09-30 00:53] wakes up
        [1518-05-23 00:07] falls asleep
        [1518-08-04 00:43] falls asleep
        [1518-07-26 00:11] falls asleep
        [1518-10-27 23:58] Guard #1051 begins shift
        [1518-04-15 00:46] wakes up
        [1518-07-10 00:43] falls asleep
        [1518-11-03 00:55] wakes up
        [1518-05-22 00:51] wakes up
        [1518-07-21 00:42] wakes up
        [1518-09-22 00:52] falls asleep
        [1518-11-09 00:12] falls asleep
        [1518-07-07 00:57] falls asleep
        [1518-09-06 00:29] wakes up
        [1518-06-25 00:35] wakes up
        [1518-07-02 23:59] Guard #2657 begins shift
        [1518-10-01 00:03] Guard #251 begins shift
        [1518-06-28 00:04] falls asleep
        [1518-05-10 00:29] wakes up
        [1518-06-14 23:51] Guard #1811 begins shift
        [1518-08-28 00:21] falls asleep
        [1518-06-29 00:00] Guard #1811 begins shift
        [1518-10-23 23:58] Guard #179 begins shift
        [1518-08-26 00:57] wakes up
        [1518-06-13 00:56] wakes up
        [1518-10-01 00:59] wakes up
        [1518-04-08 00:35] falls asleep
        [1518-07-07 00:58] wakes up
        [1518-09-22 00:59] wakes up
        [1518-04-27 00:54] wakes up
        [1518-08-27 00:00] Guard #3467 begins shift
        [1518-08-13 23:56] Guard #457 begins shift
        [1518-11-03 00:00] Guard #1811 begins shift
        [1518-05-21 23:59] Guard #3467 begins shift
        [1518-11-14 00:35] falls asleep
        [1518-06-13 00:24] falls asleep
        [1518-05-18 23:52] Guard #2917 begins shift
        [1518-09-30 00:42] wakes up
        [1518-04-16 23:59] Guard #751 begins shift
        [1518-10-08 00:34] falls asleep
        [1518-08-21 00:35] falls asleep
        [1518-11-01 00:47] wakes up
        [1518-04-24 00:33] falls asleep
        [1518-09-02 23:57] Guard #2917 begins shift
        [1518-10-24 00:28] falls asleep
        [1518-06-12 00:30] wakes up
        [1518-05-19 00:27] wakes up
        [1518-05-30 00:41] falls asleep
        [1518-09-14 00:18] falls asleep
        [1518-05-19 00:00] falls asleep
        [1518-07-25 00:31] wakes up
        [1518-04-22 00:00] Guard #619 begins shift
        [1518-05-17 23:56] Guard #499 begins shift
        [1518-08-09 00:59] wakes up
        [1518-06-01 23:59] Guard #3167 begins shift
        [1518-09-22 23:52] Guard #643 begins shift
        [1518-07-27 00:27] falls asleep
        [1518-04-30 00:24] wakes up
        [1518-04-06 00:09] falls asleep
        [1518-06-20 00:29] falls asleep
        [1518-06-12 00:33] falls asleep
        [1518-08-09 00:53] falls asleep
        [1518-05-11 00:59] wakes up
        [1518-11-08 00:28] falls asleep
        [1518-06-11 00:01] falls asleep
        [1518-05-25 00:55] wakes up
        [1518-06-03 00:52] wakes up
        [1518-09-23 00:31] wakes up
        [1518-05-01 00:39] wakes up
        [1518-05-05 00:53] wakes up
        [1518-04-28 23:57] Guard #643 begins shift
        [1518-07-17 23:59] Guard #2657 begins shift
        [1518-10-10 00:49] wakes up
        [1518-10-30 00:30] falls asleep
        [1518-09-12 00:33] wakes up
        [1518-05-17 00:03] Guard #179 begins shift
        [1518-06-03 00:25] falls asleep
        [1518-07-03 00:51] wakes up
        [1518-07-12 00:57] falls asleep
        [1518-11-19 00:35] falls asleep
        [1518-10-16 00:11] falls asleep
        [1518-11-16 00:22] falls asleep
        [1518-11-08 00:56] falls asleep
        [1518-10-20 00:07] falls asleep
        [1518-09-10 00:25] wakes up
        [1518-08-23 00:02] Guard #2467 begins shift
        [1518-05-09 00:57] wakes up
        [1518-08-10 00:28] wakes up
        [1518-10-21 00:43] falls asleep
        [1518-11-22 00:04] Guard #3319 begins shift
        [1518-04-20 00:43] wakes up
        [1518-07-09 00:40] falls asleep
        [1518-10-29 00:29] wakes up
        [1518-05-04 00:45] wakes up
        [1518-05-08 00:02] Guard #457 begins shift
        [1518-11-21 00:48] falls asleep
        [1518-07-09 00:05] falls asleep
        [1518-05-30 00:43] wakes up
        [1518-08-11 00:31] wakes up
        [1518-11-19 00:02] Guard #2971 begins shift
        [1518-06-24 00:01] Guard #2467 begins shift
        [1518-11-10 00:57] wakes up
        [1518-06-20 00:58] wakes up
        [1518-11-09 00:28] wakes up
        [1518-04-09 00:50] wakes up
        [1518-09-11 00:00] Guard #1783 begins shift
        [1518-10-07 23:53] Guard #2971 begins shift
        [1518-04-06 00:00] Guard #499 begins shift
        [1518-06-08 00:01] Guard #617 begins shift
        [1518-11-04 00:47] wakes up
        [1518-07-28 00:20] wakes up
        [1518-10-27 00:53] wakes up
        [1518-05-15 00:32] wakes up
        [1518-08-07 00:04] falls asleep
        [1518-07-19 00:35] falls asleep
        [1518-09-28 00:00] Guard #1783 begins shift
        [1518-05-08 00:48] falls asleep
        [1518-05-24 00:52] falls asleep
        [1518-05-13 00:45] wakes up
        [1518-08-20 00:03] Guard #499 begins shift
        [1518-06-05 00:03] Guard #179 begins shift
        [1518-09-02 00:41] falls asleep
        [1518-09-17 00:56] wakes up
        [1518-04-11 00:06] falls asleep
        [1518-10-07 00:28] wakes up
        [1518-08-27 00:49] wakes up
        [1518-09-07 00:24] wakes up
        [1518-06-25 00:48] falls asleep
        [1518-09-02 00:00] Guard #3319 begins shift
        [1518-08-10 00:26] falls asleep
        [1518-08-14 00:21] falls asleep
        [1518-07-12 00:00] Guard #2477 begins shift
        [1518-08-05 00:03] Guard #251 begins shift
        [1518-05-07 00:00] Guard #571 begins shift
        [1518-06-03 23:51] Guard #2477 begins shift
        [1518-10-12 00:58] wakes up
        [1518-09-22 00:00] Guard #3467 begins shift
        [1518-11-19 23:46] Guard #3167 begins shift
        [1518-04-21 00:03] Guard #619 begins shift
        [1518-08-21 00:56] wakes up
        [1518-10-02 00:01] Guard #2971 begins shift
        [1518-05-18 00:23] falls asleep
        [1518-10-14 00:57] wakes up
        [1518-11-19 00:39] falls asleep
        [1518-08-12 00:14] wakes up
        [1518-04-19 00:56] wakes up
        [1518-11-11 00:46] falls asleep
        [1518-08-07 23:58] Guard #911 begins shift
        [1518-10-06 00:02] Guard #643 begins shift
        [1518-04-25 00:41] falls asleep
        [1518-08-08 23:57] Guard #251 begins shift
        [1518-07-25 00:00] falls asleep
        [1518-06-22 00:29] falls asleep
        [1518-11-12 00:55] wakes up
        [1518-04-11 00:48] wakes up
        [1518-10-16 23:58] Guard #571 begins shift
        [1518-05-01 00:35] falls asleep
        [1518-08-15 00:08] falls asleep
        [1518-09-24 00:14] falls asleep
        [1518-07-30 00:21] wakes up
        [1518-11-21 00:19] wakes up
        [1518-11-16 23:56] Guard #911 begins shift
        [1518-06-25 23:57] Guard #3167 begins shift
        [1518-05-16 00:46] falls asleep
        [1518-06-29 00:52] wakes up
        [1518-10-06 00:48] falls asleep
        [1518-05-27 00:04] Guard #2917 begins shift
        [1518-04-18 00:54] falls asleep
        [1518-04-15 00:54] falls asleep
        [1518-08-12 23:56] Guard #3467 begins shift
        [1518-10-05 00:41] falls asleep
        [1518-07-17 00:13] falls asleep
        [1518-06-20 23:56] Guard #2273 begins shift
        [1518-04-24 00:17] falls asleep
        [1518-04-16 00:55] wakes up
        [1518-09-19 00:13] wakes up
        [1518-07-06 00:29] wakes up
        [1518-04-07 00:55] wakes up
        [1518-05-19 00:44] falls asleep
        [1518-11-05 00:42] wakes up
        [1518-07-11 00:00] Guard #179 begins shift
        [1518-04-29 23:59] Guard #457 begins shift
        [1518-06-14 00:45] wakes up
        [1518-06-19 23:58] Guard #2917 begins shift
        [1518-05-26 00:16] falls asleep
        [1518-08-31 00:38] wakes up
        [1518-07-18 00:55] wakes up
        [1518-05-23 00:54] falls asleep
        [1518-07-08 23:46] Guard #3467 begins shift
        [1518-06-06 00:01] Guard #643 begins shift
        [1518-07-01 00:48] falls asleep
        [1518-05-18 00:24] wakes up
        [1518-09-03 00:29] falls asleep
        [1518-06-18 00:34] falls asleep
        [1518-04-24 00:27] wakes up
        [1518-06-12 00:54] wakes up
        [1518-09-19 00:08] falls asleep
        [1518-10-25 00:22] falls asleep
        [1518-05-24 00:58] wakes up
        [1518-10-11 00:00] Guard #571 begins shift
        [1518-08-14 00:27] wakes up
        [1518-09-04 00:47] wakes up
        [1518-07-18 00:33] falls asleep
        [1518-09-10 00:00] Guard #2273 begins shift
        [1518-05-22 23:59] Guard #3467 begins shift
        [1518-05-01 00:45] wakes up
        [1518-05-04 00:17] falls asleep
        [1518-09-05 00:43] wakes up
        [1518-06-17 00:47] wakes up
        [1518-08-24 23:58] Guard #619 begins shift
        [1518-04-20 00:02] Guard #3319 begins shift
        [1518-10-21 00:39] wakes up
        [1518-09-01 00:39] falls asleep
        [1518-07-08 00:40] wakes up
        [1518-09-09 00:42] falls asleep
        [1518-07-17 00:47] falls asleep
        [1518-07-08 00:00] Guard #643 begins shift
        [1518-09-01 00:31] wakes up
        [1518-11-09 00:43] wakes up
        [1518-04-09 00:00] Guard #751 begins shift
        [1518-07-06 00:09] falls asleep
        [1518-04-23 00:06] falls asleep
        [1518-08-15 00:49] wakes up
        [1518-07-24 00:58] wakes up
        [1518-05-20 00:15] wakes up
        [1518-06-29 23:50] Guard #2467 begins shift
        [1518-11-23 00:03] falls asleep
        [1518-10-18 00:24] falls asleep
        [1518-08-24 00:32] falls asleep
        [1518-10-09 00:32] falls asleep
        [1518-09-20 00:36] wakes up
        [1518-09-19 00:52] wakes up
        [1518-06-16 00:46] wakes up
        [1518-11-08 00:58] wakes up
        [1518-07-29 00:50] falls asleep
        [1518-10-15 23:57] Guard #1811 begins shift
        [1518-08-21 00:30] wakes up
        [1518-06-25 00:56] wakes up
        [1518-05-06 00:44] wakes up
        [1518-11-12 00:04] Guard #499 begins shift
        [1518-05-20 00:59] wakes up
        [1518-08-20 00:52] wakes up
        [1518-08-22 00:02] Guard #1783 begins shift
        [1518-09-17 00:04] falls asleep
        [1518-09-17 00:39] falls asleep
        [1518-05-06 00:16] falls asleep
        [1518-04-06 00:43] falls asleep
        [1518-11-04 00:42] falls asleep
        [1518-07-10 00:48] wakes up
        [1518-08-14 23:58] Guard #2273 begins shift
        [1518-08-05 00:34] wakes up
        [1518-06-10 23:54] Guard #3167 begins shift
        [1518-11-19 00:36] wakes up
        [1518-07-28 00:53] falls asleep
        [1518-10-03 00:39] falls asleep
        [1518-05-27 23:52] Guard #251 begins shift
        [1518-04-19 00:38] falls asleep
        [1518-09-20 00:13] wakes up
        [1518-06-24 00:55] wakes up
        [1518-04-13 23:54] Guard #179 begins shift
        [1518-08-27 00:39] falls asleep
        [1518-05-20 00:11] falls asleep
        [1518-06-07 00:28] wakes up
        [1518-10-08 00:03] falls asleep
        [1518-10-31 00:32] wakes up
        [1518-05-11 00:01] Guard #179 begins shift
        [1518-11-02 00:28] wakes up
        [1518-10-22 00:01] Guard #251 begins shift
        [1518-11-02 00:51] falls asleep
        [1518-06-10 00:56] falls asleep
        [1518-08-11 00:52] wakes up
        [1518-05-14 00:13] wakes up
        [1518-11-10 00:55] falls asleep
        [1518-08-07 00:59] wakes up
        [1518-06-21 00:27] wakes up
        [1518-09-12 00:02] Guard #3319 begins shift
        [1518-10-23 00:49] wakes up
        [1518-11-03 23:58] Guard #1783 begins shift
        [1518-10-20 00:03] Guard #3167 begins shift
        [1518-04-18 00:04] falls asleep
        [1518-07-06 23:57] Guard #3467 begins shift
        [1518-08-03 00:28] falls asleep
        [1518-11-12 00:20] wakes up
        [1518-07-20 00:52] wakes up
        [1518-08-23 00:39] falls asleep
        [1518-04-21 00:23] falls asleep
        [1518-11-03 00:08] falls asleep
        [1518-07-26 00:57] wakes up
        [1518-04-17 00:31] falls asleep
        [1518-06-14 00:04] Guard #1783 begins shift
        [1518-09-14 00:45] wakes up
        [1518-06-16 00:58] wakes up
        [1518-06-19 00:41] wakes up
        [1518-06-19 00:28] falls asleep
        [1518-09-09 00:18] falls asleep
        [1518-08-10 00:01] Guard #457 begins shift
        [1518-05-26 00:00] Guard #1783 begins shift
        [1518-09-12 23:46] Guard #3467 begins shift
        [1518-06-12 00:25] falls asleep
        [1518-11-07 00:00] Guard #3467 begins shift
        [1518-07-22 00:31] falls asleep
        [1518-04-17 00:59] wakes up
        [1518-08-13 00:32] falls asleep
        [1518-05-08 00:52] wakes up
        [1518-05-12 00:11] falls asleep
        [1518-06-01 00:31] falls asleep
        [1518-05-01 23:58] Guard #1783 begins shift
        [1518-05-14 00:47] wakes up
        [1518-05-02 23:56] Guard #457 begins shift
        [1518-07-30 00:03] Guard #3167 begins shift
        [1518-05-30 23:51] Guard #911 begins shift
        [1518-09-25 00:54] wakes up
        [1518-07-04 00:52] wakes up
        [1518-05-22 00:27] falls asleep
        [1518-05-10 00:55] wakes up
        [1518-04-14 00:30] wakes up
        [1518-04-26 00:04] Guard #2917 begins shift
        [1518-05-24 00:27] falls asleep
        [1518-07-02 00:57] wakes up
        [1518-07-01 23:48] Guard #3467 begins shift
        [1518-11-09 00:03] Guard #619 begins shift
        [1518-11-03 00:17] falls asleep
        [1518-07-19 23:57] Guard #457 begins shift
        [1518-09-21 00:02] Guard #3167 begins shift
        [1518-04-30 00:21] falls asleep
        [1518-07-31 00:25] falls asleep
        [1518-10-27 00:02] Guard #1783 begins shift
        [1518-09-13 00:49] wakes up
        [1518-09-20 00:00] Guard #499 begins shift
        [1518-07-01 00:04] Guard #499 begins shift
        [1518-08-15 23:50] Guard #2467 begins shift
        [1518-05-03 00:25] wakes up
        [1518-06-08 23:48] Guard #1811 begins shift
        [1518-04-12 00:57] wakes up
        [1518-08-16 00:00] falls asleep
        [1518-09-02 00:13] wakes up
        [1518-11-19 00:14] falls asleep
        [1518-10-25 00:52] wakes up
        [1518-04-23 00:20] wakes up
        [1518-05-28 00:08] wakes up
        [1518-08-07 00:54] wakes up
        [1518-09-19 00:48] falls asleep
        [1518-05-16 00:58] wakes up
        [1518-04-14 00:40] falls asleep
        [1518-05-02 00:45] wakes up
        [1518-06-13 00:44] wakes up
        [1518-05-14 23:47] Guard #2467 begins shift
        [1518-11-13 00:58] wakes up
        [1518-05-11 23:59] Guard #2477 begins shift
        [1518-09-01 00:40] wakes up
        [1518-06-07 00:12] falls asleep
        [1518-10-15 00:00] Guard #751 begins shift
        [1518-11-02 00:52] wakes up
        [1518-05-16 00:54] falls asleep
        [1518-10-25 00:01] Guard #2657 begins shift
        [1518-10-06 00:55] falls asleep
        [1518-07-12 00:53] wakes up
        [1518-10-27 00:41] falls asleep
        [1518-11-06 00:54] falls asleep
        [1518-06-06 23:57] Guard #2477 begins shift
        [1518-11-09 23:56] Guard #619 begins shift
        [1518-04-21 00:51] wakes up
        [1518-11-19 00:58] wakes up
        [1518-04-29 00:38] falls asleep
        [1518-08-28 00:03] falls asleep
        [1518-04-28 00:52] wakes up
        [1518-11-01 00:15] falls asleep
        [1518-10-31 00:27] falls asleep
        [1518-10-02 00:49] falls asleep
        [1518-04-07 00:08] falls asleep
        [1518-10-14 00:03] Guard #2467 begins shift
        [1518-10-01 00:29] falls asleep
        [1518-09-28 00:38] falls asleep
        [1518-06-10 00:18] falls asleep
        [1518-07-25 00:35] falls asleep
        [1518-06-28 00:35] wakes up
        [1518-11-14 00:51] wakes up
        [1518-06-21 00:14] falls asleep
        [1518-07-16 00:39] wakes up
        [1518-10-31 00:02] Guard #179 begins shift
        [1518-04-14 00:57] wakes up
        [1518-07-11 00:59] wakes up
        [1518-05-14 00:11] falls asleep
        [1518-04-15 00:58] wakes up
        [1518-09-03 00:30] wakes up
        [1518-07-21 00:03] falls asleep
        [1518-11-10 00:26] falls asleep
        [1518-09-26 00:29] falls asleep
        [1518-11-17 00:27] wakes up
        [1518-09-23 00:51] wakes up
        [1518-07-07 00:10] falls asleep
        [1518-08-30 00:01] falls asleep
        [1518-08-06 00:26] wakes up
        [1518-10-08 00:18] wakes up
        [1518-07-01 00:27] wakes up
        [1518-05-09 00:03] Guard #2467 begins shift
        [1518-06-24 00:08] falls asleep
        [1518-10-09 23:59] Guard #1051 begins shift
        [1518-09-22 00:09] falls asleep
        [1518-05-21 00:00] Guard #1783 begins shift
        [1518-06-04 00:52] wakes up
        [1518-10-08 23:58] Guard #1051 begins shift
        [1518-09-07 23:52] Guard #619 begins shift
        [1518-08-20 00:15] wakes up
        [1518-06-11 00:53] falls asleep
        [1518-11-12 00:16] falls asleep
        [1518-05-30 00:56] wakes up
        [1518-07-24 00:24] falls asleep
        [1518-07-02 00:04] falls asleep
        [1518-06-16 00:25] falls asleep
        [1518-10-14 00:06] falls asleep
        [1518-04-12 00:51] wakes up
        [1518-08-01 00:00] Guard #3167 begins shift
        [1518-11-21 00:53] wakes up
        [1518-08-05 00:08] falls asleep
        [1518-05-10 00:25] falls asleep
        [1518-07-05 00:45] falls asleep
        [1518-07-14 00:34] falls asleep
        [1518-04-15 23:46] Guard #179 begins shift
        [1518-09-30 00:45] falls asleep
        [1518-07-27 00:31] wakes up
        [1518-06-10 00:44] wakes up
        [1518-04-27 00:58] wakes up
        [1518-07-19 00:18] falls asleep
        [1518-06-04 00:02] falls asleep
        [1518-06-22 00:55] wakes up
        [1518-07-30 00:20] falls asleep
        [1518-08-04 00:46] wakes up
        [1518-08-02 00:59] wakes up
        [1518-06-13 00:49] falls asleep
        [1518-04-22 00:28] falls asleep
        [1518-11-14 23:57] Guard #3319 begins shift
        [1518-08-26 00:09] falls asleep
        [1518-06-13 00:04] Guard #179 begins shift
        [1518-11-06 00:55] wakes up
        [1518-06-09 00:00] falls asleep
        [1518-05-14 00:23] falls asleep
        [1518-09-22 00:21] falls asleep
        [1518-10-10 00:31] wakes up
        [1518-10-29 00:09] falls asleep
        [1518-04-12 23:58] Guard #3167 begins shift
        [1518-10-13 00:42] wakes up
        [1518-08-18 00:03] wakes up
        [1518-05-02 00:26] falls asleep
        [1518-04-23 00:01] Guard #499 begins shift
        [1518-08-20 00:25] falls asleep
        [1518-08-22 00:42] falls asleep
        [1518-04-17 23:53] Guard #2273 begins shift
        [1518-08-23 23:58] Guard #179 begins shift
        [1518-04-24 23:59] Guard #1783 begins shift
        [1518-04-29 00:56] wakes up
        [1518-10-19 00:16] falls asleep
        [1518-09-30 00:04] falls asleep
        [1518-09-21 00:47] falls asleep
        [1518-06-06 00:42] wakes up
        [1518-08-26 00:38] wakes up
        [1518-07-26 00:18] wakes up
        [1518-06-25 00:04] falls asleep
        [1518-07-13 23:56] Guard #2467 begins shift
        [1518-09-10 00:31] falls asleep
        [1518-09-13 00:05] falls asleep
        [1518-10-19 00:28] wakes up
        [1518-10-04 00:52] wakes up
        [1518-10-04 00:51] falls asleep
        [1518-10-29 00:53] wakes up
        [1518-07-29 00:53] wakes up
        [1518-11-21 00:02] Guard #2273 begins shift
        [1518-10-27 00:17] falls asleep
        [1518-05-23 00:28] wakes up
        [1518-04-27 00:03] Guard #2273 begins shift
        [1518-06-05 00:49] falls asleep
        [1518-07-22 23:56] Guard #619 begins shift
        [1518-09-07 00:22] falls asleep
        [1518-07-29 00:00] Guard #499 begins shift
        [1518-07-16 00:03] Guard #2273 begins shift
        [1518-11-03 00:13] wakes up
        [1518-11-13 23:59] Guard #911 begins shift
        [1518-08-30 23:59] Guard #643 begins shift
        [1518-07-22 00:07] wakes up
        [1518-05-11 00:55] falls asleep
        [1518-09-24 00:41] wakes up
        [1518-06-11 00:58] wakes up
        [1518-09-16 00:29] falls asleep
        [1518-04-22 00:53] wakes up
        [1518-09-18 23:57] Guard #2273 begins shift
        [1518-04-26 00:51] wakes up
        [1518-06-16 00:04] Guard #457 begins shift
        [1518-10-24 00:56] wakes up
        [1518-06-23 00:05] falls asleep
        [1518-05-27 00:39] wakes up
        [1518-08-17 00:36] wakes up
        [1518-07-08 00:53] wakes up
        [1518-07-04 00:13] falls asleep
        [1518-10-18 00:34] wakes up
        [1518-10-10 00:06] falls asleep
        [1518-08-01 00:46] wakes up
        [1518-07-06 00:53] wakes up
        [1518-04-13 00:51] wakes up
        [1518-07-30 23:56] Guard #619 begins shift
        [1518-07-15 00:20] falls asleep
        [1518-07-25 23:57] Guard #179 begins shift
        [1518-07-19 00:04] Guard #911 begins shift
        [1518-10-15 00:43] wakes up
        [1518-09-26 00:58] wakes up
        [1518-06-17 00:31] falls asleep
        [1518-06-12 00:01] Guard #911 begins shift
        [1518-05-15 00:58] wakes up
        [1518-04-06 23:59] Guard #1783 begins shift
        [1518-09-03 23:57] Guard #619 begins shift
        [1518-04-27 00:27] falls asleep
        [1518-07-01 00:20] falls asleep
        [1518-09-29 23:50] Guard #911 begins shift
        [1518-06-10 00:58] wakes up
        [1518-10-12 00:52] falls asleep
        [1518-04-06 00:32] wakes up
        [1518-08-18 00:01] falls asleep
        [1518-07-15 00:45] wakes up
        [1518-11-04 23:57] Guard #2657 begins shift
        [1518-06-03 00:50] falls asleep
        [1518-10-07 00:04] falls asleep
        [1518-10-15 00:58] wakes up
        [1518-08-07 00:57] falls asleep
        [1518-08-17 00:01] falls asleep
        [1518-06-26 23:58] Guard #3467 begins shift
        [1518-07-22 00:03] falls asleep
        [1518-10-23 00:42] falls asleep
        [1518-10-04 00:03] Guard #619 begins shift
        [1518-11-17 00:45] falls asleep
        [1518-11-23 00:26] wakes up
        [1518-09-10 00:07] falls asleep
        [1518-08-12 00:04] Guard #1051 begins shift
        [1518-06-15 00:43] wakes up
        [1518-11-05 00:07] falls asleep
        [1518-07-07 00:53] wakes up
        [1518-04-07 23:58] Guard #2657 begins shift
        [1518-04-27 00:53] falls asleep
        [1518-10-21 00:52] wakes up
        [1518-11-11 00:18] wakes up
        [1518-10-22 00:26] falls asleep
        [1518-07-24 23:47] Guard #3319 begins shift
    "};
}
