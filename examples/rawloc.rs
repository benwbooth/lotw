fn main() {
    let mut per_ch = [0u32; 4];
    let mut named_pitch_raw = 0u32; // raw whose pitch IS nameable (real duration issue)
    for idx in 0..19 {
        let Some(s) = lotw::music::get(idx) else { continue };
        for c in 0..4 {
            for t in &s.channels[c].1 {
                if let lotw::audio::Tok::Note{dur,pitch} = t {
                    let nameable = lotw::audio::pitch_str(*pitch);
                    let valued = lotw::audio::val_name_pub(*dur, /*tempo*/ 0); // placeholder
                    let _ = (dur, valued);
                    if !nameable.starts_with(char::from(b\"~\"[0])) && pitch>>4 <= 9 {} // nameable
                    else { per_ch[c]+=1; }
                }
            }
        }
    }
    println!(\"raw-pitch notes per channel [p1,p2,tri,noise]: {:?}\", per_ch);
}
