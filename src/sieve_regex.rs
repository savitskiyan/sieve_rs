use regex::Regex;

// Регулярное выражение для разделения строки по запятой
lazy_static::lazy_static! {
    /// Регулярное выражение для разделения фильтров по запятой
    pub static ref COMMA_PATTERN: Regex = Regex::new(r",\s*").unwrap();
    
    // /// Регулярное выражение для определения операторов фильтрации
    // pub static ref OPERATORS_REGEX: Regex = Regex::new(r"(!@=\*|!_=\*|!_-=\*|!=\*|!@=|!_=|!_-=|==\*|@=\*|_=\*|_-=\*|==|!=|>=|<=|>|<|@=|_=|_-=)").unwrap();
}