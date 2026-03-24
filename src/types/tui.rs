use std::collections::HashSet;
use std::fmt::Display;

pub struct TuiMultipleChoice<T>
where
    T:Display+Clone+CheckedInfo,
{
    pub items: Vec<T>,
    pub selected_index: usize,
    pub checked_items: HashSet<usize>,
}

pub trait CheckedInfo{
    fn info(&self)->&str;

}
impl <T>TuiMultipleChoice<T>
where
    T:Display+Clone+CheckedInfo,
{
    pub fn new(list:&Vec<T>) -> Self {
        Self {
            items: list.clone(),
            selected_index: 0,
            checked_items: HashSet::new(),
        }
    }
    pub fn get_checked(&self) -> Vec<T> {
        let list=self.checked_items
            .iter()
            .map(|&i| self.items[i].clone())
            .collect();
        list
    }
    pub fn next(&mut self) {
        if self.items.is_empty() {
            return;
        }
        let i = self.selected_index;
        if i >= self.items.len() - 1 {
            self.selected_index = 0;
        } else {
            self.selected_index = i + 1;
        }
    }

    pub fn previous(&mut self) {
        if self.items.is_empty() {
            return;
        }
        let i = self.selected_index;
        if i == 0 {
            self.selected_index = self.items.len() - 1;
        } else {
            self.selected_index = i - 1;
        }
    }
    pub fn toggle_current(&mut self) {
        if self.checked_items.contains(&self.selected_index) {
            self.checked_items.remove(&self.selected_index);
        } else {
            self.checked_items.insert(self.selected_index);
        }
    }
}
