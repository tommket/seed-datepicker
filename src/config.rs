use std::collections::HashSet;

use chrono::{Datelike, Local, Month, NaiveDate, Weekday};
use num_traits::FromPrimitive;

use crate::{year_group_range, DialogViewType, YearMonth};

/// Configuration for the datepicker.
#[derive(Default, Builder, Getters)]
#[builder(setter(strip_option))]
#[builder(default)]
#[builder(build_fn(validate = "Self::validate"))]
pub struct PickerConfig {
    /// inclusive minimal date constraint
    /// the earliest date that can be selected
    min_date: Option<NaiveDate>,

    /// inclusive maximal date constraint
    /// the latest date that can be selected
    max_date: Option<NaiveDate>,

    /// disabled weekdays, that should not be selectable
    disabled_weekdays: HashSet<Weekday>,

    /// entire completely disabled months
    disabled_months: HashSet<Month>,

    /// entire completely disabled years
    disabled_years: HashSet<i32>,

    /// disabled monthly periodically repeating dates, so it is just a day number
    /// starting from 1 for the first day of the month
    /// if unique dates in a certain year should not be selectable use `disabled_unique_dates`
    disabled_monthly_dates: HashSet<u32>,

    /// disabled yearly periodically repeating dates that should not be selectable,
    /// if unique dates in a certain year should not be selectable use `disabled_unique_dates`
    /// it is a `Vec` since we need to iterate over it anyway, since we hae no MonthDay type
    disabled_yearly_dates: Vec<NaiveDate>,

    /// disabled unique dates with a specific year, month and day that should not be selectable,
    /// if some periodically repeated dates should not be selectable use the correct option
    disabled_unique_dates: HashSet<NaiveDate>,

    /// initializes the datepicker to this value
    initial_date: Option<NaiveDate>,

    /// initializes the view type to this value
    initial_view_type: DialogViewType,

    /// selection type, to make it possible to select for example only a year, or only a month.
    selection_type: DialogViewType,

    /// whether the dialog should be immediatelly opened after initalization
    initially_opened: bool,

    /// chrono formatting string for the title of the month
    #[builder(default = "String::from(\"%b %Y\")", setter(into))]
    month_title_format: String,
}

impl PickerConfigBuilder {
    fn validate(&self) -> Result<(), String> {
        match (self.min_date, self.max_date) {
            (Some(min_date), Some(max_date)) => {
                if min_date > max_date {
                    return Err("min_date must be earlier or exactly at max_date".into());
                }
            }
            (_, _) => {}
        }
        if self.initial_view_type > self.selection_type {
            return Err("initial_view_type can have at most selection_type scale".into());
        }
        // TODO: check that the initial_date is not forbidden
        Ok(())
    }
}

impl PickerConfig {
    pub fn is_day_forbidden(&self, date: &NaiveDate) -> bool {
        self.min_date.map_or(false, |min_date| &min_date > date)
            || self.max_date.map_or(false, |max_date| &max_date < date)
            || self.disabled_weekdays.contains(&date.weekday())
            || self
                .disabled_months
                .contains(&Month::from_u32(date.month()).unwrap())
            || self.disabled_years.contains(&date.year())
            || self.disabled_unique_dates.contains(&date)
            || self.disabled_monthly_dates.contains(&date.day())
            || self
                .disabled_yearly_dates
                .iter()
                .any(|disabled| disabled.day() == date.day() && disabled.month() == date.month())
    }

    pub fn is_month_forbidden(&self, year_month_info: &YearMonth) -> bool {
        year_month_info
            .first_day_of_month()
            .iter_days()
            .take_while(|date| date.month() == year_month_info.month.number_from_month())
            .all(|date| self.is_day_forbidden(&date))
    }

    pub fn is_year_forbidden(&self, year: i32) -> bool {
        (Month::January.number_from_month()..=Month::December.number_from_month()).all(|month| {
            self.is_month_forbidden(&YearMonth {
                year,
                month: Month::from_u32(month).unwrap(),
            })
        })
    }

