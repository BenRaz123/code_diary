use chrono::Datelike;
use chrono::Timelike;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
/// # TimeStamp
/// `timestamps::TimeStamp` is a simple struct for storing timestamps.
/// Example:
/// ```rust
/// # use timestamps::TimeStamp;
/// # fn main() -> Option<()> {
/// // With manual constructor
/// let timestamp = TimeStamp {
///     year: 2023,
///     month: 03,
///     day: 14,
///     hour: 3,
///     minute: 0,
///     second: 10;
/// };
/// // Automatically generated
/// let timestamp = TimeStamp::new();
///
/// // From a string
/// let timestamp = TimeStamp::from_string("2023-03-14 3:00:00")?;
///
/// // Display (or cast to String)
/// println!("{timestamp}");
/// # }
/// ```
pub struct TimeStamp {
    /// The year in question (due to it being a u16, the possible years start at 0 and go up to
    /// 65535)
    pub year: u16,
    /// A standard gregorian month (1-12)
    pub month: u8,
    /// A day of the month from 1-(3(1))|2{8|9}
    pub day: u8,
    /// The hour in question (1-24)
    pub hour: u8,
    /// The minute in question (0-60)
    pub minute: u8,
    /// The second in question (0-60)
    pub second: u8,
}

impl TimeStamp {
    /// Takes the current time and returns a timestamp.

    /// Example:
    /// ```rust
    /// use timestamps::TimeStamp;
    ///
    /// fn main() {
    ///     let now = TimeStamp::new();
    ///     println!("Current time: {now}");
    /// }
    /// ```
    pub fn new() -> Self {
        let dt = chrono::Local::now();
        let year = dt.year() as u16;
        let month = dt.month() as u8;
        let day = dt.day() as u8;
        let hour = dt.hour() as u8;
        let minute = dt.minute() as u8;
        let second = dt.second() as u8;

        Self {
            year: year,
            month: month,
            day: day,
            hour: hour,
            minute: minute,
            second: second,
        }
    }
    /// Takes a string and converts it into a timestamp
    pub fn from_string<'a>(timestamp: &'a str) -> Option<Self> {
        let regex = regex::Regex::new(
            "([0-9]{3,4})-([0-9]{1,2})-([0-9]{1,2}) ([0-9]{1,2}):([0-9]{1,2}):([0-9]{1,2})",
        )
        .unwrap();
        if !regex.is_match(timestamp) {
            return None;
        }

        let date_component = timestamp.split(" ").collect::<Vec<_>>()[0];
        let dates = date_component.split("-").collect::<Vec<_>>();
        let year: u16 = dates[0].parse().unwrap();
        let month: u8 = dates[1].parse().unwrap();
        let day: u8 = dates[2].parse().unwrap();

        let time_component = timestamp.split(" ").collect::<Vec<_>>()[1];
        let times = time_component.split(":").collect::<Vec<_>>();
        let hour: u8 = times[0].parse().unwrap();
        let minute: u8 = times[1].parse().unwrap();
        let second: u8 = times[2].parse().unwrap();

        Some(Self {
            year: year,
            month: month,
            day: day,
            hour: hour,
            minute: minute,
            second: second,
        })
    }
}

impl std::fmt::Display for TimeStamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{y}-{m}-{d} {h}:{n}:{s}",
            y = self.year,
            m = self.month,
            d = self.day,
            h = self.hour,
            n = self.minute,
            s = self.second,
        )
    }
}
