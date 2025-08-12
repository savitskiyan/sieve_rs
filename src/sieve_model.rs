use std::str::FromStr;
use super::sieve_regex::COMMA_PATTERN;
use super::filter_term::FilterTerm;
use super::sort_term::SortTerm;


/// Модель для обработки параметров запроса с поддержкой фильтрации, сортировки и пагинации
/// 
/// # Примеры использования
/// 
/// ## Фильтрация
/// ```
/// let model = SieveModel::new(
///     Some(1),
///     Some(10),
///     Some("title@=Rock,artist==Queen".to_string()),
///     Some("title".to_string())
/// );
/// ```
/// 
/// ## Сортировка
/// ```
/// let model = SieveModel::new(
///     Some(1),
///     Some(10),
///     None,
///     Some("-created_time,title".to_string())
/// );
/// ```
/// 
/// ## Пагинация
/// ```
/// let model = SieveModel::new(
///     Some(2),
///     Some(20),
///     None,
///     None
/// };
/// ```
#[derive(Debug, Clone)]
pub struct SieveModel {
    /// Номер страницы (начиная с 1)
    /// 
    /// # Примеры
    /// - `1` - первая страница
    /// - `2` - вторая страница
    /// 
    /// # Значение по умолчанию
    /// Если не указано, используется значение 1
    page: u64,

    /// Размер страницы (количество элементов на странице)
    /// 
    /// # Примеры
    /// - `10` - 10 элементов на странице
    /// - `50` - 50 элементов на странице
    /// 
    /// # Значение по умолчанию
    /// Если не указано, используется значение 100
    page_size: u64,

    /// Условие фильтрации записей по полю
    /// Поддерживаются следующие форматы:
    /// - `поле@=значение` - поиск записей, где поле содержит значение
    /// - `поле==значение` - точное совпадение
    /// - `поле!=значение` - не равно
    /// - `поле>=значение` - больше или равно
    /// - `поле<=значение` - меньше или равно
    /// - `поле>значение` - больше
    /// - `поле<значение` - меньше
    /// - `поле_=значение` - начинается с
    /// - `поле_-=значение` - заканчивается на
    /// - `(поле1|поле2)@=значение` - поиск по нескольким полям
    /// - `поле@=значение1|значение2` - поиск по нескольким значениям
    /// 
    /// Можно комбинировать условия через запятую:
    /// `поле1@=значение1,поле2==значение2`
    /// 
    /// # Операторы фильтрации
    /// - `==` - равно
    /// - `!=` - не равно
    /// - `>` - больше
    /// - `<` - меньше
    /// - `>=` - больше или равно
    /// - `<=` - меньше или равно
    /// - `@=` - содержит (регистронезависимый поиск)
    /// - `_=` - начинается с
    /// - `_-=` - заканчивается на
    /// 
    /// # Примеры
    /// - `title@=Rock` - поиск треков, содержащих "Rock" в названии
    /// - `artist==Queen` - поиск треков исполнителя "Queen"
    /// - `duration>3:00` - поиск треков длительностью более 3 минут
    /// - `created_time>=2023-01-01` - поиск треков, созданных после 1 января 2023 года
    /// 
    /// # Экранирование
    /// Для экранирования специальных символов используйте обратный слеш:
    /// - `title@=Rock\, Metal` - поиск треков, содержащих "Rock, Metal" в названии
    /// 
    /// # Негация
    /// Для инвертирования условия добавьте `!` перед оператором:
    /// - `title@=!Rock` - поиск треков, не содержащих "Rock" в названии
    /// 
    /// # Регистронезависимый поиск
    /// Для регистронезависимого поиска добавьте `*` после оператора:
    /// - `title@=*Rock` - поиск треков, содержащих "Rock" в названии (регистронезависимый)
    filters: Option<Vec<FilterTerm>>,
    // pub filters: Option<String>,

