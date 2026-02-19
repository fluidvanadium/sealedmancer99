use scryfall::search::prelude::*;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::thread::sleep;
use std::time::{Duration, SystemTime};
use timestamp::Report;

mod timestamp;

async fn query_split_list(query: &Query) -> (String, Report) {
    println!("query ready: {}", query);

    let (non_split_cards, non_split_report) = download_query(query, false).await;
    let (split_cards, split_report) = download_query(query, true).await;

    (
        non_split_cards + &split_cards,
        non_split_report + split_report,
    )
}

async fn download_query(query: &Query, splits: bool) -> (String, Report) {
    let mut card_list = "".to_string();

    let complete_query = if splits {
        Query::And(vec![query.clone(), Query::Custom("is:split".to_string())])
    } else {
        Query::And(vec![query.clone(), Query::Custom("not:split".to_string())])
    };

    let mut lazy_report = Report::new();

    if let Ok(mut cards) = complete_query.clone().search().await {
        println!("search download completed (splits = {splits})");
        let mut backup_cards = cards.clone();

        loop {
            let before_time = SystemTime::now();

            let next_card = cards.next().await;

            let lookup_time = SystemTime::now()
                .duration_since(before_time)
                .unwrap()
                .as_nanos();

            match next_card {
                None => {
                    println!("no more cards");
                    break;
                }
                Some(card_result) => match card_result {
                    Ok(card) => {
                        // dont overload the api rate limit
                        let sleep_time: u128 = 100_000;
                        sleep(Duration::from_nanos(sleep_time as u64));

                        // in case of error
                        // backup_cards = cards.clone();

                        // let now = timestamp::now_string();
                        // println!("{now} . {lookup_time} > {new_entry}");

                        card_list = card_list
                            + "
" + create_card_entry(card, splits, true)
                            .await
                            .unwrap()
                            .as_str();
                    }
                    Err(e) => {
                        dbg!(e);

                        let sleep_time: u128 = 1_000_000_000;
                        sleep(Duration::from_nanos(sleep_time as u64));

                        lazy_report.number_of_errors += 1;
                        lazy_report.error_sleep_nanos += sleep_time;
                        lazy_report.error_server_nanos += lookup_time;

                        cards = backup_cards.clone();
                    }
                },
            }
        }
    }

    (card_list, lazy_report)
}

async fn create_card_entry(
    card: scryfall::Card,
    truncate_splits: bool,
    count_copies: bool,
) -> Result<String, scryfall::Error> {
    // count prints of the same rarity
    let other_prints = card.prints_search_uri;
    let mut copies = 0;
    let print_list = other_prints.fetch_all().await?;
    for reprinted_card in print_list {
        if reprinted_card.promo_types.is_empty() && reprinted_card.rarity == card.rarity {
            copies += 1;
            // the card was reprinted
        }
    }

    let card_name = card.name;
    let name = if truncate_splits {
        card_name
    } else {
        card_name.split("//").next().unwrap().to_string()
    };

    if !count_copies {
        copies = 1;
    }

    Ok(copies.to_string() + " " + &name)
}

#[tokio::main]
async fn main() {
    // dbg!(exact("Dungeon Delver").search().await.unwrap().next().await);

    std::env::set_current_dir("results").unwrap();

    // 14 card pickup decks
    for (destination_filename, der_query) in query_operations::default_formats() {
        write_query_to_file(destination_filename, &der_query).await;
    }
}

async fn write_query_to_file(destination_filename: &str, der_query: &Query) {
    // let index = 6;
    // let (destination_filename, der_query) = format[index].clone();
    let query = &der_query;
    let (list, report) = query_split_list(query).await;

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

mod query_operations;
