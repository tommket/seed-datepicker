use std::ops::RangeInclusive;

use chrono::{Datelike, Month, NaiveDate};
use num_traits::FromPrimitive;

pub const YEARS_IN_YEAR_SELECTION: i32 = 20;

/// Internal representation of viewed Year & Month
#[derive(Clone)]
pub struct YearMonth {
    pub year: i32,
    pub month: Month,
}

impl From<NaiveDate> for YearMonth {
    fn from(date: NaiveDate) -> Self {
        YearMonth {
            year: date.year(),
            month: Month::from_u32(date.month()).unwrap(),
        }
    }
}

impl YearMonth {
    pub fn previous_month(&self) -> YearMonth {
        YearMonth {
            year: if self.month == Month::January {
                self.year - 1
            } else {
                self.year
            },
            month: self.month.pred(),
        }
    }

    pub fn next_month(&self) -> YearMonth {
        YearMonth {
            year: if self.month == Month::December {
                self.year + 1
            } else {
                self.year
            },
            month: self.month.succ(),
        }
    }

    pub fn first_day_of_month(&self) -> NaiveDate {
        NaiveDate::from_ymd(self.year, self.month.number_from_month(), 1)
    }

    pub fn contains(&self, date: &NaiveDate) -> bool {
        self.year == date.year() && self.month.number_from_month() == date.month()
    }

    pub fn previous_year(&self) -> YearMonth {
        YearMonth {
            year: self.year - 1,
            month: self.month,
        }
    }

    pub fn next_year(&self) -> YearMonth {
        YearMonth {
            year: self.year + 1,
            month: self.month,
        }
    }

    pub fn previous_year_group(&self) -> YearMonth {
        YearMonth {
            year: year_group_start(self.year) - 1,
            month: self.month,
        }
    }

    pub fn next_year_group(&self) -> YearMonth {
        YearMonth {
            year: year_group_end(self.year) + 1,
            month: self.month,
        }
    }
}

pub fn year_group_start(year: i32) -> i32 {
    year - (year % YEARS_IN_YEAR_SELECTION)
}

pub fn year_group_end(year: i32) -> i32 {
    year_group_start(year) + (YEARS_IN_YEAR_SELECTION - 1)
}

pub fn year_group_range(year: i32) -> RangeInclusive<i32> {
    year_group_start(year)..=year_group_end(year)
}

#[cfg(test)]
mod tests {
    use chrono::{Datelike, Month, NaiveDate};
    use num_traits::FromPrimitive;
    use proptest::prelude::*;

    use super::YearMonth;

    proptest! {
        #[test]
        fn from_naive_date_proptest(day in 1..365*5000i32) {
            let date = NaiveDate::from_num_days_from_ce(day);
            let year_month: YearMonth = date.into();
            assert_eq!(date.year(), year_month.year);
            assert_eq!(Month::from_u32(date.month()).unwrap(), year_month.month);
        }
    }

    proptest! {
        #[test]
        fn previous_month_from_january(year_given in 1..5000i32) {
            let given = YearMonth {
                year: year_given,
                month: Month::January,
            };

            let previous_month = given.previous_month();

            assert_eq!(Month::December, previous_month.month);
            assert_eq!(year_given - 1, previous_month.year);
        }
    }

    proptest! {
        #[test]
        fn previous_month_not_from_january(month_num in 2..=12u32, year_given in 1..5000i32) {
            let given = YearMonth {
                year: year_given,
                month: Month::from_u32(month_num).unwrap(),
            };

            let previous_month = given.previous_month();

            assert_eq!(Month::from_u32(month_num - 1).unwrap(), previous_month.month);
            assert_eq!(year_given, previous_month.year);
        }
    }

    proptest! {
        #[test]
        fn next_month_from_december(year_given in 1..5000i32) {
            let given = YearMonth {
                year: year_given,
                month: Month::December,
            };

            let next_month = given.next_month();

            assert_eq!(Month::January, next_month.month);
            assert_eq!(year_given + 1, next_month.year);
        }
    }

    proptest! {
        #[test]
        fn next_month_not_from_december(month_num in 1..=11u32, year_given in 1..5000i32) {
            let given = YearMonth {
                year: year_given,
                month: Month::from_u32(month_num).unwrap(),
            };

            let next_month = given.next_month();

            assert_eq!(Month::from_u32(month_num + 1).unwrap(), next_month.month);
            assert_eq!(year_given, next_month.year);
        }
    }

    proptest! {
        #[test]
        fn previous_year(month_num in 1..=12u32, year_given in 1..5000i32) {
            let given = YearMonth {
                year: year_given,
                month: Month::from_u32(month_num).unwrap(),
            };

            let previous_year = given.previous_year();

            assert_eq!(given.month, previous_year.month);
            assert_eq!(year_given - 1, previous_year.year);
        }
    }

    proptest! {
        #[test]
        fn next_year(month_num in 1..=12u32, year_given in 1..5000i32) {
            let given = YearMonth {
                year: year_given,
                month: Month::from_u32(month_num).unwrap(),
            };

            let next_year = given.next_year();

            assert_eq!(given.month, next_year.month);
            assert_eq!(year_given + 1, next_year.year);
        }
    }

    proptest! {
        #[test]
        fn contains_naive_date_true(given_year in 1..5000i32, given_month in 1..=12u32, given_day in 1..=28u32) {
            let given_date = NaiveDate::from_ymd(given_year, given_month, given_day);
            let given_year_month = YearMonth {
                year: given_year,
                month: Month::from_u32(given_month).unwrap(),
            };

            assert!(given_year_month.contains(&given_date));
        }
    }

    proptest! {
        #[test]
        fn contains_naive_date_false(day in 1..365*5000i32, day_difference in 32..1000i32, difference_sign: bool) {
            let given_date = NaiveDate::from_num_days_from_ce(day);
            let given_day_difference = (if difference_sign { 1 } else { -1 }) * day_difference;
            let given_different_date = NaiveDate::from_num_days_from_ce(day + given_day_difference);
            let given_year_month = YearMonth {
                year: given_different_date.year(),
                month: Month::from_u32(given_different_date.month()).unwrap(),
            };

            assert!(!given_year_month.contains(&given_date));
        }
    }
}
