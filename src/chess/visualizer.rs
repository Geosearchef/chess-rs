use std::fmt::format;
use std::time::Duration;
use egui::epaint::RectShape;
use egui::{pos2, vec2, Color32, ColorImage, Frame, PointerButton, Pos2, Rect, Shape, SizeHint, StrokeKind, TextureOptions};
use egui::load::TexturePoll;
use crate::chess::board::{Board, Color, Piece, PieceType};
use crate::chess::vector::Vector;

const LIGHT_SQUARE_COLOR: Color32 = Color32::from_rgb(240, 217, 181);
const DARK_SQUARE_COLOR: Color32 = Color32::from_rgb(181, 136, 99);
const SQUARE_SIZE: f32 = 50.0;
const RECT_UV_ALL: Rect = Rect { min: pos2(0.0, 0.0), max: pos2(1.0, 1.0) };

const PIECE_IMAGES_PATH: &str = "file://./assets/pieces/";

pub struct ChessVisualizer {
    board: Board,
    move_index: u64,
    selected_square: Option<Vector>,
}

impl Default for ChessVisualizer {
    fn default() -> Self {
        Self {
            board: Board::default(),
            move_index: 0,
            selected_square: None,
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

                let (response, painter) = ui.allocate_painter(ui.available_size_before_wrap(), egui::Sense::hover());


                for coord in self.board.coords() {
                    let Vector(x, y) = coord;

                    let min = Self::to_screen_space(coord);
                    let max = min + vec2(SQUARE_SIZE, SQUARE_SIZE);
                    let rect = Rect { min, max };

                    let color = if (x + y) % 2 == 0 { LIGHT_SQUARE_COLOR } else { DARK_SQUARE_COLOR };

                    // Paint square
                    painter.add(Shape::Rect(RectShape {
                        rect,
                        corner_radius: Default::default(),
                        fill: color,
                        stroke: Default::default(),
                        stroke_kind: StrokeKind::Middle,
                        round_to_pixels: None,
                        blur_width: 0.0,
                        brush: None,
                    }));

                    // Paint piece
                    if let Some(piece) = self.board.piece_at(coord) {
                        // try_load_texture does the caching for us (unlike load_texture)
                        let texture = ctx.try_load_texture(Self::piece_to_image_uri(piece).as_str(), TextureOptions::LINEAR, SizeHint::Size(SQUARE_SIZE as u32, SQUARE_SIZE as u32)).expect("loading texture for piece");

                        if let TexturePoll::Ready { texture } = texture { // TODO: there needs to be a better way to do caching in immediate mode
                            painter.image(texture.id, rect, RECT_UV_ALL, Color32::WHITE);
                        } else {
                            ctx.request_repaint();
                        }
                    }
                }



                if response.clicked_by(PointerButton::Primary) {
                    self.board_left_clicked(response.interact_pointer_pos().expect("was just clicked"));
                    ctx.request_repaint();
                }
        });
    }
}

impl ChessVisualizer {
    fn board_left_clicked(&mut self, pos: Pos2) {
        let square = Self::to_board_space(pos);

        if let Some(selected_square) = self.selected_square {
            self.selected_square = None;
        } else {
            self.selected_square = Some(square);
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
        pos2(square.0 as f32 * SQUARE_SIZE, square.1 as f32 * SQUARE_SIZE)
    }

    fn to_board_space(pos: Pos2) -> Vector {
        Vector((pos.x / SQUARE_SIZE).floor() as i8, (pos.y / SQUARE_SIZE).floor() as i8)
    }
}




// - checkerboard
// - pieces
// - selected
// - last move
// - possible moves
