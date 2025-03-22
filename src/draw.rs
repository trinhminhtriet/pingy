use crate::ip_data::IpData;
use crate::ui::{draw_graph_view, draw_point_view, draw_table_view};
use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::Terminal;
use std::error::Error;
use std::io::{self, Stdout};

pub fn init_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    Ok(terminal)
}

pub fn restore_terminal(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    terminal.show_cursor()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}

pub fn draw_interface<B: Backend>(
    terminal: &mut Terminal<B>,
    view_type: &str,
    ip_data: &[IpData],
    errs: &[String],
) -> Result<(), Box<dyn Error>> {
    terminal.draw(|f| match view_type {
        "graph" => {
            draw_graph_view::<B>(f, ip_data, errs);
        }
        "table" => {
            let size = f.area();
            draw_table_view::<B>(f, ip_data, errs, size);
        }
        "point" => {
            let size = f.area();
            draw_point_view::<B>(f, ip_data, errs, size);
        }

        _ => {
            draw_graph_view::<B>(f, ip_data, errs);
        }
    })?;
    Ok(())
}
