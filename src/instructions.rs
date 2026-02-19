use scryfall::search::query::Query;

#[derive(derive_more::Display)]
#[display("with count_copies={count_copies}. {query}")]
pub(crate) struct DiscreteQuery {
    pub query: Query,
    pub count_copies: bool,
}

impl DiscreteQuery {
    pub fn from_parts(query: Query, count_copies: bool) -> Self {
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
