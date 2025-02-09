use scryfall::format::Format;
use scryfall::search::param;
use scryfall::search::param::exact;
// use scryfall::format::Format;
use scryfall::search::prelude::*;
use scryfall::Card;
// use scryfall::set::Set;
use scryfall::Error;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;

async fn write_query(destination_filename: &String, query: &Query) -> Result<u64, Error> {
    let dest_path = Path::new(destination_filename.as_str());

    let mut dest_file = File::create(dest_path).unwrap();

    // meld. basics. commander
    // let query = Query::And(vec![proto_query, Query::Custom("r:common".to_string())]);
    println!("query ready");

    let mut cards = Query::And(vec![query.clone(), Query::Custom("not:split".to_string())])
        .search()
        .await
        .unwrap();
    println!("search download completed (not:split)");

    for _ in 0..cards.size_hint().0 {
        let next_card = cards.next().await;
        let card_name_result = process_next_card(&next_card).await;

        match card_name_result {
            Ok(card_name) => {
                dest_file
                    .write_all(card_name.split("//").next().unwrap().as_bytes())
                    .expect("Unable to write data");
                dest_file
                    .write_all("\n".as_bytes())
                    .expect("Unable to write data");
            }
            Err(err) => {
                dest_file
                    .write_all(err.to_string().as_bytes())
                    .expect("Unable to write data");
                break;
            }
        }
    }

    if let Ok(mut cards) = Query::And(vec![query.clone(), Query::Custom("is:split".to_string())])
        .search()
        .await
    {
        println!("search download completed (split)");
        for _ in 0..cards.size_hint().0 {
            let next_card = cards.next().await;
            let card_name_result = process_next_card(&next_card).await;

            match card_name_result {
                Ok(card_name) => {
                    dest_file
                        .write_all(card_name.as_bytes())
                        .expect("Unable to write data");
                    dest_file
                        .write_all("\n".as_bytes())
                        .expect("Unable to write data");
                }
                Err(err) => {
                    dest_file
                        .write_all(err.to_string().as_bytes())
                        .expect("Unable to write data");
                    break;
                }
            }
        }
    }

    Ok(0)
}

async fn process_next_card(card: &Option<Result<Card, Error>>) -> Result<String, String> {
    let v1 = card.as_ref().unwrap();
    let v2 = v1.as_ref().map_err(|e| e.to_string())?;
    let name = v2.name.clone();
    println!("{name}");
    Ok(name)
    // Ok(match name.split("//") {})
}

#[tokio::main]
async fn main() {
    let basics = type_line("basic".to_string());
    let draft_duds = Query::And(vec![
        oracle_text("draft"), //
        not(name("draft")),   //
    ]);
    let format = [
        // ("./draftmancer-for-subset-constructed.txt".to_string(),
        //     Query::Custom("(legal:vintage -t:stickers -o:sticker -o:ticket -o:{TK} (-t:attraction -o:Attraction or name:attraction) (-o:draft or 'draft') -t:basic -(-fo:meld is:meld)) or (name:/^a-/) or 'Stone-Throwing Devils' or 'Pradesh Gypsies' or 'Shahrazad'".to_string())),
        ("./basics.txt".to_string(), basics.clone()),
        ("./excluded_draft_duds.txt".to_string(), draft_duds.clone()),
        (
            "./draftmancer-for-subset-fundamental.txt".to_string(),
            Query::And(vec![
                format(Format::Vintage),
                not(draft_duds),
                not(basics),
                set("bfz"),                             // A `Param` variant.
                rarity(scryfall::card::Rarity::Mythic), // A `Param` variant.
                CardIs::OddCmc.into(),
            ]),
        ),
    ];
    for (dest_filename, query) in format.iter() {
        write_query(dest_filename, query).await.unwrap();
    }
}
