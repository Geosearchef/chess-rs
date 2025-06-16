use std::time::Duration;
use egui::epaint::RectShape;
use egui::{pos2, vec2, Color32, ColorImage, Frame, Pos2, Rect, Shape, StrokeKind, TextureOptions};
use crate::chess::board::{Board, Color, Piece, PieceType};
use crate::chess::vector::Vector;

const LIGHT_SQUARE_COLOR: Color32 = Color32::from_rgb(240, 217, 181);
const DARK_SQUARE_COLOR: Color32 = Color32::from_rgb(181, 136, 99);
const SQUARE_SIZE: f32 = 50.0;
const RECT_UV_ALL: Rect = Rect { min: pos2(0.0, 0.0), max: pos2(1.0, 1.0) };

const PIECE_IMAGE_BYTES: [&[u8]; 12] = [ // TODO: better way to include this in the binary?
    include_bytes!("../../assets/pieces/plt.png"),
    include_bytes!("../../assets/pieces/nlt.png"),
    include_bytes!("../../assets/pieces/blt.png"),
    include_bytes!("../../assets/pieces/rlt.png"),
    include_bytes!("../../assets/pieces/qlt.png"),
    include_bytes!("../../assets/pieces/klt.png"),
    include_bytes!("../../assets/pieces/pdt.png"),
    include_bytes!("../../assets/pieces/ndt.png"),
    include_bytes!("../../assets/pieces/bdt.png"),
    include_bytes!("../../assets/pieces/rdt.png"),
    include_bytes!("../../assets/pieces/qdt.png"),
    include_bytes!("../../assets/pieces/kdt.png"),
];

pub struct ChessVisualizer {
    board: Board,
    piece_images: [ColorImage; 12],
    move_index: u64
}

impl Default for ChessVisualizer {
    fn default() -> Self {
        Self {
            board: Board::default(),
            piece_images: PIECE_IMAGE_BYTES.map(|data| egui_extras::image::load_image_bytes(data).expect("couldn't load image")),
            move_index: 0,
        }
    }
}

impl eframe::App for ChessVisualizer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(Frame::NONE)
            .show(ctx, |ui| {

                // TODO: clone -> cache?
                let piece_textures = self.piece_images.clone().map(|img| ctx.load_texture("asd", img, TextureOptions::LINEAR));

                // ui.heading("Hello World");
                // ui.label("hi there");
                ui.style_mut().spacing.item_spacing = vec2(0.0, 0.0);
                ui.style_mut().spacing.indent = 0.0;

                let (response, painter) = ui.allocate_painter(ui.available_size_before_wrap(), egui::Sense::hover());


                for coord in self.board.coords() {
                    let Vector(x, y) = coord;

                    let min = pos2(SQUARE_SIZE * x as f32, SQUARE_SIZE * y as f32);
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
                        painter.image(piece_textures[piece_to_image_index(piece)].id(), rect, RECT_UV_ALL, Color32::WHITE);
                    }
                }
        });
    }
}

fn piece_to_image_index(piece: &Piece) -> usize {
    match piece.color() {
        Color::White => {
            match piece.piece_type() {
                PieceType::Pawn => 0,
                PieceType::Knight => 1,
                PieceType::Bishop => 2,
                PieceType::Rook => 3,
                PieceType::Queen => 4,
                PieceType::King => 5,
            }
        }
        Color::Black => {
            match piece.piece_type() {
                PieceType::Pawn => 6,
                PieceType::Knight => 7,
                PieceType::Bishop => 8,
                PieceType::Rook => 9,
                PieceType::Queen => 10,
                PieceType::King => 11,
            }
        }
    }
}


// - checkerboard
// - pieces
// - last move
// - possible moves