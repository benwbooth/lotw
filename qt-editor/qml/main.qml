import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import com.lotw.editor

ApplicationWindow {
    visible: true
    width: 1300
    height: 720
    title: "LotW asset editor (cxx-qt)"
    color: "#222222"

    property string status: ""

    header: ToolBar {
        RowLayout {
            anchors.fill: parent
            Button {
                text: "Save ROM"
                onClicked: status = roomView.save_rom("build/lotw-edited.nes")
            }
            Label { text: "room " + roomView.room_label(roomView.selected) + "   metatile " + roomView.sel_metatile; color: "#ddd" }
            Item { Layout.fillWidth: true }
            Label { text: status; color: "#9f9" }
        }
    }

    RowLayout {
        anchors.fill: parent
        spacing: 6

        // --- room navigator ---
        ScrollView {
            Layout.preferredWidth: 200
            Layout.fillHeight: true
            Grid {
                columns: 4
                spacing: 2
                Repeater {
                    model: roomView.room_count()
                    Button {
                        width: 44
                        text: roomView.room_label(index)
                        highlighted: roomView.selected === index
                        font.pixelSize: 9
                        padding: 2
                        onClicked: roomView.selected = index
                    }
                }
            }
        }

        // --- room canvas + palette ---
        ColumnLayout {
            Layout.fillWidth: true
            Layout.fillHeight: true

            Flickable {
                Layout.fillWidth: true
                Layout.fillHeight: true
                contentWidth: roomWrap.width
                contentHeight: roomWrap.height
                clip: true
                Item {
                    id: roomWrap
                    width: roomView.width * roomView.scale
                    height: roomView.height * roomView.scale

                    RoomCanvas {
                        id: roomView
                        mode: 0
                        width: img_w()
                        height: img_h()
                        scale: 2.0
                        transformOrigin: Item.TopLeft

                        PinchHandler {
                            target: roomView
                            minimumScale: 0.5
                            maximumScale: 12.0
                            rotationAxis.enabled: false
                            xAxis.enabled: false
                            yAxis.enabled: false
                        }
                        WheelHandler {
                            acceptedModifiers: Qt.ControlModifier
                            onWheel: (e) => roomView.scale = Math.max(0.5, Math.min(12.0, roomView.scale * (1 + e.angleDelta.y / 1200)))
                        }
                        HoverHandler { id: roomHover }
                        MouseArea {
                            anchors.fill: parent
                            onPressed: (m) => roomView.paint_tile(Math.floor(m.x / 16), Math.floor(m.y / 16))
                            onPositionChanged: (m) => { if (m.buttons) roomView.paint_tile(Math.floor(m.x / 16), Math.floor(m.y / 16)) }
                        }
                        // tile-snapped cursor (white outer + black inner)
                        Rectangle {
                            visible: roomHover.hovered
                            x: Math.floor(roomHover.point.position.x / 16) * 16
                            y: Math.floor(roomHover.point.position.y / 16) * 16
                            width: 16; height: 16
                            color: "transparent"
                            border.color: "white"; border.width: 1
                            Rectangle { anchors.fill: parent; anchors.margins: 1; color: "transparent"; border.color: "black"; border.width: 1 }
                        }
                    }
                }
            }

            Label { text: "Metatile palette (click to pick):"; color: "#bbb" }
            RoomCanvas {
                id: atlasView
                mode: 1
                selected: roomView.selected
                width: img_w()
                height: img_h()
                Layout.preferredWidth: 256
                Layout.preferredHeight: 256
                MouseArea {
                    anchors.fill: parent
                    onClicked: (m) => roomView.sel_metatile = Math.floor(m.y / 16) * 16 + Math.floor(m.x / 16)
                }
                Rectangle {
                    x: (roomView.sel_metatile % 16) * 16
                    y: Math.floor(roomView.sel_metatile / 16) * 16
                    width: 16; height: 16
                    color: "transparent"; border.color: "#0f0"; border.width: 2
                }
            }
        }
    }
}
