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
    graph: CircleGraph,
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
        println!("drawing");
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
}

impl Application for MetropolisVisualizer {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let emulator = MetropolisVisualizer {
            graph: CircleGraph::new(),
            curve: BellCurve::default(),
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
        let x64 = self.graph.point.x as f64;
        println!("{} {}", self.graph.point.x + 1.0, gaussian::distribution_density(2.0, 0.2, (x64 / 10.0) / divisor) + 100.0);
        match message {
            Message::Toggle => {
                self.is_playing = !self.is_playing;
            }
            Message::Run(_) => {
                let x64 = self.graph.point.x as f64;
                println!(
                    "x: {} y: {}",
                    self.graph.point.x + 1.0,
                    gaussian::distribution_density(2.0, 0.2, x64 /  2.0) + 100.0
                );
                self.graph.point = Point {
                    x: self.graph.point.x + 1.0,
                    y: (gaussian::distribution_density(2.0, 0.4, (x64 / 10.0) / divisor) * 100.0 + 100.0)
                        as f32,
                };
            }
            Message::PointAdded(point) => {
                self.graph.point = point;
            }
        }
        self.graph.redraw();
        return Command::none();
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        column![
            button("Toggle").on_press(Message::Toggle),
            Canvas::new(&self.curve)
                .height(Length::Fill)
                .width(Length::Fill),
            Canvas::new(&self.graph)
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
