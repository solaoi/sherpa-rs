/// wget https://github.com/k2-fsa/sherpa-onnx/releases/download/asr-models/sherpa-onnx-whisper-tiny.tar.bz2
/// tar xvf sherpa-onnx-whisper-tiny.tar.bz2
/// rm sherpa-onnx-whisper-tiny.tar.bz2
/// cargo run --example language_id
use eyre::{bail, Result};
use sherpa_rs::language_id;
use std::io::Cursor;

fn main() -> Result<()> {
    // Read audio data from the file
    let audio_data: &[u8] = include_bytes!("../samples/16hz_mono_pcm_s16le.wav");

    let cursor = Cursor::new(audio_data);
    let mut reader = hound::WavReader::new(cursor)?;
    let sample_rate = reader.spec().sample_rate as i32;

    if sample_rate != 16000 {
        bail!("The sample rate must be 16000.");
    }

    let samples: Vec<f32> = reader
        .samples::<i16>()
        .map(|s| s.unwrap() as f32 / i16::MAX as f32)
        .collect();

    let encoder = "sherpa-onnx-whisper-tiny/tiny-encoder.onnx".into();
    let decoder = "sherpa-onnx-whisper-tiny/tiny-decoder.onnx".into();
    let mut extractor = language_id::SpokenLanguageId::new(encoder, decoder, None, None, None);

    let language = extractor.compute(samples, sample_rate)?;
    println!("Spoken language: {}", language);

    Ok(())
}