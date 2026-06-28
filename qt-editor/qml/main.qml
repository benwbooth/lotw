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
    // The Sprites tab (view 3) must scale by an integer factor so the pixel-art
    // sprites stay crisp (non-integer nearest-neighbour scaling makes some pixels
    // wider than others). Background-tile views keep the continuous zoom.
    property real pixScale: view === 3 ? Math.max(1, Math.round(zoom)) : zoom
    property int dragC0: -1
    property int dragR0: -1
    property int objSel: -1
    property int newKind: 0x51

    function modeFor(v) { return v === 1 ? 2 : v === 2 ? 3 : v === 3 ? 4 : 0 }
    property bool animate: false
    property int animTick: 0
    property string hoverInfo: ""

    Timer {
        running: animate
        interval: 250
        repeat: true
        onTriggered: { animTick = animTick === 0 ? 4 : 0; roomView.set_anim(animTick) }
    }
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
                    model: [["Room",0],["World",1],["Title",2],["Sprites",3]]
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
                    model: [["paint","✏️","Paint (drag)"],["erase","🧹","Eraser (revert tile to original)"],["pick","🎨","Eyedropper"],["hand","✋","Pan"],
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
            ToolSeparator {}
            CheckBox {
                text: "Animate"
                checked: animate
                onToggled: { animate = checked; if (!checked) { animTick = 0; roomView.set_anim(0) } }
                ToolTip.visible: hovered
                ToolTip.text: "Cycle sprite frames (approx)"
            }
            Button {
                visible: view === 3
                text: "Bank palette: " + (roomView.sprite_pal === 0 ? "grey" : roomView.sprite_pal)
                onClicked: { roomView.sprite_pal = (roomView.sprite_pal + 1) % 5; roomView.refresh() }
                ToolTip.visible: hovered
                ToolTip.text: "Palette for the shared boss/object bank rows (grey / room sprite palettes). Players & area enemies already use their real palettes."
            }
            ToolSeparator {}
            Button { text: "−"; ToolTip.visible: hovered; ToolTip.text: "Zoom out"; onClicked: setZoom(zoom / 1.25) }
            Button { text: "+"; ToolTip.visible: hovered; ToolTip.text: "Zoom in (pinch also works)"; onClicked: setZoom(zoom * 1.25) }
            Label { text: "room " + roomView.room_label(roomView.selected) + "  mt " + roomView.sel_metatile + "  " + zoom.toFixed(2) + "x"; color: "#ddd" }
            Label { text: view === 3 ? hoverInfo : ""; color: "#9cf" }
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
            interactive: true   // keep wheel/trackpad scroll working in all tools
            flickableDirection: Flickable.HorizontalAndVerticalFlick
            ScrollBar.vertical: ScrollBar { policy: ScrollBar.AlwaysOn }
            ScrollBar.horizontal: ScrollBar { policy: ScrollBar.AlwaysOn }

            Item {
                id: wrap
                width: roomView.width * pixScale
                height: roomView.height * pixScale

                RoomCanvas {
                    id: roomView
                    mode: 0
                    width: mode === 2 ? 4096 : mode === 3 ? 256 : mode === 4 ? img_w() : 1024
                    height: mode === 2 ? 18 * 192 : mode === 3 ? 240 : mode === 4 ? img_h() : 192
                    scale: pixScale
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
                    HoverHandler {
                        id: hov
                        enabled: view === 0
                        onPointChanged: { roomView.cursor_col = tile(point.position.x); roomView.cursor_row = tile(point.position.y); roomView.refresh() }
                        onHoveredChanged: if (!hovered) { roomView.cursor_col = -1; roomView.refresh() }
                    }
                    HoverHandler {
                        enabled: view === 3
                        onPointChanged: hoverInfo = roomView.tile_info(point.position.x, point.position.y)
                    }

                    MouseArea {
                        anchors.fill: parent
                        enabled: tool !== "hand"
                        preventStealing: true   // drag paints, doesn't pan the Flickable
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
                            else if (tool === "erase") roomView.erase_tile(c, r)
                            else if (tool === "object") objSel = roomView.create_obj(c, Math.floor(m.y), newKind)
                            else { dragC0 = c; dragR0 = r }
                        }
                        onPositionChanged: (m) => {
                            if (view !== 0 || !m.buttons) return
                            if (tool === "paint") roomView.paint_tile(tile(m.x), tile(m.y))
                            else if (tool === "erase") roomView.erase_tile(tile(m.x), tile(m.y))
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
                            radius: 8
                            color: objSel === index ? "#66ff8c00" : "#553a9bff"   // translucent spawn marker
                            border.color: objSel === index ? "#ff8c00" : "#8cc8ff"
                            border.width: 2 / zoom
                            // crosshair centre
                            Rectangle { anchors.centerIn: parent; width: 16; height: 2 / zoom; color: parent.border.color }
                            Rectangle { anchors.centerIn: parent; width: 2 / zoom; height: 16; color: parent.border.color }
                            Text {
                                anchors.horizontalCenter: parent.horizontalCenter
                                anchors.top: parent.bottom
                                text: "0x" + ((roomView.obj_rev, roomView.obj_kind(index))).toString(16)
                                color: parent.border.color
                                font.pixelSize: 9
                                scale: 1 / zoom
                                transformOrigin: Item.Top
                            }
                            ToolTip {
                                visible: objHov.hovered
                                text: "slot " + index + "\nsprite tile 0x" + roomView.obj_byte(index,0).toString(16) +
                                      "\nattr 0x" + roomView.obj_byte(index,1).toString(16) + " (palette " + (roomView.obj_byte(index,1)&3) + ")" +
                                      "\nbehavior " + roomView.obj_byte(index,8) +
                                      "\nHP " + roomView.obj_byte(index,4) + "  dmg " + roomView.obj_byte(index,5) +
                                      "\npos tile " + roomView.obj_x(index) + ", y " + roomView.obj_y(index)
                            }
                            HoverHandler { id: objHov }
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
                Label {
                    text: "Record bytes — 0: sprite tile, 1: attr/palette, 2: x tile, 3: y px, 4: HP, 5: damage, 8: behavior (0–8). Hover a marker for details."
                    color: "#999"; wrapMode: Text.WordWrap; Layout.preferredWidth: 256; font.pixelSize: 10
                }
            }
            Item { Layout.fillHeight: true }
        }
    }
}
