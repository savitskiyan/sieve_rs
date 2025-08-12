pub mod sieve_regex;
pub mod sieve_model;
pub mod filter_term;
pub mod filter_operator;
pub mod sort_order;
pub mod sort_term;
pub mod paged_list;

pub use sieve_regex::COMMA_PATTERN;
pub use filter_term::FilterTerm;
pub use filter_operator::FilterOperator;
pub use sieve_model::SieveModel;
pub use sort_order::SortOrder;
pub use sort_term::SortTerm;
pub use paged_list::PagedList;

pub trait SieveFilter {
    type Output;

    fn filter_with(self, sieve: &SieveModel) -> Self::Output;
}

pub trait SieveOrder {
    type Output;

    fn order_with(self, sieve: &SieveModel) -> Self::Output;
}

pub trait SievePaginate {
    type Output;

    fn paginate_with(self, sieve: &SieveModel) -> Self::Output;
}