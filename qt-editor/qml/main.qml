import QtQuick
import QtQuick.Window
import com.lotw.editor

Window {
    visible: true
    width: 1100
    height: 360
    title: "LotW editor (cxx-qt)"
    color: "#222222"

    Text {
        anchors.top: parent.top
        anchors.horizontalCenter: parent.horizontalCenter
        color: "#cccccc"
        text: "Room " + canvas.selected + " — pinch to zoom (scale " + canvas.scale.toFixed(2) + ")"
    }

    RoomCanvas {
        id: canvas
        selected: 0
        width: 1024
        height: 192
        anchors.centerIn: parent
        transformOrigin: Item.Center

        PinchHandler {
            target: canvas
            minimumScale: 0.25
            maximumScale: 8.0
            // Zoom only — no rotation, no translation drift.
            rotationAxis.enabled: false
            xAxis.enabled: false
            yAxis.enabled: false
        }
        WheelHandler {
            acceptedModifiers: Qt.ControlModifier
            onWheel: (e) => canvas.scale = Math.max(0.25, Math.min(8.0, canvas.scale * (1 + e.angleDelta.y / 1200)))
        }
    }
}
