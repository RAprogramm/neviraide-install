use color_eyre::eyre::Result;
use crossterm::{
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use tokio::sync::mpsc;

use ratatui::{backend::CrosstermBackend, Terminal};
use std::{error::Error, io};

use crate::tui;

pub async fn run_app() -> Result<()> {
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = setup_backend();
    let mut terminal = setup_terminal(backend.unwrap()).unwrap();

    let (tx, rx) = mpsc::channel(100);

    // Запускаем обработчик событий в отдельном асинхронном потоке
    tokio::spawn(async move {
        tui::app::event_handler(tx).await;
    });

    let mut app = tui::app::App::new();
    app.run(&mut terminal, rx).await?;

    if let Err(e) = cleanup_terminal(&mut stdout) {
        eprintln!("Ошибка при очистке терминала: {:?}", e);
    }
    Ok(())
}

fn setup_backend() -> Result<CrosstermBackend<io::Stderr>, Box<dyn Error>> {
    let backend = CrosstermBackend::new(io::stderr());
    Ok(backend)
}

fn setup_terminal(
    backend: CrosstermBackend<io::Stderr>,
) -> Result<Terminal<CrosstermBackend<io::Stderr>>, Box<dyn Error>> {
    Ok(Terminal::new(backend)?)
}

fn cleanup_terminal(stdout: &mut io::Stdout) -> Result<(), Box<dyn Error>> {
    terminal::disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen)?;
    Ok(())
}