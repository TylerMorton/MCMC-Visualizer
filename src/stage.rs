/**
 * The 'stage' area where the metropis hastings alg will be plotted.
 */
use crate::gaussian;

use iced::widget::canvas::{self, stroke, Cache, Stroke};
use iced::{Color, Point, Theme};

pub struct Stage {
    pub position: Point,
    pub candidate_position: Point,
    line_cache: Cache,
    position_cache: Cache,
}

impl Default for Stage {
    fn default() -> Self {
        Self {
            position: Point {
                x: gaussian::sample_custom(2.0, 0.2) as f32 * 250.0,
                y: 0.0,
            },
            candidate_position: Point {
                x: gaussian::sample_custom(2.0, 0.2) as f32 * 250.0,
                y: 0.0,
            },
            line_cache: canvas::Cache::default(),
            position_cache: canvas::Cache::default(),
        }
    }
}

impl Stage {
    pub fn new() -> Self {
        Self {
            position: Point { x: 0.0, y: 0.0 },
            candidate_position: Point { x: 0.0, y: 0.0 },
            line_cache: canvas::Cache::default(),
            position_cache: canvas::Cache::default(),
        }
    }

    pub fn redraw(&mut self) {
        self.position_cache.clear();
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
                    path.line_to(Point {
                        x: bounds.width,
                        y: 0.0,
                    });
                }),
                canvas::Stroke {
                    width: 5.0,
                    style: stroke::Style::Solid(Color::WHITE),
                    ..Stroke::default()
                },
            )
        });

        let pos = self.position_cache.draw(bounds.size(), |frame| {
            let path: canvas::Path = canvas::Path::circle(self.candidate_position, 10.0);
            frame.fill(&path, Color::from_rgb8(0xe7, 0x6f, 0x51));
            let path: canvas::Path = canvas::Path::circle(self.position, 5.0);
            frame.fill(&path, Color::from_rgb8(0x12, 0x93, 0xD8));
        });
        vec![geom, pos]
    }
}
