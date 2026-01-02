use scryfall::card::Rarity;
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
    println!("query ready: {}", query.to_string());

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
                        // dont overload the api rate limit
                        let sleep_time: u128 = 100_000;
                        sleep(Duration::from_nanos(sleep_time as u64));

                        lazy_report.number_of_cards += 1;
                        lazy_report.success_sleep_nanos += sleep_time;
                        lazy_report.success_server_nanos += lookup_time;

                        // in case of error
                        backup_cards = cards.clone();

                        // count prints of the same rarity
                        let other_prints = card.prints_search_uri;
                        let mut copies = 0;
                        let print_list = other_prints.fetch_all().await;
                        match print_list {
                            Err(e) => {
                                // same script as below. helperize?
                                dbg!(e);

                                let sleep_time: u128 = 1_000_000_000;
                                sleep(Duration::from_nanos(sleep_time as u64));

                                lazy_report.number_of_errors += 1;
                                lazy_report.error_sleep_nanos += sleep_time;
                                lazy_report.error_server_nanos += lookup_time;

                                cards = backup_cards.clone();
                            }
                            Ok(print_list_success) => {
                                for reprinted_card in print_list_success {
                                    if reprinted_card.promo_types.is_empty()
                                        && reprinted_card.rarity == card.rarity
                                    {
                                        copies += 1
                                        // the card was reprinted
                                    }
                                }
                            }
                        }

                        // if let Some(text) = card.oracle_text {
                        //     if text.contains("elf")
                        //         || text.contains("elves")
                        //         || text.contains("zombie")
                        //         || text.contains("goblin")
                        //         || text.contains("merfolk")
                        //         || text.contains("human")
                        //     {
                        //         copies *= 2;
                        //     }
                        // }

                        let card_name = card.name;
                        let name = if splits {
                            card_name
                        } else {
                            card_name.split("//").next().unwrap().to_string()
                        };

                        let new_entry = 1.to_string() + " " + &name;

                        let now = timestamp::now_string();
                        println!("{now} . {lookup_time} > {new_entry}");

                        card_list = card_list
                            + "
" + &new_entry;
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
    // often a dud in limited
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
            // 6
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
                    not(draft_involved.clone()),
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
                    not(draft_involved.clone()),
                ]),
                // conspiracy.clone(),
            ]),
        ),
        // 14 card pickup decks
        (
            "./tdm.txt".to_string(),
            Query::And(vec![set("TDM"), not(type_line("basic"))]),
        ),
        (
            "./dft.txt".to_string(),
            Query::And(vec![set("DFT"), not(type_line("basic"))]),
        ),
        (
            "./fdn.txt".to_string(),
            Query::And(vec![set("FDN"), not(type_line("basic"))]),
        ),
        (
            "./apc.txt".to_string(),
            Query::And(vec![set("APC"), not(type_line("basic"))]),
        ),
        (
            "./dsk.txt".to_string(),
            Query::And(vec![set("DSK"), not(type_line("basic"))]),
        ),
        (
            "./block_rav.txt".to_string(),
            Query::And(vec![block("RAV"), not(type_line("basic"))]),
        ),
    ];
    let format = [
        (
            "./fin-common.txt".to_string(),
            Query::And(vec![
                set("FIN"),
                not(type_line("basic")),
                rarity(Rarity::Common),
            ]),
        ),
        (
            "./fin-uncommon.txt".to_string(),
            Query::And(vec![
                set("FIN"),
                not(type_line("basic")),
                rarity(Rarity::Uncommon),
            ]),
        ),
        (
            "./fin-rare.txt".to_string(),
            Query::And(vec![
                set("FIN"),
                not(type_line("basic")),
                rarity(Rarity::Rare),
            ]),
        ),
        (
            "./fin-mythic.txt".to_string(),
            Query::And(vec![
                set("FIN"),
                not(type_line("basic")),
                rarity(Rarity::Mythic),
            ]),
        ),
    ];
    let format = [
        (
            "./commanders.txt".to_string(),
            Query::And(vec![
                type_line("legendary"),
                type_line("creature"),
            ]),
        ),
    ];
    let formargt = [
        (
            "./for-subset-draft-Bonus.txt".to_string(),
            Query::Or(vec![
                Query::And(vec![rarity(Rarity::Bonus), rebalanced.clone()]),
                Query::And(vec![
                    rarity(Rarity::Bonus),
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
        (
            "./for-subset-draft-Mythic.txt".to_string(),
            Query::Or(vec![
                Query::And(vec![rarity(Rarity::Mythic), rebalanced.clone()]),
                Query::And(vec![
                    rarity(Rarity::Mythic),
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
        (
            "./for-subset-draft-Special.txt".to_string(),
            Query::Or(vec![
                Query::And(vec![rarity(Rarity::Special), rebalanced.clone()]),
                Query::And(vec![
                    rarity(Rarity::Special),
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
        (
            "./for-subset-draft-Rare.txt".to_string(),
            Query::Or(vec![
                Query::And(vec![rarity(Rarity::Rare), rebalanced.clone()]),
                Query::And(vec![
                    rarity(Rarity::Rare),
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
        (
            "./for-subset-draft-Uncommon.txt".to_string(),
            Query::Or(vec![
                Query::And(vec![rarity(Rarity::Uncommon), rebalanced.clone()]),
                Query::And(vec![
                    rarity(Rarity::Uncommon),
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
        (
            "./for-subset-draft-Common.txt".to_string(),
            Query::Or(vec![
                Query::And(vec![rarity(Rarity::Common), rebalanced.clone()]),
                Query::And(vec![
                    rarity(Rarity::Common),
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
    ];
    for (destination_filename, der_query) in format.iter() {
        // let index = 6;
        // let (destination_filename, der_query) = format[index].clone();
        let query = &der_query;
        let (list, report) = query_to_draftmancer_list(query).await;

        let list_path_name = "lists/".to_string() + destination_filename.as_str();
        let list_path = Path::new(list_path_name.as_str());
        let report_path_name = "reports/".to_string() + destination_filename.as_str();
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
}
