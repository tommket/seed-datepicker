use chrono::prelude::*;
use seed::{prelude::*, *};
use seed_datepicker::config::date_constraints::DateConstraintsBuilder;
use seed_datepicker::config::PickerConfigBuilder;

type DatePickerModel = seed_datepicker::Model;

/// `Model` describes our app state.
pub struct Model {
    date_picker: DatePickerModel,
}

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    let config = PickerConfigBuilder::default()
        .initial_date(NaiveDate::from_ymd(2020, 12, 15))
        .date_constraints(
            DateConstraintsBuilder::default()
                // earliest selectable date
                .min_date(NaiveDate::from_ymd(2020, 12, 1))
                // latest selectable date
                .max_date(NaiveDate::from_ymd(2022, 12, 14))
                // chrono Weekday-s that can be disabled
                .disabled_weekdays([Weekday::Sat, Weekday::Sun].iter().cloned().collect())
                // entire chrono Month-s that can be disabled
                .disabled_months([Month::July, Month::August].iter().cloned().collect())
                // entire years that can be disabled
                .disabled_years([2021].iter().cloned().collect())
                // a particular day of month that is disabled in all months
                .disabled_monthly_dates([13].iter().cloned().collect())
                // particular dates that are disabled each year (the year number is ignored here)
                .disabled_yearly_dates(vec![
                    NaiveDate::from_ymd(1, 12, 24),
                    NaiveDate::from_ymd(1, 12, 25),
                    NaiveDate::from_ymd(1, 12, 26),
                ])
                // particular unique dates that can be disabled
                .disabled_unique_dates([NaiveDate::from_ymd(2020, 12, 8)].iter().cloned().collect())
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();
    Model {
        date_picker: seed_datepicker::init(url, orders, config, Msg::DatePickerMsg),
    }
}

// `Msg` describes the different events you can modify state with.
pub enum Msg {
    DatePickerMsg(seed_datepicker::Msg),
    DateSelected,
}

// `update` describes how to handle each `Msg`.
pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::DatePickerMsg(picker_msg) => {
            seed_datepicker::update(
                picker_msg,
                &mut model.date_picker,
                orders,
                Msg::DateSelected,
                Msg::DatePickerMsg,
            );
        }
        // do anything with the newly selected date
        Msg::DateSelected => {
            log!("New date was selected: ", model.date_picker.selected_date());
        }
    };
}

// `view` describes what to display.
pub fn view(model: &Model) -> Node<Msg> {
    div![
        // text before the input itself to offset it, so that the dialog should open below the textbox
        input![
            C!["textbox"],
            attrs! {
                At::Value => model.date_picker.selected_date().map_or("".into(), |optval|optval.format("%e %b %Y").to_string()),
                At::Type => "text",
                At::ReadOnly => "",
                At::Placeholder => "Click HERE",
            },
            ev(Ev::Click, |_| Msg::DatePickerMsg(
                seed_datepicker::Msg::OpenDialog(None)
            )),
        ],
        seed_datepicker::view(&model.date_picker, Msg::DatePickerMsg),
        pre![format!("Current config: {:#?}", model.date_picker.config())]
    ]
}

pub fn main() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
