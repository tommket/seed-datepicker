use crate::{
    config::date_constraints::HasDateConstraints,
    dialog_view_type::DialogViewType,
    year_month::{year_group_end, year_group_start, YearMonth},
};

/// Creates the text that should be the title of the datepicker dialog.
pub fn create_dialog_title_text(
    dialog_view_type: &DialogViewType,
    year_month: &YearMonth,
    month_title_format: &str,
) -> String {
    match dialog_view_type {
        DialogViewType::Days => year_month
            .first_day_of_month()
            .format(month_title_format)
            .to_string(),
        DialogViewType::Months => year_month.first_day_of_month().format("%Y").to_string(),
        DialogViewType::Years => format!(
            "{} - {}",
            year_group_start(year_month.year),
            year_group_end(year_month.year)
        ),
    }
}

/// Returns true if the "previous" button should be displayed.
pub fn should_display_previous_button(
    dialog_view_type: &DialogViewType,
    year_month: &YearMonth,
    config: &dyn HasDateConstraints,
) -> bool {
    match dialog_view_type {
        DialogViewType::Days => !config.is_month_forbidden(&year_month.previous_month()),
        DialogViewType::Months => !config.is_year_forbidden(year_month.year - 1),
        DialogViewType::Years => {
            !config.is_year_group_forbidden(year_group_start(year_month.year) - 1)
        }
    }
}

/// Returns true if the "next" button should be displayed.
pub fn should_display_next_button(
    dialog_view_type: &DialogViewType,
    year_month: &YearMonth,
    config: &dyn HasDateConstraints,
) -> bool {
    match dialog_view_type {
        DialogViewType::Days => !config.is_month_forbidden(&year_month.next_month()),
        DialogViewType::Months => !config.is_year_forbidden(year_month.year + 1),
        DialogViewType::Years => {
            !config.is_year_group_forbidden(year_group_end(year_month.year) + 1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::date_constraints::MockHasDateConstraints;
    use chrono::Month;

    use mockall::predicate;
    use rstest::*;

    #[fixture(year=1990, month=Month::January)]
    fn create_year_month(year: i32, month: Month) -> YearMonth {
        YearMonth { year, month }
    }

    #[rstest(
        expected, dialog_view_type, year_month, month_title_format, //
        case::days_default("Jan 1990", DialogViewType::Days, create_year_month(1990, Month::January), "%b %Y"),
        case::days_different_format("January 1990", DialogViewType::Days, create_year_month(1990, Month::January), "%B %Y"),
        case::months("1990", DialogViewType::Months, create_year_month(1990, Month::January), ""),
        case::years("1980 - 1999", DialogViewType::Years, create_year_month(1990, Month::January), ""),
    )]
    fn test_create_dialog_title_text(
        expected: &str,
        dialog_view_type: DialogViewType,
        year_month: YearMonth,
        month_title_format: &str,
    ) {
        assert_eq!(
            expected,
            create_dialog_title_text(&dialog_view_type, &year_month, month_title_format)
        );
    }

    #[fixture(year_month=create_year_month(1990, Month::January), retval=false)]
    fn month_forbidden(year_month: YearMonth, retval: bool) -> MockHasDateConstraints {
        let mut mock = MockHasDateConstraints::new();
        mock.expect_is_month_forbidden()
            .with(predicate::eq(year_month))
            .times(1)
            .returning(move |_| retval);
        mock
    }

    #[fixture(year = 1990, retval = false)]
    fn year_forbidden(year: i32, retval: bool) -> MockHasDateConstraints {
        let mut mock = MockHasDateConstraints::new();
        mock.expect_is_year_forbidden()
            .with(predicate::eq(year))
            .times(1)
            .returning(move |_| retval);
        mock
    }

    #[fixture(year = 1990, retval = false)]
    fn year_group_forbidden(year: i32, retval: bool) -> MockHasDateConstraints {
        let mut mock = MockHasDateConstraints::new();
        mock.expect_is_year_group_forbidden()
            .with(predicate::eq(year))
            .times(1)
            .returning(move |_| retval);
        mock
    }

    #[rstest(
        expected, dialog_view_type, year_month, mock_constraints, //
        case::month_forbidden(false, DialogViewType::Days, create_year_month(1990, Month::February), month_forbidden(create_year_month(1990, Month::January), true)),
        case::month_allowed(true, DialogViewType::Days, create_year_month(1990, Month::February), month_forbidden(create_year_month(1990, Month::January), false)),
        case::year_forbidden(false, DialogViewType::Months, create_year_month(1990, Month::February), year_forbidden(1989, true)),
        case::year_allowed(true, DialogViewType::Months, create_year_month(1990, Month::February), year_forbidden(1989, false)),
        case::year_group_forbidden(false, DialogViewType::Years, create_year_month(1990, Month::February), year_group_forbidden(1979, true)),
        case::year_group_allowed(true, DialogViewType::Years, create_year_month(1990, Month::February), year_group_forbidden(1979, false)),
    )]
    fn test_should_display_previous_button(
        expected: bool,
        dialog_view_type: DialogViewType,
        year_month: YearMonth,
        mock_constraints: MockHasDateConstraints,
    ) {
        assert_eq!(
            expected,
            should_display_previous_button(&dialog_view_type, &year_month, &mock_constraints)
        );
    }

    #[rstest(
        expected, dialog_view_type, year_month, mock_constraints, //
        case::month_forbidden(false, DialogViewType::Days, create_year_month(1990, Month::February), month_forbidden(create_year_month(1990, Month::March), true)),
        case::month_allowed(true, DialogViewType::Days, create_year_month(1990, Month::February), month_forbidden(create_year_month(1990, Month::March), false)),
        case::year_forbidden(false, DialogViewType::Months, create_year_month(1990, Month::February), year_forbidden(1991, true)),
        case::year_allowed(true, DialogViewType::Months, create_year_month(1990, Month::February), year_forbidden(1991, false)),
        case::year_group_forbidden(false, DialogViewType::Years, create_year_month(1990, Month::February), year_group_forbidden(2000, true)),
        case::year_group_allowed(true, DialogViewType::Years, create_year_month(1990, Month::February), year_group_forbidden(2000, false)),
    )]
    fn test_should_display_next_button(
        expected: bool,
        dialog_view_type: DialogViewType,
        year_month: YearMonth,
        mock_constraints: MockHasDateConstraints,
    ) {
        assert_eq!(
            expected,
            should_display_next_button(&dialog_view_type, &year_month, &mock_constraints)
        );
    }
}
