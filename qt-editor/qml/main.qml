import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import com.lotw.editor

ApplicationWindow {
    visible: true
    width: 1300
    height: 760
    title: "LotW asset editor (cxx-qt)"
    color: "#222222"

    property string status: ""
    property int view: 0          // 0 room, 1 world, 2 title
    property string tool: "paint" // paint | pick | hand | line | rect | ellipse
    property int dragC0: -1
    property int dragR0: -1

    function modeFor(v) { return v === 1 ? 2 : v === 2 ? 3 : 0 }

    header: ToolBar {
        RowLayout {
            anchors.fill: parent
            spacing: 4
            // view switch
            Row {
                spacing: 1
                Repeater {
                    model: [["Room",0],["World",1],["Title",2]]
                    Button {
                        text: modelData[0]
                        checkable: true
                        checked: view === modelData[1]
                        onClicked: { view = modelData[1]; roomView.mode = modeFor(view); roomView.refresh() }
                    }
                }
            }
            ToolSeparator {}
            // tools (room view only)
            Row {
                spacing: 1
                visible: view === 0
                Repeater {
                    model: ["paint","pick","hand","line","rect","ellipse"]
                    Button {
                        text: modelData
                        checkable: true
                        checked: tool === modelData
                        onClicked: tool = modelData
                    }
                }
            }
            ToolSeparator { visible: view === 0 }
            Button { text: "Save ROM"; onClicked: status = roomView.save_rom("build/lotw-edited.nes") }
            Label { text: "room " + roomView.room_label(roomView.selected) + "  mt " + roomView.sel_metatile; color: "#ddd" }
            Item { Layout.fillWidth: true }
            Label { text: status; color: "#9f9" }
        }
    }

    RowLayout {
        anchors.fill: parent
        spacing: 6

        // ---- canvas area ----
        Flickable {
            id: flick
            Layout.fillWidth: true
            Layout.fillHeight: true
            contentWidth: wrap.width
            contentHeight: wrap.height
            clip: true
            interactive: tool === "hand" || view !== 0   // pan only with Hand (or in world/title)
            ScrollBar.vertical: ScrollBar {}
            ScrollBar.horizontal: ScrollBar {}

            Item {
                id: wrap
                width: roomView.width * roomView.scale
                height: roomView.height * roomView.scale

                RoomCanvas {
                    id: roomView
                    mode: 0
                    width: mode === 2 ? 4096 : mode === 3 ? 256 : 1024
                    height: mode === 2 ? 18 * 192 : mode === 3 ? 240 : 192
                    scale: view === 1 ? 0.4 : 2.0
                    transformOrigin: Item.TopLeft
                    onSelectedChanged: refresh()

                    PinchHandler {
                        target: roomView
                        minimumScale: 0.2
                        maximumScale: 12.0
                        rotationAxis.enabled: false
                        xAxis.enabled: false
                        yAxis.enabled: false
                    }
                    WheelHandler {
                        acceptedModifiers: Qt.ControlModifier
                        onWheel: (e) => roomView.scale = Math.max(0.2, Math.min(12.0, roomView.scale * (1 + e.angleDelta.y / 1200)))
                    }

                    // inverse-color tile cursor (room view, non-hand tools)
                    HoverHandler {
                        id: hov
                        enabled: view === 0
                        onPointChanged: {
                            roomView.cursor_col = Math.floor(point.position.x / 16)
                            roomView.cursor_row = Math.floor(point.position.y / 16)
                            roomView.refresh()
                        }
                        onHoveredChanged: if (!hovered) { roomView.cursor_col = -1; roomView.refresh() }
                    }

                    MouseArea {
                        anchors.fill: parent
                        enabled: tool !== "hand"
                        function cell(m) { return [Math.floor(m.x / 16), Math.floor(m.y / 16)] }
                        onPressed: (m) => {
                            if (view === 1) { // world: navigate
                                var idx = roomView.world_room_at(m.x, m.y)
                                if (idx >= 0) { roomView.selected = idx; view = 0; roomView.mode = 0; roomView.refresh() }
                                return
                            }
                            if (view !== 0) return
                            var c = cell(m)
                            if (tool === "paint") roomView.paint_tile(c[0], c[1])
                            else if (tool === "pick") { var v = roomView.metatile_at(c[0], c[1]); if (v >= 0) roomView.sel_metatile = v }
                            else { dragC0 = c[0]; dragR0 = c[1] }
                        }
                        onPositionChanged: (m) => {
                            if (view === 0 && tool === "paint" && m.buttons) { var c = cell(m); roomView.paint_tile(c[0], c[1]) }
                        }
                        onReleased: (m) => {
                            if (view !== 0 || dragC0 < 0) return
                            var c = cell(m)
                            if (tool === "line") roomView.paint_line(dragC0, dragR0, c[0], c[1])
                            else if (tool === "rect") roomView.paint_rect(dragC0, dragR0, c[0], c[1])
                            else if (tool === "ellipse") roomView.paint_ellipse(dragC0, dragR0, c[0], c[1])
                            dragC0 = -1
                        }
                    }
                }
            }
        }

        // ---- metatile palette (room view) ----
        ColumnLayout {
            visible: view === 0
            Layout.preferredWidth: 264
            Label { text: "Metatile palette:"; color: "#bbb" }
            RoomCanvas {
                id: atlasView
                mode: 1
                selected: roomView.selected
                width: 256; height: 256
                onSelectedChanged: refresh()
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
            Item { Layout.fillHeight: true }
        }
    }
}
