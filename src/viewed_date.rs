use std::ops::RangeInclusive;

use chrono::{Datelike, NaiveDate};

use crate::dialog_view_type::DialogViewType;

pub const YEARS_IN_YEAR_SELECTION: i32 = 20;

pub type YearNumber = i32;
pub type MonthNumber = u32;
pub type DayNumber = u32;

/// Trait used for the variable that describes the currently viewed datepicker.
pub trait ViewedDate {
    /// returns a date with the first day of the previous month
    fn previous_month(&self) -> NaiveDate;

    /// returns a date with the first day of the next month
    fn next_month(&self) -> NaiveDate;

    /// returns a date with the first day of the previous year
    fn previous_year(&self) -> NaiveDate;

    /// returns a date with the first day of the next year
    fn next_year(&self) -> NaiveDate;

    /// returns a date with the first day of the last year of the previous year group
    fn previous_year_group(&self) -> NaiveDate;

    /// returns a date with the first day of the first year of the next year group
    fn next_year_group(&self) -> NaiveDate;

    /// returns a date with the first day of the currently set month
    fn first_day_of_month(&self) -> NaiveDate;

    /// returns true if the currently `ViewedDate` with the given `DialogViewType` includes the given date
    fn contains(&self, dialog_view_type: &DialogViewType, date: &NaiveDate) -> bool;
}

impl ViewedDate for NaiveDate {
    fn previous_month(&self) -> NaiveDate {
        let mut year = self.year();
        let mut month = self.month();
        if month == 1 {
            month = 12;
            year -= 1;
        } else {
            month -= 1;
        }
        NaiveDate::from_ymd(year, month, 1)
    }

    fn next_month(&self) -> NaiveDate {
        let mut year = self.year();
        let mut month = self.month();
        if month == 12 {
            month = 1;
            year += 1;
        } else {
            month += 1;
        }
        NaiveDate::from_ymd(year, month, 1)
    }

    fn previous_year(&self) -> NaiveDate {
        NaiveDate::from_ymd(self.year() - 1, 1, 1)
    }

    fn next_year(&self) -> NaiveDate {
        NaiveDate::from_ymd(self.year() + 1, 1, 1)
    }

    fn previous_year_group(&self) -> NaiveDate {
        NaiveDate::from_ymd(year_group_start(self.year()) - 1, 1, 1)
    }

    fn next_year_group(&self) -> NaiveDate {
        NaiveDate::from_ymd(year_group_end(self.year()) + 1, 1, 1)
    }

    fn first_day_of_month(&self) -> NaiveDate {
        NaiveDate::from_ymd(self.year(), self.month(), 1)
    }

    fn contains(&self, dialog_view_type: &DialogViewType, date: &NaiveDate) -> bool {
        match dialog_view_type {
            DialogViewType::Years => self.year() == date.year(),
            DialogViewType::Months => self.year() == date.year() && self.month() == date.month(),
            DialogViewType::Days => self == date,
        }
    }
}

pub fn year_group_start(year: YearNumber) -> YearNumber {
    year - (year % YEARS_IN_YEAR_SELECTION)
}

pub fn year_group_end(year: YearNumber) -> YearNumber {
    year_group_start(year) + (YEARS_IN_YEAR_SELECTION - 1)
}

pub fn year_group_range(year: YearNumber) -> RangeInclusive<YearNumber> {
    year_group_start(year)..=year_group_end(year)
}

#[cfg(test)]
mod tests {
    use crate::rstest_utils::create_date;
    use rstest::*;

    use super::*;

