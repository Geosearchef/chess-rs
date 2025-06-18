#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::io::Write;
use crate::chess::{board::{Board, Color}, visualizer::ChessVisualizer};
use egui::{Style, Visuals};
use eyre::{eyre, Context};
use eyre::Result;

mod chess;

fn main() -> Result<()> {
    // let mut board = Board::default();

    // for i in 0..10 {
    //     let moves = board.generate_moves(if i % 2 == 0 { Color::White } else { Color::Black });
    //     println!("{} moves found", moves.len());

    //     board.execute_move(*moves.choose(&mut rand::rng()).unwrap());


    //     println!("{}\n\n", board);
    // }

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 800.0]),
        centered: true,
        ..Default::default()
    };

    eframe::run_native(
        "Chess-RS",
        options,
        Box::new(|cc| {
            let style = Style {
                visuals: Visuals::light(),
                ..Style::default()
            };
            cc.egui_ctx.set_style(style);
            cc.egui_ctx.set_zoom_factor(2.0);

            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<ChessVisualizer>::default())
        }),
    ).map_err(|e| eyre!("{:?}", e))
}
