use scryfall::{
    format::Format,
    search::{
        param::exact,
        prelude::{format, full_oracle_text, name, oracle_text, type_line, CardIs, Regex},
        query::{not, Query},
    },
};

pub(crate) fn vintage_taste_ban() -> Query {
    Query::Or(vec![
        Query::And(vec![
            format(Format::Vintage), //
            not(exact("Hobble")),    //
        ]),
        exact("Stone-Throwing Devils"),
        exact("Pradesh Gypsies"),
        exact("Shahrazad"),
    ])
}

pub(crate) fn basics() -> Query {
    type_line("basic")
}
pub(crate) fn draft_involved() -> Query {
    Query::And(vec![
        oracle_text("draft"), //
        not(name("draft")),   //
    ])
}
pub(crate) fn meld_duds() -> Query {
    Query::And(vec![
        CardIs::Meld.into(),           //
        not(full_oracle_text("meld")), //
    ])
}
pub(crate) fn unfun() -> Query {
    Query::And(vec![
        Query::Or(vec![
            full_oracle_text("sticker"),    //
            full_oracle_text("ticket"),     //
            full_oracle_text("{TK}"),       //
            full_oracle_text("attraction"), //
        ]),
        not(exact("Ticket Tortoise")),
        not(name("Ticket Booth")),
        not(name("Fatal Attraction")),
    ])
}
// often a dud in limited
pub(crate) fn commander_synergy() -> Query {
    Query::And(vec![
        oracle_text("commander"),                       //
        not(name("commander")),                         //
        not(full_oracle_text("can be your commander")), //
    ])
}
pub(crate) fn rebalanced() -> Query {
    name(Regex::from(r"^A-"))
}
pub(crate) fn conspiracy() -> Query {
    type_line("conspiracy")
}

pub(crate) fn default_formats() -> [(&'static str, scryfall::search::prelude::Query); 10] {
    [
        ("./basics.txt", basics()),
        ("./draft_involved.txt", draft_involved()),
        ("./meld_duds.txt", meld_duds()),
        ("./unfun.txt", unfun()),
        ("./commander_synergy.txt", commander_synergy()),
        ("./rebalanced.txt", rebalanced()),
        // equal access low choice
        (
            // 6
            "./for-subset-draft.txt",
            Query::Or(vec![
                rebalanced(),
                Query::And(vec![
                    vintage_taste_ban(),
                    not(basics()),
                    not(meld_duds()),
                    not(unfun()),
                    //
                    not(commander_synergy()),
                    // not(draft_involved()),
                ]),
                conspiracy(),
            ]),
        ),
        // equal access high choice
        (
            "./for-subset-constructed.txt",
            Query::Or(vec![
                rebalanced(),
                Query::And(vec![
                    vintage_taste_ban(),
                    not(basics()),
                    not(meld_duds()),
                    not(unfun()),
                    //
                    // not(commander_synergy()),
                    not(draft_involved()),
                ]),
                conspiracy(),
            ]),
        ),
        // unequal access low choice.
        // also appropriate for fundamental magic
        (
            "./for-subset-sealed.txt",
            Query::Or(vec![
                rebalanced(),
                Query::And(vec![
                    vintage_taste_ban(),
                    not(basics()),
                    not(meld_duds()),
                    not(unfun()),
                    //
                    not(commander_synergy()),
                    not(draft_involved()),
                ]),
                // conspiracy(),
            ]),
        ),
        // unequal access high choice
        (
            "./for-subset-allstars.txt",
            Query::Or(vec![
                rebalanced(),
                Query::And(vec![
                    vintage_taste_ban(),
                    not(basics()),
                    not(meld_duds()),
                    not(unfun()),
                    //
                    // not(commander_synergy()),
                    not(draft_involved()),
                ]),
                // conspiracy(),
            ]),
        ),
    ]
}

mod example_formats;
