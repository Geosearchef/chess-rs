use std::io::Write;
use std::time::Instant;
use crate::chess::board::{Board, Piece, PieceType};
use crate::chess::r#move::Move;
use crate::chess::vector::Vector;
use egui::load::TexturePoll;
use egui::{pos2, vec2, Color32, Frame, Key, PointerButton, Pos2, Rect, Shape, StrokeKind, TextureOptions, Vec2};
use crate::chess::negamax::{negamax_move, OptimizationContext};
use crate::chess::zobrist::ZobristTable;

const LIGHT_SQUARE_COLOR: Color32 = Color32::from_rgb(240, 217, 181);
const DARK_SQUARE_COLOR: Color32 = Color32::from_rgb(181, 136, 99);
const SELECTION_COLOR: Color32 = Color32::from_rgb(50, 150, 168);
const POSSIBLE_MOVE_COLOR: Color32 = Color32::from_rgb(209, 176, 56);
const LAST_MOVE_COLOR: Color32 = Color32::from_rgb(90, 90, 90);
const SUGGESTED_MOVE_COLOR: Color32 = Color32::from_rgb(118, 150, 72);

const SQUARE_SIZE: Vec2 = vec2(50.0, 50.0);
const RECT_UV_ALL: Rect = Rect { min: pos2(0.0, 0.0), max: pos2(1.0, 1.0) };

const PIECE_IMAGES_PATH: &str = "file://./assets/pieces/";

pub struct ChessVisualizer {
    auto_move_enabled: bool,
    board: Board,
    move_index: u64,
    selected_square: Option<Vector>,
    possible_moves: Vec<Move>,
    suggested_move: Option<Move>,
    auto_move: DoubleTrigger,
    zobrist_table: ZobristTable
}

impl Default for ChessVisualizer {
    fn default() -> Self {
        Self {
            auto_move_enabled: true,
            board: Board::default(),
            move_index: 0,
            selected_square: None,
            possible_moves: vec![],
            suggested_move: None,
            auto_move: DoubleTrigger::default(),
            zobrist_table: ZobristTable::default(),
        }
    }
}

impl eframe::App for ChessVisualizer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(Frame::NONE)
            .show(ctx, |ui| {

                // ui.heading("Hello World");
                // ui.label("hi there");
                ui.style_mut().spacing.item_spacing = vec2(0.0, 0.0);
                ui.style_mut().spacing.indent = 0.0;

                let (response, painter) = ui.allocate_painter(ui.available_size_before_wrap(), egui::Sense::hover() | egui::Sense::click_and_drag());


                // Paint pieces
                for coord in self.board.coords() {
                    let Vector(x, y) = coord;

                    let min = Self::to_screen_space(coord);
                    let rect = Rect::from_min_size(min, SQUARE_SIZE);

                    let color = if (x + y) % 2 == 0 { LIGHT_SQUARE_COLOR } else { DARK_SQUARE_COLOR };

                    // Paint square
                    painter.add(Shape::rect_filled(rect, 0, color));

                    // Paint piece
                    if let Some(piece) = self.board.piece_at(coord) {
                        // try_load_texture does the caching for us (unlike load_texture)
                        let texture = ctx.try_load_texture(Self::piece_to_image_uri(piece).as_str(), TextureOptions::LINEAR, SQUARE_SIZE.into()).expect("loading texture for piece");

                        if let TexturePoll::Ready { texture } = texture { // TODO: there needs to be a better way to do caching in immediate mode
                            painter.image(texture.id, rect, RECT_UV_ALL, Color32::WHITE);
                        } else {
                            ctx.request_repaint();
                        }
                    }
                }

                // Paint last move
                if let Some(Move { src, dst, .. }) = self.board.last_move {
                    let src_pos = Self::to_screen_space(src);
                    let dst_pos = Self::to_screen_space(dst);
                    painter.add(Shape::rect_stroke(Rect::from_min_size(src_pos, SQUARE_SIZE), 0.0, (3.0, LAST_MOVE_COLOR), StrokeKind::Inside));
                    painter.add(Shape::rect_stroke(Rect::from_min_size(dst_pos, SQUARE_SIZE), 0.0, (3.0, LAST_MOVE_COLOR), StrokeKind::Inside));
                }

                // Paint possible moves
                for Move { dst, .. } in self.possible_moves.iter() {
                    let pos = Self::to_screen_space(*dst);
                    painter.add(Shape::rect_stroke(Rect::from_min_size(pos, SQUARE_SIZE), 0.0, (3.0, POSSIBLE_MOVE_COLOR), StrokeKind::Inside));
                }

                // Paint suggestion
                if let Some(Move { src, dst, .. }) = self.suggested_move {
                    let src_pos = Self::to_screen_space(src);
                    let dst_pos = Self::to_screen_space(dst);
                    painter.add(Shape::rect_stroke(Rect::from_min_size(src_pos, SQUARE_SIZE), 0.0, (3.0, SUGGESTED_MOVE_COLOR), StrokeKind::Inside));
                    painter.add(Shape::rect_stroke(Rect::from_min_size(dst_pos, SQUARE_SIZE), 0.0, (3.0, SUGGESTED_MOVE_COLOR), StrokeKind::Inside));
                }

                // Paint selection
                if let Some(selected_square) = self.selected_square {
                    let pos = Self::to_screen_space(selected_square);
                    painter.add(Shape::rect_stroke(Rect::from_min_size(pos, SQUARE_SIZE), 0.0, (3.0, SELECTION_COLOR), StrokeKind::Inside));
                }


                // Auto move
                if self.auto_move.enabled() {
                    if self.auto_move.double_and_check() {
                        self.compute_suggestion();
                        self.execute_suggested_move();
                    }

                    ctx.request_repaint();
                }


                // Handle input
                if response.clicked_by(PointerButton::Primary) {
                    self.board_left_clicked(response.interact_pointer_pos().expect("was just clicked"));
                    ctx.request_repaint();
                }

                if ctx.input(|i| i.key_pressed(Key::Space)) {
                    self.compute_suggestion();
                }
                if ctx.input(|i| i.key_pressed(Key::Enter)) {
                    self.execute_suggested_move();
                }
        });
    }
}

