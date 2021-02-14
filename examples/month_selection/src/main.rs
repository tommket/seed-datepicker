use chrono::prelude::*;
use seed::{prelude::*, *};
use seed_datepicker::config::PickerConfigBuilder;
use seed_datepicker::DialogViewType;

type DatePickerModel = seed_datepicker::Model;

/// `Model` describes our app state.
pub struct Model {
    date_picker: DatePickerModel,
}

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    let config = PickerConfigBuilder::default()
        .initial_view_type(DialogViewType::Months)
        .selection_type(DialogViewType::Months)
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
                At::Value => model.date_picker.selected_date().map_or("".into(), |optval|optval.format("%b %Y").to_string()),
                At::Type => "text",
                At::ReadOnly => "",
                At::Placeholder => "Click HERE",
            },
            ev(Ev::Click, |_| Msg::DatePickerMsg(
                seed_datepicker::Msg::OpenDialog
            )),
        ],
        seed_datepicker::view(&model.date_picker, Msg::DatePickerMsg),
    ]
}

pub fn main() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
