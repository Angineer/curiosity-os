use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};
use std::io;

#[derive(Debug, Default)]
pub struct Ui {
    exit: bool,
}

impl Ui {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
    }

    /// updates the application's state based on user input
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &Ui {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout_main = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);
        let layout_right = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(layout_main[1]);

        let mission_title = Line::from(" Current Mission ".bold());
        let mission_block = Block::bordered()
            .title(mission_title.centered())
            .border_set(border::PLAIN);
        let mission_text = Text::from("- Collect 10 pounds of cheese from the moon");
        Paragraph::new(mission_text)
            .left_aligned()
            .block(mission_block)
            .render(layout_right[0], buf);

        let map_title = Line::from(" Galaxy Map ".bold());
        let map_block = Block::bordered()
            .title(map_title.centered())
            .border_set(border::PLAIN);
        let map_text = Text::from("\n\n    *\n\n       △\n\n           *");
        Paragraph::new(map_text)
            .left_aligned()
            .block(map_block)
            .render(layout_right[1], buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::style::Style;

    #[test]
    fn render() {
        let ui = Ui::default();
        let mut buf = Buffer::empty(Rect::new(0, 0, 50, 4));

        ui.render(buf.area, &mut buf);

        let mut expected = Buffer::with_lines(vec![
            "┏━━━━━━━━━━━━━ Counter Ui Tutorial ━━━━━━━━━━━━━━┓",
            "┃                    Value: 0                    ┃",
            "┃                                                ┃",
            "┗━ Decrement <Left> Increment <Right> Quit <Q> ━━┛",
        ]);
        let title_style = Style::new().bold();
        let key_style = Style::new().blue().bold();
        expected.set_style(Rect::new(14, 0, 21, 1), title_style);
        expected.set_style(Rect::new(13, 3, 6, 1), key_style);
        expected.set_style(Rect::new(30, 3, 7, 1), key_style);
        expected.set_style(Rect::new(43, 3, 4, 1), key_style);

        assert_eq!(buf, expected);
    }

    #[test]
    fn handle_key_event() {
        let mut ui = Ui::default();
        ui.handle_key_event(KeyCode::Char('q').into());
        assert!(ui.exit);
    }
}
