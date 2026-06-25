use cxx_qt_build::CxxQtBuilder;

fn main() {
    // Link QtQuick/QML; the QML file is loaded from disk at runtime (no qrc
    // module needed for this minimal pinch test).
    CxxQtBuilder::new().qt_module("Quick").build();
}
