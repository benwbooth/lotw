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
    property string tool: "paint" // paint | pick | hand | line | rect | ellipse | object
    property real zoom: 2.0
    property int dragC0: -1
    property int dragR0: -1
    property int objSel: -1
    property int newKind: 0x51

    function modeFor(v) { return v === 1 ? 2 : v === 2 ? 3 : 0 }
    function tile(v) { return Math.floor(v / 16) }   // native px -> tile index

    // Zoom about the viewport centre. zoom drives roomView.scale (GPU transform),
    // so paint() never re-runs on zoom -> smooth even for the huge world image.
    function setZoom(nz) {
        nz = Math.max(0.1, Math.min(16, nz))
        if (Math.abs(nz - zoom) < 0.0001) return
        var cx = (flick.contentX + flick.width / 2) / zoom
        var cy = (flick.contentY + flick.height / 2) / zoom
        zoom = nz
        flick.contentX = Math.max(0, cx * nz - flick.width / 2)
        flick.contentY = Math.max(0, cy * nz - flick.height / 2)
    }

    header: ToolBar {
        RowLayout {
            anchors.fill: parent
            spacing: 4
            Row {
                spacing: 1
                Repeater {
                    model: [["Room",0],["World",1],["Title",2]]
                    Button {
                        text: modelData[0]
                        checkable: true
                        checked: view === modelData[1]
                        onClicked: {
                            view = modelData[1]
                            roomView.mode = modeFor(view)
                            zoom = (view === 1) ? 0.4 : 2.0
                            flick.contentX = 0
                            flick.contentY = 0
                            roomView.refresh()
                        }
                    }
                }
            }
            ToolSeparator {}
            Row {
                spacing: 1
                visible: view === 0
                Repeater {
                    model: [["paint","✏️","Paint (drag)"],["pick","🎨","Eyedropper"],["hand","✋","Pan"],
                            ["line","╱","Line"],["rect","▭","Rectangle"],["ellipse","◯","Ellipse"],
                            ["object","📍","Objects (click=create, drag=move, Del=remove)"]]
                    Button {
                        text: modelData[1]
                        font.pixelSize: 16
                        checkable: true
                        checked: tool === modelData[0]
                        onClicked: tool = modelData[0]
                        ToolTip.visible: hovered
                        ToolTip.text: modelData[2]
                    }
                }
            }
            ToolSeparator { visible: view === 0 }
            Button { text: "↶"; ToolTip.visible: hovered; ToolTip.text: "Undo (Ctrl+Z)"; onClicked: { roomView.undo(); objSel = -1 } }
            Button { text: "↷"; ToolTip.visible: hovered; ToolTip.text: "Redo (Ctrl+Y)"; onClicked: { roomView.redo(); objSel = -1 } }
            Button { text: "Save ROM"; onClicked: status = roomView.save_rom("build/lotw-edited.nes") }
            Label { text: "room " + roomView.room_label(roomView.selected) + "  mt " + roomView.sel_metatile + "  " + zoom.toFixed(2) + "x"; color: "#ddd" }
            Item { Layout.fillWidth: true }
            Label { text: status; color: "#9f9" }
        }
    }

    RowLayout {
        anchors.fill: parent
        spacing: 6

        Flickable {
            id: flick
            Layout.fillWidth: true
            Layout.fillHeight: true
            contentWidth: wrap.width
            contentHeight: wrap.height
            clip: true
            interactive: tool === "hand" || view !== 0
            ScrollBar.vertical: ScrollBar { policy: ScrollBar.AlwaysOn }
            ScrollBar.horizontal: ScrollBar { policy: ScrollBar.AlwaysOn }

            Item {
                id: wrap
                width: roomView.width * zoom
                height: roomView.height * zoom

                RoomCanvas {
                    id: roomView
                    mode: 0
                    width: mode === 2 ? 4096 : mode === 3 ? 256 : 1024
                    height: mode === 2 ? 18 * 192 : mode === 3 ? 240 : 192
                    scale: zoom
                    transformOrigin: Item.TopLeft
                    smooth: false       // nearest-neighbour scaling = crisp pixels
                    antialiasing: false
                    onSelectedChanged: refresh()

                    PinchHandler {
                        target: null
                        property real base: 1
                        onActiveChanged: if (active) base = zoom
                        onActiveScaleChanged: setZoom(base * activeScale)
                    }
                    WheelHandler {
                        acceptedModifiers: Qt.ControlModifier
                        onWheel: (e) => setZoom(zoom * (1 + e.angleDelta.y / 800))
                    }
                    HoverHandler {
                        id: hov
                        enabled: view === 0
                        onPointChanged: { roomView.cursor_col = tile(point.position.x); roomView.cursor_row = tile(point.position.y); roomView.refresh() }
                        onHoveredChanged: if (!hovered) { roomView.cursor_col = -1; roomView.refresh() }
                    }
                    MouseArea {
                        anchors.fill: parent
                        enabled: tool !== "hand"
                        onPressed: (m) => {
                            if (view === 1) {
                                var idx = roomView.world_room_at(m.x, m.y)
                                if (idx >= 0) { roomView.selected = idx; view = 0; roomView.mode = 0; zoom = 2.0; flick.contentX = 0; flick.contentY = 0; roomView.refresh() }
                                return
                            }
                            if (view !== 0) return
                            var c = tile(m.x), r = tile(m.y)
                            if (tool === "pick") { var v = roomView.metatile_at(c, r); if (v >= 0) roomView.sel_metatile = v; return }
                            roomView.begin_edit()
                            if (tool === "paint") roomView.paint_tile(c, r)
                            else if (tool === "object") objSel = roomView.create_obj(c, Math.floor(m.y), newKind)
                            else { dragC0 = c; dragR0 = r }
                        }
                        onPositionChanged: (m) => {
                            if (view !== 0 || !m.buttons) return
                            if (tool === "paint") roomView.paint_tile(tile(m.x), tile(m.y))
                            else if (dragC0 >= 0) {
                                var k = tool === "line" ? 1 : tool === "rect" ? 2 : tool === "ellipse" ? 3 : 0
                                if (k) roomView.set_preview(k, dragC0, dragR0, tile(m.x), tile(m.y))
                            }
                        }
                        onReleased: (m) => {
                            if (view !== 0 || dragC0 < 0) return
                            roomView.clear_preview()
                            var c = tile(m.x), r = tile(m.y)
                            if (tool === "line") roomView.paint_line(dragC0, dragR0, c, r)
                            else if (tool === "rect") roomView.paint_rect(dragC0, dragR0, c, r)
                            else if (tool === "ellipse") roomView.paint_ellipse(dragC0, dragR0, c, r)
                            dragC0 = -1
                        }
                    }

                    // object markers (native coords; scaled with the canvas)
                    Repeater {
                        model: 12
                        Rectangle {
                            required property int index
                            visible: view === 0 && (roomView.obj_rev, roomView.selected, roomView.obj_active(index))
                            x: (roomView.obj_rev, roomView.obj_x(index)) * 16
                            y: (roomView.obj_rev, roomView.obj_y(index))
                            width: 16; height: 16
                            color: "transparent"
                            border.color: objSel === index ? "#ff8c00" : "#ffdd00"
                            border.width: 2 / zoom
                            Text {
                                anchors.centerIn: parent
                                text: ((roomView.obj_rev, roomView.obj_kind(index))).toString(16)
                                color: parent.border.color
                                font.pixelSize: 10
                                scale: 1 / zoom
                            }
                            MouseArea {
                                anchors.fill: parent
                                enabled: tool === "object"
                                onPressed: { objSel = index; roomView.begin_edit() }
                                onPositionChanged: (m) => {
                                    if (!pressed) return
                                    var p = mapToItem(roomView, m.x, m.y)
                                    roomView.set_obj(index, roomView.obj_kind(index), Math.floor(p.x / 16), Math.floor(p.y))
                                }
                            }
                        }
                    }
                }
            }
        }

        Shortcut {
            sequences: [StandardKey.Delete, StandardKey.Backspace]
            enabled: tool === "object" && objSel >= 0
            onActivated: { roomView.begin_edit(); roomView.delete_obj(objSel); objSel = -1 }
        }
        Shortcut { sequence: StandardKey.Undo; onActivated: { roomView.undo(); objSel = -1 } }
        Shortcut { sequences: [StandardKey.Redo, "Ctrl+Y"]; onActivated: { roomView.redo(); objSel = -1 } }
        Shortcut {
            sequence: "Ctrl+D"
            enabled: tool === "object" && objSel >= 0
            onActivated: { roomView.begin_edit(); objSel = roomView.copy_obj(objSel) }
        }

        ColumnLayout {
            visible: view === 0
            Layout.preferredWidth: 264
            Label { text: "Metatile palette:"; color: "#bbb" }
            RoomCanvas {
                id: atlasView
                mode: 1
                selected: roomView.selected
                width: 256; height: 256
                smooth: false
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

            ColumnLayout {
                visible: tool === "object"
                spacing: 4
                Label { text: "Objects — click=create, drag=move, Del=remove"; color: "#bbb"; wrapMode: Text.WordWrap; Layout.preferredWidth: 256 }
                RowLayout {
                    Label { text: "new type:"; color: "#ddd" }
                    SpinBox { from: 0; to: 255; value: newKind; textFromValue: (v) => "0x" + v.toString(16); onValueModified: newKind = value }
                }
                Label {
                    visible: objSel >= 0
                    text: "selected obj " + objSel + "  type 0x" + (objSel >= 0 ? (roomView.obj_rev, roomView.obj_kind(objSel)).toString(16) : "")
                    color: "#fc0"
                }
                RowLayout {
                    visible: objSel >= 0
                    Button { text: "Duplicate"; onClicked: { roomView.begin_edit(); objSel = roomView.copy_obj(objSel) } }
                    Button { text: "Delete"; onClicked: { roomView.begin_edit(); roomView.delete_obj(objSel); objSel = -1 } }
                }
            }
            Item { Layout.fillHeight: true }
        }
    }
}
