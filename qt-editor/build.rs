use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    CxxQtBuilder::new()
        .qt_module("Quick")
        .qml_module(QmlModule::<&str, _> {
            uri: "com.lotw.editor",
            rust_files: &["src/room_canvas.rs"],
            qml_files: &["qml/main.qml"],
            ..Default::default()
        })
        // Compile the QIcon shim alongside the bridge (shares the Qt includes).
        .cc_builder(|cc| {
            cc.file("src/icon_shim.cpp");
        })
        .build();
    println!("cargo:rerun-if-changed=src/icon_shim.cpp");
}
