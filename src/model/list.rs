use super::item::Item;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct List {
    items: Vec<Item>,
}

impl List {
    pub fn new() -> Self {
        List { items: Vec::new() }
    }

    pub fn add(&mut self, item: Item) {
        self.items.push(item);
    }

    pub fn remove(&mut self, index: usize) -> Option<()> {
        if index < self.items.len() {
            self.items.remove(index);
            Some(())
        } else {
            None
        }
    }

    pub fn get(&mut self, index: usize) -> Option<&mut Item> {
        self.items.get_mut(index)
    }

    pub fn list(&self) -> &Vec<Item> {
        &self.items
    }

    pub fn mutate_index<F>(&mut self, index: usize, mut function: F) -> Result<(), String>
    where
        F: FnMut(&mut Item),
    {
        match self.get(index) {
            Some(item) => {
                function(item);
                Ok(())
            }
            None => Err(format!(
                "Index out of bounds. Range: 0-{}",
                self.items.len().saturating_sub(1)
            )),
        }
    }
}
