extern crate chrono;
#[macro_use]
extern crate lazy_static;

use std::time;
use std::ops::{Add, Sub};
use chrono::{Local, NaiveDate, Datelike, Duration};

lazy_static! {
    static ref START_DATE: NaiveDate = NaiveDate::from_ymd(1900, 1, 31);
    static ref YEAR_DAYS: Vec<u32> = {
        let mut days = Vec::with_capacity(150);
        for i in 0..150 {
            days.push(year_info_to_year_day(YEAR_INFOS[i]));
        }
        days
    };
}

const YEAR_INFOS: [u32; 150] = [
    /* encoding:
                b bbbbbbbbbbbb bbbb
        bit#    1 111111000000 0000
                6 543210987654 3210
                . ............ ....
        month#    000000000111
                M 123456789012   L

    b_j = 1 for long month, b_j = 0 for short month
    L is the leap month of the year if 1<=L<=12; NO leap month if L = 0.
    The leap month (if exists) is long one iff M = 1.
    */
    0x04bd8,                                       /* 1900 */
    0x04ae0, 0x0a570, 0x054d5, 0x0d260, 0x0d950,   /* 1905 */
    0x16554, 0x056a0, 0x09ad0, 0x055d2, 0x04ae0,   /* 1910 */
    0x0a5b6, 0x0a4d0, 0x0d250, 0x1d255, 0x0b540,   /* 1915 */
    0x0d6a0, 0x0ada2, 0x095b0, 0x14977, 0x04970,   /* 1920 */
    0x0a4b0, 0x0b4b5, 0x06a50, 0x06d40, 0x1ab54,   /* 1925 */
    0x02b60, 0x09570, 0x052f2, 0x04970, 0x06566,   /* 1930 */
    0x0d4a0, 0x0ea50, 0x06e95, 0x05ad0, 0x02b60,   /* 1935 */
    0x186e3, 0x092e0, 0x1c8d7, 0x0c950, 0x0d4a0,   /* 1940 */
    0x1d8a6, 0x0b550, 0x056a0, 0x1a5b4, 0x025d0,   /* 1945 */
    0x092d0, 0x0d2b2, 0x0a950, 0x0b557, 0x06ca0,   /* 1950 */
    0x0b550, 0x15355, 0x04da0, 0x0a5d0, 0x14573,   /* 1955 */
    0x052d0, 0x0a9a8, 0x0e950, 0x06aa0, 0x0aea6,   /* 1960 */
    0x0ab50, 0x04b60, 0x0aae4, 0x0a570, 0x05260,   /* 1965 */
    0x0f263, 0x0d950, 0x05b57, 0x056a0, 0x096d0,   /* 1970 */
    0x04dd5, 0x04ad0, 0x0a4d0, 0x0d4d4, 0x0d250,   /* 1975 */
    0x0d558, 0x0b540, 0x0b5a0, 0x195a6, 0x095b0,   /* 1980 */
    0x049b0, 0x0a974, 0x0a4b0, 0x0b27a, 0x06a50,   /* 1985 */
    0x06d40, 0x0af46, 0x0ab60, 0x09570, 0x04af5,   /* 1990 */
    0x04970, 0x064b0, 0x074a3, 0x0ea50, 0x06b58,   /* 1995 */
    0x05ac0, 0x0ab60, 0x096d5, 0x092e0, 0x0c960,   /* 2000 */
    0x0d954, 0x0d4a0, 0x0da50, 0x07552, 0x056a0,   /* 2005 */
    0x0abb7, 0x025d0, 0x092d0, 0x0cab5, 0x0a950,   /* 2010 */
    0x0b4a0, 0x0baa4, 0x0ad50, 0x055d9, 0x04ba0,   /* 2015 */
    0x0a5b0, 0x15176, 0x052b0, 0x0a930, 0x07954,   /* 2020 */
    0x06aa0, 0x0ad50, 0x05b52, 0x04b60, 0x0a6e6,   /* 2025 */
    0x0a4e0, 0x0d260, 0x0ea65, 0x0d530, 0x05aa0,   /* 2030 */
    0x076a3, 0x096d0, 0x04afb, 0x04ad0, 0x0a4d0,   /* 2035 */
    0x1d0b6, 0x0d250, 0x0d520, 0x0dd45, 0x0b5a0,   /* 2040 */
    0x056d0, 0x055b2, 0x049b0, 0x0a577, 0x0a4b0,   /* 2045 */
    0x0aa50, 0x1b255, 0x06d20, 0x0ada0             /* 2049 */
];

fn year_info_to_year_day(year_info: u32) -> u32 {
    let mut res: u32 = 29 * 12;
    let mut leap = false;
    if year_info % 16 != 0 {
        leap = true;
        res += 29;
    }
    let mut year_info = year_info / 16;
    let inc = if leap { 1 } else { 0 };
    for _ in 0..(12 + inc) {
        if year_info % 2 == 1 {
            res += 1;
        }
        year_info = year_info / 2;
    }
    res
}

