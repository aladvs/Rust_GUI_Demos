#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use iced::theme::{self, Theme};
use iced::widget::{
    button, checkbox, column, container, horizontal_rule, progress_bar, radio,
    row, scrollable, slider, text, text_input, toggler, vertical_rule,
    vertical_space, image,
};
use iced::{Alignment, Color, Element, Length, Sandbox, Settings};

pub fn main() -> iced::Result {
    let key = "WGPU_BACKEND";
    std::env::set_var(key, "dx12");
    Styling::run(Settings::default())
}

#[derive(Default)]
struct Styling {
    theme: Theme,
    text_count: i32,
  /*  input_value: String,
    slider_value: f32,
    checkbox_value: bool,
    toggler_value: bool,*/ 
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum ThemeType {
    Light,
    Dark,
    Custom,
}

#[derive(Debug, Clone)]
enum Message {
    ThemeChanged(ThemeType),
    ButtonPressed,
}

impl Sandbox for Styling {
    type Message = Message;

    fn new() -> Self {
        Styling::default()
    }

    fn title(&self) -> String {
        String::from("Test B)")
    }


    fn update(&mut self, message: Message) {
        match message {
            Message::ThemeChanged(theme) => {
                self.theme = match theme {
                    ThemeType::Light => Theme::Light,
                    ThemeType::Dark => Theme::Dark,
                    ThemeType::Custom => Theme::custom(theme::Palette {
                        background: Color::from_rgb(1.0, 0.9, 1.0),
                        text: Color::BLACK,
                        primary: Color::from_rgb(0.5, 0.5, 0.0),
                        success: Color::from_rgb(0.0, 1.0, 0.0),
                        danger: Color::from_rgb(1.0, 0.0, 0.0),
                    }),
                }
            }
            Message::ButtonPressed =>  {
                self.text_count += 1;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let choose_theme =
            [ThemeType::Light, ThemeType::Dark, ThemeType::Custom]
                .iter()
                .fold(
                    column![text("Choose a theme:").size(50)].spacing(20),
                    |column, theme| {
                        column.push(radio(
                            format!("{theme:?}"),
                            *theme,
                            Some(match self.theme {
                                Theme::Light => ThemeType::Light,
                                Theme::Dark => ThemeType::Dark,
                                Theme::Custom { .. } => ThemeType::Custom,
                            }),
                            Message::ThemeChanged,
                        ))
                    },
                );
        
        let button = button("Gnu")
            .padding(10)
            .on_press(Message::ButtonPressed);
        
        let text_count = text(self.text_count);

        let image = image("ferris.png");


        let content = column![
            choose_theme,
            row![button, text_count.size(30)]
            .spacing(30),
            image,
        ]
        .spacing(20)
        .padding(20)
        .max_width(600);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
           // .center_y()
            .into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}

