use iced::widget::{button, column, container, row, slider, text, Canvas};
use iced::{application, executor, theme, time, Renderer};
use iced::{Application, Color, Command, Element, Length, Point, Settings, Subscription, Theme};

use std::time::{Duration, Instant};

use mcmc::bellcurve::BellCurve;
use mcmc::gaussian;
use mcmc::metropolis;
use mcmc::stage::{Player, Stage};

const X_MEAN: f64 = 1.0;
const Y_MEAN: f64 = 1.0;
const X_STDDEV: f64 = 0.2;
const Y_STDDEV: f64 = 0.2;
const SPEED: u128 = 10;

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
    x_curve: BellCurve,
    y_curve: BellCurve,
    x_mean_slider: u32,
    y_mean_slider: u32,
    x_stddev_slider: u32,
    y_stddev_slider: u32,
    xmean: f32,
    ymean: f32,
    xstddev: f32,
    ystddev: f32,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Run(Instant),
    Toggle,
    Reset,
    XMeanSliderChanged(u32),
    YMeanSliderChanged(u32),
    XStdDevSliderChanged(u32),
    YStdDevSliderChanged(u32),
}

impl Application for MetropolisVisualizer {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let emulator = MetropolisVisualizer {
            stage: Stage::new(
                Point {
                    x: X_MEAN as f32,
                    y: Y_MEAN as f32,
                },
                Point {
                    x: X_STDDEV as f32,
                    y: Y_STDDEV as f32,
                },
                100,
            ),
            xmean: X_MEAN as f32,
            ymean: Y_MEAN as f32,
            xstddev: X_STDDEV as f32,
            ystddev: Y_STDDEV as f32,
            x_curve: BellCurve::new(X_MEAN, X_STDDEV),
            y_curve: BellCurve::new(Y_MEAN, Y_STDDEV),
            now: Instant::now(),
            is_playing: false,
            speed: 100,
            x_mean_slider: 2,
            y_mean_slider: 2,
            x_stddev_slider: 2,
            y_stddev_slider: 2,
        };
        (emulator, Command::none())
    }

    fn title(&self) -> String {
        String::from("MCMC Visualizer")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        let divisor = 50.0 / 2.0;
        match message {
            Message::XMeanSliderChanged(val) => {
                self.x_mean_slider = val;
                self.xmean = val as f32 / 10.0;
                self.stage.mean.x = self.xmean;
            }
            Message::YMeanSliderChanged(val) => {
                self.y_mean_slider = val;
                self.ymean = val as f32 / 10.0;
                self.stage.mean.y = self.ymean;
            }
            Message::XStdDevSliderChanged(val) => {
                self.x_stddev_slider = val;
                self.xstddev = val as f32 / 50.0;
                self.stage.stddev.x = self.xstddev;
            }
            Message::YStdDevSliderChanged(val) => {
                self.y_stddev_slider = val;
                self.ystddev = val as f32 / 50.0;
                self.stage.stddev.y = self.ystddev;
            }
            Message::Toggle => {
                self.is_playing = !self.is_playing;
            }
            Message::Reset => {
                self.x_curve.position = Point { x: 0.0, y: 0.0 };
                self.stage.players = self
                    .stage
                    .players
                    .iter()
                    .map(|_| Player::default())
                    .collect();
            }
            Message::Run(_) => {
                for player in self.stage.players.iter_mut() {
                    let x_pos = player.current.x / 10.0 / divisor;
                    let y_pos = player.current.y / 10.0 / divisor;
                    if self.now.elapsed().as_millis() >= SPEED {
                        player.candidate = metropolis::derive_candidate_2d(
                            (self.xmean as f64, self.ymean as f64),
                            (X_STDDEV, Y_STDDEV),
                            Point { x: x_pos, y: y_pos },
                        );
                        player.candidate.position = Point {
                            x: player.candidate.position.x * 10.0 * divisor,
                            y: player.candidate.position.y * 10.0 * divisor,
                        };
                        player.current =
                            metropolis::metropolis_state_2d(player.current, &player.candidate);
                    }
                }
                let x64 = self.x_curve.position.x as f64;
                self.x_curve.position = Point {
                    x: self.x_curve.position.x + 1.0,
                    y: (gaussian::distribution_density(
                        X_MEAN,
                        Y_STDDEV,
                        (x64 / 10.0) / divisor as f64,
                    ) * 100.0) as f32
                        + 5.0,
                };
                if self.now.elapsed().as_millis() >= SPEED {
                    self.now = Instant::now();
                }
            }
        }
        self.stage.redraw();
        self.x_curve.redraw();
        self.y_curve.redraw();
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        let slider_vals = (
            self.x_mean_slider,
            self.y_mean_slider,
            self.x_stddev_slider,
            self.y_stddev_slider,
        );
        let x_mean_slider =
            container(slider(0..=100, slider_vals.0, Message::XMeanSliderChanged)).width(250);
        let y_mean_slider =
            container(slider(0..=100, slider_vals.1, Message::YMeanSliderChanged)).width(250);
        let x_stddev_slider = container(slider(
            0..=100,
            slider_vals.2,
            Message::XStdDevSliderChanged,
        ))
        .width(250);
        let y_stddev_slider = container(slider(
            0..=100,
            slider_vals.3,
            Message::YStdDevSliderChanged,
        ))
        .width(250);
        column![
            row![
                button("Toggle").on_press(Message::Toggle),
                button("Reset").on_press(Message::Reset),
            ],
            row![
                text(format!("{}", self.stage.mean.x)),
                container(x_mean_slider).width(Length::Fill).center_x(),
                text(format!("{}", self.stage.mean.y)),
                container(y_mean_slider).width(Length::Fill).center_x(),
            ],
            row![
                text(format!("{}", self.stage.stddev.x)),
                container(x_stddev_slider).width(Length::Fill).center_x(),
                text(format!("{}", self.stage.mean.y)),
                container(y_stddev_slider).width(Length::Fill).center_x(),
            ],
            // Canvas::new(&self.x_curve)
            //     .height(Length::Fill)
            //     .width(Length::Fill),
            // row![
            // Canvas::new(&self.y_curve)
            //     .height(Length::Fill)
            //     .width(Length::Fill),
            Canvas::new(&self.stage)
                .height(Length::Fill)
                .width(Length::Fill) // ]
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