    /// Условие сортировки записей
    /// Формат: `поле` или `-поле` для сортировки по убыванию
    /// Можно указать несколько полей через запятую
    /// 
    /// # Формат
    /// - `поле` - сортировка по возрастанию
    /// - `-поле` - сортировка по убыванию
    /// 
    /// # Примеры
    /// - `title` - сортировка по названию по возрастанию
    /// - `-created_time` - сортировка по дате создания по убыванию
    /// - `artist,title` - сортировка по исполнителю по возрастанию, затем по названию по возрастанию
    /// 
    /// # Поддерживаемые поля
    /// - `title` - название трека
    /// - `artist` - исполнитель
    /// - `album` - альбом
    /// - `duration` - длительность
    /// - `created_time` - дата создания
    /// - `file_size` - размер файла
    sorts: Option<Vec<SortTerm>>
    // pub sorts: Option<String>

}

/// Реализация модели SieveModel
impl SieveModel {

    /// Создает новый экземпляр SieveModel с пустыми значениями
    /// 
    /// # Примеры
    /// ```
    /// let model = SieveModel::new(
    ///     Some(1),
    ///     Some(10),
    ///     None,
    ///     None
    /// );
    /// ```
    pub fn new(page: &Option<u64>, page_size: &Option<u64>, filters: &Option<String>, sorts: &Option<String>) -> Self {
        Self {
            page: page.unwrap_or(1),
            page_size: page_size.unwrap_or(100),
            filters: SieveModel::parse_filters(&filters),
            sorts: SieveModel::parse_sorts(&sorts)
        }
    }

    /// Возвращает номер страницы
    /// 
    /// # Примеры
    /// - `1` - первая страница
    /// - `2` - вторая страница
    pub fn page(&self) -> u64 {
        self.page
    }

    /// Возвращает размер страницы
    /// 
    /// # Примеры
    /// - `10` - 10 элементов на странице
    /// - `50` - 50 элементов на странице
    pub fn page_size(&self) -> u64 {
        self.page_size
    }

    /// Возвращает условие фильтрации
    /// 
    /// # Примеры
    /// - `Some(vec![FilterTerm::new("title@=Rock,artist==Queen")])` - фильтрация по названию и исполнителю
    pub fn filters(&self) -> Option<Vec<FilterTerm>> {
        self.filters.clone()
    }

    /// Возвращает условие сортировки
    /// 
    /// # Примеры
    /// - `Some(vec![SortTerm::from_str("title")])` - сортировка по названию
    /// - `Some(vec![SortTerm::from_str("-created_time")])` - сортировка по дате создания по убыванию
    pub fn sorts(&self) -> Option<Vec<SortTerm>> {
        self.sorts.clone()
    }

    /// Разбирает строку фильтров на отдельные термы
    /// 
    /// # Примеры
    /// ```
    /// let model = SieveModel::new(
    ///     Some(1),
    ///     Some(10),
    ///     Some("title@=Rock,artist==Queen".to_string()),
    ///     None
    /// );
    /// ```
    fn parse_filters(filters: &Option<String>) -> Option<Vec<FilterTerm>> {
        filters.as_ref().map(|filters| {
            let mut result = Vec::new();
            
            // Разбиваем по запятой, учитывая экранирование
            for filter in filters.split(',') {
                let term = FilterTerm::from_str(filter);
                if let Ok(term) = term {
                    result.push(term);
                }
            }
            result
        })
    }

    /// Парсит строку сортировки и возвращает вектор SortTerm
    /// 
    /// # Формат
    /// `поле1,-поле2`
    /// 
    /// # Примеры
    /// ```
    /// let model = SieveModel {
    ///     page: Some(1),
    ///     page_size: Some(10),
    ///     filters: None,
    ///     sorts: Some("title,-created_time".to_string()),
    /// };
    /// ```
    fn parse_sorts(sorts: &Option<String>) -> Option<Vec<SortTerm>> {
        sorts.as_ref().map(|sorts| {
            let mut result = Vec::new();
            for sort in COMMA_PATTERN.split(sorts) {
                let term = SortTerm::from_str(sort);
                if let Ok(term) = term {
                    result.push(term);
                }
            }
            result
        })
    }
}
