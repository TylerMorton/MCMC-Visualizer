/**
 * The 'stage' area where the metropis hastings alg will be plotted.
 */

use crate::gaussian;

use iced::widget::canvas::{self, stroke, Cache, LineCap, Stroke, Style};
use iced::widget::Canvas;
use iced::Color;
use iced::Theme;
use iced::{Element, Length, Point, Renderer, Size};


pub struct Stage {
    mu: f64,
    sigma: f64,
    pub position: Point,
    line_cache: Cache,
    position_cache: Cache,
}

impl Stage {
    pub fn default() -> Self {
        Self {
            mu: 2.0,
            sigma: 0.2,
            position: Point { x: 0.0, y: 0.0 },
            line_cache: canvas::Cache::default(),
            position_cache: canvas::Cache::default(),
        }
    }

    pub fn new(mu: f64, sigma: f64) -> Self {
        Self {
            mu,
            sigma,
            position: Point { x: 0.0, y: 0.0 },
            line_cache: canvas::Cache::default(),
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


impl<Message> canvas::Program<Message> for Stage {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        _theme: &Theme,
        bounds: iced::Rectangle,
        _cursor: canvas::Cursor,
    ) -> Vec<canvas::Geometry> {

        let geom = self.line_cache.draw(bounds.size(), |frame| {
            let current_point = Point { x: 0.0, y: 0.0 };
            frame.stroke(
              &canvas::Path::new(|path| {
                  path.move_to(current_point);
                  path.line_to(Point {x: bounds.width, y: 0.0});
              }),
              canvas::Stroke {
                  width: 5.0,
                  style: stroke::Style::Solid(Color::WHITE),
                  ..Stroke::default()
              })
        });

        let pos = self.position_cache.draw(bounds.size(), |frame| {
          let path: canvas::Path = canvas::Path::circle(self.position, 5.0);
          frame.fill(&path, Color::from_rgb8(0x12, 0x93, 0xD8));
        });
        vec![geom, pos]
    }
}
