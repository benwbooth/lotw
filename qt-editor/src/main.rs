mod room_canvas;

use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QString, QUrl};
use std::io::Write;
use std::path::PathBuf;

/// Render the front-facing home Pochi sprite (Drasle family pet) from the ROM
/// into an upscaled RGBA icon: CHR bank 54 (the home/title family bank),
/// metasprite 1, drawn in the home room's sprite sub-palette 0. Done at runtime
/// so no ROM-derived art lives in the repo. Returns (pixels, size).
fn pochi_icon() -> Option<(Vec<u8>, u32)> {
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
    // Home room (mapy 16) sprite sub-palette 0 — the palette the home family
    // is drawn with. Palette is the last 32 bytes of the room's meta page.
    let pal_off = lotw::render::room_offset(0, 16) + 768 + 0xE0;
    let hp = prg.get(pal_off..pal_off + 32)?;
    let pal = [
        (0u8, 0u8, 0u8),
        lotw::render::nes_rgb(hp[17]),
        lotw::render::nes_rgb(hp[18]),
        lotw::render::nes_rgb(hp[19]),
    ];
    let base = 54 * 64 + 1 * 4; // bank 54, metasprite 1 -> 4 consecutive tiles
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
    // Nearest-neighbour upscale x16 -> 256x256 for a crisp icon.
    let s = 16usize;
    let w = 16 * s;
    let mut big = vec![0u8; w * w * 4];
    for y in 0..w {
        for x in 0..w {
            let so = ((y / s) * 16 + x / s) * 4;
            let dst = (y * w + x) * 4;
            big[dst..dst + 4].copy_from_slice(&img[so..so + 4]);
        }
    }
    Some((big, w as u32))
}

/// `$XDG_DATA_HOME` (or `~/.local/share`).
fn data_home() -> Option<PathBuf> {
    if let Some(d) = std::env::var_os("XDG_DATA_HOME").filter(|d| !d.is_empty()) {
        return Some(PathBuf::from(d));
    }
    std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".local/share"))
}

/// Write the Pochi icon as a PNG and install a `lotw-editor.desktop` entry that
/// points at it. On Wayland the window has no client-set icon; the compositor
/// (KDE) instead matches the window's app id (set via `setDesktopFileName`) to
/// this desktop file and uses its `Icon=`. NoDisplay keeps it out of menus.
fn install_icon_desktop() -> Option<()> {
    let (rgba, size) = pochi_icon()?;
    let base = data_home()?;
    let icon_path = base.join("icons/hicolor/256x256/apps/lotw-editor.png");
    std::fs::create_dir_all(icon_path.parent()?).ok()?;
    {
        let file = std::fs::File::create(&icon_path).ok()?;
        let mut enc = png::Encoder::new(std::io::BufWriter::new(file), size, size);
        enc.set_color(png::ColorType::Rgba);
        enc.set_depth(png::BitDepth::Eight);
        enc.write_header().ok()?.write_image_data(&rgba).ok()?;
    }
    let apps = base.join("applications");
    std::fs::create_dir_all(&apps).ok()?;
    let exec = std::env::current_exe().ok().map(|p| p.display().to_string()).unwrap_or_else(|| "qt-editor".into());
    let desktop = format!(
        "[Desktop Entry]\n\
         Type=Application\n\
         Name=LotW Asset Editor\n\
         Exec={exec}\n\
         Icon={icon}\n\
         StartupWMClass=lotw-editor\n\
         NoDisplay=true\n\
         Categories=Development;\n",
        icon = icon_path.display(),
    );
    std::fs::File::create(apps.join("lotw-editor.desktop")).ok()?.write_all(desktop.as_bytes()).ok()?;
    Some(())
}

fn main() {
    // Round the display scale factor to a whole number so the device-pixel ratio
    // is an integer. A fractional ratio makes QQuickPaintedItem render the sprite
    // tiles into a fractionally-scaled backing store, so nearest-neighbour rounds
    // some source pixels to 2 device pixels and others to 1 — identical tiles end
    // up looking different. With an integer ratio every source pixel maps to a
    // whole number of device pixels. Native (non-distance-field) text keeps fonts
    // crisp under this policy.
    // Safe: this runs at the very top of main(), before QGuiApplication spawns
    // any threads, so the environment has no concurrent reader.
    unsafe {
        std::env::set_var("QT_SCALE_FACTOR_ROUNDING_POLICY", "Round");
        std::env::set_var("QML_DISABLE_DISTANCEFIELD", "1");
    }

    // Install the Pochi icon + desktop entry before the window appears, so the
    // compositor can pick it up via the app id (best-effort).
    install_icon_desktop();

    let mut app = QGuiApplication::new();

    // Stable identity so QSettings (window geometry) has a fixed location, and so
    // the Wayland compositor (KDE) associates the window with lotw-editor.desktop.
    if let Some(mut app) = app.as_mut() {
        app.as_mut().set_organization_name(&QString::from("lotw"));
        app.as_mut().set_application_name(&QString::from("lotw-editor"));
    }
    QGuiApplication::set_desktop_file_name(&QString::from("lotw-editor"));

    let mut engine = QQmlApplicationEngine::new();

    if let Some(engine) = engine.as_mut() {
        let qml = format!("file://{}/qml/main.qml", env!("CARGO_MANIFEST_DIR"));
        engine.load(&QUrl::from(&QString::from(&qml)));
    }

    if let Some(app) = app.as_mut() {
        app.exec();
    }
}
