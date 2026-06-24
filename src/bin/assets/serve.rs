//! `assettool serve` — a local web editor for the assets.
//!
//! Minimal std-only HTTP server on 127.0.0.1 that serves a canvas room/metatile
//! editor. It renders each room's metatile atlas (from CHR + tile_table +
//! palette) for the browser to paint with, and reads/writes the room CSV grids
//! in `assets/`. After editing, `assettool build` recompiles the ROM (and the
//! byte-identical gate still applies to unedited regions).

use std::error::Error;
use std::fs;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpListener;
use std::path::{Path, PathBuf};

use super::render;

pub fn serve(rom: &str, assets_dir: &str, port: u16) -> Result<(), Box<dyn Error>> {
    let romdata = fs::read(rom)?;
    let prg_len = romdata[4] as usize * 16_384;
    let prg = romdata[16..16 + prg_len].to_vec();
    let chr = romdata[16 + prg_len..].to_vec();
    let dir = PathBuf::from(assets_dir);
    let listener = TcpListener::bind(("127.0.0.1", port))?;
    println!("asset editor at http://127.0.0.1:{port}  (Ctrl-C to stop)");
    for stream in listener.incoming() {
        let mut s = match stream {
            Ok(s) => s,
            Err(_) => continue,
        };
        if let Err(e) = handle(&mut s, &prg, &chr, &dir) {
            let _ = respond(&mut s, "500 Internal Error", "text/plain", e.to_string().as_bytes());
        }
    }
    Ok(())
}

fn handle(s: &mut std::net::TcpStream, prg: &[u8], chr: &[u8], dir: &Path) -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(s.try_clone()?);
    let mut line = String::new();
    reader.read_line(&mut line)?;
    let mut parts = line.split_whitespace();
    let method = parts.next().unwrap_or("");
    let path = parts.next().unwrap_or("/").to_string();
    // Drain headers, capturing Content-Length.
    let mut clen = 0usize;
    loop {
        let mut h = String::new();
        reader.read_line(&mut h)?;
        if h == "\r\n" || h.is_empty() {
            break;
        }
        if let Some(v) = h.to_ascii_lowercase().strip_prefix("content-length:") {
            clen = v.trim().parse().unwrap_or(0);
        }
    }
    let mut body = vec![0u8; clen];
    if clen > 0 {
        reader.read_exact(&mut body)?;
    }

    let seg: Vec<&str> = path.trim_start_matches('/').split('/').collect();
    match (method, seg.as_slice()) {
        ("GET", [""]) => respond(s, "200 OK", "text/html; charset=utf-8", EDITOR_HTML.as_bytes()),
        ("GET", ["api", "manifest"]) => {
            let m = fs::read(dir.join("rooms/manifest.json"))?;
            respond(s, "200 OK", "application/json", &m)
        }
        ("GET", ["api", "atlas", my, mx]) => {
            let (header, pal) = room_meta(dir, my, mx)?;
            let img = render::render_metatile_atlas(prg, chr, &header, &pal);
            let png = render::rgb_to_png_bytes(256, 256, &img)?;
            respond(s, "200 OK", "image/png", &png)
        }
        ("GET", ["api", "grid", my, mx]) => {
            let g = fs::read(dir.join(format!("rooms/room-{my:0>2}-{mx}.csv")))?;
            respond(s, "200 OK", "text/plain", &g)
        }
        ("POST", ["api", "grid", my, mx]) => {
            fs::write(dir.join(format!("rooms/room-{my:0>2}-{mx}.csv")), &body)?;
            respond(s, "200 OK", "text/plain", b"saved")
        }
        _ => respond(s, "404 Not Found", "text/plain", b"not found"),
    }
}

/// Fetch a room's 32-byte header + 32-byte palette from the manifest.
fn room_meta(dir: &Path, my: &str, mx: &str) -> Result<(Vec<u8>, Vec<u8>), Box<dyn Error>> {
    let m: serde_json::Value = serde_json::from_str(&fs::read_to_string(dir.join("rooms/manifest.json"))?)?;
    let (my, mx): (u64, u64) = (my.parse()?, mx.parse()?);
    for room in m["rooms"].as_array().ok_or("bad manifest")? {
        if room["mapy"].as_u64() == Some(my) && room["mapx"].as_u64() == Some(mx) {
            let header = (0..room["header_hex"].as_str().unwrap().len() / 2)
                .map(|i| u8::from_str_radix(&room["header_hex"].as_str().unwrap()[2 * i..2 * i + 2], 16).unwrap())
                .collect();
            let pal = room["palette"]["indices"].as_array().unwrap().iter().map(|v| v.as_u64().unwrap() as u8).collect();
            return Ok((header, pal));
        }
    }
    Err("room not found".into())
}

