// Imports
use crate::solution::set_piece::{SetPiece, define_set_piece};

// Constants
pub const SET_PIECES: [SetPiece; 92] = [
    SET_PIECE_3_HORIZONTAL,
    SET_PIECE_3_VERTICAL,
    SET_PIECE_3_FORWARD,
    SET_PIECE_3_BACKWARD,
    SET_PIECE_1_ARM_HORIZONTAL_LEFT,
    SET_PIECE_1_ARM_HORIZONTAL_RIGHT,
    SET_PIECE_1_ARM_VERTICAL_UP,
    SET_PIECE_1_ARM_VERTICAL_DOWN,
    SET_PIECE_1_ARM_FORWARD_FRONT,
    SET_PIECE_1_ARM_FORWARD_BACK,
    SET_PIECE_1_ARM_BACKWARD_FRONT,
    SET_PIECE_1_ARM_BACKWARD_BACK,
    SET_PIECE_2_ARM_HORIZONTAL_LEFT,
    SET_PIECE_2_ARM_HORIZONTAL_RIGHT,
    SET_PIECE_2_ARM_VERTICAL_UP,
    SET_PIECE_2_ARM_VERTICAL_DOWN,
    SET_PIECE_2_ARM_FORWARD_FRONT,
    SET_PIECE_2_ARM_FORWARD_BACK,
    SET_PIECE_2_ARM_BACKWARD_FRONT,
    SET_PIECE_2_ARM_BACKWARD_BACK,
    SET_PIECE_1_OPEN_HORIZONTAL,
    SET_PIECE_1_OPEN_VERTICAL,
    SET_PIECE_1_OPEN_FORWARD,
    SET_PIECE_1_OPEN_BACKWARD,
    SET_PIECE_2_OPEN_HORIZONTAL,
    SET_PIECE_2_OPEN_VERTICAL,
    SET_PIECE_2_OPEN_FORWARD,
    SET_PIECE_2_OPEN_BACKWARD,
    SET_PIECE_3_OPEN_HORIZONTAL,
    SET_PIECE_3_OPEN_VERTICAL,
    SET_PIECE_3_OPEN_FORWARD,
    SET_PIECE_3_OPEN_BACKWARD,
    SET_PIECE_1_SMOTHER_HORIZONTAL,
    SET_PIECE_1_SMOTHER_VERTICAL,
    SET_PIECE_1_SMOTHER_FORWARD,
    SET_PIECE_1_SMOTHER_BACKWARD,
    SET_PIECE_2_SMOTHER_HORIZONTAL,
    SET_PIECE_2_SMOTHER_VERTICAL,
    SET_PIECE_2_SMOTHER_FORWARD,
    SET_PIECE_2_SMOTHER_BACKWARD,
    SET_PIECE_1_EDGE_HORIZONTAL_LEFT,
    SET_PIECE_1_EDGE_HORIZONTAL_RIGHT,
    SET_PIECE_1_EDGE_VERTICAL_UP,
    SET_PIECE_1_EDGE_VERTICAL_DOWN,
    SET_PIECE_1_EDGE_FORWARD_FRONT,
    SET_PIECE_1_EDGE_FORWARD_BACK,
    SET_PIECE_1_EDGE_BACKWARD_FRONT,
    SET_PIECE_1_EDGE_BACKWARD_BACK,
    SET_PIECE_2_EDGE_HORIZONTAL_LEFT,
    SET_PIECE_2_EDGE_HORIZONTAL_RIGHT,
    SET_PIECE_2_EDGE_VERTICAL_UP,
    SET_PIECE_2_EDGE_VERTICAL_DOWN,
    SET_PIECE_2_EDGE_FORWARD_FRONT,
    SET_PIECE_2_EDGE_FORWARD_BACK,
    SET_PIECE_2_EDGE_BACKWARD_FRONT,
    SET_PIECE_2_EDGE_BACKWARD_BACK,
    SET_PIECE_3_CLOSE_HORIZONTAL,
    SET_PIECE_3_CLOSE_VERTICAL,
    SET_PIECE_3_CLOSE_FORWARD,
    SET_PIECE_3_CLOSE_BACKWARD,
    SET_PIECE_DIAGONAL_FORK_UP,
    SET_PIECE_DIAGONAL_FORK_DOWN,
    SET_PIECE_DIAGONAL_FORK_LEFT,
    SET_PIECE_DIAGONAL_FORK_RIGHT,
    SET_PIECE_SPOON_FORK_UP_LEFT,
    SET_PIECE_SPOON_FORK_UP_RIGHT,
    SET_PIECE_SPOON_FORK_DOWN_LEFT,
    SET_PIECE_SPOON_FORK_DOWN_RIGHT,
    SET_PIECE_SPOON_FORK_LEFT_UP,
    SET_PIECE_SPOON_FORK_LEFT_DOWN,
    SET_PIECE_SPOON_FORK_RIGHT_UP,
    SET_PIECE_SPOON_FORK_RIGHT_DOWN,
    SET_PIECE_STRAIGHT_FORK_UP,
    SET_PIECE_STRAIGHT_FORK_DOWN,
    SET_PIECE_STRAIGHT_FORK_LEFT,
    SET_PIECE_STRAIGHT_FORK_RIGHT,
    SET_PIECE_SKEWER_FORK_S_UP,
    SET_PIECE_SKEWER_FORK_Z_UP,
    SET_PIECE_SKEWER_FORK_S_DOWN,
    SET_PIECE_SKEWER_FORK_Z_DOWN,
    SET_PIECE_SKEWER_FORK_S_LEFT,
    SET_PIECE_SKEWER_FORK_Z_LEFT,
    SET_PIECE_SKEWER_FORK_S_RIGHT,
    SET_PIECE_SKEWER_FORK_Z_RIGHT,
    SET_PIECE_TEE_FORK_UP_LEFT,
    SET_PIECE_TEE_FORK_UP_RIGHT,
    SET_PIECE_TEE_FORK_DOWN_LEFT,
    SET_PIECE_TEE_FORK_DOWN_RIGHT,
    SET_PIECE_TEE_FORK_LEFT_UP,
    SET_PIECE_TEE_FORK_LEFT_DOWN,
    SET_PIECE_TEE_FORK_RIGHT_UP,
    SET_PIECE_TEE_FORK_RIGHT_DOWN
];

