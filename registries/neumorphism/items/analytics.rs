//! Date-only aggregation for the analytics dashboard and its three chart blocks.
//! Money is stored in cents. A missing observation is distinct from a recorded zero.
use gpui_deck::Date;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct Totals {
    pub(crate) visits: u64,
    pub(crate) converted_visits: u64,
    pub(crate) revenue_cents: u64,
    pub(crate) target_cents: u64,
}

impl Totals {
    pub(crate) fn conversion(self) -> Option<f64> {
        (self.visits != 0).then(|| self.converted_visits as f64 / self.visits as f64)
    }

    fn add(&mut self, other: Self) -> Result<(), &'static str> {
        fn sum(left: u64, right: u64) -> Result<u64, &'static str> {
            left.checked_add(right)
                .ok_or("Analytics total out of range")
        }
        *self = Self {
            visits: sum(self.visits, other.visits)?,
            converted_visits: sum(self.converted_visits, other.converted_visits)?,
            revenue_cents: sum(self.revenue_cents, other.revenue_cents)?,
            target_cents: sum(self.target_cents, other.target_cents)?,
        };
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Record {
    pub(crate) date: String,
    pub(crate) channel: String,
    pub(crate) totals: Totals,
}

#[derive(Default)]
pub(crate) struct Summary {
    pub(crate) points: Vec<(Date, Option<Totals>)>,
    pub(crate) channels: Vec<(String, Totals)>,
    pub(crate) totals: Totals,
}

impl Summary {
    pub(crate) fn csv(&self) -> String {
        use std::fmt::Write as _;
        let mut csv = "date,visits,converted_visits,revenue_cents,target_cents,conversion_rate\r\n"
            .to_owned();
        for (date, value) in &self.points {
            if let Some(t) = value {
                let rate = t.conversion().map(|v| v.to_string()).unwrap_or_default();
                write!(
                    csv,
                    "{date},{},{},{},{},{rate}\r\n",
                    t.visits, t.converted_visits, t.revenue_cents, t.target_cents
                )
                .unwrap();
            } else {
                write!(csv, "{date},,,,,\r\n").unwrap();
            }
        }
        csv
    }
}

pub(crate) fn summarize(
    records: &[Record],
    days: usize,
    channel: Option<&str>,
) -> Result<Summary, &'static str> {
    if !(1..=366).contains(&days) {
        return Err("Choose a window from 1 to 366 days");
    }
    let mut keys = BTreeSet::new();
    let mut end: Option<Date> = None;
    for record in records {
        let date: Date = record.date.parse().map_err(|_| "Invalid calendar date")?;
        if date.year() < 1000 || date.to_string() != record.date {
            return Err("Use an ISO calendar date");
        }
        if record.channel.trim().is_empty() {
            return Err("A channel name is required");
        }
        if !keys.insert((&record.date, &record.channel)) {
            return Err("Duplicate date and channel");
        }
        if record.totals.converted_visits > record.totals.visits {
            return Err("Conversions exceed visits");
        }
        end = Some(end.map_or(date, |old| old.max(date)));
    }
    let Some(mut start) = end else {
        return Ok(Summary::default());
    };
    for _ in 1..days {
        start = start.yesterday().map_err(|_| "Date window out of range")?;
    }
    if start.year() < 1000 {
        return Err("Date window out of range");
    }
    let start_iso = start.to_string();
    let mut daily: BTreeMap<&str, Totals> = BTreeMap::new();
    let mut channels: BTreeMap<String, Totals> = BTreeMap::new();
    let mut result = Summary::default();
    for record in records
        .iter()
        .filter(|r| r.date >= start_iso && channel.is_none_or(|c| c == r.channel))
    {
        daily.entry(&record.date).or_default().add(record.totals)?;
        channels
            .entry(record.channel.clone())
            .or_default()
            .add(record.totals)?;
        result.totals.add(record.totals)?;
    }
    let mut date = start;
    for index in 0..days {
        let key = date.to_string();
        let value = daily.get(key.as_str()).copied();
        result.points.push((date, value));
        if index + 1 < days {
            date = date.tomorrow().map_err(|_| "Date window out of range")?;
        }
    }
    result.channels = channels.into_iter().collect();
    result
        .channels
        .sort_by(|a, b| b.1.visits.cmp(&a.1.visits).then_with(|| a.0.cmp(&b.0)));
    Ok(result)
}

/// The original registry's reproducible September 2026 example.
pub(crate) fn demo() -> Vec<Record> {
    (0..30)
        .flat_map(|index| {
            ["Organic", "Direct", "Referral"]
                .into_iter()
                .enumerate()
                .map(move |(group, channel)| {
                    let group = group as u64;
                    let visits =
                        230 + (2 - group) * 115 + index * 8 + (index * 17 + group * 29) % 85;
                    let converted_visits = (visits as f64
                        * (0.035 + group as f64 * 0.012 + (index % 5) as f64 * 0.003))
                        .floor() as u64;
                    Record {
                        date: format!("2026-09-{:02}", index + 1),
                        channel: channel.to_owned(),
                        totals: Totals {
                            visits,
                            converted_visits,
                            revenue_cents: converted_visits * (4200 + group * 800),
                            target_cents: 115000 + (2 - group) * 25000 + index * 1800,
                        },
                    }
                })
        })
        .collect()
}
