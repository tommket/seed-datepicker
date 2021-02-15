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
}
