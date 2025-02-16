use scryfall::format::Format;
use scryfall::search::param::exact;
use scryfall::search::prelude::*;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::thread::sleep;
use std::time::{Duration, SystemTime};
use timestamp::Report;

mod timestamp;

async fn query_to_draftmancer_list(query: &Query) -> (String, Report) {
    println!("query ready");

    let (non_split_cards, non_split_report) = name_strings_for_draftmancer(query, false).await;
    let (split_cards, split_report) = name_strings_for_draftmancer(query, true).await;

    (
        non_split_cards + &split_cards,
        non_split_report + split_report,
    )
}

async fn name_strings_for_draftmancer(query: &Query, splits: bool) -> (String, Report) {
    let mut card_list = "".to_string();

    let complete_query = if splits {
        Query::And(vec![query.clone(), Query::Custom("is:split".to_string())])
    } else {
        Query::And(vec![query.clone(), Query::Custom("not:split".to_string())])
    };

    let mut lazy_report = Report {
        number_of_cards: 0,
        number_of_errors: 0,
        success_sleep_nanos: 0,
        error_sleep_nanos: 0,
        success_server_nanos: 0,
        error_server_nanos: 0,
    };

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
                        let sleep_time: u128 = 50_000_000;
                        sleep(Duration::from_nanos(sleep_time as u64));

                        lazy_report.number_of_cards += 1;
                        lazy_report.success_sleep_nanos += sleep_time;
                        lazy_report.success_server_nanos += lookup_time;

                        backup_cards = cards.clone();

                        let card_name = card.name;
                        let name = if splits {
                            card_name
                        } else {
                            card_name.split("//").next().unwrap().to_string()
                        };
                        println!("{lookup_time} > {name}");
                        card_list = card_list
                            + "
" + &name;
                    }
                    Err(e) => {
                        dbg!(e);

                        let sleep_time: u128 = 5_000_000_000;
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

#[tokio::main]
async fn main() {
    // dbg!(exact("Dungeon Delver").search().await.unwrap().next().await);

    std::env::set_current_dir("results").unwrap();

    let vintage_taste_ban = Query::Or(vec![
        Query::And(vec![
            format(Format::Vintage), //
            not(exact("Hobble")),    //
        ]),
        exact("Stone-Throwing Devils"),
        exact("Pradesh Gypsies"),
        exact("Shahrazad"),
    ]);

    let basics = type_line("basic".to_string());
    let draft_involved = Query::And(vec![
        oracle_text("draft"), //
        not(name("draft")),   //
    ]);
    let meld_duds = Query::And(vec![
        CardIs::Meld.into(),           //
        not(full_oracle_text("meld")), //
    ]);
    let unfun = Query::And(vec![
        Query::Or(vec![
            full_oracle_text("sticker"),    //
            full_oracle_text("ticket"),     //
            full_oracle_text("{TK}"),       //
            full_oracle_text("attraction"), //
        ]),
        not(exact("Ticket Tortoise")),
        not(name("Ticket Booth")),
        not(name("Fatal Attraction")),
    ]);
    let commander_synergy = Query::And(vec![
        oracle_text("commander"),                       //
        not(name("commander")),                         //
        not(full_oracle_text("can be your commander")), //
    ]);
    let rebalanced = name(Regex::from(r"^A-"));
    let conspiracy = type_line("conspiracy");
    let format = [
        ("./basics.txt".to_string(), basics.clone()),
        ("./draft_involved.txt".to_string(), draft_involved.clone()),
        ("./meld_duds.txt".to_string(), meld_duds.clone()),
        ("./unfun.txt".to_string(), unfun.clone()),
        (
            "./commander_synergy.txt".to_string(),
            commander_synergy.clone(),
        ),
        ("./rebalanced.txt".to_string(), rebalanced.clone()),
        // equal access low choice
        (
            "./for-subset-draft.txt".to_string(),
            Query::Or(vec![
                rebalanced.clone(),
                Query::And(vec![
                    vintage_taste_ban.clone(),
                    not(basics.clone()),
                    not(meld_duds.clone()),
                    not(unfun.clone()),
                    //
                    not(commander_synergy.clone()),
                    // not(draft_involved.clone()),
                ]),
                conspiracy.clone(),
            ]),
        ),
        // equal access high choice
        (
            "./for-subset-constructed.txt".to_string(),
            Query::Or(vec![
                rebalanced.clone(),
                Query::And(vec![
                    vintage_taste_ban.clone(),
                    not(basics.clone()),
                    not(meld_duds.clone()),
                    not(unfun.clone()),
                    //
                    // not(commander_synergy.clone()),
                    // not(draft_involved.clone()),
                ]),
                conspiracy.clone(),
            ]),
        ),
        // unequal access low choice.
        // also appropriate for fundamental magic
        (
            "./for-subset-sealed.txt".to_string(),
            Query::Or(vec![
                rebalanced.clone(),
                Query::And(vec![
                    vintage_taste_ban.clone(),
                    not(basics.clone()),
                    not(meld_duds.clone()),
                    not(unfun.clone()),
                    //
                    not(commander_synergy.clone()),
                    not(draft_involved.clone()),
                ]),
                // conspiracy.clone(),
            ]),
        ),
        // unequal access high choice
        (
            "./for-subset-allstars.txt".to_string(),
            Query::Or(vec![
                rebalanced.clone(),
                Query::And(vec![
                    vintage_taste_ban.clone(),
                    not(basics.clone()),
                    not(meld_duds.clone()),
                    not(unfun.clone()),
                    //
                    // not(commander_synergy.clone()),
                    // not(draft_involved.clone()),
                ]),
                // conspiracy.clone(),
            ]),
        ),
    ];
    for (name, query) in format.iter()
    // let index = 6;
    // let (destination_filename, query) = format[index].clone();
    {
        let (list, report) = query_to_draftmancer_list(query).await;

        let var_name = "lists/".to_string() + name.as_str();
        let list_path = Path::new(var_name.as_str());
        let var_name = "lists/".to_string() + name.as_str();
        let report_path = Path::new(var_name.as_str());
        let mut list_file = File::create(list_path).unwrap();
        let mut report_file = File::create(report_path).unwrap();

        list_file
            .write_all(list.as_bytes())
            .expect("Unable to write data");
        report_file
            .write_all(report.to_string().as_bytes())
            .expect("Unable to write data");
    }
}
