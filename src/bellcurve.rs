use crate::gaussian;

use iced::widget::canvas::{self, stroke, Cache, LineCap, Stroke, Style};
use iced::widget::Canvas;
use iced::Color;
use iced::Theme;
use iced::{Element, Length, Point, Renderer, Size};

enum Message {}

pub struct BellCurve {
    mu: f64,
    sigma: f64,
    cache: Cache,
}

impl BellCurve {
    pub fn default() -> Self {
        Self {
            mu: 2.0,
            sigma: 0.2,
            cache: canvas::Cache::default(),
        }
    }

    pub fn new(mu: f64, sigma: f64) -> Self {
        Self {
            mu,
            sigma,
            cache: canvas::Cache::default(),
        }
    }

    fn value_at(&self, x: f64) -> f64 {
        gaussian::distribution_density(self.mu, self.sigma, x)
    }
}

// pub fn bellcurve_default() -> Canvas<(), Theme, BellCurve> {
//   Canvas::new(BellCurve::default())
//   .width(Length::Fill)
//   .height(Length::Fill)
// }

impl<Message> canvas::Program<Message> for BellCurve {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        _theme: &Theme,
        bounds: iced::Rectangle,
        _cursor: canvas::Cursor,
    ) -> Vec<canvas::Geometry> {

        let geom = self.cache.draw(bounds.size(), |frame| {
            let divisor = 50.0 / self.mu as f32;
            let mut current_point = Point { x: 0.0, y: 0.0 };
            for i in 0..100 {
                let i = i as f32;
                let next_point = Point {
                    x: i * 10.0,
                    y: self.value_at((i as f64) / (divisor as f64)) as f32 * 100.0,
                };
                frame.stroke(
                    &canvas::Path::new(|path| {
                        path.move_to(current_point);
                        path.line_to(Point {
                            x: i * 10.0,
                            y: self.value_at((i as f64) / (divisor as f64)) as f32 * 100.0,
                        });
                    }),
                    canvas::Stroke {
                        width: 5.0,
                        style: stroke::Style::Solid(Color::WHITE),
                        ..Stroke::default()
                    },
                );
                current_point = next_point
            }
        });
        vec![geom]
    }
}

// fn draw(&self, frame: &mut canvas::Frame) {
//   let bounds = frame.bounds();
//   let width = bounds.width;