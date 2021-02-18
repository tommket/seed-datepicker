use chrono::{prelude::*, Duration};
use config::PickerConfig;
use num_traits::FromPrimitive;
use seed::{prelude::*, *};
use style_names::{
    BODY, BUTTON, CLOSE, GRID_HEADER, HEADER, NEXT, OTHER_MONTH, PREVIOUS, SEED_DATEPICKER,
    SELECTABLE, SELECTED, TITLE, UNAVAILABLE,
};
use year_month::{year_group_end, year_group_range, year_group_start, YearMonth};

pub mod config;
mod style_names;
mod year_month;

#[macro_use]
extern crate derive_getters;

#[macro_use]
extern crate derive_builder;

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

/// `Model` describes the current datepicker state.
pub struct Model {
    /// value of the date that is selected
    selected_date: Option<NaiveDate>,

    /// whether the dialog is shown
    dialog_opened: bool,

    /// viewed time range
    year_month_info: YearMonth,

    /// dialog type
    dialog_view_type: DialogViewType,

    /// dialog position style, describing the position of the dialog
    dialog_position_style: Option<Style>,

    /// configuration of the picker, should be passed in during init and not modified later
    config: PickerConfig,
}

impl Model {
    /// selected value of the datepicker
    pub fn selected_date(&self) -> &Option<NaiveDate> {
        &self.selected_date
    }

    pub fn config(&self) -> &PickerConfig {
        &self.config
    }
}

/// `init` describes what should happen when your app started.
pub fn init<Ms: 'static>(
    _: Url,
    _: &mut impl Orders<Ms>,
    config: PickerConfig,
    _to_msg: impl FnOnce(Msg) -> Ms + Clone + 'static,
) -> Model {
    Model {
        selected_date: *config.initial_date(),
        dialog_opened: *config.initially_opened(),
        year_month_info: config.guess_allowed_year_month(),
        dialog_view_type: *config.initial_view_type(),
        dialog_position_style: None,
        config,
    }
}

/// `Msg` describes the different events you can modify state with.
pub enum Msg {
    DateSelected(NaiveDate),
    MonthSelected(Month),
    YearSelected(i32),
    /// open the dialog, optionally at the given (left, top) position
    OpenDialog(Option<(String, String)>),
    CloseDialog,
    PreviousButtonClicked,
    NextButtonClicked,

    /// clicks on the dialog title change the `DialogViewType`
    DialogTitleClicked,
}

/// `update` describes how to handle each `Msg`.
pub fn update<Ms: 'static>(
    msg: Msg,
    model: &mut Model,
    orders: &mut impl Orders<Ms>,
    on_change: Ms,
    to_msg: impl FnOnce(Msg) -> Ms + Clone + 'static,
) {
    match msg {
        Msg::DateSelected(new_date) => {
            model.selected_date = Some(new_date);
            model.year_month_info = new_date.into();
            orders.send_msg(to_msg(Msg::CloseDialog));
            orders.send_msg(on_change);
        }
        Msg::MonthSelected(new_month) => {
            if model.config.selection_type() == &DialogViewType::Months {
                let new_date = NaiveDate::from_ymd(
                    model.year_month_info.year,
                    new_month.number_from_month(),
                    1,
                );
                orders.send_msg(to_msg(Msg::DateSelected(new_date)));
            } else {
                model.dialog_view_type = DialogViewType::Days;
                model.year_month_info.month = new_month;
            }
        }
        Msg::YearSelected(new_year) => {
            if model.config.selection_type() == &DialogViewType::Years {
                let new_date = NaiveDate::from_ymd(new_year, 1, 1);
                orders.send_msg(to_msg(Msg::DateSelected(new_date)));
            } else {
                model.dialog_view_type = DialogViewType::Months;
                model.year_month_info.year = new_year;
            }
        }
        Msg::OpenDialog(position) => {
            model.dialog_opened = true;
            if let Some((left, top)) = position {
                model.dialog_position_style = Some(style! {
                    St::Left => left,
                    St::Top => top,
                });
            }
        }
        Msg::CloseDialog => model.dialog_opened = false,
        Msg::PreviousButtonClicked => {
            model.year_month_info = match model.dialog_view_type {
                DialogViewType::Days => model.year_month_info.previous_month(),
                DialogViewType::Months => model.year_month_info.previous_year(),
                DialogViewType::Years => model.year_month_info.previous_year_group(),
            };
        }
        Msg::NextButtonClicked => {
            model.year_month_info = match model.dialog_view_type {
                DialogViewType::Days => model.year_month_info.next_month(),
                DialogViewType::Months => model.year_month_info.next_year(),
                DialogViewType::Years => model.year_month_info.next_year_group(),
            };
        }
        Msg::DialogTitleClicked => {
            model.dialog_view_type = match model.dialog_view_type {
                DialogViewType::Days => DialogViewType::Months,
                DialogViewType::Months => DialogViewType::Years,
                DialogViewType::Years => DialogViewType::Years,
            }
        }
    };
}

