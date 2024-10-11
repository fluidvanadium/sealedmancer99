// use scryfall::card::{Le-ality, Rarity};
// use scryfall::format::Format;
use scryfall::search::prelude::*;
// use scryfall::set::Set;
use scryfall::Error;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;

async fn app() -> Result<u64, Error> {
    let source_path = Path::new("./dmbase.txt");
    let dest_path = Path::new("./draftmancer-sealed99.txt");

    let mut source_file = File::open(source_path).unwrap();
    let mut source_data = String::new();
    source_file.read_to_string(&mut source_data).unwrap();

    let mut dest_file = File::create(dest_path).unwrap();
    dest_file.write_all(source_data.as_bytes()).unwrap();

    let query = Query::Custom(
        "(legal:vintage -t:stickers -o:sticker -o:ticket -o:{TK} (-t:attraction -o:Attraction or name:attraction) -o:commander not:meld -o:draft or (fo:meld)) or (name:/^a-/) or ('commander' -o:'your commander' o:'cast a commander') or 'Stone-Throwing Devils' or 'Pradesh Gypsies' or 'Shahrazad' or 'Downdraft' or 'Backdraft'".to_string(),
    );
    // meld. basics. commander
    // let query = Query::And(vec![proto_query, Query::Custom("r:common".to_string())]);
    println!("query ready");

    let mut cards = Query::And(vec![query.clone(), Query::Custom("not:split".to_string())])
        .search()
        .await?;
    println!("search download completed (not:split)");

    for _ in 0..cards.size_hint().0 {
        dest_file
            .write_all(
                dbg!(cards.next().await.unwrap()?.name)
                    .split("//")
                    .next()
                    .unwrap()
                    .as_bytes(),
            )
            .expect("Unable to write data");
        dest_file
            .write_all("\n".as_bytes())
            .expect("Unable to write data");
    }

    let mut cards = Query::And(vec![query, Query::Custom("is:split".to_string())])
        .search()
        .await?;
    println!("search download completed (split)");
    for _ in 0..cards.size_hint().0 {
        dest_file
            .write_all(dbg!(cards.next().await.unwrap()?.name).as_bytes())
            .unwrap();
        dest_file.write_all("\n".as_bytes()).unwrap();
    }

    Ok(0)
}

#[tokio::main]
async fn main() {
    app().await.unwrap();
}
