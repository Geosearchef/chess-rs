use std::io::Write;
use crate::chess::board::{Board, Piece, PieceType};
use crate::chess::r#move::Move;
use crate::chess::vector::Vector;
use egui::load::TexturePoll;
use egui::{pos2, vec2, Color32, Frame, PointerButton, Pos2, Rect, Shape, StrokeKind, TextureOptions, Vec2};

const LIGHT_SQUARE_COLOR: Color32 = Color32::from_rgb(240, 217, 181);
const DARK_SQUARE_COLOR: Color32 = Color32::from_rgb(181, 136, 99);
const SELECTION_COLOR: Color32 = Color32::from_rgb(118, 150, 72);
const POSSIBLE_MOVE_COLOR: Color32 = Color32::from_rgb(209, 176, 56);
const LAST_MOVE_COLOR: Color32 = Color32::from_rgb(90, 90, 90);

const SQUARE_SIZE: Vec2 = vec2(50.0, 50.0);
const RECT_UV_ALL: Rect = Rect { min: pos2(0.0, 0.0), max: pos2(1.0, 1.0) };

const PIECE_IMAGES_PATH: &str = "file://./assets/pieces/";

pub struct ChessVisualizer {
    board: Board,
    move_index: u64,
    selected_square: Option<Vector>,
    possible_moves: Vec<Move>,
}

impl Default for ChessVisualizer {
    fn default() -> Self {
        Self {
            board: Board::default(),
            move_index: 0,
            selected_square: None,
            possible_moves: vec![],
        }
    }
}

impl eframe::App for ChessVisualizer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(Frame::NONE)
            .show(ctx, |ui| {

                // TODO: clone -> cache?
                // let piece_textures = self.piece_images.clone().map(|img| ctx.load_texture("texture", img, TextureOptions::LINEAR));

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

                // Paint selection
                if let Some(selected_square) = self.selected_square {
                    let pos = Self::to_screen_space(selected_square);
                    painter.add(Shape::rect_stroke(Rect::from_min_size(pos, SQUARE_SIZE), 0.0, (3.0, SELECTION_COLOR), StrokeKind::Inside));
                }

                // Paint possible moves
                for Move { dst, .. } in self.possible_moves.iter() {
                    let pos = Self::to_screen_space(*dst);
                    painter.add(Shape::rect_stroke(Rect::from_min_size(pos, SQUARE_SIZE), 0.0, (3.0, POSSIBLE_MOVE_COLOR), StrokeKind::Inside));
                }


                // Handle input

                if response.clicked_by(PointerButton::Primary) {
                    self.board_left_clicked(response.interact_pointer_pos().expect("was just clicked"));
                    ctx.request_repaint();
                }
        });
    }
}

impl ChessVisualizer {
    fn board_left_clicked(&mut self, pos: Pos2) {
        let clicked_square = Self::to_board_space(pos);

        if let Some(selected_square) = self.selected_square {
            if let Some(r#move) = self.possible_moves.iter().filter(|m| m.dst == clicked_square).last() {
                self.board.execute_move(r#move.clone());

                println!("Evaluation: {:.2}", self.board.evaluate_position());
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




// - checkerboard
// - pieces
// - selected
// - last move
// - possible moves
