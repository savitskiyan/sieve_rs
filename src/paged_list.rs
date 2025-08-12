use serde::Serialize;

/// Обобщенная структура для постраничного списка
#[derive(Debug, Serialize)]
pub struct PagedList<T> {
    items: Vec<T>,  // Список элементов
    total: u64,     // Общее количество элементов
}

impl<T> PagedList<T> {
    pub fn new(items: Vec<T>, total: u64) -> Self {
        Self { items, total }
    }

    pub fn items(&self) -> &Vec<T> {
        &self.items
    }

    pub fn total(&self) -> u64 {
        self.total
    }
}