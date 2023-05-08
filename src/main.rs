use iced::application;
use iced::theme;
use iced::widget::canvas::event::{self, Event};
use iced::window::frames;
use iced::{
    executor,
    widget::{button, canvas, column, row, text, Canvas},
    Application, Color, Command, Element, Length, Point, Settings, Subscription, Theme,
};

use iced::{time, Renderer};
use mcmc::gaussian;
use mcmc::stage::Stage;
//use plotters_iced::{Chart, ChartBuilder, ChartWidget, DrawingBackend};
use std::time::{Duration, Instant};

use mcmc::bellcurve;
use mcmc::bellcurve::BellCurve;

pub fn main() -> iced::Result {
    MetropolisVisualizer::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}

struct MetropolisVisualizer {
    value: i32,
    is_playing: bool,
    speed: i32,
    stage: Stage,
    now: Instant,
    //graph: CircleGraph,
    curve: BellCurve,
}

struct CircleGraph {
    point: Point,
    cache: canvas::Cache,
    iteration: i32,
}

impl canvas::Program<Message> for CircleGraph {
    type State = ();

    fn update(
        &self,
        _state: &mut Self::State,
        _event: canvas::Event,
        _bounds: iced::Rectangle,
        _cursor: canvas::Cursor,
    ) -> (event::Status, Option<Message>) {
        let new_point: Point = Point {
            x: self.point.x + 1.0,
            y: self.point.y,
        };
        (
            event::Status::Captured,
            Some(Message::PointAdded(new_point)),
        )
    }

    fn draw(
        &self,
        state: &Self::State,
        theme: &Theme,
        bounds: iced::Rectangle,
        cursor: canvas::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut dist: Vec<canvas::Geometry> = Vec::new();
        let geom = self.cache.draw(bounds.size(), |frame| {
            frame.stroke(
                &canvas::Path::rectangle(Point::ORIGIN, frame.size()),
                canvas::Stroke::default(),
            );
            let path = canvas::Path::circle(self.point, 5.0);
            frame.fill(&path, Color::from_rgb8(0x12, 0x93, 0xD8));
        });

        let mut actual = vec![geom];
        dist.append(&mut actual);
        dist
    }
}

impl CircleGraph {
    fn new() -> CircleGraph {
        CircleGraph {
            point: Point { x: 0.0, y: 0.0 },
            cache: canvas::Cache::default(),
            iteration: 0,
        }
    }

    fn redraw(&mut self) {
        self.cache.clear();
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    PointAdded(Point),
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
            //graph: CircleGraph::new(),
            stage: Stage::default(),
            curve: BellCurve::default(),
            now: Instant::now(),
            value: 0,
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
            Message::Run(_) => {
                if self.now.elapsed().as_secs() >= 1 {
                    self.now = Instant::now();
                self.stage.position = Point {
                    x: gaussian::sample() as f32  * 200.0,
                    y: 5.0,
                };
                }
                let x64 = self.curve.position.x as f64;
                self.curve.position = Point {
                    x: self.curve.position.x + 1.0,
                    y: (gaussian::distribution_density(2.0, 0.2, (x64 / 10.0) / divisor) * 100.0)
                        as f32 + 5.0,
                };
            }
            Message::PointAdded(point) => {
            }
            Message::Reset => {
                self.curve.position = Point {x: 0.0, y: 0.0};
                self.stage.position = Point {x: 0.0, y: 0.0};
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
