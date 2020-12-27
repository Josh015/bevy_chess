use std::fmt;

use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum PieceColor {
    White,
    Black,
}

impl PieceColor {
    pub fn opposite(self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

impl fmt::Display for PieceColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::White => write!(f, "White"),
            Self::Black => write!(f, "Black"),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum PieceType {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

struct PieceMaterials {
    black_color: Handle<StandardMaterial>,
    white_color: Handle<StandardMaterial>,
}

impl FromResources for PieceMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<StandardMaterial>>().unwrap();
        PieceMaterials {
            black_color: materials.add(Color::rgb(0.3, 0.3, 0.3).into()),
            white_color: materials.add(Color::rgb(1., 0.8, 0.8).into()),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Piece {
    pub color: PieceColor,
    pub piece_type: PieceType,
    // Current position
    pub x: u8,
    pub y: u8,
}
impl Piece {
    /// Returns the possible_positions that are available
    pub fn is_move_valid(&self, new_position: (u8, u8), pieces: Vec<Piece>) -> bool {
        // If there's a piece of the same color in the same square, it can't move
        if color_of_square(new_position, &pieces) == Some(self.color) {
            return false;
        }

        match self.piece_type {
            PieceType::King => {
                // Horizontal
                ((self.x as i8 - new_position.0 as i8).abs() == 1
                    && (self.y == new_position.1))
                // Vertical
                || ((self.y as i8 - new_position.1 as i8).abs() == 1
                    && (self.x == new_position.0))
                // Diagonal
                || ((self.x as i8 - new_position.0 as i8).abs() == 1
                    && (self.y as i8 - new_position.1 as i8).abs() == 1)
            }
            PieceType::Queen => {
                is_path_empty((self.x, self.y), new_position, &pieces)
                    && ((self.x as i8 - new_position.0 as i8).abs()
                        == (self.y as i8 - new_position.1 as i8).abs()
                        || ((self.x == new_position.0 && self.y != new_position.1)
                            || (self.y == new_position.1 && self.x != new_position.0)))
            }
            PieceType::Bishop => {
                is_path_empty((self.x, self.y), new_position, &pieces)
                    && (self.x as i8 - new_position.0 as i8).abs()
                        == (self.y as i8 - new_position.1 as i8).abs()
            }
            PieceType::Knight => {
                ((self.x as i8 - new_position.0 as i8).abs() == 2
                    && (self.y as i8 - new_position.1 as i8).abs() == 1)
                    || ((self.x as i8 - new_position.0 as i8).abs() == 1
                        && (self.y as i8 - new_position.1 as i8).abs() == 2)
            }
            PieceType::Rook => {
                is_path_empty((self.x, self.y), new_position, &pieces)
                    && ((self.x == new_position.0 && self.y != new_position.1)
                        || (self.y == new_position.1 && self.x != new_position.0))
            }
            PieceType::Pawn => {
                if self.color == PieceColor::White {
                    // Normal move
                    if new_position.0 as i8 - self.x as i8 == 1 && (self.y == new_position.1) {
                        if color_of_square(new_position, &pieces).is_none() {
                            return true;
                        }
                    }

                    // Move 2 squares
                    if self.x == 1
                        && new_position.0 as i8 - self.x as i8 == 2
                        && (self.y == new_position.1)
                        && is_path_empty((self.x, self.y), new_position, &pieces)
                    {
                        if color_of_square(new_position, &pieces).is_none() {
                            return true;
                        }
                    }

                    // Take piece
                    if new_position.0 as i8 - self.x as i8 == 1
                        && (self.y as i8 - new_position.1 as i8).abs() == 1
                    {
                        if color_of_square(new_position, &pieces) == Some(PieceColor::Black) {
                            return true;
                        }
                    }
                } else {
                    // Normal move
                    if new_position.0 as i8 - self.x as i8 == -1 && (self.y == new_position.1) {
                        if color_of_square(new_position, &pieces).is_none() {
                            return true;
                        }
                    }

                    // Move 2 squares
                    if self.x == 6
                        && new_position.0 as i8 - self.x as i8 == -2
                        && (self.y == new_position.1)
                        && is_path_empty((self.x, self.y), new_position, &pieces)
                    {
                        if color_of_square(new_position, &pieces).is_none() {
                            return true;
                        }
                    }

                    // Take piece
                    if new_position.0 as i8 - self.x as i8 == -1
                        && (self.y as i8 - new_position.1 as i8).abs() == 1
                    {
                        if color_of_square(new_position, &pieces) == Some(PieceColor::White) {
                            return true;
                        }
                    }
                }

                false
            }
        }
    }
}

fn is_path_empty(begin: (u8, u8), end: (u8, u8), pieces: &Vec<Piece>) -> bool {
    // Same column
    if begin.0 == end.0 {
        for piece in pieces {
            if piece.x == begin.0
                && ((piece.y > begin.1 && piece.y < end.1)
                    || (piece.y > end.1 && piece.y < begin.1))
            {
                return false;
            }
        }
    }
    // Same row
    if begin.1 == end.1 {
        for piece in pieces {
            if piece.y == begin.1
                && ((piece.x > begin.0 && piece.x < end.0)
                    || (piece.x > end.0 && piece.x < begin.0))
            {
                return false;
            }
        }
    }

    // Diagonals
    let x_diff = (begin.0 as i8 - end.0 as i8).abs();
    let y_diff = (begin.1 as i8 - end.1 as i8).abs();
    if x_diff == y_diff {
        for i in 1..x_diff {
            let pos = if begin.0 < end.0 && begin.1 < end.1 {
                // left bottom - right top
                (begin.0 + i as u8, begin.1 + i as u8)
            } else if begin.0 < end.0 && begin.1 > end.1 {
                // left top - right bottom
                (begin.0 + i as u8, begin.1 - i as u8)
            } else if begin.0 > end.0 && begin.1 < end.1 {
                // right bottom - left top
                (begin.0 - i as u8, begin.1 + i as u8)
            } else {
                // begin.0 > end.0 && begin.1 > end.1
                // right top - left bottom
                (begin.0 - i as u8, begin.1 - i as u8)
            };

            if color_of_square(pos, pieces).is_some() {
                return false;
            }
        }
    }

    true
}

/// Returns None if square is empty, returns a Some with the color if not
fn color_of_square(pos: (u8, u8), pieces: &Vec<Piece>) -> Option<PieceColor> {
    for piece in pieces {
        if piece.x == pos.0 && piece.y == pos.1 {
            return Some(piece.color);
        }
    }
    None
}

fn move_pieces(time: Res<Time>, mut query: Query<(&mut Transform, &Piece)>) {
    for (mut transform, piece) in query.iter_mut() {
        // Get the direction to move in
        let direction = Vec3::new(piece.x as f32, 0., piece.y as f32) - transform.translation;

        // Only move if the piece isn't already there (distance is big)
        if direction.length() > 0.1 {
            transform.translation += direction.normalize() * time.delta_seconds();
        }
    }
}

fn create_pieces(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    materials: Res<PieceMaterials>,
) {
    // Spawn the entities.
    let pieces = vec![
        (
            PieceType::Pawn,
            Vec3::new(-0.2, 0., 2.6),
            vec![asset_server.load("models/chess_kit/pieces.glb#Mesh2/Primitive0")],
            vec![
                (PieceColor::White, (0..8).map(|i| (1, i)).collect()),
                (PieceColor::Black, (0..8).map(|i| (6, i)).collect()),
            ],
        ),
        (
            PieceType::Rook,
            Vec3::new(-0.1, 0., 1.8),
            vec![asset_server.load("models/chess_kit/pieces.glb#Mesh5/Primitive0")],
            vec![
                (PieceColor::White, vec![(0, 0), (0, 7)]),
                (PieceColor::Black, vec![(7, 0), (7, 7)]),
            ],
        ),
        (
            PieceType::Knight,
            Vec3::new(-0.2, 0., 0.9),
            vec![
                asset_server.load("models/chess_kit/pieces.glb#Mesh3/Primitive0"),
                asset_server.load("models/chess_kit/pieces.glb#Mesh4/Primitive0"),
            ],
            vec![
                (PieceColor::White, vec![(0, 1), (0, 6)]),
                (PieceColor::Black, vec![(7, 1), (7, 6)]),
            ],
        ),
        (
            PieceType::Bishop,
            Vec3::new(-0.1, 0., 0.),
            vec![asset_server.load("models/chess_kit/pieces.glb#Mesh6/Primitive0")],
            vec![
                (PieceColor::White, vec![(0, 2), (0, 5)]),
                (PieceColor::Black, vec![(7, 2), (7, 5)]),
            ],
        ),
        (
            PieceType::Queen,
            Vec3::new(-0.2, 0., -0.95),
            vec![asset_server.load("models/chess_kit/pieces.glb#Mesh7/Primitive0")],
            vec![
                (PieceColor::White, vec![(0, 3)]),
                (PieceColor::Black, vec![(7, 3)]),
            ],
        ),
        (
            PieceType::King,
            Vec3::new(-0.2, 0., -1.9),
            vec![
                asset_server.load("models/chess_kit/pieces.glb#Mesh0/Primitive0"),
                asset_server.load("models/chess_kit/pieces.glb#Mesh1/Primitive0"),
            ],
            vec![
                (PieceColor::White, vec![(0, 4)]),
                (PieceColor::Black, vec![(7, 4)]),
            ],
        ),
    ];

    for piece in pieces {
        for color in piece.3 {
            for position in color.1 {
                spawn_piece(
                    commands,
                    &materials,
                    color.0,
                    piece.0,
                    position,
                    piece.1,
                    piece.2.iter().map(|mesh| mesh.clone()).collect(),
                );
            }
        }
    }
}

fn spawn_piece(
    commands: &mut Commands,
    materials: &Res<PieceMaterials>,
    piece_color: PieceColor,
    piece_type: PieceType,
    position: (u8, u8),
    child_translation: Vec3,
    meshes: Vec<Handle<Mesh>>,
) {
    commands
        // Spawn parent entity
        .spawn(PbrBundle {
            transform: Transform::from_translation(Vec3::new(
                position.0 as f32,
                0.,
                position.1 as f32,
            )),
            ..Default::default()
        })
        .with(Piece {
            color: piece_color,
            piece_type: piece_type,
            x: position.0,
            y: position.1,
        })
        // Add children to the parent
        .with_children(|parent| {
            for mesh in meshes {
                parent.spawn(PbrBundle {
                    mesh: mesh,
                    material: match piece_color {
                        PieceColor::White => materials.white_color.clone(),
                        PieceColor::Black => materials.black_color.clone(),
                    },
                    transform: {
                        let mut transform = Transform::from_translation(child_translation);
                        transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                        transform
                    },
                    ..Default::default()
                });
            }
        });
}

pub struct PiecesPlugin;
impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<PieceMaterials>()
            .add_startup_system(create_pieces.system())
            .add_system(move_pieces.system());
    }
}
