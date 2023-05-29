/**
 * The 'stage' area where the metropis hastings alg will be plotted.
 */
use crate::gaussian;
use crate::metropolis::Candidate;

use iced::widget::canvas::{self, stroke, Cache, Stroke};
use iced::{Color, Point, Theme};

pub struct Player {
    pub candidate: Candidate,
    pub current: Point,
}
impl Default for Player {
    fn default() -> Self {
        Player {
            candidate: Candidate::new((0.0, 0.0), Point { x: 0.0, y: 5.0 }),
            current: Point {
                x: gaussian::sample_custom(2.0, 0.2) as f32 * 250.0,
                y: gaussian::sample_custom(2.0, 0.2) as f32 * 250.0,
            },
        }
    }
}
impl Clone for Player {
    fn clone(&self) -> Self {
        Player::default()
    }
}
pub struct Stage {
    pub players: Vec<Player>,
    pub mean: Point,
    pub stddev: Point,
    line_cache: Cache,
    position_cache: Cache,
    x_curve_cache: Cache,
    y_curve_cache: Cache,
}

impl Default for Stage {
    fn default() -> Self {
        Self {
            mean: Point { x: 2.0, y: 2.0 },
            stddev: Point { x: 0.2, y: 0.2 },
            players: vec![Player::default(); 100],
            line_cache: canvas::Cache::default(),
            position_cache: canvas::Cache::default(),
            x_curve_cache: canvas::Cache::default(),
            y_curve_cache: canvas::Cache::default(),
        }
    }
}

impl Stage {
    pub fn new(mean: Point, stddev: Point, player_num: usize) -> Self {
        Self {
            mean,
            stddev,
            players: vec![Player::default(); player_num],
            line_cache: canvas::Cache::default(),
            position_cache: canvas::Cache::default(),
            x_curve_cache: canvas::Cache::default(),
            y_curve_cache: canvas::Cache::default(),
        }
    }

    pub fn redraw(&mut self) {
        self.position_cache.clear();
        self.x_curve_cache.clear();
        self.y_curve_cache.clear()
    }

    fn x_value_at(&self, x: f64) -> f64 {
        gaussian::distribution_density(self.mean.x as f64, self.stddev.x as f64, x)
    }
    fn y_value_at(&self, y: f64) -> f64 {
        gaussian::distribution_density(self.mean.y as f64, self.stddev.y as f64, y)
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
        let x_curve = self.x_curve_cache.draw(bounds.size(), |frame| {
            //let divisor = 50.0 / self.mu as f32;
            let divisor = 50.0 / 2.0;
            let mut current_point = Point { x: 0.0, y: 0.0 };
            for i in 0..100 {
                let i = i as f32;
                let next_point = Point {
                    x: i * 10.0,
                    y: self.x_value_at((i as f64) / (divisor)) as f32 * 100.0,
                };
                frame.stroke(
                    &canvas::Path::new(|path| {
                        path.move_to(current_point);
                        path.line_to(Point {
                            x: i * 10.0,
                            y: self.x_value_at((i as f64) / divisor) as f32 * 100.0,
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
        let y_curve = self.y_curve_cache.draw(bounds.size(), |frame| {
            //let divisor = 50.0 / self.mu as f32;
            let divisor = 50.0 / 2.0;
            let mut current_point = Point { x: 0.0, y: 0.0 };
            for i in 0..100 {
                let i = i as f32;
                let next_point = Point {
                    y: i * 10.0,
                    x: self.y_value_at((i as f64) / (divisor)) as f32 * 100.0,
                };
                frame.stroke(
                    &canvas::Path::new(|path| {
                        path.move_to(current_point);
                        path.line_to(Point {
                            y: i * 10.0,
                            x: self.y_value_at((i as f64) / divisor) as f32 * 100.0,
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
            );
            let current_point = Point {
                x: bounds.width / 2.0,
                y: 0.0,
            };
            frame.stroke(
                &canvas::Path::new(|path| {
                    path.move_to(current_point);
                    path.line_to(Point {
                        x: bounds.width / 2.0,
                        y: bounds.height,
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
            for player in self.players.iter() {
                // let path: canvas::Path = canvas::Path::circle(player.candidate.position, 10.0);
                // frame.fill(&path, Color::from_rgb8(0xe7, 0x6f, 0x51));
                let path: canvas::Path = canvas::Path::circle(player.current, 5.0);
                frame.fill(&path, Color::from_rgb8(0x12, 0x93, 0xD8));
            }
        });
        vec![geom, pos, x_curve, y_curve]
    }
}
