use clap::Parser;
use td::{
    cli::{Cli, Command},
    session::Session,
    tasks::TaskList
};

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let cli = Cli::parse();

    let session = match &cli.command {
        Some(Command::Session { session }) => {
            Session::set_current_sesion(session)?;
            println!("Switched to session: {}", session);
            return Ok(());
        }
        _ => Session::get_current_session(),
    };

    let file_path = session.get_session_tasks_file()?;
    let mut task_list = TaskList::load_from_file(&file_path)?;

    match &cli.command {
        None | Some(Command::Ls) => {
            println!("Session: {}", session.name);
            task_list.list_tasks();
        },
        Some(Command::Add { text }) => {
            task_list.add_task(text.clone());
            task_list.save_to_file(&file_path)?;
            println!("Session: {}", session.name);
            println!("Added: {}", text);
        }
        Some(Command::Edit { index, text }) => {
            task_list.edit_task(*index, text.clone())?;
            task_list.save_to_file(&file_path)?;
            println!("Session: {}", session.name);
            println!("Edit #{}: {}", index, text);
        }
        Some(Command::Done { index }) => {
            task_list.mark_done(*index)?;
            task_list.save_to_file(&file_path)?;
            println!("Session: {}", session.name);
            println!("Marked #{} as done", index);
        }

        Some(Command::Undone { index }) => {
            task_list.mark_undone(*index)?;
            task_list.save_to_file(&file_path)?;
            println!("Session: {}", session.name);
            println!("Marked #{} as undone", index);
        }
        Some(Command::Rm { index }) => {
            task_list.remove_task(*index)?;
            task_list.save_to_file(&file_path)?;
            println!("Session: {}", session.name);
            println!("Removed task #{}", index);
        }
        Some(Command::Sessions ) => {
            let sessions = Session::get_sessions()?;
            println!("Sessions:");
            let current = session.name;
            for session in sessions {
                let marker = if session == current { " (current)" } else { "" };
                println!("- {}{}", session, marker)
            }
        } 
        Some(Command::Session { .. }) => unreachable!(),
    }

    Ok(())
}
