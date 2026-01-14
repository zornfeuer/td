use owo_colors::OwoColorize;
use std::{fs, path::Path};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Task {
    done: bool,
    text: String,
}

impl Task {
    fn new(text: String) -> Self {
        Self { done: false, text }
    }

    fn toggle_done(&mut self) {
        self.done = !self.done;
    }

    fn is_done(&self) -> bool {
        self.done
    }

    fn edit(&mut self, text: String) {
        self.text = text;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskList {
    tasks: Vec<Task>,
}

impl TaskList {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, text: String) {
        self.tasks.push(Task::new(text));
    }

    pub fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("{}", "No tasks.".dimmed());
            return;
        }

        for (i, task) in self.tasks.iter().enumerate() {
            let idx = i + 1;
            
            if task.is_done() {
                println!(
                    "{:>2}{} [{}] {}",
                    idx.to_string().cyan(),
                    ".".cyan(),
                    "✓".green(),
                    task.text.green()
                );
            } else {
                println!(
                    "{:>2}{} [{}] {}",
                    idx.to_string().cyan(),
                    ".".cyan(),
                    " ",
                    task.text.white()
                );
            }
        }
    }

    pub fn edit_task(&mut self, idx: usize, text: String) -> Result<(), String> {
        if idx == 0 || idx > self.tasks.len() {
            return Err("Invalid task number".to_string());
        }
        self.tasks[idx - 1].edit(text);
        Ok(())
    }

    pub fn mark_done(&mut self, idx: usize) -> Result<(), String> {
        if idx == 0 || idx > self.tasks.len() {
            return Err("Invalid task number".to_string());
        }
        if !self.tasks[idx - 1].is_done() {
            self.tasks[idx - 1].toggle_done();
        }
        Ok(())
    }

    pub fn mark_undone(&mut self, idx: usize) -> Result<(), String> {
        if idx == 0 || idx > self.tasks.len() {
            return Err("Invalid task number".to_string());
        }
        if self.tasks[idx - 1].is_done() {
            self.tasks[idx - 1].toggle_done();
        }
        Ok(())
    }

    pub fn count_tasks(&self) -> usize {
        self.tasks.len()
    }

    pub fn count_undone(&self) -> usize {
        self.tasks.iter().filter(|task| !task.is_done()).count()
    }

    pub fn remove_tasks(&mut self, indices: &[usize]) -> Result<(), String> {
        if indices.is_empty() {
            return Ok(());
        }

        let max_valid = self.tasks.len();
        for idx in indices.iter() {
            if *idx == 0 || *idx > max_valid {
                return Err(format!("Invalid task number {}", *idx));
            }
        }

        let mut unique_indices: Vec<usize> = indices.iter().copied().collect();
        unique_indices.sort_unstable();
        unique_indices.dedup();
        unique_indices.reverse();

        for idx in unique_indices {
            self.tasks.remove(idx - 1);
        }

        Ok(())
    }

    pub fn save_to_file(&self, path: &Path) -> std::io::Result<()> {
        let data = toml::to_string(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        fs::write(path, data)
    }

    pub fn load_from_file(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        if !path.exists() {
            return Ok(Self::new())
        }
        let contents = fs::read_to_string(path)?;
        let task_list: TaskList = toml::from_str(&contents)?;
        Ok(task_list)
    }
}

