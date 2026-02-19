use scryfall::search::prelude::*;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::SystemTime;
use timestamp::Report;

use crate::query_operations::DiscreteQuery;

mod query_operations;

mod timestamp;
const MAX_RETRIES: usize = 4;

#[tokio::main]
async fn main() {
    // dbg!(exact("Dungeon Delver").search().await.unwrap().next().await);

    std::env::set_current_dir("results").unwrap();

    for (destination_filename, der_query) in query_operations::default_formats() {
        write_query_to_file(destination_filename, &der_query).await;
    }
}

async fn write_query_to_file(destination_filename: &str, der_query: &DiscreteQuery) {
    // let index = 6;
    // let (destination_filename, der_query) = format[index].clone();
    let query = &der_query;
    println!("Beginning fetch of {der_query} to store in file {destination_filename}.");
    let (list, report) = download_list(query).await;

    let list_path_name = "lists/".to_string() + destination_filename;
    let list_path = Path::new(list_path_name.as_str());
    let report_path_name = "reports/".to_string() + destination_filename;
    let report_path = Path::new(report_path_name.as_str());
    let mut list_file = File::create(list_path).unwrap();
    let mut report_file = File::create(report_path).unwrap();

    list_file
        .write_all(list.as_bytes())
        .expect("Unable to write data");
    report_file
        .write_all(report.to_string().as_bytes())
        .expect("Unable to write data");
}

async fn download_list(query: &DiscreteQuery) -> (String, Report) {
    let mut card_list = String::new();

    let mut lazy_report = Report::new();

    match query.query.search().await {
        Err(e) => {
            dbg!(e);
        }
        Ok(mut cards) => {
            println!("Search setup completed. Beginning iteration.");
            let backup_cards = cards.clone();

            loop {
                let before_time = SystemTime::now();

                let next_card = cards.next().await;

                match next_card {
                    None => {
                        println!("no more cards");
                        break;
                    }
                    Some(card_result) => match card_result {
                        Ok(card) => {
                            lazy_report = lazy_report + Report::card_success(before_time);
                            let timestamp = timestamp::now_string();

                            let (card_entry, copies_report) =
                                create_card_entry(card, query.count_copies).await;

                            lazy_report = lazy_report + copies_report;

                            println!("{timestamp} . {card_entry}");

                            card_list = card_list
                                + "
" + card_entry.as_str();
                        }
                        Err(e) => {
                            dbg!(e);

                            lazy_report = lazy_report + Report::card_error(before_time);

                            cards = backup_cards.clone();
                        }
                    },
                }
            }
        }
    }

    (card_list, lazy_report)
}

async fn create_card_entry(card: scryfall::Card, count_copies: bool) -> (String, Report) {
    // count prints of the same rarity
    let other_prints = card.prints_search_uri;
    let (copies, copy_search_report) = if count_copies {
        let mut copies = 0;
        let mut report = Report::new();
        for _i in 1..MAX_RETRIES {
            let before_time = SystemTime::now();
            let print_list_result = other_prints.fetch_all().await;
            match print_list_result {
                Ok(print_list) => {
                    report = report + Report::card_error(before_time);
                    for reprinted_card in print_list {
                        if reprinted_card.promo_types.is_empty()
                            && reprinted_card.rarity == card.rarity
                        {
                            copies += 1;
                            // the card was reprinted
                        }
                    }
                    break;
                }
                Err(e) => {
                    dbg!(e);

                    // error, try again
                    report = report + Report::card_error(before_time);

                    continue;
                }
            }
        }
        (copies, report)
    } else {
        (1, Report::new())
    };

    let card_name = card.name;
    let name = card_name.split("//").next().unwrap().to_string();

    (copies.to_string() + " " + &name, copy_search_report)
}
