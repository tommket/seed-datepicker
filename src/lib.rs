#![forbid(unsafe_code)]

use chrono::{prelude::*, Duration};
use chrono_datepicker_core::{
    config::{date_constraints::HasDateConstraints, PickerConfig},
    dialog_view_type::DialogViewType,
    style_names::*,
    utils::{create_dialog_title_text, should_display_next_button, should_display_previous_button},
    viewed_date::{year_group_range, MonthNumber, ViewedDate, YearNumber},
};
use num_traits::FromPrimitive;
use seed::{prelude::*, *};

/// reexport only necessary things for using the seed-datepicker
pub use chrono_datepicker_core::config;
pub use chrono_datepicker_core::dialog_view_type;

/// `Model` describes the current datepicker state.
pub struct Model<T>
where
    T: HasDateConstraints + Default + Clone,
{
    /// value of the date that is selected
    selected_date: Option<NaiveDate>,

    /// whether the dialog is shown
    dialog_opened: bool,

    /// viewed date
    viewed_date: NaiveDate,

    /// dialog type
    dialog_view_type: DialogViewType,

    /// dialog position style, describing the position of the dialog
    dialog_position_style: Option<Style>,

    /// configuration of the picker, should be passed in during init and not modified later
    config: PickerConfig<T>,
}

impl<T: HasDateConstraints + Default + Clone> Model<T> {
    /// selected value of the datepicker
    pub fn selected_date(&self) -> &Option<NaiveDate> {
        &self.selected_date
    }

    pub fn config(&self) -> &PickerConfig<T> {
        &self.config
    }
}

/// `init` describes what should happen when your app started.
pub fn init<Ms: 'static, T: HasDateConstraints + std::default::Default + Clone>(
    _: Url,
    _: &mut impl Orders<Ms>,
    config: PickerConfig<T>,
    _to_msg: impl FnOnce(Msg) -> Ms + Clone + 'static,
) -> Model<T> {
    Model {
        selected_date: *config.initial_date(),
        dialog_opened: *config.initially_opened(),
        viewed_date: config.guess_allowed_year_month(),
        dialog_view_type: *config.initial_view_type(),
        dialog_position_style: None,
        config,
    }
}

/// `Msg` describes the different events you can modify state with.
pub enum Msg {
    DateSelected(NaiveDate),
    MonthSelected(MonthNumber),
    YearSelected(YearNumber),
    /// open the dialog, optionally at the given (left, top) position
    OpenDialog(Option<(String, String)>),
    CloseDialog,
    PreviousButtonClicked,
    NextButtonClicked,

    /// clicks on the dialog title change the `DialogViewType`
    DialogTitleClicked,
}

/// `update` describes how to handle each `Msg`.
pub fn update<Ms: 'static, T: HasDateConstraints + std::default::Default + Clone>(
    msg: Msg,
    model: &mut Model<T>,
    orders: &mut impl Orders<Ms>,
    on_change: Ms,
    to_msg: impl FnOnce(Msg) -> Ms + Clone + 'static,
) {
    match msg {
        Msg::DateSelected(new_date) => {
            model.selected_date = Some(new_date);
            model.viewed_date = new_date;
            orders.send_msg(to_msg(Msg::CloseDialog));
            orders.send_msg(on_change);
        }
        Msg::MonthSelected(new_month) => {
            model.viewed_date = NaiveDate::from_ymd(model.viewed_date.year(), new_month, 1);
            if model.config.selection_type() == &DialogViewType::Months {
                orders.send_msg(to_msg(Msg::DateSelected(model.viewed_date)));
            } else {
                model.dialog_view_type = DialogViewType::Days;
            }
        }
        Msg::YearSelected(new_year) => {
            model.viewed_date = NaiveDate::from_ymd(new_year, 1, 1);
            if model.config.selection_type() == &DialogViewType::Years {
                orders.send_msg(to_msg(Msg::DateSelected(model.viewed_date)));
            } else {
                model.dialog_view_type = DialogViewType::Months;
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
            model.viewed_date = match model.dialog_view_type {
                DialogViewType::Days => model.viewed_date.previous_month(),
                DialogViewType::Months => model.viewed_date.previous_year(),
                DialogViewType::Years => model.viewed_date.previous_year_group(),
            };
        }
        Msg::NextButtonClicked => {
            model.viewed_date = match model.dialog_view_type {
                DialogViewType::Days => model.viewed_date.next_month(),
                DialogViewType::Months => model.viewed_date.next_year(),
                DialogViewType::Years => model.viewed_date.next_year_group(),
            };
        }
        Msg::DialogTitleClicked => {
            if let Some(new_dialog_type) = model.dialog_view_type.larger_type() {
                model.dialog_view_type = new_dialog_type;
            }
        }
    };
}

/// `view` describes what to display.
pub fn view<Ms: 'static, T: HasDateConstraints + std::default::Default + Clone>(
    model: &Model<T>,
    to_msg: impl FnOnce(Msg) -> Ms + Clone + 'static,
) -> Node<Ms> {
    IF!(model.dialog_opened => div![
        C![DATEPICKER_ROOT],
        model.dialog_position_style.as_ref(),
        view_dialog_header(model, to_msg.clone()),
        view_dialog_body(model, to_msg),
    ])
    .unwrap_or(empty![])
}

