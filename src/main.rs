use iced::widget::{button, column, row, Canvas};
use iced::{application, executor, theme, time, Renderer};
use iced::{Application, Color, Command, Element, Length, Point, Settings, Subscription, Theme};

use std::time::{Duration, Instant};

use mcmc::bellcurve::BellCurve;
use mcmc::gaussian;
use mcmc::metropolis;
use mcmc::stage::Stage;

pub fn main() -> iced::Result {
    MetropolisVisualizer::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}

struct MetropolisVisualizer {
    is_playing: bool,
    speed: i32,
    stage: Stage,
    now: Instant,
    curve: BellCurve,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Run(Instant),
    Toggle,
    Reset,
}

impl Application for MetropolisVisualizer {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let emulator = MetropolisVisualizer {
            stage: Stage::default(),
            curve: BellCurve::default(),
            now: Instant::now(),
            is_playing: false,
            speed: 100,
        };
        (emulator, Command::none())
    }

    fn title(&self) -> String {
        String::from("MCMC Visualizer")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        let divisor = 50.0 / 2.0;
        match message {
            Message::Toggle => {
                self.is_playing = !self.is_playing;
            }
            Message::Reset => {
                self.curve.position = Point { x: 0.0, y: 0.0 };
                self.stage.position = Point { x: 0.0, y: 0.0 };
            }
            Message::Run(_) => {
                if self.now.elapsed().as_millis() >= 50 {
                    self.now = Instant::now();
                    self.stage.position = Point {
                        x: metropolis::metropolis_state(10.0, self.stage.position.x as f64) as f32,
                        //x: gaussian::sample() as f32 * 250.0,
                        y: 5.0,
                    };
                }
                let x64 = self.curve.position.x as f64;
                self.curve.position = Point {
                    x: self.curve.position.x + 1.0,
                    y: (gaussian::distribution_density(2.0, 0.2, (x64 / 10.0) / divisor) * 100.0)
                        as f32
                        + 5.0,
                };
            }
        }
        self.stage.redraw();
        self.curve.redraw();
        return Command::none();
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        column![
            row![
                button("Toggle").on_press(Message::Toggle),
                button("Reset").on_press(Message::Reset),
            ],
            Canvas::new(&self.curve)
                .height(Length::Fill)
                .width(Length::Fill),
            Canvas::new(&self.stage)
                .height(Length::Fill)
                .width(Length::Fill)
        ]
        .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn style(&self) -> theme::Application {
        fn dark_background(_theme: &Theme) -> application::Appearance {
            application::Appearance {
                background_color: Color::BLACK,
                text_color: Color::WHITE,
            }
        }

        theme::Application::from(dark_background as fn(&Theme) -> _)
    }

    fn subscription(&self) -> Subscription<Message> {
        if self.is_playing {
            time::every(Duration::from_millis(1000 / self.speed as u64)).map(Message::Run)
        } else {
            Subscription::none()
        }
    }
}
