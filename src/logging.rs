use rich_rust::console::Console;
use rich_rust::markup::escape;

pub struct ConsoleHandler {
    handle: Console,
}

impl ConsoleHandler {
    pub fn new() -> Self {
        ConsoleHandler {
            handle: Console::new(),
        }
    }

    pub fn log_info(&self, message: &str) {
        let safe = escape(message);
        self.handle
            .print(&format!("[bold blue]INFO: {safe}[/bold blue]"));
    }

    pub fn log_error(&self, message: &str) {
        let safe = escape(message);
        self.handle
            .print(&format!("[bold red]ERROR: {safe}[/bold red]"));
    }

    pub fn log_message(&self, message: &str) {
        let safe = escape(message);
        self.handle
            .print(&format!("[bold green]{safe}[/bold green]"));
    }
}
