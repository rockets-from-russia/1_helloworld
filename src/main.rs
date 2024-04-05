use iced::{Application, Settings};
use iced::widget::Space;

struct MyApp;

impl Application for MyApp {
    type Executor = iced::executor::Default;
    type Message = ();
    type Flags = ();
    type Theme = iced::theme::Theme;

    fn new(_flags: ()) -> (MyApp, iced::Command<Self::Message>) {
        (MyApp, iced::Command::none())
    }

    fn title(&self) -> String {
        String::from("Hello world!")
    }

    fn update(&mut self, _message: Self::Message) -> iced::Command<Self::Message> {
        iced::Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        iced::Element::new(Space::new(iced::Length::Fill, iced::Length::Fill))
    }
}

fn main() {
    let _ = MyApp::run(Settings {
        window: iced::window::Settings {
            size: iced::Size::new(400.0, 300.0),
            resizable: false,
            ..iced::window::Settings::default()
        },
        ..Settings::default()
    });
}