// Set pieces : 3-in-a-row
pub const SET_PIECE_3_HORIZONTAL: SetPiece = define_set_piece([
    'f', 'f', 'f', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 3, 1, 150.0);
pub const SET_PIECE_3_VERTICAL: SetPiece = define_set_piece([
    'f', ' ', ' ', ' ', ' ',
    'f', ' ', ' ', ' ', ' ',
    'f', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 1, 3, 150.0);
pub const SET_PIECE_3_FORWARD: SetPiece = define_set_piece([
    ' ', ' ', 'f', ' ', ' ',
    ' ', 'f', ' ', ' ', ' ',
    'f', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 3, 3, 150.0);
pub const SET_PIECE_3_BACKWARD: SetPiece = define_set_piece([
    'f', ' ', ' ', ' ', ' ',
    ' ', 'f', ' ', ' ', ' ',
    ' ', ' ', 'f', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 3, 3, 150.0);

// Set pieces : 1-in-a-row (Arm Position)
pub const SET_PIECE_1_ARM_HORIZONTAL_LEFT: SetPiece = define_set_piece([
    'f', 'e', 'e', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 3, 1, 20.0);
pub const SET_PIECE_1_ARM_HORIZONTAL_RIGHT: SetPiece = define_set_piece([
    'e', 'e', 'f', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 3, 1, 20.0);
pub const SET_PIECE_1_ARM_VERTICAL_UP: SetPiece = define_set_piece([
    'f', ' ', ' ', ' ', ' ',
    'e', ' ', ' ', ' ', ' ',
    'e', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 1, 3, 20.0);
pub const SET_PIECE_1_ARM_VERTICAL_DOWN: SetPiece = define_set_piece([
    'e', ' ', ' ', ' ', ' ',
    'e', ' ', ' ', ' ', ' ',
    'f', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 1, 3, 20.0);
pub const SET_PIECE_1_ARM_FORWARD_FRONT: SetPiece = define_set_piece([
    ' ', ' ', 'f', ' ', ' ',
    ' ', 'e', ' ', ' ', ' ',
    'e', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 3, 3, 20.0);
pub const SET_PIECE_1_ARM_FORWARD_BACK: SetPiece = define_set_piece([
    ' ', ' ', 'e', ' ', ' ',
    ' ', 'e', ' ', ' ', ' ',
    'f', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 3, 3, 20.0);
pub const SET_PIECE_1_ARM_BACKWARD_FRONT: SetPiece = define_set_piece([
    'e', ' ', ' ', ' ', ' ',
    ' ', 'e', ' ', ' ', ' ',
    ' ', ' ', 'f', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 3, 3, 20.0);
pub const SET_PIECE_1_ARM_BACKWARD_BACK: SetPiece = define_set_piece([
    'f', ' ', ' ', ' ', ' ',
    ' ', 'e', ' ', ' ', ' ',
    ' ', ' ', 'e', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 3, 3, 20.0);

// Set pieces : 2-in-a-row (Arm Position)
pub const SET_PIECE_2_ARM_HORIZONTAL_LEFT: SetPiece = define_set_piece([
    'f', 'f', 'e', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 3, 1, 40.0);
pub const SET_PIECE_2_ARM_HORIZONTAL_RIGHT: SetPiece = define_set_piece([
    'e', 'f', 'f', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 3, 1, 40.0);
pub const SET_PIECE_2_ARM_VERTICAL_UP: SetPiece = define_set_piece([
    'f', ' ', ' ', ' ', ' ',
    'f', ' ', ' ', ' ', ' ',
    'e', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 1, 3, 40.0);
pub const SET_PIECE_2_ARM_VERTICAL_DOWN: SetPiece = define_set_piece([
    'e', ' ', ' ', ' ', ' ',
    'f', ' ', ' ', ' ', ' ',
    'f', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 1, 3, 40.0);
pub const SET_PIECE_2_ARM_FORWARD_FRONT: SetPiece = define_set_piece([
    ' ', ' ', 'f', ' ', ' ',
    ' ', 'f', ' ', ' ', ' ',
    'e', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 3, 3, 40.0);
pub const SET_PIECE_2_ARM_FORWARD_BACK: SetPiece = define_set_piece([
    ' ', ' ', 'e', ' ', ' ',
    ' ', 'f', ' ', ' ', ' ',
    'f', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 3, 3, 40.0);
pub const SET_PIECE_2_ARM_BACKWARD_FRONT: SetPiece = define_set_piece([
    'e', ' ', ' ', ' ', ' ',
    ' ', 'f', ' ', ' ', ' ',
    ' ', ' ', 'f', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 3, 3, 40.0);
pub const SET_PIECE_2_ARM_BACKWARD_BACK: SetPiece = define_set_piece([
    'f', ' ', ' ', ' ', ' ',
    ' ', 'f', ' ', ' ', ' ',
    ' ', ' ', 'e', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 3, 3, 40.0);

// Set pieces : 1-in-a-row (Open Position)
pub const SET_PIECE_1_OPEN_HORIZONTAL: SetPiece = define_set_piece([
    'e', 'f', 'e', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 3, 1, 50.0);
pub const SET_PIECE_1_OPEN_VERTICAL: SetPiece = define_set_piece([
    'e', ' ', ' ', ' ', ' ',
    'f', ' ', ' ', ' ', ' ',
    'e', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 1, 3, 50.0);
pub const SET_PIECE_1_OPEN_FORWARD: SetPiece = define_set_piece([
    ' ', ' ', 'e', ' ', ' ',
    ' ', 'f', ' ', ' ', ' ',
    'e', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 3, 3, 50.0);
pub const SET_PIECE_1_OPEN_BACKWARD: SetPiece = define_set_piece([
    'e', ' ', ' ', ' ', ' ',
    ' ', 'f', ' ', ' ', ' ',
    ' ', ' ', 'e', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 3, 3, 50.0);

// Set pieces : 2-in-a-row (Open Position)
pub const SET_PIECE_2_OPEN_HORIZONTAL: SetPiece = define_set_piece([
    'e', 'f', 'f', 'e', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 4, 1, 100.0);
pub const SET_PIECE_2_OPEN_VERTICAL: SetPiece = define_set_piece([
    'e', ' ', ' ', ' ', ' ',
    'f', ' ', ' ', ' ', ' ',
    'f', ' ', ' ', ' ', ' ',
    'e', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 1, 4, 100.0);
pub const SET_PIECE_2_OPEN_FORWARD: SetPiece = define_set_piece([
    ' ', ' ', ' ', 'e', ' ',
    ' ', ' ', 'f', ' ', ' ',
    ' ', 'f', ' ', ' ', ' ',
    'e', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 4, 4, 100.0);
pub const SET_PIECE_2_OPEN_BACKWARD: SetPiece = define_set_piece([
    'e', ' ', ' ', ' ', ' ',
    ' ', 'f', ' ', ' ', ' ',
    ' ', ' ', 'f', ' ', ' ',
    ' ', ' ', ' ', 'e', ' ',
    ' ', ' ', ' ', ' ', ' '
], 4, 4, 100.0);

// Set pieces : 3-in-a-row (Open Position)
pub const SET_PIECE_3_OPEN_HORIZONTAL: SetPiece = define_set_piece([
    'e', 'f', 'f', 'f', 'e',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 5, 1, 100.0);
pub const SET_PIECE_3_OPEN_VERTICAL: SetPiece = define_set_piece([
    'e', ' ', ' ', ' ', ' ',
    'f', ' ', ' ', ' ', ' ',
    'f', ' ', ' ', ' ', ' ',
    'f', ' ', ' ', ' ', ' ',
    'e', ' ', ' ', ' ', ' '
], 1, 5, 100.0);
pub const SET_PIECE_3_OPEN_FORWARD: SetPiece = define_set_piece([
    ' ', ' ', ' ', ' ', 'e',
    ' ', ' ', ' ', 'f', ' ',
    ' ', ' ', 'f', ' ', ' ',
    ' ', 'f', ' ', ' ', ' ',
    'e', ' ', ' ', ' ', ' '
], 5, 5, 100.0);
pub const SET_PIECE_3_OPEN_BACKWARD: SetPiece = define_set_piece([
    'e', ' ', ' ', ' ', ' ',
    ' ', 'f', ' ', ' ', ' ',
    ' ', ' ', 'f', ' ', ' ',
    ' ', ' ', ' ', 'f', ' ',
    ' ', ' ', ' ', ' ', 'e'
], 5, 5, 100.0);

// Set pieces : 1-in-a-row (Smother Position)
pub const SET_PIECE_1_SMOTHER_HORIZONTAL: SetPiece = define_set_piece([
    'b', 'f', 'b', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 3, 1, -20.0);
pub const SET_PIECE_1_SMOTHER_VERTICAL: SetPiece = define_set_piece([
    'b', ' ', ' ', ' ', ' ',
    'f', ' ', ' ', ' ', ' ',
    'b', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 1, 3, -20.0);
pub const SET_PIECE_1_SMOTHER_FORWARD: SetPiece = define_set_piece([
    ' ', ' ', 'b', ' ', ' ',
    ' ', 'f', ' ', ' ', ' ',
    'b', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 3, 3, -20.0);
pub const SET_PIECE_1_SMOTHER_BACKWARD: SetPiece = define_set_piece([
    'b', ' ', ' ', ' ', ' ',
    ' ', 'f', ' ', ' ', ' ',
    ' ', ' ', 'b', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 3, 3, -20.0);

// Set pieces : 2-in-a-row (Smother Position)
pub const SET_PIECE_2_SMOTHER_HORIZONTAL: SetPiece = define_set_piece([
    'b', 'f', 'f', 'b', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 4, 1, -40.0);
pub const SET_PIECE_2_SMOTHER_VERTICAL: SetPiece = define_set_piece([
    'b', ' ', ' ', ' ', ' ',
    'f', ' ', ' ', ' ', ' ',
    'f', ' ', ' ', ' ', ' ',
    'b', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 1, 4, -40.0);
pub const SET_PIECE_2_SMOTHER_FORWARD: SetPiece = define_set_piece([
    ' ', ' ', ' ', 'b', ' ',
    ' ', ' ', 'f', ' ', ' ',
    ' ', 'f', ' ', ' ', ' ',
    'b', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 4, 4, -40.0);
pub const SET_PIECE_2_SMOTHER_BACKWARD: SetPiece = define_set_piece([
    'b', ' ', ' ', ' ', ' ',
    ' ', 'f', ' ', ' ', ' ',
    ' ', ' ', 'f', ' ', ' ',
    ' ', ' ', ' ', 'b', ' ',
    ' ', ' ', ' ', ' ', ' '
], 4, 4, -40.0);

// Set Pieces : 1-in-a-row (Edge Position)
pub const SET_PIECE_1_EDGE_HORIZONTAL_LEFT: SetPiece = define_set_piece([
    'f', 'b', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 5, 1, -20.0);
pub const SET_PIECE_1_EDGE_HORIZONTAL_RIGHT: SetPiece = define_set_piece([
    ' ', ' ', ' ', 'b', 'f',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 5, 1, -20.0);
pub const SET_PIECE_1_EDGE_VERTICAL_UP: SetPiece = define_set_piece([
    'f', ' ', ' ', ' ', ' ',
    'b', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 1, 5, -20.0);
pub const SET_PIECE_1_EDGE_VERTICAL_DOWN: SetPiece = define_set_piece([
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    'b', ' ', ' ', ' ', ' ',
    'f', ' ', ' ', ' ', ' '
], 1, 5, -20.0);
pub const SET_PIECE_1_EDGE_FORWARD_FRONT: SetPiece = define_set_piece([
    ' ', ' ', ' ', ' ', 'f',
    ' ', ' ', ' ', 'b', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 5, 5, -20.0);
pub const SET_PIECE_1_EDGE_FORWARD_BACK: SetPiece = define_set_piece([
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', 'b', ' ', ' ', ' ',
    'f', ' ', ' ', ' ', ' '
], 5, 5, -20.0);
pub const SET_PIECE_1_EDGE_BACKWARD_FRONT: SetPiece = define_set_piece([
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', 'b', ' ',
    ' ', ' ', ' ', ' ', 'f'
], 5, 5, -20.0);
pub const SET_PIECE_1_EDGE_BACKWARD_BACK: SetPiece = define_set_piece([
    'f', ' ', ' ', ' ', ' ',
    ' ', 'b', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 5, 5, -20.0);

// Set Pieces : 2-in-a-row (Edge Position)
pub const SET_PIECE_2_EDGE_HORIZONTAL_LEFT: SetPiece = define_set_piece([
    'f', 'f', 'b', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 5, 1, -40.0);
pub const SET_PIECE_2_EDGE_HORIZONTAL_RIGHT: SetPiece = define_set_piece([
    ' ', ' ', 'b', 'f', 'f',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 5, 1, -40.0);
pub const SET_PIECE_2_EDGE_VERTICAL_UP: SetPiece = define_set_piece([
    'f', ' ', ' ', ' ', ' ',
    'f', ' ', ' ', ' ', ' ',
    'b', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 1, 5, -40.0);
pub const SET_PIECE_2_EDGE_VERTICAL_DOWN: SetPiece = define_set_piece([
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    'b', ' ', ' ', ' ', ' ',
    'f', ' ', ' ', ' ', ' ',
    'f', ' ', ' ', ' ', ' '
], 1, 5, -40.0);
pub const SET_PIECE_2_EDGE_FORWARD_FRONT: SetPiece = define_set_piece([
    ' ', ' ', ' ', ' ', 'f',
    ' ', ' ', ' ', 'f', ' ',
    ' ', ' ', 'b', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 5, 5, -40.0);
pub const SET_PIECE_2_EDGE_FORWARD_BACK: SetPiece = define_set_piece([
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', 'b', ' ', ' ',
    ' ', 'f', ' ', ' ', ' ',
    'f', ' ', ' ', ' ', ' '
], 5, 5, -40.0);
pub const SET_PIECE_2_EDGE_BACKWARD_FRONT: SetPiece = define_set_piece([
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', 'b', ' ', ' ',
    ' ', ' ', ' ', 'b', ' ',
    ' ', ' ', ' ', ' ', 'f'
], 5, 5, -40.0);
pub const SET_PIECE_2_EDGE_BACKWARD_BACK: SetPiece = define_set_piece([
    'f', ' ', ' ', ' ', ' ',
    ' ', 'f', ' ', ' ', ' ',
    ' ', ' ', 'b', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 5, 5, -40.0);

// Set pieces : 3-in-a-row (Close Position)
pub const SET_PIECE_3_CLOSE_HORIZONTAL: SetPiece = define_set_piece([
    'f', 'b', 'f', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 3, 1, -20.0);
pub const SET_PIECE_3_CLOSE_VERTICAL: SetPiece = define_set_piece([
    'f', ' ', ' ', ' ', ' ',
    'b', ' ', ' ', ' ', ' ',
    'f', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 1, 3, -20.0);
pub const SET_PIECE_3_CLOSE_FORWARD: SetPiece = define_set_piece([
    ' ', ' ', 'f', ' ', ' ',
    ' ', 'b', ' ', ' ', ' ',
    'f', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 3, 3, -20.0);
pub const SET_PIECE_3_CLOSE_BACKWARD: SetPiece = define_set_piece([
    'f', ' ', ' ', ' ', ' ',
    ' ', 'b', ' ', ' ', ' ',
    ' ', ' ', 'f', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 3, 3, -20.0);

// Set pieces : Diagonal fork
pub const SET_PIECE_DIAGONAL_FORK_UP: SetPiece = define_set_piece([
    ' ', ' ', 'f', ' ', ' ',
    ' ', 'f', ' ', 'f', ' ',
    'e', ' ', ' ', ' ', 'e',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 5, 3, 100.0);
pub const SET_PIECE_DIAGONAL_FORK_DOWN: SetPiece = define_set_piece([
    'e', ' ', ' ', ' ', 'e',
    ' ', 'f', ' ', 'f', ' ',
    ' ', ' ', 'f', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 5, 3, 100.0);
pub const SET_PIECE_DIAGONAL_FORK_LEFT: SetPiece = define_set_piece([
    ' ', ' ', 'e', ' ', ' ',
    ' ', 'f', ' ', ' ', ' ',
    'f', ' ', ' ', ' ', ' ',
    ' ', 'f', ' ', ' ', ' ',
    ' ', ' ', 'e', ' ', ' '
], 3, 5, 100.0);
pub const SET_PIECE_DIAGONAL_FORK_RIGHT: SetPiece = define_set_piece([
    'e', ' ', ' ', ' ', ' ',
    ' ', 'f', ' ', ' ', ' ',
    ' ', ' ', 'f', ' ', ' ',
    ' ', 'f', ' ', ' ', ' ',
    'e', ' ', ' ', ' ', ' '
], 3, 5, 100.0);

// Set pieces : Spoon fork
pub const SET_PIECE_SPOON_FORK_UP_LEFT: SetPiece = define_set_piece([
    ' ', ' ', 'f', ' ', ' ',
    ' ', 'f', 'e', 'f', ' ',
    'e', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 4, 3, 100.0);
pub const SET_PIECE_SPOON_FORK_UP_RIGHT: SetPiece = define_set_piece([
    ' ', 'f', ' ', ' ', ' ', 
    'f', 'e', 'f', ' ', ' ', 
    ' ', ' ', ' ', 'e', ' ', 
    ' ', ' ', ' ', ' ', ' ', 
    ' ', ' ', ' ', ' ', ' '
], 4, 3, 100.0);
pub const SET_PIECE_SPOON_FORK_DOWN_LEFT: SetPiece = define_set_piece([
    'e', ' ', ' ', ' ', ' ',
    ' ', 'f', 'e', 'f', ' ',
    ' ', ' ', 'f', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 4, 3, 100.0);
pub const SET_PIECE_SPOON_FORK_DOWN_RIGHT: SetPiece = define_set_piece([
    ' ', ' ', ' ', 'e', ' ', 
    'f', 'e', 'f', ' ', ' ', 
    ' ', 'f', ' ', ' ', ' ', 
    ' ', ' ', ' ', ' ', ' ', 
    ' ', ' ', ' ', ' ', ' '
], 4, 3, 100.0);
pub const SET_PIECE_SPOON_FORK_LEFT_UP: SetPiece = define_set_piece([
    ' ', ' ', 'e', ' ', ' ',
    ' ', 'f', ' ', ' ', ' ',
    'f', 'e', ' ', ' ', ' ',
    ' ', 'f', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 3, 4, 100.0);
pub const SET_PIECE_SPOON_FORK_LEFT_DOWN: SetPiece = define_set_piece([
    ' ', 'f', ' ', ' ', ' ', 
    'f', 'e', ' ', ' ', ' ', 
    ' ', 'f', ' ', ' ', ' ', 
    ' ', ' ', 'e', ' ', ' ', 
    ' ', ' ', ' ', ' ', ' '
], 3, 4, 100.0);
pub const SET_PIECE_SPOON_FORK_RIGHT_UP: SetPiece = define_set_piece([
    'e', ' ', ' ', ' ', ' ',
    ' ', 'f', ' ', ' ', ' ',
    ' ', 'e', 'f', ' ', ' ',
    ' ', 'f', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 3, 4, 100.0);
pub const SET_PIECE_SPOON_FORK_RIGHT_DOWN: SetPiece = define_set_piece([
    ' ', 'f', ' ', ' ', ' ', 
    ' ', 'e', 'f', ' ', ' ', 
    ' ', 'f', ' ', ' ', ' ', 
    'e', ' ', ' ', ' ', ' ', 
    ' ', ' ', ' ', ' ', ' '
], 3, 4, 100.0);

// Set pieces : Straight fork
pub const SET_PIECE_STRAIGHT_FORK_UP: SetPiece = define_set_piece([
    'f', 'f', 'e', ' ', ' ',
    'f', ' ', ' ', ' ', ' ',
    'e', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 3, 3, 100.0);
pub const SET_PIECE_STRAIGHT_FORK_DOWN: SetPiece = define_set_piece([
    ' ', ' ', 'e', ' ', ' ',
    ' ', ' ', 'f', ' ', ' ',
    'e', 'f', 'f', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 3, 3, 100.0);
pub const SET_PIECE_STRAIGHT_FORK_LEFT: SetPiece = define_set_piece([
    'e', ' ', ' ', ' ', ' ',
    'f', ' ', ' ', ' ', ' ',
    'f', 'f', 'e', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 3, 3, 100.0);
pub const SET_PIECE_STRAIGHT_FORK_RIGHT: SetPiece = define_set_piece([
    'e', 'f', 'f', ' ', ' ',
    ' ', ' ', 'f', ' ', ' ',
    ' ', ' ', 'e', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 3, 3, 100.0);

// Set pieces : Skewer fork
pub const SET_PIECE_SKEWER_FORK_S_UP: SetPiece = define_set_piece([
    ' ', 'f', 'f', ' ', ' ',
    ' ', 'f', ' ', ' ', ' ',
    'e', 'e', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 3, 3, 100.0);
pub const SET_PIECE_SKEWER_FORK_Z_UP: SetPiece = define_set_piece([
    'f', 'f', ' ', ' ', ' ',
    ' ', 'f', ' ', ' ', ' ',
    ' ', 'e', 'e', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 3, 3, 100.0);
pub const SET_PIECE_SKEWER_FORK_S_DOWN: SetPiece = define_set_piece([
    ' ', 'e', 'e', ' ', ' ',
    ' ', 'f', ' ', ' ', ' ',
    'f', 'f', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 3, 3, 100.0);
pub const SET_PIECE_SKEWER_FORK_Z_DOWN: SetPiece = define_set_piece([
    'e', 'e', ' ', ' ', ' ',
    ' ', 'f', ' ', ' ', ' ',
    ' ', 'f', 'f', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 3, 3, 100.0);
pub const SET_PIECE_SKEWER_FORK_S_LEFT: SetPiece = define_set_piece([
    'f', ' ', ' ', ' ', ' ',
    'f', 'f', 'e', ' ', ' ',
    ' ', ' ', 'e', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 3, 3, 100.0);
pub const SET_PIECE_SKEWER_FORK_Z_LEFT: SetPiece = define_set_piece([
    ' ', ' ', 'e', ' ', ' ',
    'f', 'f', 'e', ' ', ' ',
    'f', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 3, 3, 100.0);
pub const SET_PIECE_SKEWER_FORK_S_RIGHT: SetPiece = define_set_piece([
    'e', ' ', ' ', ' ', ' ',
    'e', 'f', 'f', ' ', ' ',
    ' ', ' ', 'f', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 3, 3, 100.0);
pub const SET_PIECE_SKEWER_FORK_Z_RIGHT: SetPiece = define_set_piece([
    ' ', ' ', 'f', ' ', ' ',
    'e', 'f', 'f', ' ', ' ',
    'e', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 3, 3, 100.0);

// Set pieces : Tee fork
pub const SET_PIECE_TEE_FORK_UP_LEFT: SetPiece = define_set_piece([
    'f', 'f', 'e', ' ', ' ',
    ' ', 'f', ' ', ' ', ' ',
    ' ', 'e', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 3, 3, 100.0);
pub const SET_PIECE_TEE_FORK_UP_RIGHT: SetPiece = define_set_piece([
    'e', 'f', 'f', ' ', ' ',
    ' ', 'f', ' ', ' ', ' ',
    ' ', 'e', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 3, 3, 100.0);
pub const SET_PIECE_TEE_FORK_DOWN_LEFT: SetPiece = define_set_piece([
    ' ', 'e', ' ', ' ', ' ',
    ' ', 'f', ' ', ' ', ' ',
    'f', 'f', 'e', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 3, 3, 100.0);
pub const SET_PIECE_TEE_FORK_DOWN_RIGHT: SetPiece = define_set_piece([
    ' ', 'e', ' ', ' ', ' ',
    ' ', 'f', ' ', ' ', ' ',
    'e', 'f', 'f', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 3, 3, 100.0);
pub const SET_PIECE_TEE_FORK_LEFT_UP: SetPiece = define_set_piece([
    'f', ' ', ' ', ' ', ' ',
    'f', 'f', 'e', ' ', ' ',
    'e', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 3, 3, 100.0);
pub const SET_PIECE_TEE_FORK_LEFT_DOWN: SetPiece = define_set_piece([
    'e', ' ', ' ', ' ', ' ',
    'f', 'f', 'e', ' ', ' ',
    'f', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 3, 3, 100.0);
pub const SET_PIECE_TEE_FORK_RIGHT_UP: SetPiece = define_set_piece([
    ' ', ' ', 'f', ' ', ' ',
    'e', 'f', 'f', ' ', ' ',
    ' ', ' ', 'e', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 3, 3, 100.0);
pub const SET_PIECE_TEE_FORK_RIGHT_DOWN: SetPiece = define_set_piece([
    ' ', ' ', 'e', ' ', ' ',
    'e', 'f', 'f', ' ', ' ',
    ' ', ' ', 'f', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' '
], 3, 3, 100.0);
