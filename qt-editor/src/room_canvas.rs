use core::pin::Pin;
use cxx_qt::CxxQtType;
use cxx_qt_lib::{QImage, QImageFormat, QRect};

#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "C++" {
        include!(<QtQuick/QQuickPaintedItem>);
        type QQuickPaintedItem;
        include!("cxx-qt-lib/qpainter.h");
        type QPainter = cxx_qt_lib::QPainter;
    }

    unsafe extern "RustQt" {
        #[qml_element]
        #[base = QQuickPaintedItem]
        #[qobject]
        #[qproperty(i32, selected)]
        type RoomCanvas = super::RoomCanvasRust;

        #[cxx_override]
        unsafe fn paint(self: Pin<&mut RoomCanvas>, painter: *mut QPainter);
    }

    impl cxx_qt::Constructor<()> for RoomCanvas {}
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

pub struct RoomCanvasRust {
    selected: i32,
    prg: Vec<u8>,
    chr: Vec<u8>,
    rooms: Vec<lotw::render::Room>,
}

impl Default for RoomCanvasRust {
    fn default() -> Self {
        let rom = std::fs::read("rom/lotw.nes").expect("read rom/lotw.nes");
        let prg_len = rom[4] as usize * 16_384;
        let prg = rom[16..16 + prg_len].to_vec();
        let chr = rom[16 + prg_len..].to_vec();
        let rooms = lotw::render::decode_rooms(&prg, 18);
        Self { selected: 0, prg, chr, rooms }
    }
}

impl qobject::RoomCanvas {
    fn paint(self: Pin<&mut Self>, painter: *mut qobject::QPainter) {
        let painter = match unsafe { painter.as_mut() } {
            Some(p) => unsafe { Pin::new_unchecked(p) },
            None => return,
        };
        let r = self.rust();
        let sel = (r.selected.max(0) as usize).min(r.rooms.len().saturating_sub(1));
        let rgb = r.rooms[sel].render(&r.prg, &r.chr);
        let img = unsafe {
            QImage::from_raw_bytes(
                rgb,
                lotw::render::RW as i32,
                lotw::render::RH as i32,
                QImageFormat::Format_RGB888,
            )
        };
        painter.draw_image(&QRect::new(0, 0, lotw::render::RW as i32, lotw::render::RH as i32), &img);
    }
}