    #[rstest(
        expected, given, //
        case::from_january(create_date(1989, 12, 1), create_date(1990, 1, 15)),
        case::not_from_january(create_date(1990, 2, 1), create_date(1990, 3, 22)),
    )]
    fn previous_month(expected: NaiveDate, given: NaiveDate) {
        assert_eq!(expected, given.previous_month());
    }

    #[rstest(
        expected, given, //
        case::from_december(create_date(1991, 1, 1), create_date(1990, 12, 22)),
        case::not_from_december(create_date(1990, 4, 1), create_date(1990, 3, 15)),
    )]
    fn next_month(expected: NaiveDate, given: NaiveDate) {
        assert_eq!(expected, given.next_month());
    }

    #[rstest(
        expected, given, //
        case(create_date(1989, 1, 1), create_date(1990, 12, 25)),
        case(create_date(1990, 1, 1), create_date(1991, 3, 22)),
    )]
    fn previous_year(expected: NaiveDate, given: NaiveDate) {
        assert_eq!(expected, given.previous_year());
    }

    #[rstest(
        expected, given, //
        case(create_date(1991, 1, 1), create_date(1990, 12, 25)),
        case(create_date(1992, 1, 1), create_date(1991, 3, 22)),
    )]
    fn next_year(expected: NaiveDate, given: NaiveDate) {
        assert_eq!(expected, given.next_year());
    }

    #[rstest(
        expected, given, //
        case::in_middle(create_date(1979, 1, 1), create_date(1990, 1, 1)),
        case::at_start(create_date(1979, 1, 1), create_date(1980, 3, 20)),
        case::at_end(create_date(1979, 1, 1), create_date(1999, 7, 24)),
        case::next_group(create_date(1999, 1, 1), create_date(2000, 8, 22)),
    )]
    fn previous_year_group(expected: NaiveDate, given: NaiveDate) {
        assert_eq!(expected, given.previous_year_group());
    }

    #[rstest(
        expected, given, //
        case::in_middle(create_date(2000, 1, 1), create_date(1990, 1, 1)),
        case::at_start(create_date(2000, 1, 1), create_date(1980, 3, 20)),
        case::at_end(create_date(2000, 1, 1), create_date(1999, 7, 24)),
        case::next_group(create_date(2020, 1, 1), create_date(2000, 8, 22)),
    )]
    fn next_year_group(expected: NaiveDate, given: NaiveDate) {
        assert_eq!(expected, given.next_year_group());
    }

    #[rstest(
        expected, given, //
        case(create_date(1990, 12, 1), create_date(1990, 12, 15)),
        case(create_date(1991, 3, 1), create_date(1991, 3, 24)),
    )]
    fn first_day_of_month(expected: NaiveDate, given: NaiveDate) {
        assert_eq!(expected, given.first_day_of_month());
    }

    #[rstest(
        expected, viewed_date, dialog_view_type, tested_date, //
        case::years_different(false, create_date(1990, 1, 1), DialogViewType::Years, create_date(1989, 1, 1)),
        case::years_equal(true, create_date(1990, 1, 1), DialogViewType::Years, create_date(1990, 5, 15)),

        case::months_different_year(false, create_date(1990, 3, 1), DialogViewType::Months, create_date(1989, 3, 1)),
        case::months_different_month(false, create_date(1990, 3, 1), DialogViewType::Months, create_date(1990, 4, 1)),
        case::months_equal(true, create_date(1990, 3, 1), DialogViewType::Months, create_date(1990, 3, 15)),

        case::days_different_year(false, create_date(1990, 3, 1), DialogViewType::Days, create_date(1989, 3, 1)),
        case::days_different_month(false, create_date(1990, 3, 1), DialogViewType::Days, create_date(1990, 4, 1)),
        case::days_different_day(false, create_date(1990, 3, 1), DialogViewType::Days, create_date(1990, 3, 15)),
        case::months_equal(true, create_date(1990, 3, 1), DialogViewType::Months, create_date(1990, 3, 15)),
    )]
    fn contains(
        expected: bool,
        viewed_date: NaiveDate,
        dialog_view_type: DialogViewType,
        tested_date: NaiveDate,
    ) {
        assert_eq!(
            expected,
            viewed_date.contains(&dialog_view_type, &tested_date)
        );
    }

    #[rstest(
        expected, input, //
        case::at_zero(0, 0),
        case::in_middle(1980, 1990),
        case::at_start(1980, 1980),
        case::at_end(1980, 1999),
        case::after_end(2000, 2000)
    )]
    fn test_year_group_start(expected: YearNumber, input: YearNumber) {
        assert_eq!(expected, year_group_start(input));
    }

    #[rstest(
        expected, input, //
        case::at_zero(19, 0),
        case::in_middle(1999, 1990),
        case::at_start(1999, 1980),
        case::at_end(1999, 1999),
        case::after_end(2019, 2000)
    )]
    fn test_year_group_end(expected: YearNumber, input: YearNumber) {
        assert_eq!(expected, year_group_end(input));
    }

    #[rstest(
        expected, input, //
        case::at_zero(0..=19, 0),
        case::in_middle(1980..=1999, 1990),
        case::at_start(1980..=1999, 1980),
        case::at_end(1980..=1999, 1999),
        case::after_end(2000..=2019, 2000)
    )]
    fn test_year_group_range(expected: RangeInclusive<YearNumber>, input: YearNumber) {
        assert_eq!(expected, year_group_range(input));
    }
}
