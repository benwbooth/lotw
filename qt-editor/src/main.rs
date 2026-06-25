use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QString, QUrl};

fn main() {
    let mut app = QGuiApplication::new();
    let mut engine = QQmlApplicationEngine::new();

    if let Some(engine) = engine.as_mut() {
        let qml = format!("file://{}/qml/main.qml", env!("CARGO_MANIFEST_DIR"));
        engine.load(&QUrl::from(&QString::from(&qml)));
    }

    if let Some(app) = app.as_mut() {
        app.exec();
    }
}
