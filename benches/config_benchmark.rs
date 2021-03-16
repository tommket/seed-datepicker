use chrono::{Month, NaiveDate, Weekday};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use seed_datepicker::config::{
    date_constraints::{DateConstraints, DateConstraintsBuilder, HasDateConstraints},
    PickerConfig, PickerConfigBuilder,
};

criterion_group!(
    benches,
    is_year_forbidden_in_disabled_year,
    is_day_forbidden_day_allowed,
    is_day_forbidden_sooner_than_min_date,
    is_day_forbidden_later_than_max_date,
    is_day_forbidden_on_disabled_weekday,
    is_day_forbidden_in_disabled_month,
    is_day_forbidden_on_disabled_monthly_date,
    is_day_forbidden_on_disabled_yearly_date,
    is_day_forbidden_on_disabled_unique_date,
);
criterion_main!(benches);

fn create_config() -> PickerConfig<DateConstraints> {
    PickerConfigBuilder::default()
        .initial_date(NaiveDate::from_ymd(2020, 12, 15))
        .date_constraints(
            DateConstraintsBuilder::default()
                .min_date(NaiveDate::from_ymd(2020, 12, 1))
                .max_date(NaiveDate::from_ymd(2022, 12, 14))
                .disabled_weekdays([Weekday::Sat, Weekday::Sun].iter().cloned().collect())
                .disabled_months([Month::July, Month::August].iter().cloned().collect())
                .disabled_years([2021].iter().cloned().collect())
                .disabled_monthly_dates([13].iter().cloned().collect())
                .disabled_yearly_dates(vec![
                    NaiveDate::from_ymd(1, 12, 24),
                    NaiveDate::from_ymd(1, 12, 25),
                    NaiveDate::from_ymd(1, 12, 26),
                ])
                .disabled_unique_dates([NaiveDate::from_ymd(2020, 12, 8)].iter().cloned().collect())
                .build()
                .unwrap(),
        )
        .build()
        .unwrap()
}

#[allow(dead_code)]
fn is_day_forbidden_day_allowed(c: &mut Criterion) {
    let start_date = NaiveDate::from_ymd(2020, 12, 9);
    let config = create_config();
    c.bench_function("is_day_forbidden_day_allowed", |b| {
        b.iter(|| config.is_day_forbidden(black_box(&start_date)))
    });
}

#[allow(dead_code)]
fn is_day_forbidden_sooner_than_min_date(c: &mut Criterion) {
    let start_date = NaiveDate::from_ymd(2020, 11, 30);
    let config = create_config();
    c.bench_function("is_day_forbidden_sooner_than_min_date", |b| {
        b.iter(|| config.is_day_forbidden(black_box(&start_date)))
    });
}

#[allow(dead_code)]
fn is_day_forbidden_later_than_max_date(c: &mut Criterion) {
    let start_date = NaiveDate::from_ymd(2023, 2, 15);
    let config = create_config();
    c.bench_function("is_day_forbidden_later_than_max_date", |b| {
        b.iter(|| config.is_day_forbidden(black_box(&start_date)))
    });
}

#[allow(dead_code)]
fn is_day_forbidden_on_disabled_weekday(c: &mut Criterion) {
    let start_date = NaiveDate::from_ymd(2020, 12, 12);
    let config = create_config();
    c.bench_function("is_day_forbidden_on_disabled_weekday", |b| {
        b.iter(|| config.is_day_forbidden(black_box(&start_date)))
    });
}

#[allow(dead_code)]
fn is_day_forbidden_in_disabled_month(c: &mut Criterion) {
    let start_date = NaiveDate::from_ymd(2022, 7, 12);
    let config = create_config();
    c.bench_function("is_day_forbidden_in_disabled_month", |b| {
        b.iter(|| config.is_day_forbidden(black_box(&start_date)))
    });
}

#[allow(dead_code)]
fn is_day_forbidden_in_disabled_year(c: &mut Criterion) {
    let start_date = NaiveDate::from_ymd(2021, 12, 9);
    let config = create_config();
    c.bench_function("is_day_forbidden_in_disabled_year", |b| {
        b.iter(|| config.is_day_forbidden(black_box(&start_date)))
    });
}

#[allow(dead_code)]
fn is_day_forbidden_on_disabled_monthly_date(c: &mut Criterion) {
    let start_date = NaiveDate::from_ymd(2022, 1, 13);
    let config = create_config();
    c.bench_function("is_day_forbidden_on_disabled_monthly_date", |b| {
        b.iter(|| config.is_day_forbidden(black_box(&start_date)))
    });
}

#[allow(dead_code)]
fn is_day_forbidden_on_disabled_yearly_date(c: &mut Criterion) {
    let start_date = NaiveDate::from_ymd(2020, 12, 24);
    let config = create_config();
    c.bench_function("is_day_forbidden_on_disabled_yearly_date", |b| {
        b.iter(|| config.is_day_forbidden(black_box(&start_date)))
    });
}

#[allow(dead_code)]
fn is_day_forbidden_on_disabled_unique_date(c: &mut Criterion) {
    let start_date = NaiveDate::from_ymd(2020, 12, 8);
    let config = create_config();
    c.bench_function("is_day_forbidden_on_disabled_unique_date", |b| {
        b.iter(|| config.is_day_forbidden(black_box(&start_date)))
    });
}

#[allow(dead_code)]
fn is_year_forbidden_in_disabled_year(c: &mut Criterion) {
    let config = create_config();
    c.bench_function("is_year_forbidden_in_disabled_year", |b| {
        b.iter(|| config.is_year_forbidden(black_box(2021)))
    });
}