    pub fn is_year_group_forbidden(&self, year: i32) -> bool {
        year_group_range(year).all(|year| self.is_year_forbidden(year))
    }

    pub fn guess_allowed_year_month(&self) -> YearMonth {
        if let Some(init_date) = self.initial_date {
            return YearMonth {
                year: init_date.year(),
                month: Month::from_u32(init_date.month()).unwrap(),
            };
        }
        // if there were no other constraints use the current_date
        let current_date = Local::now().date().naive_local();
        YearMonth {
            year: current_date.year(),
            month: Month::from_u32(current_date.month()).unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{year_month::YearMonth, DialogViewType};

    use super::PickerConfig;
    use super::PickerConfigBuilder;
    use chrono::{Duration, Month, NaiveDate, Weekday};
    use num_traits::FromPrimitive;
    use proptest::prelude::*;

    #[test]
    fn picker_config_min_date_greater_than_max_date() {
        let date = NaiveDate::from_ymd(2020, 10, 15);
        let config = PickerConfigBuilder::default()
            .min_date(date.clone())
            .max_date(date.clone() - Duration::days(1))
            .build();
        assert!(config.is_err());
        assert_eq!(
            config.err(),
            Some("min_date must be earlier or exactly at max_date".into())
        );
    }

    #[test]
    fn picker_config_min_date_equals_max_date() {
        let date = NaiveDate::from_ymd(2020, 10, 15);
        let config = PickerConfigBuilder::default()
            .min_date(date.clone())
            .max_date(date.clone())
            .build();
        assert!(config.is_ok());
    }

    #[test]
    fn picker_config_initial_view_type_greater_than_selection_type() {
        let config = PickerConfigBuilder::default()
            .initial_view_type(DialogViewType::Days)
            .selection_type(DialogViewType::Months)
            .build();
        assert!(config.is_err());
        assert_eq!(
            config.err(),
            Some("initial_view_type can have at most selection_type scale".into())
        );
    }

    #[test]
    fn picker_config_initial_view_type_equal_to_selection_type() {
        let config = PickerConfigBuilder::default()
            .initial_view_type(DialogViewType::Months)
            .selection_type(DialogViewType::Months)
            .build();
        assert!(config.is_ok());
    }

    #[test]
    fn picker_config_initial_view_type_smaller_than_selection_type() {
        let config = PickerConfigBuilder::default()
            .initial_view_type(DialogViewType::Years)
            .selection_type(DialogViewType::Months)
            .build();
        assert!(config.is_ok());
    }

    proptest! {
        #[test]
        fn is_day_forbidden_default_no_bounds(day in 1..365*5000i32) {
            let date = NaiveDate::from_num_days_from_ce(day);
            assert!(!PickerConfig::default().is_day_forbidden(&date))
        }
    }

    proptest! {
        #[test]
        fn is_month_forbidden_default_no_bounds(year in 1..5000i32, month_num in 1..=12u32) {
            let month = Month::from_u32(month_num).unwrap();
            let year_month_info = YearMonth {
                year,
                month,
            };
            assert!(!PickerConfig::default().is_month_forbidden(&year_month_info))
        }
    }

    proptest! {
        #[test]
        fn is_year_forbidden_default_no_bounds(year in 1..5000i32) {
            assert!(!PickerConfig::default().is_year_forbidden(year))
        }
    }

    #[test]
    fn is_day_forbidden_at_min_date_allowed() {
        let date = NaiveDate::from_ymd(2020, 10, 15);
        let config = PickerConfigBuilder::default()
            .min_date(date.clone())
            .build()
            .unwrap();
        assert!(!config.is_day_forbidden(&date))
    }

    #[test]
    fn is_day_forbidden_before_min_date_not_allowed() {
        let date = NaiveDate::from_ymd(2020, 10, 15);
        let config = PickerConfigBuilder::default()
            .min_date(date.clone())
            .build()
            .unwrap();
        assert!(config.is_day_forbidden(&(date - Duration::days(1))))
    }

    #[test]
    fn is_day_forbidden_at_max_date_allowed() {
        let date = NaiveDate::from_ymd(2020, 10, 15);
        let config = PickerConfigBuilder::default()
            .max_date(date.clone())
            .build()
            .unwrap();
        assert!(!config.is_day_forbidden(&date))
    }

    #[test]
    fn is_day_forbidden_after_max_date_not_allowed() {
        let date = NaiveDate::from_ymd(2020, 10, 15);
        let config = PickerConfigBuilder::default()
            .max_date(date.clone())
            .build()
            .unwrap();
        assert!(config.is_day_forbidden(&(date + Duration::days(1))))
    }

    proptest! {
        #[test]
        fn is_day_forbidden_disabled_weekday_not_allowed(weekday in 0..7u8, year in 1..5000i32, iso_week in 1..52u32) {
            let disabled_weekday = Weekday::from_u8(weekday).unwrap();
            let date = NaiveDate::from_isoywd(year, iso_week, disabled_weekday);
            let config = PickerConfigBuilder::default()
                .disabled_weekdays([disabled_weekday].iter().cloned().collect())
                .build()
                .unwrap();
            assert!(config.is_day_forbidden(&date));
        }
    }

    proptest! {
        #[test]
        fn is_day_forbidden_disabled_month_not_allowed(month_num in 1..=12u32, year in 1..5000i32, day in 1..=28u32) {
            let config = PickerConfigBuilder::default()
                .disabled_months([Month::from_u32(month_num).unwrap()].iter().cloned().collect())
                .build()
                .unwrap();
            assert!(config.is_day_forbidden(&NaiveDate::from_ymd(year, month_num, day)))
        }
    }

    proptest! {
        #[test]
        fn is_day_forbidden_disabled_year_not_allowed(month_num in 1..=12u32, year in 1..5000i32, day in 1..=28u32) {
            let config = PickerConfigBuilder::default()
                .disabled_years([year].iter().cloned().collect())
                .build()
                .unwrap();
            assert!(config.is_day_forbidden(&NaiveDate::from_ymd(year, month_num, day)))
        }
    }

    #[test]
    fn is_day_forbidden_disabled_unique_dates_not_allowed() {
        let date = NaiveDate::from_ymd(2020, 1, 16);
        let config = PickerConfigBuilder::default()
            .disabled_unique_dates([date].iter().cloned().collect())
            .build()
            .unwrap();
        assert!(config.is_day_forbidden(&date))
    }

    #[test]
    fn is_day_forbidden_disabled_unique_dates_after_a_year_allowed() {
        let date = NaiveDate::from_ymd(2020, 1, 16);
        let config = PickerConfigBuilder::default()
            .disabled_unique_dates([date].iter().cloned().collect())
            .build()
            .unwrap();
        assert!(!config.is_day_forbidden(&NaiveDate::from_ymd(2021, 1, 16)))
    }

    proptest! {
        #[test]
        fn is_day_forbidden_disabled_yearly_dates_not_allowed(year_in_disabled in 1..5000i32, year_in_input in 1..5000i32, month in 1..=12u32, day in 1..=28u32) {
            let disabled_yearly_date = NaiveDate::from_ymd(year_in_disabled, month, day);
            let config = PickerConfigBuilder::default()
                .disabled_yearly_dates(vec![disabled_yearly_date])
                .build()
                .unwrap();
            assert!(config.is_day_forbidden(&NaiveDate::from_ymd(year_in_input, month, day)))
        }
    }

    proptest! {
        #[test]
        fn is_day_forbidden_disabled_monthly_dates_not_allowed(year in 1..5000i32, month in 1..=12u32, day in 1..=28u32) {
            let config = PickerConfigBuilder::default()
                .disabled_monthly_dates([day].iter().cloned().collect())
                .build()
                .unwrap();
            assert!(config.is_day_forbidden(&NaiveDate::from_ymd(year, month, day)))
        }
    }
}
