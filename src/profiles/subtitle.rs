use super::ProfileContext;
use super::ProfileType;
use super::StreamType;
use super::TranscodingProfile;

pub struct WebvttTranscodeProfile;

impl TranscodingProfile for WebvttTranscodeProfile {
    fn profile_type(&self) -> ProfileType {
        ProfileType::Transcode
    }

    fn stream_type(&self) -> StreamType {
        StreamType::Subtitle
    }

    fn build(&self, ctx: ProfileContext) -> Option<Vec<String>> {
        let args = vec![
            "-y".into(),
            "-i".into(),
            ctx.file,
            "-map".into(),
            format!("0:{}", ctx.stream),
            "-f".into(),
            "webvtt".into(),
            ctx.outdir
        ];

        Some(args)
    }

    fn supports(&self, codec_in: &str, codec_out: &str) -> bool {
        codec_out == "webvtt" && ["srt", "ass"].contains(&codec_in)
    }

    fn tag(&self) -> &str {
        "webvtt"
    }
}
