use std::str::FromStr;
use super::filter_operator::FilterOperator;

/// Термин фильтрации, представляющий одно условие фильтрации
/// 
/// # Примеры
/// ```
/// let filter = FilterTerm::from_str("title@=Rock");
/// assert_eq!(filter.names(), vec!["title"]);
/// assert_eq!(filter.operator(), FilterOperator::Contains);
/// assert_eq!(filter.values(), vec!["Rock"]);
/// ```
#[derive(Debug, Clone)]
pub struct FilterTerm {
    /// Имена полей для фильтрации (может быть несколько при использовании |)
    names: Vec<String>,

    /// Значения для фильтрации (может быть несколько при использовании |)
    values: Vec<String>,

    /// Оператор фильтрации
    operator: FilterOperator,

    /// Флаг регистронезависимого поиска
    case_insensitive: bool,
}

/// Реализация методов для FilterTerm
impl FilterTerm {
    /// Возвращает имена полей для фильтрации
    /// 
    /// # Примеры
    /// - `["title"]` - фильтрация по названию
    /// - `["artist"]` - фильтрация по исполнителю
    /// - `["album"]` - фильтрация по альбому
    /// - `["duration"]` - фильтрация по длительности
    /// - `["created_time"]` - фильтрация по дате создания
    pub fn names(&self) -> Vec<String> {
        self.names.clone()
    }

    /// Возвращает значения для фильтрации
    /// 
    /// # Примеры
    /// - `["Rock"]` - фильтрация по названию
    /// - `["Queen"]` - фильтрация по исполнителю
    /// - `["Rock, Metal"]` - фильтрация по названию
    pub fn values(&self) -> Vec<String> {
        self.values.clone()
    }

    /// Возвращает оператор фильтрации
    /// 
    /// # Примеры
    /// - `FilterOperator::Contains` - фильтрация по содержанию
    /// - `FilterOperator::Equals` - фильтрация по точному совпадению
    /// - `FilterOperator::NotEquals` - фильтрация по неравенству
    pub fn operator(&self) -> FilterOperator {
        self.operator.clone()
    }

    /// Возвращает флаг регистронезависимого поиска
    /// 
    /// # Примеры
    /// - `true` - регистронезависимый поиск
    /// - `false` - регистрозависимый поиск
    pub fn case_insensitive(&self) -> bool {
        self.case_insensitive
    }
}

impl FromStr for FilterTerm {
    type Err = String;

    /// Преобразует строку в FilterTerm
    /// 
    /// # Примеры
    /// ```
    /// let filter = FilterTerm::from_str("title@=Rock");
    /// assert_eq!(filter.names(), vec!["title"]);
    /// assert_eq!(filter.operator(), FilterOperator::Contains);
    /// assert_eq!(filter.values(), vec!["Rock"]);
    /// 
    /// let filter = FilterTerm::from_str("title==Rock|Pop");
    /// assert_eq!(filter.names(), vec!["title"]);
    /// assert_eq!(filter.operator(), FilterOperator::Equals);
    /// assert_eq!(filter.values(), vec!["Rock", "Pop"]);
    fn from_str(filter: &str) -> Result<Self, Self::Err> {
        let filter = filter.trim();
        if filter.is_empty() {
            return Err("Filter is empty".to_string());
        }

        // Проверяем на множественные поля (поле1|поле2)
        let (names, rest) = if filter.starts_with('(') && filter.contains(')') {
            let end = filter.find(')').unwrap();
            let fields = &filter[1..end];
            let names = fields.split('|').map(|s| s.trim().to_string()).collect();
            (names, &filter[end + 1..])
        } else {
            let mut names = Vec::new();
            let end = filter.find(|c| "=!<>@_-".contains(c)).unwrap_or(filter.len());
            names.push(filter[..end].trim().to_string());
            (names, &filter[end..])
        };

        // Находим оператор
        let operator = if rest.starts_with("==") {
            "=="
        } else if rest.starts_with("!=") {
            "!="
        } else if rest.starts_with(">=") {
            ">="
        } else if rest.starts_with("<=") {
            "<="
        } else if rest.starts_with('>') {
            ">"
        } else if rest.starts_with('<') {
            "<"
        } else if rest.starts_with("@=") {
            "@="
        } else if rest.starts_with("_=") {
            "_="
        } else if rest.starts_with("_-=") {
            "_-="
        } else {
            "=="
        };

        // Получаем значение после оператора
        let value_str = &rest[operator.len()..];
        
        // Разбираем множественные значения (значение1|значение2)
        let values = value_str.split('|')
            .map(|s| s.trim().to_string())
            .collect();

        Ok(FilterTerm {
            names,
            values,
            operator: FilterOperator::from_str(operator).unwrap_or(FilterOperator::Equals),
            case_insensitive: operator.ends_with('*') || operator == "@="
        })
    }
}