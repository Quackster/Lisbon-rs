//! Mirrors `net.h4bbo.lisbon.game.pathfinder.Position`.
//!
//! Java's `z` is a `double` (kept as `f64` here; the earlier `i32` stub
//! marker is resolved).

#[derive(Clone, Debug)]
pub struct Position {
    x: i32,
    y: i32,
    z: f64,
    body_rotation: i32,
    head_rotation: i32,
}

impl Position {
    /// Const constructor (the `new` equivalent for `const` contexts).
    pub const fn const_new(x: i32, y: i32, z: f64) -> Self {
        Self {
            x,
            y,
            z,
            body_rotation: 0,
            head_rotation: 0,
        }
    }

    /// Mirrors the 3-arg `Position(int, int, double)` constructor.
    pub fn new(x: i32, y: i32, z: f64) -> Self {
        Self {
            x,
            y,
            z,
            body_rotation: 0,
            head_rotation: 0,
        }
    }

    /// Mirrors the 2-arg `Position(int, int)` constructor (`z` defaults to 0).
    pub fn new_xy(x: i32, y: i32) -> Self {
        Self::new(x, y, 0.0)
    }

    /// Mirrors the 5-arg `Position(int, int, double, int, int)` constructor.
    pub fn with_rotations(x: i32, y: i32, z: f64, head_rotation: i32, body_rotation: i32) -> Self {
        Self {
            x,
            y,
            z,
            head_rotation,
            body_rotation,
        }
    }

    /// Checks if the current tile touches the target tile.
    pub fn touches(&self, position: &Position) -> bool {
        self.get_distance_squared(position) <= 1
    }

    /// Mirrors `getX`.
    pub fn get_x(&self) -> i32 {
        self.x
    }

    /// Mirrors `setX`.
    pub fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    /// Mirrors `getY`.
    pub fn get_y(&self) -> i32 {
        self.y
    }

    /// Mirrors `setY`.
    pub fn set_y(&mut self, y: i32) {
        self.y = y;
    }

    /// Mirrors `getZ`.
    pub fn get_z(&self) -> f64 {
        self.z
    }

    /// Mirrors `setZ`.
    pub fn set_z(&mut self, z: f64) {
        self.z = z;
    }

    /// Mirrors `getBodyRotation`.
    pub fn get_body_rotation(&self) -> i32 {
        self.body_rotation
    }

    /// Mirrors `setBodyRotation`.
    pub fn set_body_rotation(&mut self, body_rotation: i32) {
        self.body_rotation = body_rotation;
    }

    /// Mirrors `getHeadRotation`.
    pub fn get_head_rotation(&self) -> i32 {
        self.head_rotation
    }

    /// Mirrors `setHeadRotation`.
    pub fn set_head_rotation(&mut self, head_rotation: i32) {
        self.head_rotation = head_rotation;
    }

    /// Mirrors `getRotation`.
    pub fn get_rotation(&self) -> i32 {
        self.body_rotation
    }

    /// Mirrors `setRotation` (Java sets both `headRotation` and
    /// `bodyRotation` — quirk preserved).
    pub fn set_rotation(&mut self, rotation: i32) {
        self.head_rotation = rotation;
        self.body_rotation = rotation;
    }

    /// Mirrors `add`.
    pub fn add(&self, other: &Position) -> Position {
        Self::new(
            other.get_x() + self.get_x(),
            other.get_y() + self.get_y(),
            other.get_z() + self.get_z(),
        )
    }

    /// Mirrors `subtract`.
    pub fn subtract(&self, other: &Position) -> Position {
        Self::new(
            other.get_x() - self.get_x(),
            other.get_y() - self.get_y(),
            other.get_z() - self.get_z(),
        )
    }

    /// Mirrors `getDistanceSquared` (Java returns the square root of the
    /// squared distance — quirk preserved).
    pub fn get_distance_squared(&self, point: &Position) -> i32 {
        let dx = self.get_x() - point.get_x();
        let dy = self.get_y() - point.get_y();

        ((dx * dx + dy * dy) as f64).sqrt() as i32
    }

    /// Mirrors `getSquareInFront`.
    pub fn get_square_in_front(&self) -> Position {
        let mut square = self.copy();

        match self.body_rotation {
            0 => square.y -= 1,
            1 => {
                square.x += 1;
                square.y -= 1;
            }
            2 => square.x += 1,
            3 => {
                square.x += 1;
                square.y += 1;
            }
            4 => square.y += 1,
            5 => {
                square.x -= 1;
                square.y += 1;
            }
            6 => square.x -= 1,
            7 => {
                square.x -= 1;
                square.y -= 1;
            }
            _ => {}
        }

        square
    }

    /// Mirrors `getSquareBehind`.
    pub fn get_square_behind(&self) -> Position {
        let mut square = self.copy();

        match self.body_rotation {
            0 => square.y += 1,
            1 => {
                square.x -= 1;
                square.y += 1;
            }
            2 => square.x -= 1,
            3 => {
                square.x -= 1;
                square.y -= 1;
            }
            4 => square.y -= 1,
            5 => {
                square.x += 1;
                square.y -= 1;
            }
            6 => square.x += 1,
            7 => {
                square.x += 1;
                square.y += 1;
            }
            _ => {}
        }

        square
    }

    /// Mirrors `getSquareRight`.
    pub fn get_square_right(&self) -> Position {
        let mut square = self.copy();

        match self.body_rotation {
            0 => square.x += 1,
            1 => {
                square.x += 1;
                square.y += 1;
            }
            2 => square.y += 1,
            3 => {
                square.x -= 1;
                square.y += 1;
            }
            4 => square.x -= 1,
            5 => {
                square.x -= 1;
                square.y -= 1;
            }
            6 => square.y -= 1,
            7 => {
                square.x += 1;
                square.y -= 1;
            }
            _ => {}
        }

        square
    }

    /// Mirrors `getSquareLeft`.
    pub fn get_square_left(&self) -> Position {
        let mut square = self.copy();

        match self.body_rotation {
            0 => square.x -= 1,
            1 => {
                square.x -= 1;
                square.y -= 1;
            }
            2 => square.y -= 1,
            3 => {
                square.x += 1;
                square.y -= 1;
            }
            4 => square.x += 1,
            5 => {
                square.x += 1;
                square.y += 1;
            }
            6 => square.y += 1,
            7 => {
                square.x -= 1;
                square.y += 1;
            }
            _ => {}
        }

        square
    }

    /// Coords to create a list of coordinates for a flat circle.
    pub fn get_circle(&self, radius: i32) -> Vec<Position> {
        let mut sphere = Vec::new();

        for x in -radius..=radius {
            for y in -radius..=radius {
                let b = self.add(&Self::new_xy(x, y));

                if self.get_distance_squared(&b) <= radius {
                    sphere.push(b);
                }
            }
        }

        sphere
    }

    /// Mirrors `copy`.
    pub fn copy(&self) -> Position {
        Self::with_rotations(self.x, self.y, self.z, self.head_rotation, self.body_rotation)
    }
}

impl PartialEq for Position {
    /// Mirrors Java's `equals`, which only compares the X and Y
    /// coordinates (intentional in the Java source).
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Position {}

impl Default for Position {
    /// Mirrors the no-arg `Position()` constructor.
    fn default() -> Self {
        Self::new(0, 0, 0.0)
    }
}

impl std::fmt::Display for Position {
    /// Mirrors `toString`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}
