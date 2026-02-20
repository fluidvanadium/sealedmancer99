use scryfall::search::query::Query;

#[derive(Clone)] //
#[derive(Copy)] //
#[derive(Debug)]
pub(crate) struct CountCopiesConfig {
    /// ignore copies with different from the selected card
    pub only_same_rarity: bool,
    /// ignore copies with the same set as each-other
    pub only_different_sets: bool,
}
impl CountCopiesConfig {
    pub fn from_parts(same_rarity: bool, different_sets: bool) -> Self {
        CountCopiesConfig {
            only_same_rarity: same_rarity,
            only_different_sets: different_sets,
        }
    }
}
impl Default for CountCopiesConfig {
    fn default() -> Self {
        Self {
            only_same_rarity: false,
            only_different_sets: true,
        }
    }
}

#[derive(derive_more::Display)]
#[display("with count_copies=[{count_copies:#?}]. {query}")]
pub(crate) struct DiscreteQuery {
    pub query: Query,
    pub count_copies: Option<CountCopiesConfig>,
}
impl DiscreteQuery {
    pub fn from_parts(query: Query, count_copies: Option<CountCopiesConfig>) -> Self {
        DiscreteQuery {
            query,
            count_copies,
        }
    }
}

#[derive(derive_more::Display)]
#[display("save to file {destination_file} {discrete_query}")]
pub(crate) struct Order {
    pub destination_file: &'static str,
    pub discrete_query: DiscreteQuery,
}
impl Order {
    pub fn from_parts(destination_file: &'static str, discrete_query: DiscreteQuery) -> Self {
        Order {
            destination_file,
            discrete_query,
        }
    }
}