/// `view` describes what to display.
pub fn view<Ms: 'static>(
    model: &Model,
    to_msg: impl FnOnce(Msg) -> Ms + Clone + 'static,
) -> Node<Ms> {
    IF!(model.dialog_opened => div![
        C![SEED_DATEPICKER],
        model.dialog_position_style.as_ref(),
        view_dialog_header(model, to_msg.clone()),
        view_dialog_body(model, to_msg),
    ])
    .unwrap_or(empty![])
}

fn view_dialog_header<Ms: 'static>(
    model: &Model,
    to_msg: impl FnOnce(Msg) -> Ms + Clone + 'static,
) -> Node<Ms> {
    div![
        C![HEADER],
        button![
            C![BUTTON, PREVIOUS],
            style! {
                St::Visibility => if should_display_previous_button(model) { "visible" } else {"hidden"},
            },
            "«",
            ev(Ev::Click, {
                let to_msg = to_msg.clone();
                |_| to_msg(Msg::PreviousButtonClicked)
            }),
        ],
        span![
            C![TITLE],
            attrs! {
                At::from("role") => "heading",
            },
            create_dialog_title_text(model),
            ev(Ev::Click, {
                let to_msg = to_msg.clone();
                |_| to_msg(Msg::DialogTitleClicked)
            }),
        ],
        button![
            C![BUTTON, NEXT],
            style! {
                St::Visibility => if should_display_next_button(model) { "visible" } else { "hidden" },
            },
            "»",
            ev(Ev::Click, {
                let to_msg = to_msg.clone();
                |_| to_msg(Msg::NextButtonClicked)
            }),
        ],
        button![
            C![BUTTON, CLOSE],
            "x",
            ev(Ev::Click, |_| to_msg(Msg::CloseDialog)),
        ],
    ]
}

fn create_dialog_title_text(model: &Model) -> String {
    match model.dialog_view_type {
        DialogViewType::Days => model
            .year_month_info
            .first_day_of_month()
            .format(model.config.month_title_format())
            .to_string(),
        DialogViewType::Months => model
            .year_month_info
            .first_day_of_month()
            .format("%Y")
            .to_string(),
        DialogViewType::Years => format!(
            "{} - {}",
            year_group_start(model.year_month_info.year),
            year_group_end(model.year_month_info.year)
        ),
    }
}

fn should_display_previous_button(model: &Model) -> bool {
    match model.dialog_view_type {
        DialogViewType::Days => !model
            .config
            .is_month_forbidden(&model.year_month_info.previous_month()),
        DialogViewType::Months => !model
            .config
            .is_year_forbidden(model.year_month_info.year - 1),
        DialogViewType::Years => !model
            .config
            .is_year_group_forbidden(year_group_start(model.year_month_info.year) - 1),
    }
}

fn should_display_next_button(model: &Model) -> bool {
    match model.dialog_view_type {
        DialogViewType::Days => !model
            .config
            .is_month_forbidden(&model.year_month_info.next_month()),
        DialogViewType::Months => !model
            .config
            .is_year_forbidden(model.year_month_info.year + 1),
        DialogViewType::Years => !model
            .config
            .is_year_group_forbidden(year_group_end(model.year_month_info.year) + 1),
    }
}

fn view_dialog_body<Ms: 'static>(
    model: &Model,
    to_msg: impl FnOnce(Msg) -> Ms + Clone + 'static,
) -> Node<Ms> {
    match model.dialog_view_type {
        DialogViewType::Days => view_dialog_days(model, to_msg),
        DialogViewType::Months => view_dialog_months(model, to_msg),
        DialogViewType::Years => view_dialog_years(model, to_msg),
    }
}

fn view_dialog_years<Ms: 'static>(
    model: &Model,
    to_msg: impl FnOnce(Msg) -> Ms + Clone + 'static,
) -> Node<Ms> {
    let years: Vec<Node<Ms>> = year_group_range(model.year_month_info.year)
        .map(|year| view_year_cell(year, model, to_msg.clone()))
        .collect();

    div![
        C![BODY],
        style! {
            St::GridTemplateColumns => "1fr ".repeat(4),
        },
        years,
    ]
}

