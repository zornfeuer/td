use std::{fs, path::PathBuf};
use dirs::data_local_dir;
use crate::tasks::TaskList;

const APP_NAME: &str = "td";
const DEFAULT_SESSION: &str = "default";

pub struct Session {
    pub name: String,
}

impl Session {
    fn get_session_dir(&self) -> PathBuf {
        let base_dir = data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(APP_NAME);
        base_dir.join(&self.name)
    }

    fn ensure_session_dir(&self) -> std::io::Result<()> {
        fs::create_dir_all(self.get_session_dir())
    }

    pub fn count_undone_tasks_in_session(&self) -> Result<usize, Box<dyn std::error::Error>> {
        let file_path = self.get_session_tasks_file()?;
        let task_list = TaskList::load_from_file(&file_path)?;
        Ok(task_list.count_undone())
    }

    pub fn get_session_tasks_file(&self) -> std::io::Result<PathBuf> {
        self.ensure_session_dir()?;
        Ok(self.get_session_dir().join("tasks.toml"))
    }

    pub fn get_sessions() -> std::io::Result<Vec<Self>> {
        let base_dir = data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(APP_NAME);
        if !base_dir.exists() {
            return Ok(Vec::new());
        }

        let sessions: Vec<Self> = fs::read_dir(base_dir)?
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().is_dir())
            .map(|entry| Self { name: entry.file_name().to_string_lossy().into_owned()})
            .collect();

        Ok(sessions)
    }

    pub fn get_current_session() -> Self {
        let session_file = data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(APP_NAME)
            .join("current_session");
        let name = if session_file.exists() {
            fs::read_to_string(&session_file)
                .map(|s| s.trim().to_string())
                .unwrap_or_else(|_| DEFAULT_SESSION.to_string())
        } else {
            DEFAULT_SESSION.to_string()
        };
        Self { name }
    }

    pub fn set_current_sesion(session: &str) -> std::io::Result<Self> {
        let session_file = data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(APP_NAME)
            .join("current_session");
        fs::create_dir_all(session_file.parent().unwrap())?;
        fs::write(session_file, session)?;
        Ok(Self { name: session.to_string() })
    }
}
