use std::time::Duration;
use console::Term;

use anyhow::Result;
use rodio::{Sink, source:: SineWave, OutputStream, Source};

fn note_to_freq(n: i32) -> f32 {
    let p_a = 440.0;
    p_a * (2.0 as f32).powf((n as f32 - 49.0) / 12.0)
}

fn wrap(n: u32) -> u32 {
    let n = n % 99;

    if n < 20 {
        20 + n
    } else {
        n
    }
}


fn main() -> Result<()> {
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;
    let term = Term::stdout();

    loop {
        let c: u32 = term.read_char()?.into();
        let freq = note_to_freq(wrap(c) as i32);
        println!("char: {c}, freq: {freq} Hz");
        let source = SineWave::new(freq).take_duration(Duration::from_millis(150));
        sink.append(source);
        sink.sleep_until_end();
    }
}
