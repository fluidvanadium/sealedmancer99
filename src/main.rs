use scryfall::search::prelude::*;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::SystemTime;
use timestamp::Report;

mod instructions;
use crate::instructions::CountCopiesConfig;
use crate::instructions::DiscreteQuery;
use crate::instructions::Order;

mod timestamp;
const MAX_RETRIES: usize = 4;

mod specifics;

#[tokio::main]
async fn main() {
    std::env::set_current_dir("results").unwrap();

    // for order in specifics::test_formats() {
    //     write_query_to_file(order).await;
    // }

    // write_query_to_file(Order::from_parts(
    //     "./test_promo_count.txt",
    //     DiscreteQuery::from_parts(
    //         specifics::vintage_taste_ban(),
    //         Some(CountCopiesConfig::default()),
    //     ),
    // ))
    // .await;

    write_query_to_file(specifics::for_twostep_prologue()).await;
    write_query_to_file(specifics::for_twostep_white()).await;
    write_query_to_file(specifics::for_twostep_blue()).await;
    write_query_to_file(specifics::for_twostep_black()).await;
    write_query_to_file(specifics::for_twostep_red()).await;
    write_query_to_file(specifics::for_twostep_green()).await;

    write_query_to_file(specifics::for_draft()).await;
    write_query_to_file(specifics::for_constructed()).await;
    write_query_to_file(specifics::for_sealed()).await;
    write_query_to_file(specifics::for_allstars()).await;
}

async fn write_query_to_file(order: Order) {
    let query = &order.discrete_query;
    let destination_filename = &order.destination_file;
    println!("Beginning fetch of {query} to store in file {destination_filename}.");
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

            loop {
                let backup_cards = cards.clone();

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

async fn create_card_entry(
    card: scryfall::Card,
    cc: Option<CountCopiesConfig>,
) -> (String, Report) {
    // count prints of the same rarity
    let other_prints = card.prints_search_uri;
    let (copies, copy_search_report) = if let Some(count_copies_config) = cc {
        let mut copies = 0;
        let mut sets_found = vec![];
        let mut report = Report::new();
        for _i in 1..MAX_RETRIES {
            let before_time = SystemTime::now();
            let print_list_result = other_prints.fetch_all().await;
            match print_list_result {
                Ok(print_list) => {
                    report = report + Report::card_success(before_time);
                    'reprint: for reprinted_card in print_list {
                        dbg!(reprinted_card.name);
                        // the card was reprinted
                        if count_copies_config.only_same_rarity
                            && reprinted_card.rarity != card.rarity
                        {
                            continue;
                        }
                        if count_copies_config.only_different_sets {
                            dbg!(card.variation_of);
                            let this_set = card.set_id.clone();
                            for set in &sets_found {
                                if *set == this_set {
                                    println!("duplicate from set {this_set}. to next print.");
                                    continue 'reprint;
                                }
                            }
                            sets_found.push(this_set);
                        }
                        println!("found a copy.");
                        copies += 1;
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
