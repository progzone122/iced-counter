use iced::{executor, Application, Element, Theme, Command, Length, Settings, window, Size};
use iced::widget::{text, column, row, button, Container};

struct Counter {
    value: u32,
}
#[derive(Debug, Clone)]
pub enum Message {
    Increment,
    Decrement,
}
impl Application for Counter {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();
    
    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (
            Counter {
                value: 0
            },
            Command::none()
        )

    }

    fn title(&self) -> String {
        String::from("Counter")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Increment => {
                self.value = self.value
                    .checked_add(1)
                    .unwrap_or(self.value);
                Command::none()
            }
            Message::Decrement => {
                self.value = self.value
                    .checked_sub(1)
                    .unwrap_or(self.value);
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        column![
            row![
                Container::new(
                    text(self.value).size(40)
                )
                    .width(Length::Fill)
                    .height(Length::Shrink)
                    .center_x()
                    .padding([20, 0, 0, 0])
            ],
            row![
                button(
                    Container::new(
                        text("+").size(20)
                    )
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .center_x()
                        .center_y()
                )
                    .on_press(Message::Increment)
                    .width(Length::FillPortion(1)),

                button(
                    Container::new(
                        text("-").size(20)
                    )
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .center_x()
                        .center_y()
                )
                    .on_press(Message::Decrement)
                    .width(Length::FillPortion(1))
            ]
                .padding(20)
                .spacing(20),
        ].into()
    }
    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
pub fn main() -> iced::Result {
    Counter::run(Settings {
        window: window::Settings {
            size: Size::new(350.0, 200.0),
            min_size: Some(Size::new(350.0, 200.0)),
            max_size: Some(Size::new(650.0, 500.0)),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}
