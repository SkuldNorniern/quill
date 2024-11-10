use iced::widget::{button, column, container, row, text_editor};
use iced::{Element, Length, Task};
use iced::theme::Theme;

use std::path::PathBuf;
use std::fs;

pub fn main() -> iced::Result {
    iced::application("Quill - Text Editor", Quill::update, Quill::view)
        .theme(Quill::theme)
        .run_with(|| (Quill::default(), Task::none()))
}

#[derive(Debug, Default)]
struct Quill {
    content: text_editor::Content,
    path: Option<PathBuf>,
    dirty: bool,
}

#[derive(Debug, Clone)]
enum Message {
    Edit(text_editor::Action),
    New,
    Open,
    Save,
    FileOpened(Result<String, String>),
    FileSaved(Result<(), String>),
}

impl Quill {
    fn update(&mut self, message: Message) {
        match message {
            Message::Edit(action) => {
                self.content.perform(action);
                self.dirty = true;
            }
            Message::New => {
                self.content = text_editor::Content::new();
                self.path = None;
                self.dirty = false;
            }
            Message::Open => {
                // TODO: Implement file dialog
            }
            Message::Save => {
                if let Some(path) = &self.path {
                    let content = self.content.text();
                    // TODO: Implement save
                }
            }
            Message::FileOpened(Ok(content)) => {
                self.content = text_editor::Content::with_text(&content);
                self.dirty = false;
            }
            Message::FileOpened(Err(_)) => {
                // TODO: Show error
            }
            Message::FileSaved(Ok(())) => {
                self.dirty = false;
            }
            Message::FileSaved(Err(_)) => {
                // TODO: Show error
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let controls = row![
            button("New").on_press(Message::New),
            button("Open").on_press(Message::Open),
            button("Save").on_press(Message::Save),
        ]
        .spacing(10)
        .padding(10);

        let editor = text_editor(&self.content)
            .on_action(Message::Edit)
            .height(Length::Fill);

        let content = column![controls, editor].height(Length::Fill);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

async fn load_file() -> Result<String, String> {
    // TODO: Implement file dialog
    let path = PathBuf::from("test.txt");
    fs::read_to_string(path).map_err(|err| err.to_string())
}

async fn save_file(path: PathBuf, content: String) -> Result<(), String> {
    fs::write(path, content).map_err(|err| err.to_string())
}
