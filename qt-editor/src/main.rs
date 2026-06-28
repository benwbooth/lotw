mod room_canvas;

use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QString, QUrl};

extern "C" {
    // Defined in src/icon_shim.cpp (built by build.rs).
    fn lotw_set_window_icon_rgba(data: *const u8, w: i32, h: i32);
}

/// Render the front-facing Pochi sprite (Drasle family pet, character index 4,
/// CHR bank 60, standing pose 13) from the ROM into an upscaled RGBA icon. Done
/// at runtime so no ROM-derived art lives in the repo. Returns (pixels, size).
fn pochi_icon() -> Option<(Vec<u8>, i32)> {
    let env_path = std::env::var("LOTW_ROM").unwrap_or_default();
    let rom = [env_path.as_str(), "rom/lotw.nes", "../rom/lotw.nes"]
        .iter()
        .filter(|p| !p.is_empty())
        .find_map(|p| std::fs::read(p).ok())?;
    if rom.len() < 16 {
        return None;
    }
    let prg_len = rom[4] as usize * 16_384;
    let prg = &rom[16..16 + prg_len];
    let chr = &rom[16 + prg_len..];
    // Family palette for character 4 ($FFC5 + 4*4 -> PRG 0x1FFD5), colours 1-3.
    let fp = prg.get(0x1FFD5..0x1FFD5 + 4)?;
    let pal = [
        (0u8, 0u8, 0u8),
        lotw::render::nes_rgb(fp[1]),
        lotw::render::nes_rgb(fp[2]),
        lotw::render::nes_rgb(fp[3]),
    ];
    let base = 60 * 64 + 13 * 4; // bank 60, pose 13 -> first of 4 metasprite tiles
    let mut img = vec![0u8; 16 * 16 * 4]; // transparent background
    for (i, &(cx, cy)) in [(0usize, 0usize), (0, 8), (8, 0), (8, 8)].iter().enumerate() {
        let off = (base + i) * 16;
        for y in 0..8 {
            let p0 = chr.get(off + y).copied().unwrap_or(0);
            let p1 = chr.get(off + y + 8).copied().unwrap_or(0);
            for x in 0..8 {
                let v = ((p0 >> (7 - x)) & 1) | (((p1 >> (7 - x)) & 1) << 1);
                if v == 0 {
                    continue; // transparent
                }
                let (r, g, b) = pal[v as usize];
                let o = ((cy + y) * 16 + cx + x) * 4;
                img[o] = r;
                img[o + 1] = g;
                img[o + 2] = b;
                img[o + 3] = 255;
            }
        }
    }
    // Nearest-neighbour upscale x8 -> 128x128 for a crisp icon.
    let s = 8usize;
    let w = 16 * s;
    let mut big = vec![0u8; w * w * 4];
    for y in 0..w {
        for x in 0..w {
            let so = ((y / s) * 16 + x / s) * 4;
            let dst = (y * w + x) * 4;
            big[dst..dst + 4].copy_from_slice(&img[so..so + 4]);
        }
    }
    Some((big, w as i32))
}

fn main() {
    // Crisp text on fractionally-scaled (KDE/Wayland) displays: render at the
    // exact fractional scale rather than a rounded one (rounding then letting
    // the compositor resample is what makes fonts soft/misaligned, and worse as
    // the window grows), and use the native font rasteriser instead of Qt
    // Quick's distance-field text (which interpolates from a cached texture).
    std::env::set_var("QT_SCALE_FACTOR_ROUNDING_POLICY", "PassThrough");
    std::env::set_var("QML_DISABLE_DISTANCEFIELD", "1");

    let mut app = QGuiApplication::new();

    // Stable identity so QSettings (window geometry) has a fixed location, and so
    // the Wayland compositor (KDE) can associate/remember the window.
    if let Some(mut app) = app.as_mut() {
        app.as_mut().set_organization_name(&QString::from("lotw"));
        app.as_mut().set_application_name(&QString::from("lotw-editor"));
    }
    QGuiApplication::set_desktop_file_name(&QString::from("lotw-editor"));

    // Set the window/app icon to Pochi (best-effort; ignored if the ROM is absent).
    if let Some((rgba, size)) = pochi_icon() {
        unsafe { lotw_set_window_icon_rgba(rgba.as_ptr(), size, size) };
    }

    let mut engine = QQmlApplicationEngine::new();

    if let Some(engine) = engine.as_mut() {
        let qml = format!("file://{}/qml/main.qml", env!("CARGO_MANIFEST_DIR"));
        engine.load(&QUrl::from(&QString::from(&qml)));
    }

    if let Some(app) = app.as_mut() {
        app.exec();
    }
}
