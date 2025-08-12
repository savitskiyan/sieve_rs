use std::str::FromStr;

/// Операторы фильтрации
#[derive(Debug, Clone, PartialEq)]
pub enum FilterOperator {
    /// Равно (`==`)
    Equals,
    
    /// Не равно (`!=`)
    NotEquals,
    
    /// Больше (`>`)
    GreaterThan,
    
    /// Меньше (`<`)
    LessThan,
    
    /// Больше или равно (`>=`)
    GreaterThanOrEqualTo,
    
    /// Меньше или равно (`<=`)
    LessThanOrEqualTo,
    
    /// Содержит (`@=`)
    Contains,
    
    /// Начинается с (`_=`)
    StartsWith,
    
    /// Заканчивается на (`_-=`)
    EndsWith,
}

/// Реализация FromStr для FilterOperator
impl FromStr for FilterOperator {
    type Err = String;

    /// Преобразует строку в FilterOperator
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim_end_matches('*') {
            "==" => Ok(FilterOperator::Equals),
            "!=" => Ok(FilterOperator::NotEquals),
            ">" => Ok(FilterOperator::GreaterThan),
            "<" => Ok(FilterOperator::LessThan),
            ">=" => Ok(FilterOperator::GreaterThanOrEqualTo),
            "<=" => Ok(FilterOperator::LessThanOrEqualTo),
            "@=" | "!@=" => Ok(FilterOperator::Contains),
            "_=" | "!_=" => Ok(FilterOperator::StartsWith),
            "_-=" | "!_-=" => Ok(FilterOperator::EndsWith),
            _ => Err(format!("Неизвестный оператор: {}", s))
        }
    }
}
