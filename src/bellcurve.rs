use crate::gaussian;

use iced::widget::canvas::{self, stroke, Cache, Stroke};
use iced::{Color, Point, Theme};

pub struct BellCurve {
    mu: f64,
    sigma: f64,
    pub position: Point,
    curve_cache: Cache,
    position_cache: Cache,
}

impl BellCurve {
    pub fn default() -> Self {
        Self {
            mu: 2.0,
            sigma: 0.2,
            position: Point { x: 0.0, y: 0.0 },
            curve_cache: canvas::Cache::default(),
            position_cache: canvas::Cache::default(),
        }
    }

    pub fn new(mu: f64, sigma: f64) -> Self {
        Self {
            mu,
            sigma,
            position: Point { x: 0.0, y: 0.0 },
            curve_cache: canvas::Cache::default(),
            position_cache: canvas::Cache::default(),
        }
    }

    pub fn redraw(&mut self) {
        self.position_cache.clear();
    }

    fn value_at(&self, x: f64) -> f64 {
        gaussian::distribution_density(self.mu, self.sigma, x)
    }
}

impl<Message> canvas::Program<Message> for BellCurve {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        _theme: &Theme,
        bounds: iced::Rectangle,
        _cursor: canvas::Cursor,
    ) -> Vec<canvas::Geometry> {
        let geom = self.curve_cache.draw(bounds.size(), |frame| {
            //let divisor = 50.0 / self.mu as f32;
            let divisor = 50.0 / 2.0 as f32;
            let mut current_point = Point { x: 0.0, y: 0.0 };
            for i in 0..100 {
                let i = i as f32;
                let next_point = Point {
                    x: i * 10.0,
                    y: self.value_at((i as f64) / (divisor as f64)) as f32 * 100.0,
                };
                if i as i32 == 50 {
                    println!("50: x:{} y:{}", next_point.x, next_point.y);
                }
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
        let pos = self.position_cache.draw(bounds.size(), |frame| {
            let path = canvas::Path::circle(self.position, 5.0);
            frame.fill(&path, Color::from_rgb8(0x12, 0x93, 0xD8));
        });
        vec![geom, pos]
    }
}
