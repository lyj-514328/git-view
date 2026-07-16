use crate::git::{GitRepo, StatusEntry, StatusType};
use crate::theme::Theme;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use std::cmp;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatusFocus {
    Unstaged,
    Staged,
    Diff,
}

pub struct StatusTab {
    pub staged: Vec<StatusEntry>,
    pub unstaged: Vec<StatusEntry>,
    pub staged_selected: usize,
    pub unstaged_selected: usize,
    pub focus: StatusFocus,
}

impl StatusTab {
    pub fn new() -> Self {
        Self {
            staged: Vec::new(),
            unstaged: Vec::new(),
            staged_selected: 0,
            unstaged_selected: 0,
            focus: StatusFocus::Unstaged,
        }
    }

    pub fn refresh(&mut self, repo: &GitRepo) {
        if let Ok((staged, unstaged)) = repo.get_status() {
            self.staged = staged;
            self.unstaged = unstaged;
            self.staged_selected = 0;
            self.unstaged_selected = 0;
            self.focus = StatusFocus::Unstaged;
        }
    }

    pub fn move_down(&mut self) {
        match self.focus {
            StatusFocus::Unstaged => {
                if self.unstaged.is_empty() { return; }
                if self.unstaged_selected + 1 < self.unstaged.len() {
                    self.unstaged_selected += 1;
                } else if !self.staged.is_empty() {
                    self.focus = StatusFocus::Staged;
                    self.staged_selected = 0;
                }
            }
            StatusFocus::Staged => {
                let max = self.staged.len().saturating_sub(1);
                self.staged_selected = cmp::min(self.staged_selected + 1, max);
            }
            StatusFocus::Diff => {}
        }
    }

    pub fn move_up(&mut self) {
        match self.focus {
            StatusFocus::Unstaged => {
                self.unstaged_selected = self.unstaged_selected.saturating_sub(1);
            }
            StatusFocus::Staged => {
                if self.staged.is_empty() { return; }
                if self.staged_selected > 0 {
                    self.staged_selected -= 1;
                } else if !self.unstaged.is_empty() {
                    self.focus = StatusFocus::Unstaged;
                    self.unstaged_selected = self.unstaged.len().saturating_sub(1);
                }
            }
            StatusFocus::Diff => {}
        }
    }

    pub fn focus_left(&mut self) {
        self.focus = match self.focus {
            StatusFocus::Unstaged => StatusFocus::Staged,
            StatusFocus::Staged => StatusFocus::Unstaged,
            StatusFocus::Diff => StatusFocus::Unstaged,
        };
    }

    pub fn focus_right(&mut self) {
        match self.focus {
            StatusFocus::Unstaged | StatusFocus::Staged => {
                if self.current_file().is_some() {
                    self.focus = StatusFocus::Diff;
                } else {
                    self.focus = if self.focus == StatusFocus::Unstaged {
                        StatusFocus::Staged
                    } else {
                        StatusFocus::Unstaged
                    };
                }
            }
            StatusFocus::Diff => {}
        }
    }

    pub fn current_file(&self) -> Option<String> {
        match self.focus {
            StatusFocus::Unstaged => self.unstaged.get(self.unstaged_selected).map(|e| e.path.clone()),
            StatusFocus::Staged => self.staged.get(self.staged_selected).map(|e| e.path.clone()),
            StatusFocus::Diff => {
                self.unstaged.get(self.unstaged_selected).map(|e| e.path.clone())
                    .or_else(|| self.staged.get(self.staged_selected).map(|e| e.path.clone()))
            }
        }
    }

    pub fn current_staged(&self) -> bool {
        match self.focus {
            StatusFocus::Staged => true,
            StatusFocus::Unstaged => false,
            StatusFocus::Diff => false,
        }
    }

    pub fn render(&self, f: &mut Frame, area: Rect, theme: &Theme) {
        let panels = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
            .split(area);

        self.render_panel(
            f, panels[0], theme, "Unstaged Changes",
            &self.unstaged, self.unstaged_selected,
            self.focus == StatusFocus::Unstaged,
        );
        self.render_panel(
            f, panels[1], theme, "Staged Changes",
            &self.staged, self.staged_selected,
            self.focus == StatusFocus::Staged,
        );
    }

    fn render_panel(
        &self, f: &mut Frame, area: Rect, theme: &Theme,
        title: &str, entries: &[StatusEntry], selected: usize, focused: bool,
    ) {
        let border_style = if focused {
            theme.border_focused_style()
        } else {
            theme.border_style()
        };
        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(border_style);
        let inner = block.inner(area);
        f.render_widget(block, area);

        let mut lines: Vec<Line> = Vec::new();

        for (i, entry) in entries.iter().enumerate() {
            let is_selected = i == selected && focused;
            let style = status_style(entry, is_selected, theme);
            let marker = if is_selected { ">" } else { " " };
            let sc = status_char(&entry.status);
            lines.push(Line::from(Span::styled(
                format!(" {} {} {}", marker, sc, entry.path),
                style,
            )));
        }

        if lines.is_empty() {
            lines.push(Line::from(Span::styled(" (clean)", theme.dim_text())));
        }

        let visible: Vec<Line> = lines
            .into_iter()
            .take(inner.height as usize)
            .collect();
        f.render_widget(Paragraph::new(visible), inner);
    }
}

fn status_char(st: &StatusType) -> &'static str {
    match st {
        StatusType::Added => "A",
        StatusType::Modified => "M",
        StatusType::Deleted => "D",
        StatusType::Renamed => "R",
        StatusType::Copied => "C",
        StatusType::Untracked => "?",
        StatusType::TypeChange => "T",
    }
}

fn status_style(entry: &StatusEntry, selected: bool, theme: &Theme) -> Style {
    if selected {
        return theme.selected();
    }
    if entry.staged {
        Style::default().fg(theme.file_entry_staged).bg(theme.light_bg)
    } else {
        let fg = match entry.status {
            StatusType::Untracked => theme.file_entry_untracked,
            StatusType::Modified => theme.file_entry_modified,
            _ => theme.file_entry,
        };
        Style::default().fg(fg).bg(theme.light_bg)
    }
}
