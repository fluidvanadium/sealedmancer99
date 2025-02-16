use std::fmt::Display;

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

impl Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let number_of_cards = format_duh(&self.number_of_cards);
        let number_of_errors = format_duh(&self.number_of_errors);

        let success_sleep_nanos = format_duh(&self.success_sleep_nanos);
        let error_sleep_nanos = format_duh(&self.error_sleep_nanos);

        let success_server_nanos = format_duh(&self.success_server_nanos);
        let success_server_nanos_per_card =
            format_duh(&(self.success_server_nanos / self.number_of_cards));

        let cards_per_error = format_duh(&(self.number_of_cards / self.number_of_errors));

        let error_server_nanos = format_duh(&self.error_server_nanos);
        let error_server_nanos_per_error =
            format_duh(&(self.error_server_nanos / self.number_of_errors));
        let error_server_nanos_per_card =
            format_duh(&(self.error_server_nanos / self.number_of_cards));

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
