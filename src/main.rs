use iced::{Settings, Theme, Task, Element};
use iced::application;
use iced::widget::Text;

fn main() -> iced::Result {
    Hello::run(Settings::default())
}

struct Hello;

#[derive(Debug, Clone)]
enum Message {}

impl application::Application for Hello {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Task<Self::Message>) {
        (Hello, Task::none())
    }

    fn title(&self) -> String {
        String::from("My First Iced App")
    }

    fn update(&mut self, _message: Self::Message) -> Task<Self::Message> {
        Task::none()
    }

    fn view(&self) -> Element<Self::Message> {
        Text::new("Hello, Iced 0.13!").into()
    }
}
