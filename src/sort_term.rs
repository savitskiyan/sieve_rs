use std::str::FromStr;
use super::sort_order::SortOrder;

/// Термин сортировки, представляющий одно условие сортировки
/// 
/// # Примеры
/// ```
/// let sort = SortTerm::from_str("-created_time");
/// assert_eq!(sort.name(), "created_time");
/// assert_eq!(sort.order(), SortOrder::Descending);
/// ```
#[derive(Debug, Clone)]
pub struct SortTerm {
    /// Имя поля для сортировки
    /// 
    /// # Примеры
    /// - `"title"` - сортировка по названию
    /// - `"artist"` - сортировка по исполнителю
    /// - `"album"` - сортировка по альбому
    /// - `"duration"` - сортировка по длительности
    /// - `"created_time"` - сортировка по дате создания
    /// - `"file_size"` - сортировка по размеру файла
    name: String,
    
    /// Порядок сортировки
    /// 
    /// # Примеры
    /// - `SortOrder::Ascending` - сортировка по возрастанию
    /// - `SortOrder::Descending` - сортировка по убыванию
    order: SortOrder,
}

/// Реализует методы для SortTerm
impl SortTerm {

    /// Возвращает имя поля для сортировки
    /// 
    /// # Примеры
    /// - `"title"` - сортировка по названию
    /// - `"artist"` - сортировка по исполнителю
    /// - `"album"` - сортировка по альбому
    /// - `"duration"` - сортировка по длительности
    /// - `"created_time"` - сортировка по дате создания
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// Возвращает порядок сортировки
    /// 
    /// # Примеры
    /// - `SortOrder::Ascending` - сортировка по возрастанию
    /// - `SortOrder::Descending` - сортировка по убыванию
    pub fn order(&self) -> SortOrder {
        self.order.clone()
    }
}

/// Реализует методы для SortTerm
impl FromStr for SortTerm {
    type Err = String;

    /// Создает новый экземпляр SortTerm из строки сортировки
    /// 
    /// # Примеры
    /// ```
    /// let sort = SortTerm::new("-created_time");
    /// assert_eq!(sort.name(), "created_time");
    /// assert_eq!(sort.order(), SortOrder::Descending);
    /// 
    /// let sort = SortTerm::new("title");
    /// assert_eq!(sort.name(), "title");
    /// assert_eq!(sort.order(), SortOrder::Ascending);
    /// ```
    fn from_str(sort: &str) -> Result<Self, Self::Err> {
        if sort.trim().is_empty() {
            return Err("Sort is empty".to_string());
        }

        let order = if sort.starts_with('-') {
            SortOrder::Descending
        } else {
            SortOrder::Ascending
        };
        let name = match order {
            SortOrder::Descending => sort[1..].to_string(),
            SortOrder::Ascending => sort.to_string(),
        };
        Ok(Self { name, order })
    }
}