import QtQuick
import QtQuick.Window

Window {
    visible: true
    width: 640
    height: 480
    title: "cxx-qt pinch test"
    color: "#222222"

    Text {
        anchors.top: parent.top
        anchors.horizontalCenter: parent.horizontalCenter
        anchors.topMargin: 8
        color: "#cccccc"
        text: "Pinch the square (two-finger) — or ctrl+scroll. scale: " + content.scale.toFixed(2)
    }

    Rectangle {
        id: content
        anchors.centerIn: parent
        width: 220
        height: 220
        color: "tomato"
        Text {
            anchors.centerIn: parent
            text: "pinch me"
            color: "white"
        }
        PinchHandler {
            target: content
            minimumScale: 0.2
            maximumScale: 12.0
        }
        WheelHandler {
            acceptedModifiers: Qt.ControlModifier
            onWheel: (e) => content.scale = Math.max(0.2, Math.min(12.0, content.scale * (1 + e.angleDelta.y / 1200)))
        }
    }
}
