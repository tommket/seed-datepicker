/// Types of views for the datepicker.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum DialogViewType {
    /// YEARS_IN_YEAR_SELECTION Years, from a year which modulo `% 20 == 0`
    Years = 1,
    /// 1 full year with the selection of a month
    Months = 2,
    /// 1 full month with the selection of a day
    Days = 3,
}

impl Default for DialogViewType {
    fn default() -> Self {
        DialogViewType::Days
    }
}
