use std::fmt;
use tabled::Tabled;

#[derive(Tabled)]
pub struct Task {
    id: i32,
    title: String,
    done: bool,
    due_time: DisplayOption<String>,
    created_at: String,
}

pub struct DisplayOption<T>(Option<T>);

impl<T: fmt::Display> fmt::Display for DisplayOption<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.0 {
            Some(value) => write!(f, "{}", value),
            None => write!(f, "N/A"),
        }
    }
}
