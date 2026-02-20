use scryfall::{
    format::Format,
    search::{
        param::exact,
        prelude::{
            color_identity, format, full_oracle_text, gte, name, oracle_text, type_line, CardIs,
            Regex,
        },
        query::{not, Query},
    },
};

use crate::instructions::{CountCopiesConfig, DiscreteQuery, Order};

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

pub(crate) fn test_formats() -> Vec<Order> {
    vec![
        Order::from_parts("./basics.txt", DiscreteQuery::from_parts(basics(), None)),
        Order::from_parts(
            "./draft_involved.txt",
            DiscreteQuery::from_parts(draft_involved(), Some(CountCopiesConfig::default())),
        ),
        Order::from_parts(
            "./meld_duds.txt",
            DiscreteQuery::from_parts(meld_duds(), Some(CountCopiesConfig::default())),
        ),
        Order::from_parts(
            "./unfun.txt",
            DiscreteQuery::from_parts(unfun(), Some(CountCopiesConfig::default())),
        ),
        Order::from_parts(
            "./commander_synergy.txt",
            DiscreteQuery::from_parts(commander_synergy(), Some(CountCopiesConfig::default())),
        ),
        Order::from_parts(
            "./conspiracy.txt",
            DiscreteQuery::from_parts(conspiracy(), Some(CountCopiesConfig::default())),
        ),
        Order::from_parts(
            "./rebalanced.txt",
            DiscreteQuery::from_parts(rebalanced(), None),
        ),
    ]
}

pub(crate) fn for_draft() -> Order {
    // equal access low choice
    Order::from_parts(
        "./for-subset-draft.txt",
        DiscreteQuery::from_parts(
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
            Some(CountCopiesConfig::default()),
        ),
    )
}
pub(crate) fn for_constructed() -> Order {
    // equal access high choice
    Order::from_parts(
        "./for-subset-constructed.txt",
        DiscreteQuery::from_parts(
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
            Some(CountCopiesConfig::default()),
        ),
    )
}
pub(crate) fn for_sealed() -> Order {
    // unequal access low choice.
    // also appropriate for fundamental magic
    Order::from_parts(
        "./for-subset-sealed.txt",
        DiscreteQuery::from_parts(
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
            Some(CountCopiesConfig::default()),
        ),
    )
}
pub(crate) fn for_allstars() -> Order {
    // unequal access high choice
    Order::from_parts(
        "./for-subset-allstars.txt",
        DiscreteQuery::from_parts(
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
            Some(CountCopiesConfig::default()),
        ),
    )
}

pub(crate) fn default_plus(res: Vec<Query>) -> DiscreteQuery {
    let mut resa = res;
    resa.push(Query::Or(vec![
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
        conspiracy(),
    ]));
    DiscreteQuery::from_parts(Query::And(resa), Some(CountCopiesConfig::default()))
}

pub(crate) fn for_twostep_prologue() -> Order {
    // equal access low choice
    Order::from_parts(
        "./for-twostep_prologue.txt",
        default_plus(vec![color_identity("0")]),
    )
}
pub(crate) fn for_twostep_white() -> Order {
    // equal access low choice
    Order::from_parts(
        "./for-twostep_white.txt",
        default_plus(vec![color_identity(gte("W"))]),
    )
}
pub(crate) fn for_twostep_blue() -> Order {
    // equal access low choice
    Order::from_parts(
        "./for-twostep_blue.txt",
        default_plus(vec![color_identity(gte("U"))]),
    )
}
pub(crate) fn for_twostep_black() -> Order {
    // equal access low choice
    Order::from_parts(
        "./for-twostep_black.txt",
        default_plus(vec![color_identity(gte("B"))]),
    )
}
pub(crate) fn for_twostep_red() -> Order {
    // equal access low choice
    Order::from_parts(
        "./for-twostep_red.txt",
        default_plus(vec![color_identity(gte("R"))]),
    )
}
pub(crate) fn for_twostep_green() -> Order {
    // equal access low choice
    Order::from_parts(
        "./for-twostep_green.txt",
        default_plus(vec![color_identity(gte("G"))]),
    )
}

mod example_formats;
