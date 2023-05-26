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
            candidate: Candidate::new(0.0, Point { x: 0.0, y: 5.0 }),
            current: Point {
                x: gaussian::sample_custom(2.0, 0.2) as f32 * 250.0,
                y: 5.0,
            },
        }
    }
}

pub struct Stage {
    pub players: Vec<Player>,
    line_cache: Cache,
    position_cache: Cache,
}

impl Default for Stage {
    fn default() -> Self {
        Self {
            players: vec![Player::default(), Player::default(), Player::default()],
            line_cache: canvas::Cache::default(),
            position_cache: canvas::Cache::default(),
        }
    }
}

impl Stage {
    pub fn new() -> Self {
        Self {
            players: vec![Player {
                candidate: Candidate::new(0.0, Point { x: 0.0, y: 5.0 }),
                current: Point {
                    x: gaussian::sample_custom(2.0, 0.2) as f32 * 250.0,
                    y: 5.0,
                },
            }],
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
            for player in self.players.iter() {
                let path: canvas::Path = canvas::Path::circle(player.candidate.position, 10.0);
                frame.fill(&path, Color::from_rgb8(0xe7, 0x6f, 0x51));
                let path: canvas::Path = canvas::Path::circle(player.current, 5.0);
                frame.fill(&path, Color::from_rgb8(0x12, 0x93, 0xD8));
            }
        });
        vec![geom, pos]
    }
}
