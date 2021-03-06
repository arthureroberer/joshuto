use std::path::Path;

use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Paragraph, Widget, Wrap};

use crate::{HOSTNAME, USERNAME};

pub struct TuiTopBar<'a> {
    path: &'a Path,
}

impl<'a> TuiTopBar<'a> {
    pub fn new(path: &'a Path) -> Self {
        Self { path }
    }
}

impl<'a> Widget for TuiTopBar<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let username_style = Style::default()
            .fg(Color::LightGreen)
            .add_modifier(Modifier::BOLD);

        let path_style = Style::default()
            .fg(Color::LightBlue)
            .add_modifier(Modifier::BOLD);

        let curr_path_str = self.path.to_string_lossy();

        let text = Spans::from(vec![
            Span::styled(USERNAME.as_str(), username_style),
            Span::styled("@", username_style),
            Span::styled(HOSTNAME.as_str(), username_style),
            Span::styled(" ", username_style),
            Span::styled(curr_path_str, path_style),
        ]);

        Paragraph::new(text)
            .wrap(Wrap { trim: true })
            .render(area, buf);
    }
}
