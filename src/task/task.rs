pub struct Task {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub due_date: String,
    pub completed: bool,
}

impl Task {
    pub fn new(id: u32, title: String, description: String, due_date: String) -> Task {
        Task {
            id,
            title,
            description,
            due_date,
            completed: false,
        }
    }

    pub fn complete(&mut self) {
        self.completed = true;
    }

    pub fn display(&self) {
        println!(
            "ID: {}\nTitle: {}\nDescription: {}\nDue Date: {}\nCompleted: {}",
            self.id, self.title, self.description, self.due_date, self.completed
        );
    }
}