impl ChessVisualizer {
    fn board_left_clicked(&mut self, pos: Pos2) {
        let clicked_square = Self::to_board_space(pos);

        if let Some(selected_square) = self.selected_square {
            if let Some(r#move) = self.possible_moves.iter().filter(|m| m.dst == clicked_square).last() {
                self.board.execute_move(r#move.clone(), &self.zobrist_table);

                // println!("Evaluation: {:.2}, Zobrist Hash: {}", self.board.evaluate_position(), self.board.zobrist_hash);

                self.suggested_move = None;

                if self.auto_move_enabled {
                    self.auto_move.initiate();
                }
            }


            self.selected_square = None;
            self.possible_moves = vec![];
        } else {
            self.selected_square = Some(clicked_square);

            if let Some(piece) = self.board.piece_at(clicked_square) {
                if piece.color() == self.board.next_player {
                    self.possible_moves = self.board.generate_piece_moves(clicked_square);
                }
            }
        }
    }

    fn compute_suggestion(&mut self) {
        let start = Instant::now();
        if let Some((suggested_move, score)) = negamax_move(self.board.clone(), 5, &self.zobrist_table) {
            self.suggested_move = Some(suggested_move);
            println!("Suggested move score: {:.2}, took {:.1} ms\n", score, start.elapsed().as_millis());
        } else {
            self.suggested_move = None;
            println!("No possible move found\n");
        }
    }

    fn execute_suggested_move(&mut self) {
        if let Some(suggested_move) = self.suggested_move {
            self.board.execute_move(suggested_move, &self.zobrist_table); // why can I pass ownership of the move if &mut self is used below?
            self.suggested_move = None;
        } else {
            println!("No move being suggested");
        }
    }
}

impl ChessVisualizer {
    // TODO: support WASM (load from http instead of file, enable feature on egui_extras)
    // TODO: could support svg
    fn piece_to_image_uri(piece: &Piece) -> String {
        let piece_code = match piece.piece_type() {
            PieceType::Pawn => "p",
            PieceType::Knight => "n",
            PieceType::Bishop => "b",
            PieceType::Rook => "r",
            PieceType::Queen => "q",
            PieceType::King => "k",
        };

        let color_code = if piece.is_white() { "l" } else { "d" };

        format!("{}{}{}t.png", PIECE_IMAGES_PATH, piece_code, color_code)
    }

    fn to_screen_space(square: Vector) -> Pos2 {
        pos2(square.0 as f32 * SQUARE_SIZE.x, square.1 as f32 * SQUARE_SIZE.y)
    }

    fn to_board_space(pos: Pos2) -> Vector {
        Vector((pos.x / SQUARE_SIZE.x).floor() as i8, (pos.y / SQUARE_SIZE.y).floor() as i8)
    }
}


/// Double trigger to delay actions until after two renders
///
/// To return true on `double_and_check`, the functions initaite->double_and_check->double_and_check need to be called.
#[derive(Default)]
struct DoubleTrigger {
    count: u8
}
impl DoubleTrigger {
    fn initiate(&mut self) {
        self.count = 1;
    }
    fn double_and_check(&mut self) -> bool {
        if self.count == 1 || self.count == 2 {
            self.count += 1;
            false
        } else if self.count == 3 {
            self.count = 0;
            true
        } else {
            false
        }
    }
    fn enabled(&self) -> bool {
        self.count > 0
    }
}