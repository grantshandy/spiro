mod app;

use app::Spiro;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    eframe::run_native(
        "Sprio",
        eframe::NativeOptions::default(),
        Box::new(|cc| Box::new(Spiro::new(cc))),
    )
    .unwrap();
}

#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "application",
                eframe::WebOptions::default(),
                Box::new(|cc| Box::new(Spiro::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
}
