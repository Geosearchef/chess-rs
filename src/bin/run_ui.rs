#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use chess::visualizer::ChessVisualizer;
use egui::{Style, Visuals};
use eyre::Result;
use eyre::{eyre, Context};
use std::io::Write;
use eframe::AppCreator;

#[path = "../chess/mod.rs"]
mod chess;

const NATIVE_SEARCH_DEPTH: u8 = 6;
const WEB_SEARCH_DEPTH: u8 = 5;

#[cfg(not(target_arch = "wasm32"))]
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
        app_creator(NATIVE_SEARCH_DEPTH),
    ).map_err(|e| eyre!("{:?}", e))
}

#[cfg(target_arch = "wasm32")]
fn main() -> Result<()> {
    use wasm_bindgen::JsCast;
    use web_sys::HtmlCanvasElement;

    let options = eframe::WebOptions {

        ..Default::default()
    };

    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window().expect("no window").document().expect("no document");
        let canvas = document.get_element_by_id("main-canvas").expect("canvas not found").dyn_into::<HtmlCanvasElement>().expect("the canvas is not a canvas");
        eframe::WebRunner::new().start(canvas, options, app_creator(WEB_SEARCH_DEPTH)).await.expect("couldn't start webapp");
    });

    Ok(())
}

fn app_creator(search_depth: u8) -> AppCreator<'static> {
    Box::new(move |cc| {
        let style = Style {
            visuals: Visuals::light(),
            ..Style::default()
        };
        cc.egui_ctx.set_style(style);
        cc.egui_ctx.set_zoom_factor(2.0);

        egui_extras::install_image_loaders(&cc.egui_ctx);

        Ok(Box::new(ChessVisualizer::with_search_depth(search_depth))) // TODO: expose auto move and side as CLI params
    })
}