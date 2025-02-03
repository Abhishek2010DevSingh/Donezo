use std::fmt;
use tabled::Tabled;

#[derive(Tabled)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub done: bool,
    pub due_time: DisplayOption<String>,
    pub created_at: String,
}

pub struct DisplayOption<T>(pub Option<T>);

impl<T: fmt::Display> fmt::Display for DisplayOption<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.0 {
            Some(value) => write!(f, "{}", value),
            None => write!(f, "N/A"),
        }
    }
}
