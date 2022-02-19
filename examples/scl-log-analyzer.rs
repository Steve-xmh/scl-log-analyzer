use scl_log_analyzer::*;
use std::io::Read;

fn main() {
    let mut buf = [0; 16];
    let mut char_left = 0;
    let mut analyzer = ClientAnalyzer::default();
    let mut f = std::fs::File::open(std::env::args().nth(1).unwrap()).unwrap();
    loop {
        let n = f.read(&mut buf[char_left..]).unwrap();
        if n == 0 {
            if char_left > 0 {
                if let Ok(s) = std::str::from_utf8(&buf[..char_left]) {
                    analyzer.feed_chunk(s);
                }
            }
            break;
        }
        let n = n + char_left;
        match std::str::from_utf8(&buf[..n]) {
            Ok(s) => {
                analyzer.feed_chunk(s);
                char_left = 0;
            }
            Err(e) => {
                let valid = e.valid_up_to();
                let valid_str = unsafe { std::str::from_utf8_unchecked(&buf[..valid]) };
                analyzer.feed_chunk(valid_str);
                for i in 0..n - valid {
                    buf[i] = buf[valid + i];
                }
                char_left = n - valid;
            }
        }
    }
    analyzer.end();
    println!("Last statement: {:?}", analyzer.current_statement());
    println!("Issues: {:?}", analyzer.issues());
}