fn enum_month(year: u32) -> Vec<(u32, u32, bool)> {
    let mut months: Vec<(u32, bool)> = (1..13).map(|x| (x, false)).collect();
    let leap_month = year % 16;
    if leap_month == 0 {

    } else if leap_month <= 12 {
        months.insert(leap_month as usize, (leap_month, true));
    } else {
        // FIXME: return error
    }
    let mut ret = Vec::with_capacity(months.len());
    for (month, is_leap_month) in months {
        let days = if is_leap_month {
            (year >> 16) % 2 + 29
        } else {
            (year >> (16 - month)) % 2 + 29
        };
        ret.push((month, days, is_leap_month));
    }
    ret
}

fn calc_month_day(year: u32, offset: u32) -> (u32, u32, bool) {
    let mut month = 0;
    let mut is_leap_month = false;
    let mut offset = offset;
    for (month_, days, leap_month) in enum_month(year).into_iter() {
        month = month_;
        is_leap_month = leap_month;
        if offset < days {
            break;
        }
        offset -= days;
    }
    (month, offset + 1, is_leap_month)
}

fn calc_days(year_info: u32, month: u32, day: u32, is_leap_month: bool) -> u32 {
    let mut res = 0;
    for (_month, _days, leap_month) in enum_month(year_info) {
        if _month == month && is_leap_month == leap_month {
            if day >= 1 && day <= _days {
                res += day - 1;
                return res;
            } else {
                // FIXME: handle error day out of range
            }
        }
        res += _days;
    }
    res
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LunarDate {
    year: i32,
    month: u32,
    day: u32,
    is_leap_month: bool,
}

impl LunarDate {

    #[inline]
    pub fn new(year: i32, month: u32, day: u32, is_leap_month: bool) -> Self {
        Self {
            year,
            month,
            day,
            is_leap_month,
        }
    }

    pub fn from_solar_date(year: i32, month: u32, day: u32) -> Self {
        let solar_date = NaiveDate::from_ymd(year, month, day);
        Self::from_naive_date(&solar_date)
    }

    #[inline]
    pub fn from_naive_date(date: &NaiveDate) -> Self {
        let offset = date.signed_duration_since(*START_DATE).num_days();
        Self::from_offset(offset as u32)
    }

    #[inline]
    pub fn year(&self) -> i32 {
        self.year
    }

    #[inline]
    pub fn month(&self) -> u32 {
        self.month
    }

    #[inline]
    pub fn day(&self) -> u32 {
        self.day
    }

    #[inline]
    pub fn is_leap_month(&self) -> bool {
        self.is_leap_month
    }

    pub fn to_solar_date(&self) -> NaiveDate {
        let mut offset = 0;
        if self.year < 1900 || self.year >= 2050 {
            // FIXME: handle error
        }
        let year_index = self.year as usize - 1900;
        for i in 0..year_index {
            offset += YEAR_INFOS[i];
        }
        offset += calc_days(YEAR_INFOS[year_index], self.month, self.day, self.is_leap_month);
        *START_DATE + Duration::days(offset as i64)
    }

    #[inline]
    pub fn today() -> Self {
        let date = Local::today();
        Self::from_solar_date(date.year(), date.month(), date.day())
    }

    fn from_offset(offset: u32) -> Self {
        let mut offset = offset;
        let mut index = 0;
        for (idx, year_day) in YEAR_DAYS.iter().enumerate() {
            index = idx;
            if offset < *year_day {
                break;
            }
            offset -= *year_day;
        }
        let year = 1900 + index;
        let year_info = YEAR_INFOS[index];
        let (month, day, is_leap_month) = calc_month_day(year_info, offset);
        LunarDate {
            year: year as i32,
            month,
            day,
            is_leap_month,
        }
    }
}

impl Add<Duration> for LunarDate {
    type Output = LunarDate;

    #[inline]
    fn add(self, rhs: Duration) -> Self::Output {
        let date = self.to_solar_date() + rhs;
        LunarDate::from_naive_date(&date)
    }
}

impl Add<time::Duration> for LunarDate {
    type Output = LunarDate;

    #[inline]
    fn add(self, rhs: time::Duration) -> Self::Output {
        self + Duration::from_std(rhs).unwrap()
    }
}

impl Sub<Duration> for LunarDate {
    type Output = LunarDate;

    #[inline]
    fn sub(self, rhs: Duration) -> Self::Output {
        let date = self.to_solar_date() - rhs;
        LunarDate::from_naive_date(&date)
    }
}

impl Sub<time::Duration> for LunarDate {
    type Output = LunarDate;

    #[inline]
    fn sub(self, rhs: time::Duration) -> Self::Output {
        self + Duration::from_std(rhs).unwrap()
    }
}

impl Sub<LunarDate> for LunarDate {
    type Output = Duration;

    #[inline]
    fn sub(self, rhs: LunarDate) -> Self::Output {
        self.to_solar_date() - rhs.to_solar_date()
    }
}

impl Sub<NaiveDate> for LunarDate {
    type Output = Duration;

    #[inline]
    fn sub(self, rhs: NaiveDate) -> Self::Output {
        self.to_solar_date() - rhs
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
