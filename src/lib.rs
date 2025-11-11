#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::restriction)]

use wasm_bindgen::prelude::*;

/// A 2D vector representing a position or velocity in the simulation space.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Vector2D {
    /// The horizontal component.
    pub x: f32,
    /// The vertical component.
    pub y: f32,
}

#[wasm_bindgen]
impl Vector2D {
    /// Creates a new `Vector2D` with the given components.
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32) -> Vector2D {
        Vector2D { x, y }
    }
}

/// Creates a new `Vector2D` from `x` and `y`.
#[wasm_bindgen]
pub fn new_vector2d(x: f32, y: f32) -> Vector2D {
    Vector2D { x, y }
}

/// A ball in the pool simulation with position, velocity, and radius.
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Ball {
    /// The current position of the ball.
    pub position: Vector2D,
    /// The current velocity of the ball.
    pub velocity: Vector2D,
    /// The radius of the ball.
    pub radius: f32,
}

#[wasm_bindgen]
impl Ball {
    /// Creates a new `Ball` given position, velocity, and radius.
    #[wasm_bindgen(constructor)]
    pub fn new(
        x: f32,
        y: f32,
        vx: f32,
        vy: f32,
        radius: f32,
    ) -> Ball {
        Ball {
            position: Vector2D { x, y },
            velocity: Vector2D { x: vx, y: vy },
            radius,
        }
    }

    /// Returns the x coordinate of the ball.
    pub fn x(&self) -> f32 {
        self.position.x
    }

    /// Returns the y coordinate of the ball.
    pub fn y(&self) -> f32 {
        self.position.y
    }

    /// Returns the radius of the ball.
    pub fn radius(&self) -> f32 {
        self.radius
    }
}

/// Creates a new `Ball` via helper function.
#[wasm_bindgen]
pub fn new_ball(
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    radius: f32,
) -> Ball {
    Ball::new(x, y, vx, vy, radius)
}

/// A rectangular pool table area.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Table {
    /// The width of the table.
    pub width: f32,
    /// The height of the table.
    pub height: f32,
}

#[wasm_bindgen]
impl Table {
    /// Creates a new `Table` with the given dimensions.
    #[wasm_bindgen(constructor)]
    pub fn new(width: f32, height: f32) -> Table {
        Table { width, height }
    }
}

/// Creates a new `Table` via helper function.
#[wasm_bindgen]
pub fn new_table(width: f32, height: f32) -> Table {
    Table::new(width, height)
}

/// The complete game state for the pool simulation.
#[wasm_bindgen]
pub struct GameState {
    /// The balls currently in play.
    balls: Vec<Ball>,
    /// The table on which the balls move.
    table: Table,
}

#[wasm_bindgen]
impl GameState {
    /// Returns the number of balls in the game state.
    pub fn balls_len(&self) -> usize {
        self.balls.len()
    }

    /// Returns a reference to the ball at the given index.
    ///
    /// Panics in Rust if out of bounds; when called from JS via wasm-bindgen
    /// this will surface as a trap, so callers must bounds-check first.
    pub fn ball(&self, index: usize) -> Ball {
        self.balls[index].clone()
    }

    /// Returns the table width.
    pub fn table_width(&self) -> f32 {
        self.table.width
    }

    /// Returns the table height.
    pub fn table_height(&self) -> f32 {
        self.table.height
    }
}

/// Creates a new `GameState` with a single moving ball on a default-sized table.
///
/// The table is 800 by 400 units, and the ball is placed near the center with a
/// small initial velocity.
#[wasm_bindgen]
pub fn new_game_state_single_ball() -> GameState {
    let table = Table { width: 800.0, height: 400.0 };
    let ball = Ball {
        position: Vector2D {
            x: table.width * 0.5,
            y: table.height * 0.5,
        },
        velocity: Vector2D { x: 120.0, y: 60.0 },
        radius: 10.0,
    };

    GameState {
        balls: vec![ball],
        table,
    }
}

/// Advances the simulation forward by a time step `dt` (in seconds).
///
/// This updates all ball positions according to their velocities and applies
/// simple wall-collision response against the table bounds. When a ball hits a
/// wall (considering its radius), its corresponding velocity component is
/// inverted to create a bounce effect.
#[wasm_bindgen]
pub fn tick(state: &mut GameState, dt: f32) {
    if dt <= 0.0 {
        return;
    }

    let width = state.table.width;
    let height = state.table.height;

    for ball in &mut state.balls {
        // Integrate position.
        ball.position.x += ball.velocity.x * dt;
        ball.position.y += ball.velocity.y * dt;

        // Left wall.
        if ball.position.x - ball.radius < 0.0 {
            ball.position.x = ball.radius;
            ball.velocity.x = -ball.velocity.x;
        }

        // Right wall.
        if ball.position.x + ball.radius > width {
            ball.position.x = width - ball.radius;
            ball.velocity.x = -ball.velocity.x;
        }

        // Top wall.
        if ball.position.y - ball.radius < 0.0 {
            ball.position.y = ball.radius;
            ball.velocity.y = -ball.velocity.y;
        }

        // Bottom wall.
        if ball.position.y + ball.radius > height {
            ball.position.y = height - ball.radius;
            ball.velocity.y = -ball.velocity.y;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tick_moves_ball_when_dt_positive() {
        let mut state = new_game_state_single_ball();
        let initial_x = state.balls[0].position.x;
        let initial_y = state.balls[0].position.y;
        tick(&mut state, 0.5);
        assert_ne!(state.balls[0].position.x, initial_x);
        assert_ne!(state.balls[0].position.y, initial_y);
    }

    #[test]
    fn wall_bounce_inverts_velocity_x() {
        let table = Table { width: 100.0, height: 100.0 };
        let mut state = GameState {
            balls: vec![Ball {
                position: Vector2D { x: 95.0, y: 50.0 },
                velocity: Vector2D { x: 50.0, y: 0.0 },
                radius: 10.0,
            }],
            table,
        };

        tick(&mut state, 0.5);

        let ball = &state.balls[0];
        assert!(ball.position.x <= table.width - ball.radius + f32::EPSILON);
        assert!(ball.velocity.x < 0.0);
    }

    #[test]
    fn wall_bounce_inverts_velocity_y() {
        let table = Table { width: 100.0, height: 100.0 };
        let mut state = GameState {
            balls: vec![Ball {
                position: Vector2D { x: 50.0, y: 95.0 },
                velocity: Vector2D { x: 0.0, y: 50.0 },
                radius: 10.0,
            }],
            table,
        };

        tick(&mut state, 0.5);

        let ball = &state.balls[0];
        assert!(ball.position.y <= table.height - ball.radius + f32::EPSILON);
        assert!(ball.velocity.y < 0.0);
    }
}