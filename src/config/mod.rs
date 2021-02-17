pub mod date_constraints;

use crate::{year_month::YearMonth, DialogViewType};
use chrono::prelude::*;

use mockall_double::double;

#[double]
use self::date_constraints::DateConstraints;

/// Configuration for the datepicker.
#[derive(Default, Debug, Builder, Getters)]
#[builder(setter(strip_option))]
#[builder(default)]
#[builder(build_fn(validate = "Self::validate"))]
pub struct PickerConfig {
    /// possible constraints to prevent the user from selecting some dates
    #[getter(skip)]
    date_constraints: DateConstraints,

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
        if self.initial_view_type > self.selection_type {
            return Err("initial_view_type can have at most selection_type scale".into());
        }
        match (self.initial_date, &self.date_constraints) {
            (Some(Some(initial_date)), Some(date_constraints)) => {
                if date_constraints.is_day_forbidden(&initial_date) {
                    return Err(format!(
                        "The initial_date {:?} is forbidden by the date_constraints.",
                        initial_date
                    ));
                }
            }
            (_, _) => {}
        }
        Ok(())
    }
}

impl PickerConfig {
    pub fn is_day_forbidden(&self, date: &NaiveDate) -> bool {
        self.date_constraints.is_day_forbidden(date)
    }

    pub fn is_month_forbidden(&self, year_month_info: &YearMonth) -> bool {
        self.date_constraints.is_month_forbidden(year_month_info)
    }

    pub fn is_year_forbidden(&self, year: i32) -> bool {
        self.date_constraints.is_year_forbidden(year)
    }

    pub fn is_year_group_forbidden(&self, year: i32) -> bool {
        self.date_constraints.is_year_group_forbidden(year)
    }

    pub fn guess_allowed_year_month(&self) -> YearMonth {
        if let Some(init_date) = self.initial_date {
            return init_date.into();
        }
        // if none of the above constraints matched use the current_date
        let current_date = Local::now().date().naive_local();
        current_date.into()
    }
}

#[cfg(test)]
mod tests {
    use super::date_constraints::MockDateConstraints;
    use super::*;
    use crate::DialogViewType;

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

    #[test]
    fn picker_config_initial_date_forbidden() {
        let mut date_constraints_mock = MockDateConstraints::new();
        date_constraints_mock
            .expect_is_day_forbidden()
            .returning(|_| true);
        let config = PickerConfigBuilder::default()
            .initial_date(NaiveDate::from_ymd(2020, 1, 1))
            .date_constraints(date_constraints_mock)
            .build();
        assert!(config.is_err());
        assert_eq!(
            config.err(),
            Some("The initial_date 2020-01-01 is forbidden by the date_constraints.".into())
        );
    }
}
