# Denezo To-Do App

Denezo is a simple command-line to-do list manager built with Rust. It allows users to manage tasks by adding, listing, completing, and deleting tasks from a SQLite database.

## Features
- Add a new task with an optional due date.
- List all tasks with the option to show completed tasks.
- Mark tasks as complete.
- Delete tasks by ID.

## Installation

### Prerequisites
- Rust (recommended version: 1.60+)
- SQLite

### Steps
1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/denezo.git
   cd denezo
   ```

2. Build the application:
   ```bash
   cargo build --release
   ```

3. Run the application:
   ```bash
   ./target/release/denezo
   ```

## Commands

### Add a Task
Adds a new task to the to-do list. Optionally, you can set a due date for the task.

```bash
denezo add <TASK> [-d <DATE>]
```

- `<TASK>`: The description of the task (required).
- `-d <DATE>`: The due date of the task (optional), format: `YYYY-MM-DD`.

Example:
```bash
denezo add "Finish the report" -d 2025-02-10
```

### List Tasks
Lists all tasks. Optionally, you can list all tasks including completed ones.

```bash
denezo list [-a]
```

- `-a`: Show all tasks, including completed ones.

Example:
```bash
denezo list -a
```

### Mark a Task as Complete
Marks a task as completed by its ID.

```bash
denezo complete <ID>
```

- `<ID>`: The ID of the task to mark as complete (required).

Example:
```bash
denezo complete 1
```

### Delete a Task
Deletes a task by its ID.

```bash
denezo delete <ID>
```

- `<ID>`: The ID of the task to delete (required).

Example:
```bash
denezo delete 1
```

## Database
The application uses an SQLite database to store tasks. Upon the first run, the database will be automatically initialized with a `tasks` table.

## License
Distributed under the MIT License. See LICENSE for more information.

---

If you have any questions, feel free to open an issue or contribute to the project!
