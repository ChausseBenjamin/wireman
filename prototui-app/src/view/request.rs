#![allow(clippy::module_name_repetitions)]
use crate::commons::window_border;
use crate::controller::RequestController;
use crate::model::request::ErrorKind;
use crate::theme;
use ratatui::backend::Backend;
use ratatui::layout::Alignment;
use ratatui::layout::Constraint;
use ratatui::layout::Layout;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::Span;
use ratatui::text::Spans;
use ratatui::text::Text;
use ratatui::widgets::Block;
use ratatui::widgets::Borders;
use ratatui::widgets::Paragraph;
use ratatui::widgets::Wrap;
use ratatui::Frame;

// Draw a text editor widget
pub fn draw_request<'a, B>(
    f: &mut Frame<B>,
    area: Rect,
    controller: &mut RequestController<'a>,
    block: Block<'a>,
) where
    B: Backend,
{
    // Set the cursor line style
    let cursor_line_style = Style::default();

    // Set the cursor style depending on the mode
    let cursor_style = if controller.insert_mode() {
        Style::default()
            .fg(theme::COL_CURSOR_INSERT_MODE)
            .add_modifier(theme::MOD_CURSOR_INSERT_MODE)
    } else {
        Style::default()
            .fg(theme::COL_CURSOR_NORMAL_MODE)
            .add_modifier(theme::MOD_CURSOR_NORMAL_MODE)
    };

    // Update the editor style
    controller.set_cursor_style(cursor_line_style, block, cursor_style);

    // Get the error text from the model
    let error = controller.error();

    // Get response text from model
    let response = controller.response();

    // Determine size of error and response widget
    let resp_length = response
        .as_ref()
        .map_or(0, |r| r.lines().count() as u16 + 2);
    let err_length = error.as_ref().map_or(0, |_| 3);
    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Min(0),
                Constraint::Length(err_length),
                Constraint::Length(resp_length),
            ]
            .as_ref(),
        )
        .split(area);

    // Render request window
    f.render_widget(controller.request().widget(), chunks[0]);

    // Render error window
    if let Some(error) = &error {
        f.render_widget(error_widget(error.to_owned()), chunks[1]);
    }

    // Render response window
    if let Some(response) = &response {
        f.render_widget(response_widget(&response.to_owned()), chunks[2]);
    }
}

/// Renders the gRPC response
fn response_widget(text: &str) -> Paragraph {
    Paragraph::new(Text::from(text))
        .block(window_border("Response", false))
        .wrap(Wrap { trim: false })
}

/// Renders any error in a separate box
fn error_widget<'a>(err: ErrorKind) -> Paragraph<'a> {
    let text = vec![Spans::from(Span::styled(
        err.msg,
        Style::default().fg(theme::COL_TEXT_ERROR),
    ))];
    let title = Span::styled(
        err.kind,
        Style::default()
            .fg(theme::COL_TEXT_ERROR)
            .add_modifier(theme::MOD_WINDOW_TITLE),
    );
    Paragraph::new(text)
        .block(Block::default().title(title).borders(Borders::ALL))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
}
