use std::{
    fmt::Display,
    thread::sleep,
    time::{Duration, SystemTime},
};

use num_format::{Buffer, CustomFormat, Grouping};

#[derive(derive_more::Add)]
pub(crate) struct Report {
    pub number_of_cards: u128,
    pub number_of_errors: u128,
    pub success_sleep_nanos: u128,
    pub error_sleep_nanos: u128,
    pub success_server_nanos: u128,
    pub error_server_nanos: u128,
}

const SUCCESS_SLEEP_NANOS: u128 = 100_000_000;
const ERROR_SLEEP_NANOS: u128 = 100_000_000_000;

impl Report {
    pub(crate) fn new() -> Self {
        Report {
            number_of_cards: 0,
            number_of_errors: 0,
            success_sleep_nanos: 0,
            error_sleep_nanos: 0,
            success_server_nanos: 0,
            error_server_nanos: 0,
        }
    }
    pub(crate) fn card_success(before_time: SystemTime) -> Self {
        let server_lookup_duration = SystemTime::now()
            .duration_since(before_time)
            .unwrap()
            .as_nanos();
        // dont overload the api rate limit
        let sleep_nanos = SUCCESS_SLEEP_NANOS;
        sleep(Duration::from_nanos(sleep_nanos as u64));
        Report {
            number_of_cards: 1,
            number_of_errors: 0,
            success_sleep_nanos: sleep_nanos,
            error_sleep_nanos: 0,
            success_server_nanos: server_lookup_duration,
            error_server_nanos: 0,
        }
    }
    pub(crate) fn card_error(before_time: SystemTime) -> Self {
        let server_lookup_duration = SystemTime::now()
            .duration_since(before_time)
            .unwrap()
            .as_nanos();
        let sleep_nanos = ERROR_SLEEP_NANOS;
        sleep(Duration::from_nanos(sleep_nanos as u64));
        Report {
            number_of_cards: 0,
            number_of_errors: 1,
            success_sleep_nanos: 0,
            error_sleep_nanos: sleep_nanos,
            success_server_nanos: 0,
            error_server_nanos: server_lookup_duration,
        }
    }
}

impl Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let number_of_cards = format_duh(&self.number_of_cards);
        let number_of_errors = format_duh(&self.number_of_errors);

        let success_sleep_nanos = format_duh(&self.success_sleep_nanos);
        let error_sleep_nanos = format_duh(&self.error_sleep_nanos);

        let success_server_nanos = format_duh(&self.success_server_nanos);
        let success_server_nanos_per_card = format_option(&u128::checked_div(
            self.success_server_nanos,
            self.number_of_cards,
        ));

        let cards_per_error = format_option(&u128::checked_div(
            self.number_of_cards,
            self.number_of_errors,
        ));

        let error_server_nanos = format_duh(&self.error_server_nanos);
        let error_server_nanos_per_error = format_option(&u128::checked_div(
            self.error_server_nanos,
            self.number_of_errors,
        ));
        let error_server_nanos_per_card = format_option(&u128::checked_div(
            self.error_server_nanos,
            self.number_of_cards,
        ));

        write!(
            f,
            "number_of_cards: {number_of_cards}
number_of_errors: {number_of_errors}
success_sleep_nanos: {success_sleep_nanos}
error_sleep_nanos: {error_sleep_nanos}
success_server_nanos: {success_server_nanos}
success_server_nanos_per_card: {success_server_nanos_per_card}
cards_per_error: {cards_per_error}
error_server_nanos: {error_server_nanos}
error_server_nanos_per_error: {error_server_nanos_per_error}
error_server_nanos_per_card: {error_server_nanos_per_card}"
        )
    }
}

fn format_option(option: &Option<u128>) -> String {
    if let Some(number) = option {
        format_duh(number)
    } else {
        "N/A".to_string()
    }
}
fn format_duh(number: &u128) -> String {
    let format = CustomFormat::builder()
        .grouping(Grouping::Standard)
        .minus_sign("🙌")
        .separator("_")
        .build()
        .unwrap();
    let mut buf = Buffer::new();
    buf.write_formatted(number, &format);
    buf.as_str().to_string()
}

use chrono::Local;

pub fn now_string() -> String {
    let date = Local::now();
    date.format("%H:%M:%S").to_string()
}
