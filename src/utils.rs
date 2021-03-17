use chrono::{Datelike, NaiveDate};

use crate::{
    config::date_constraints::HasDateConstraints,
    dialog_view_type::DialogViewType,
    viewed_date::{year_group_end, year_group_start, ViewedDate},
};

/// Creates the text that should be the title of the datepicker dialog.
pub fn create_dialog_title_text(
    dialog_view_type: &DialogViewType,
    viewed_date: &NaiveDate,
    month_title_format: &str,
) -> String {
    match dialog_view_type {
        DialogViewType::Days => viewed_date
            .first_day_of_month()
            .format(month_title_format)
            .to_string(),
        DialogViewType::Months => viewed_date.first_day_of_month().format("%Y").to_string(),
        DialogViewType::Years => format!(
            "{} - {}",
            year_group_start(viewed_date.year()),
            year_group_end(viewed_date.year())
        ),
    }
}

/// Returns true if the "previous" button should be displayed.
pub fn should_display_previous_button<T: HasDateConstraints>(
    dialog_view_type: &DialogViewType,
    viewed_date: &NaiveDate,
    config: &T,
) -> bool {
    match dialog_view_type {
        DialogViewType::Days => !config.is_month_forbidden(&viewed_date.previous_month()),
        DialogViewType::Months => !config.is_year_forbidden(viewed_date.previous_year().year()),
        DialogViewType::Years => {
            !config.is_year_group_forbidden(viewed_date.previous_year_group().year())
        }
    }
}

/// Returns true if the "next" button should be displayed.
pub fn should_display_next_button<T: HasDateConstraints>(
    dialog_view_type: &DialogViewType,
    viewed_date: &NaiveDate,
    config: &T,
) -> bool {
    match dialog_view_type {
        DialogViewType::Days => !config.is_month_forbidden(&viewed_date.next_month()),
        DialogViewType::Months => !config.is_year_forbidden(viewed_date.next_year().year()),
        DialogViewType::Years => {
            !config.is_year_group_forbidden(viewed_date.next_year_group().year())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{config::date_constraints::MockHasDateConstraints, viewed_date::YearNumber};

    use crate::rstest_utils::create_date;
    use mockall::predicate;
    use rstest::*;

    #[rstest(
        expected, dialog_view_type, viewed_date, month_title_format, //
        case::days_default("Jan 1990", DialogViewType::Days, create_date(1990, 1, 1), "%b %Y"),
        case::days_different_format("January 1990", DialogViewType::Days, create_date(1990, 1, 1), "%B %Y"),
        case::months("1990", DialogViewType::Months, create_date(1990, 1, 1), ""),
        case::years("1980 - 1999", DialogViewType::Years, create_date(1990, 1, 1), ""),
    )]
    fn test_create_dialog_title_text(
        expected: &str,
        dialog_view_type: DialogViewType,
        viewed_date: NaiveDate,
        month_title_format: &str,
    ) {
        assert_eq!(
            expected,
            create_dialog_title_text(&dialog_view_type, &viewed_date, month_title_format)
        );
    }

    #[fixture(viewed_date=create_date(1990, 1, 1), retval=false)]
    fn month_forbidden(viewed_date: NaiveDate, retval: bool) -> MockHasDateConstraints {
        let mut mock = MockHasDateConstraints::new();
        mock.expect_is_month_forbidden()
            .with(predicate::eq(viewed_date))
            .times(1)
            .returning(move |_| retval);
        mock
    }

    #[fixture(year = 1990, retval = false)]
    fn year_forbidden(year: YearNumber, retval: bool) -> MockHasDateConstraints {
        let mut mock = MockHasDateConstraints::new();
        mock.expect_is_year_forbidden()
            .with(predicate::eq(year))
            .times(1)
            .returning(move |_| retval);
        mock
    }

    #[fixture(year = 1990, retval = false)]
    fn year_group_forbidden(year: YearNumber, retval: bool) -> MockHasDateConstraints {
        let mut mock = MockHasDateConstraints::new();
        mock.expect_is_year_group_forbidden()
            .with(predicate::eq(year))
            .times(1)
            .returning(move |_| retval);
        mock
    }

    #[rstest(
        expected, dialog_view_type, viewed_date, mock_constraints, //
        case::month_forbidden(false, DialogViewType::Days, create_date(1990, 2, 16), month_forbidden(create_date(1990, 1, 1), true)),
        case::month_allowed(true, DialogViewType::Days, create_date(1990, 3, 25), month_forbidden(create_date(1990, 2, 1), false)),
        case::year_forbidden(false, DialogViewType::Months, create_date(1990, 4, 26), year_forbidden(1989, true)),
        case::year_allowed(true, DialogViewType::Months, create_date(1990, 7, 18), year_forbidden(1989, false)),
        case::year_group_forbidden(false, DialogViewType::Years, create_date(1990, 2, 16), year_group_forbidden(1979, true)),
        case::year_group_allowed(true, DialogViewType::Years, create_date(1990, 2, 18), year_group_forbidden(1979, false)),
    )]
    fn test_should_display_previous_button(
        expected: bool,
        dialog_view_type: DialogViewType,
        viewed_date: NaiveDate,
        mock_constraints: MockHasDateConstraints,
    ) {
        assert_eq!(
            expected,
            should_display_previous_button(&dialog_view_type, &viewed_date, &mock_constraints)
        );
    }

    #[rstest(
        expected, dialog_view_type, viewed_date, mock_constraints, //
        case::month_forbidden(false, DialogViewType::Days, create_date(1990, 2, 18), month_forbidden(create_date(1990, 3, 1), true)),
        case::month_allowed(true, DialogViewType::Days, create_date(1990, 2, 15), month_forbidden(create_date(1990, 3, 1), false)),
        case::year_forbidden(false, DialogViewType::Months, create_date(1990, 8, 16), year_forbidden(1991, true)),
        case::year_allowed(true, DialogViewType::Months, create_date(1990, 4, 21), year_forbidden(1991, false)),
        case::year_group_forbidden(false, DialogViewType::Years, create_date(1990, 11, 26), year_group_forbidden(2000, true)),
        case::year_group_allowed(true, DialogViewType::Years, create_date(1990, 12, 23), year_group_forbidden(2000, false)),
    )]
    fn test_should_display_next_button(
        expected: bool,
        dialog_view_type: DialogViewType,
        viewed_date: NaiveDate,
        mock_constraints: MockHasDateConstraints,
    ) {
        assert_eq!(
            expected,
            should_display_next_button(&dialog_view_type, &viewed_date, &mock_constraints)
        );
    }
}
