use anyhow::{anyhow, Context, Result};
use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::errors::Error as SymphoniaError;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use symphonia::default::{get_codecs, get_probe};

pub const TARGET_SAMPLE_RATE: u32 = 24_000;

pub struct DecodedAudio {
    pub samples: Vec<f32>,
    pub sample_rate: u32,
    pub duration_s: f32,
    pub channels: u16,
}

pub fn decode_audio_file(path: &str) -> Result<DecodedAudio> {
    let file =
        std::fs::File::open(path).with_context(|| format!("Cannot open audio file: {path}"))?;
    let media_source_stream = MediaSourceStream::new(Box::new(file), Default::default());
    let hint = build_hint(path);

    let probed = get_probe()
        .format(
            &hint,
            media_source_stream,
            &FormatOptions::default(),
            &MetadataOptions::default(),
        )
        .context("Unsupported audio format")?;

    let mut format = probed.format;
    let track = format
        .default_track()
        .ok_or_else(|| anyhow!("No audio track found"))?;
    let sample_rate = track.codec_params.sample_rate.unwrap_or(TARGET_SAMPLE_RATE);
    let track_id = track.id;

    let mut decoder = get_codecs()
        .make(&track.codec_params, &DecoderOptions::default())
        .context("Unsupported codec parameters")?;

    let mut all_samples = Vec::new();

    loop {
        let packet = match format.next_packet() {
            Ok(packet) => packet,
            Err(SymphoniaError::IoError(_)) | Err(SymphoniaError::ResetRequired) => break,
            Err(error) => return Err(error.into()),
        };

        if packet.track_id() != track_id {
            continue;
        }

        let decoded = match decoder.decode(&packet) {
            Ok(decoded) => decoded,
            Err(_) => continue,
        };

        let spec = *decoded.spec();
        let n_frames = decoded.frames();
        let mut buffer = SampleBuffer::<f32>::new(n_frames as u64, spec);
        buffer.copy_interleaved_ref(decoded);

        let channels = spec.channels.count();
        let samples = buffer.samples();

        if channels == 1 {
            all_samples.extend_from_slice(samples);
        } else {
            all_samples.extend(
                samples
                    .chunks(channels)
                    .map(|channel_samples| channel_samples.iter().sum::<f32>() / channels as f32),
            );
        }
    }

    let duration_s = all_samples.len() as f32 / sample_rate as f32;

    Ok(DecodedAudio {
        samples: all_samples,
        sample_rate,
        duration_s,
        channels: 1,
    })
}

pub fn get_audio_metadata(path: &str) -> Result<(f32, u64, String)> {
    let file = std::fs::File::open(path).with_context(|| format!("Cannot open: {path}"))?;
    let size = file.metadata()?.len();
    let media_source_stream = MediaSourceStream::new(Box::new(file), Default::default());
    let hint = build_hint(path);

    let probed = get_probe()
        .format(
            &hint,
            media_source_stream,
            &FormatOptions::default(),
            &MetadataOptions::default(),
        )
        .context("Unsupported format")?;

    let format = probed.format;
    let track = format.default_track().ok_or_else(|| anyhow!("No track"))?;
    let sample_rate = track.codec_params.sample_rate.unwrap_or(0);
    let frames = track.codec_params.n_frames.unwrap_or(0);
    let duration_s = if sample_rate > 0 {
        frames as f32 / sample_rate as f32
    } else {
        0.0
    };
    let format_name = std::path::Path::new(path)
        .extension()
        .and_then(|extension| extension.to_str())
        .unwrap_or("unknown")
        .to_uppercase();

    Ok((duration_s, size, format_name))
}

fn build_hint(path: &str) -> Hint {
    let mut hint = Hint::new();

    if let Some(extension) = std::path::Path::new(path)
        .extension()
        .and_then(|extension| extension.to_str())
    {
        hint.with_extension(extension);
    }

    hint
}
