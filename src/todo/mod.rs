#[derive(Debug, PartialEq, Eq)]
pub enum Status {
    Todo,
    Complete,
}

#[derive(Debug)]
pub struct Item {
    status: Status,
    pub desc: String,
}

impl Item {
    pub fn new(desc: String) -> Self {
        Item {
            status: Status::Todo,
            desc,
        }
    }

    pub fn mark_as_done(&mut self) {
        self.status = Status::Complete;
    }

    pub fn unmark(&mut self) {
        self.status = Status::Todo;
    }

    pub fn is_done(&self) -> bool {
        self.status == Status::Complete
    }
}

pub struct TodoList {
    items: Vec<Item>,
}

impl TodoList {
    pub fn new() -> Self {
        TodoList { items: Vec::new() }
    }

    pub fn add(&mut self, desc: String) {
        self.items.push(Item::new(desc));
    }

    pub fn remove(&mut self, idx: usize) {
        self.items.remove(idx);
    }

    /// Sets the status of a todo item as completed if it exists.
    ///
    /// # Arguments
    ///
    /// * `idx` - The index of the item to be marked as complete.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut list = TodoList::new();
    /// list.add("Buy Milk");
    /// list.mark_as_done(0);
    /// ```
    pub fn mark_as_done(&mut self, idx: usize) {
        if let Some(item) = self.items.get_mut(idx) {
            item.mark_as_done();
        };
    }

    pub fn clear_done_tasks(&mut self) {
        self.items.retain(|item| !item.is_done());
    }

    /// Returns an immutable list of Item values
    pub fn items(&self) -> &Vec<Item> {
        &self.items
    }
}