fn view_year_cell<Ms: 'static>(
    year: i32,
    model: &Model,
    to_msg: impl FnOnce(Msg) -> Ms + Clone + 'static,
) -> Node<Ms> {
    let is_year_forbidden = model.config.is_year_forbidden(year);
    let is_year_selected = model
        .selected_date
        .map_or(false, |optval| optval.year() == year);

    span![
        year.to_string(),
        C![
            if is_year_forbidden {
                UNAVAILABLE
            } else {
                SELECTABLE
            },
            IF!(is_year_selected => SELECTED),
        ],
        attrs! {
            At::from("role") => "gridcell",
            At::AriaSelected => is_year_selected.as_at_value(),
        },
        IF!(!is_year_forbidden => ev(Ev::Click, move |_| to_msg(Msg::YearSelected(year)))),
    ]
}

fn view_dialog_months<Ms: 'static>(
    model: &Model,
    to_msg: impl FnOnce(Msg) -> Ms + Clone + 'static,
) -> Node<Ms> {
    let months: Vec<Node<Ms>> = (Month::January.number_from_month()
        ..=Month::December.number_from_month())
        .map(|month| {
            view_month_cell(
                YearMonth {
                    year: model.year_month_info.year,
                    month: Month::from_u32(month).unwrap(),
                },
                model,
                to_msg.clone(),
            )
        })
        .collect();

    div![
        C![BODY],
        style! {
            St::GridTemplateColumns => "1fr ".repeat(3),
        },
        months
    ]
}

fn view_month_cell<Ms: 'static>(
    year_month_info: YearMonth,
    model: &Model,
    to_msg: impl FnOnce(Msg) -> Ms + Clone + 'static,
) -> Node<Ms> {
    let is_month_forbidden = model.config.is_month_forbidden(&year_month_info);
    let is_month_selected = model
        .selected_date
        .map_or(false, |optval| year_month_info.contains(&optval));

    span![
        year_month_info.month.name(),
        C![
            if is_month_forbidden {
                UNAVAILABLE
            } else {
                SELECTABLE
            },
            IF!(is_month_selected => SELECTED),
        ],
        attrs! {
            At::from("role") => "gridcell",
            At::AriaSelected => is_month_selected.as_at_value(),
        },
        IF!(!is_month_forbidden => ev(Ev::Click, move |_| to_msg(Msg::MonthSelected(year_month_info.month)))),
    ]
}

fn view_dialog_days<Ms: 'static>(
    model: &Model,
    to_msg: impl FnOnce(Msg) -> Ms + Clone + 'static,
) -> Node<Ms> {
    let first_day_of_month = model.year_month_info.first_day_of_month();
    let first_day_of_calendar = first_day_of_month
        - Duration::days(first_day_of_month.weekday().num_days_from_monday().into());

    let day_nodes: Vec<Node<Ms>> = first_day_of_calendar
        .iter_days()
        .take(7 * 6)
        .map(|day| view_day_cell(day, model, to_msg.clone()))
        .collect();

    div![
        C!["body"],
        style! {
            St::GridTemplateColumns => "1fr ".repeat(7),
        },
        view_weekday_name(Weekday::Mon),
        view_weekday_name(Weekday::Tue),
        view_weekday_name(Weekday::Wed),
        view_weekday_name(Weekday::Thu),
        view_weekday_name(Weekday::Fri),
        view_weekday_name(Weekday::Sat),
        view_weekday_name(Weekday::Sun),
        day_nodes,
    ]
}

fn view_weekday_name<Ms: 'static>(day: Weekday) -> Node<Ms> {
    span![
        day.to_string(),
        C![GRID_HEADER],
        attrs! {
            At::from("role") => "columnheader",
        },
    ]
}

fn view_day_cell<Ms: 'static>(
    date: NaiveDate,
    model: &Model,
    to_msg: impl FnOnce(Msg) -> Ms + Clone + 'static,
) -> Node<Ms> {
    let is_day_forbidden = model.config.is_day_forbidden(&date);
    let is_date_selected = model.selected_date.map_or(false, |optval| optval == date);

    span![
        date.day().to_string(),
        C![
            if is_day_forbidden {
                UNAVAILABLE
            } else {
                SELECTABLE
            },
            IF!(date.month() != model.year_month_info.month.number_from_month() => OTHER_MONTH),
            IF!(is_date_selected => SELECTED),
        ],
        attrs! {
            At::from("role") => "gridcell",
            At::AriaSelected => is_date_selected.as_at_value(),
        },
        IF!(!is_day_forbidden => ev(Ev::Click, move |_| to_msg(Msg::DateSelected(date)))),
    ]
}
