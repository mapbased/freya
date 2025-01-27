#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

static SANSITA_SWASHED: &[u8] = include_bytes!("./SansitaSwashed-Regular.ttf");

fn main() {
    launch_cfg(
        app,
        LaunchConfig::<()>::builder()
            .with_width(200.0)
            .with_height(200.0)
            .with_font("Sansita Swashed", SANSITA_SWASHED)
            .build(),
    );
}

fn app(cx: Scope) -> Element {
    render!(
        rect {
            display: "center",
            height: "100%",
            width: "100%",
            label {
                width: "100%",
                font_size: "20",
                font_family: "Sansita Swashed",
                align: "center",
                "This font is called Sansita Swashed"
            }
        }
    )
}
