use core::pin::Pin;
use cxx_qt::CxxQtType;
use cxx_qt_lib::{QImage, QImageFormat, QRect, QString};

#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "C++" {
        include!(<QtQuick/QQuickPaintedItem>);
        type QQuickPaintedItem;
        include!("cxx-qt-lib/qpainter.h");
        type QPainter = cxx_qt_lib::QPainter;
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    unsafe extern "RustQt" {
        #[qml_element]
        #[base = QQuickPaintedItem]
        #[qobject]
        #[qproperty(i32, selected)]
        #[qproperty(i32, mode)] // 0 = room, 1 = metatile atlas
        #[qproperty(i32, sel_metatile)]
        type RoomCanvas = super::RoomCanvasRust;

        #[cxx_override]
        unsafe fn paint(self: Pin<&mut RoomCanvas>, painter: *mut QPainter);

        // QQuickItem::update() to schedule a repaint after edits.
        #[inherit]
        fn update(self: Pin<&mut RoomCanvas>);

        #[qinvokable]
        fn paint_tile(self: Pin<&mut RoomCanvas>, col: i32, row: i32);
        #[qinvokable]
        fn metatile_at(self: &RoomCanvas, col: i32, row: i32) -> i32;
        #[qinvokable]
        fn room_count(self: &RoomCanvas) -> i32;
        #[qinvokable]
        fn room_label(self: &RoomCanvas, idx: i32) -> QString;
        #[qinvokable]
        fn img_w(self: &RoomCanvas) -> i32;
        #[qinvokable]
        fn img_h(self: &RoomCanvas) -> i32;
        #[qinvokable]
        fn save_rom(self: &RoomCanvas, path: QString) -> QString;
    }

    impl cxx_qt::Constructor<()> for RoomCanvas {}
}

pub struct RoomCanvasRust {
    selected: i32,
    mode: i32,
    sel_metatile: i32,
    header: Vec<u8>, // iNES header (16 bytes)
    prg: Vec<u8>,
    chr: Vec<u8>,
    rooms: Vec<lotw::render::Room>,
}

impl Default for RoomCanvasRust {
    fn default() -> Self {
        let rom = std::fs::read("rom/lotw.nes").expect("read rom/lotw.nes");
        let prg_len = rom[4] as usize * 16_384;
        let header = rom[0..16].to_vec();
        let prg = rom[16..16 + prg_len].to_vec();
        let chr = rom[16 + prg_len..].to_vec();
        let rooms = lotw::render::decode_rooms(&prg, 18);
        Self { selected: 0, mode: 0, sel_metatile: 0, header, prg, chr, rooms }
    }
}

impl RoomCanvasRust {
    fn sel(&self) -> usize {
        (self.selected.max(0) as usize).min(self.rooms.len().saturating_sub(1))
    }
}

impl cxx_qt::Constructor<()> for qobject::RoomCanvas {
    type NewArguments = ();
    type BaseArguments = ();
    type InitializeArguments = ();
    fn route_arguments(_: ()) -> (Self::NewArguments, Self::BaseArguments, Self::InitializeArguments) {
        ((), (), ())
    }
    fn new(_: ()) -> RoomCanvasRust {
        RoomCanvasRust::default()
    }
}

impl qobject::RoomCanvas {
    fn paint(self: Pin<&mut Self>, painter: *mut qobject::QPainter) {
        let painter = match unsafe { painter.as_mut() } {
            Some(p) => unsafe { Pin::new_unchecked(p) },
            None => return,
        };
        let r = self.rust();
        let sel = r.sel();
        let room = &r.rooms[sel];
        let (rgb, w, h) = if r.mode == 1 {
            (lotw::render::render_metatile_atlas(&r.prg, &r.chr, &room.header, &room.pal), 256, 256)
        } else {
            (room.render(&r.prg, &r.chr), lotw::render::RW as i32, lotw::render::RH as i32)
        };
        let img = unsafe { QImage::from_raw_bytes(rgb, w, h, QImageFormat::Format_RGB888) };
        painter.draw_image(&QRect::new(0, 0, w, h), &img);
    }

    fn paint_tile(mut self: Pin<&mut Self>, col: i32, row: i32) {
        let (sel, mt) = {
            let r = self.rust();
            (r.sel(), r.sel_metatile as u8)
        };
        if self.rust().mode != 0 || col < 0 || row < 0 || col >= 64 || row >= 12 {
            return;
        }
        {
            let rust = self.as_mut().rust_mut();
            let rust = unsafe { rust.get_unchecked_mut() };
            rust.rooms[sel].grid[row as usize][col as usize] = mt;
        }
        self.update();
    }

    fn metatile_at(&self, col: i32, row: i32) -> i32 {
        let r = self.rust();
        if r.mode != 0 || col < 0 || row < 0 || col >= 64 || row >= 12 {
            return -1;
        }
        r.rooms[r.sel()].grid[row as usize][col as usize] as i32
    }

    fn room_count(&self) -> i32 {
        self.rust().rooms.len() as i32
    }

    fn room_label(&self, idx: i32) -> QString {
        let r = self.rust();
        if let Some(room) = r.rooms.get(idx.max(0) as usize) {
            QString::from(&format!("{:02}-{}", room.mapy, room.mapx))
        } else {
            QString::from("")
        }
    }

    fn img_w(&self) -> i32 {
        if self.rust().mode == 1 { 256 } else { lotw::render::RW as i32 }
    }
    fn img_h(&self) -> i32 {
        if self.rust().mode == 1 { 256 } else { lotw::render::RH as i32 }
    }

    fn save_rom(&self, path: QString) -> QString {
        let r = self.rust();
        let mut prg = r.prg.clone();
        for room in &r.rooms {
            lotw::render::encode_room(&mut prg, room);
        }
        let mut rom = r.header.clone();
        rom.extend_from_slice(&prg);
        rom.extend_from_slice(&r.chr);
        match std::fs::write(path.to_string(), &rom) {
            Ok(()) => QString::from(&format!("saved {} ({} bytes)", path, rom.len())),
            Err(e) => QString::from(&format!("save failed: {e}")),
        }
    }
}