fn view_dialog_header<Ms: 'static, T: HasDateConstraints + std::default::Default + Clone>(
    model: &Model<T>,
    to_msg: impl FnOnce(Msg) -> Ms + Clone + 'static,
) -> Node<Ms> {
    div![
        C![HEADER],
        button![
            C![BUTTON, PREVIOUS],
            style! {
                St::Visibility => if should_display_previous_button(&model.dialog_view_type, &model.viewed_date, &model.config) { "visible" } else {"hidden"},
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
            create_dialog_title_text(
                &model.dialog_view_type,
                &model.viewed_date,
                &model.config.month_title_format()
            ),
            ev(Ev::Click, {
                let to_msg = to_msg.clone();
                |_| to_msg(Msg::DialogTitleClicked)
            }),
        ],
        button![
            C![BUTTON, NEXT],
            style! {
                St::Visibility => if should_display_next_button(&model.dialog_view_type, &model.viewed_date, &model.config) { "visible" } else { "hidden" },
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

fn view_dialog_body<Ms: 'static, T: HasDateConstraints + std::default::Default + Clone>(
    model: &Model<T>,
    to_msg: impl FnOnce(Msg) -> Ms + Clone + 'static,
) -> Node<Ms> {
    match model.dialog_view_type {
        DialogViewType::Days => view_dialog_days(model, to_msg),
        DialogViewType::Months => view_dialog_months(model, to_msg),
        DialogViewType::Years => view_dialog_years(model, to_msg),
    }
}

fn view_dialog_years<Ms: 'static, T: HasDateConstraints + std::default::Default + Clone>(
    model: &Model<T>,
    to_msg: impl FnOnce(Msg) -> Ms + Clone + 'static,
) -> Node<Ms> {
    let years: Vec<Node<Ms>> = year_group_range(model.viewed_date.year())
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

fn view_year_cell<Ms: 'static, T: HasDateConstraints + std::default::Default + Clone>(
    year: i32,
    model: &Model<T>,
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

fn view_dialog_months<Ms: 'static, T: HasDateConstraints + std::default::Default + Clone>(
    model: &Model<T>,
    to_msg: impl FnOnce(Msg) -> Ms + Clone + 'static,
) -> Node<Ms> {
    let months: Vec<Node<Ms>> = (1..=12u32)
        .map(|month| {
            view_month_cell(
                NaiveDate::from_ymd(model.viewed_date.year(), month, 1),
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

fn view_month_cell<Ms: 'static, T: HasDateConstraints + std::default::Default + Clone>(
    month_to_display: NaiveDate,
    model: &Model<T>,
    to_msg: impl FnOnce(Msg) -> Ms + Clone + 'static,
) -> Node<Ms> {
    let is_month_forbidden = model.config.is_month_forbidden(&month_to_display);
    let is_month_selected = model.selected_date.map_or(false, |optval| {
        month_to_display.contains(&model.dialog_view_type, &optval)
    });

    span![
        Month::from_u32(month_to_display.month()).unwrap().name(),
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
        IF!(!is_month_forbidden => ev(Ev::Click, move |_| to_msg(Msg::MonthSelected(month_to_display.month())))),
    ]
}

fn view_dialog_days<Ms: 'static, T: HasDateConstraints + std::default::Default + Clone>(
    model: &Model<T>,
    to_msg: impl FnOnce(Msg) -> Ms + Clone + 'static,
) -> Node<Ms> {
    let first_day_of_month = model.viewed_date.first_day_of_month();
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

fn view_day_cell<Ms: 'static, T: HasDateConstraints + std::default::Default + Clone>(
    date: NaiveDate,
    model: &Model<T>,
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
            IF!(date.month() != model.viewed_date.month() => OTHER_MONTH),
            IF!(is_date_selected => SELECTED),
        ],
        attrs! {
            At::from("role") => "gridcell",
            At::AriaSelected => is_date_selected.as_at_value(),
        },
        IF!(!is_day_forbidden => ev(Ev::Click, move |_| to_msg(Msg::DateSelected(date)))),
    ]
}
