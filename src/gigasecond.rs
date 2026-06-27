use time::{Date, PrimitiveDateTime as DateTime, Time, Month};

pub fn datetime(year: i32, month: Month, day: u8, hour: u8, minute: u8, second: u8) -> DateTime{
    let dated = Date::from_calendar_date(year, month, day).unwrap();
    let timed = Time::from_hms(hour, minute, second).unwrap();
    DateTime::new(dated, timed)
}


// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let gigasecond = time::Duration::seconds(1000000000);
    start + gigasecond
}
fn main() {
    let start = datetime(2011, Month::April, 25, 0, 0, 0);
    let actual =after(start);
    let expected = datetime(2043, Month::January, 1, 1, 46, 40);
    assert_eq!(actual, expected);
    println!("{:#?}", actual);
}