fn respond(s: &mut std::net::TcpStream, status: &str, ctype: &str, body: &[u8]) -> Result<(), Box<dyn Error>> {
    let head = format!("HTTP/1.1 {status}\r\nContent-Type: {ctype}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n", body.len());
    s.write_all(head.as_bytes())?;
    s.write_all(body)?;
    s.flush()?;
    Ok(())
}

const EDITOR_HTML: &str = r#"<!doctype html><html><head><meta charset=utf-8><title>LotW asset editor</title>
<style>
 body{font:13px monospace;background:#222;color:#ddd;margin:0;display:flex;height:100vh}
 #side{width:140px;overflow:auto;padding:6px;background:#1a1a1a}
 #side div{padding:2px 4px;cursor:pointer}#side div:hover{background:#333}
 #main{flex:1;overflow:auto;padding:8px}
 canvas{image-rendering:pixelated;border:1px solid #444}
 #room{width:1024px;height:192px}#atlas{width:256px;height:256px}
 .bar{margin:6px 0}
</style></head><body>
<div id=side></div>
<div id=main>
 <div class=bar>Room: <b id=title>—</b> &nbsp; selected metatile: <b id=sel>0</b>
   &nbsp; <button id=save>Save grid</button> <span id=msg></span></div>
 <canvas id=room></canvas>
 <div class=bar>Paint palette (click to pick):</div>
 <canvas id=atlas></canvas>
</div>
<script>
let cols=64,rows=12,grid=null,atlas=new Image(),sel=0,cur=null;
const $=id=>document.getElementById(id);
async function init(){let m=await(await fetch('/api/manifest')).json();
 m.rooms.forEach(r=>{let d=document.createElement('div');d.textContent=`room ${String(r.mapy).padStart(2,'0')}-${r.mapx}`;
  d.onclick=()=>load(r.mapy,r.mapx);$('side').appendChild(d);});}
async function load(my,mx){cur={my,mx};$('title').textContent=`${String(my).padStart(2,'0')}-${mx}`;
 atlas=new Image();atlas.src=`/api/atlas/${my}/${mx}?`+Date.now();await atlas.decode();
 let csv=await(await fetch(`/api/grid/${my}/${mx}`)).text();
 grid=csv.trim().split('\n').map(l=>l.split(',').map(Number));
 drawAtlas();drawRoom();}
function drawRoom(){let c=$('room'),x=c.getContext('2d');c.width=cols*16;c.height=rows*16;
 for(let r=0;r<rows;r++)for(let q=0;q<cols;q++){let mt=grid[r][q];
  x.drawImage(atlas,(mt%16)*16,(mt>>4)*16,16,16,q*16,r*16,16,16);}}
function drawAtlas(){let c=$('atlas'),x=c.getContext('2d');c.width=256;c.height=256;x.drawImage(atlas,0,0);
 x.strokeStyle='#0f0';x.strokeRect((sel%16)*16,(sel>>4)*16,16,16);}
$('atlas').onclick=e=>{let r=e.target.getBoundingClientRect();
 sel=(Math.floor((e.clientY-r.top)/16))*16+Math.floor((e.clientX-r.left)/16);$('sel').textContent=sel;drawAtlas();};
$('room').onclick=e=>{let r=e.target.getBoundingClientRect();
 let q=Math.floor((e.clientX-r.left)/16),rw=Math.floor((e.clientY-r.top)/16);
 if(grid[rw]&&grid[rw][q]!==undefined){grid[rw][q]=sel;drawRoom();}};
$('save').onclick=async()=>{let csv=grid.map(r=>r.join(',')).join('\n')+'\n';
 await fetch(`/api/grid/${cur.my}/${cur.mx}`,{method:'POST',body:csv});
 $('msg').textContent='saved ✓';setTimeout(()=>$('msg').textContent='',1500);};
init();
</script></body></html>"#;
