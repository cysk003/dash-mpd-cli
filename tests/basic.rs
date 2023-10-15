/// Tests for basic download functionality.
///
/// We don't run these tests on CI infrastructure, because they consume non-negligeable network
/// bandwidth.
///
// To run tests while enabling printing to stdout/stderr
//
//    cargo test --test basic -- --show-output


use std::fs;
use std::env;
use std::process::Command;
use std::path::PathBuf;
use ffprobe::ffprobe;
use file_format::FileFormat;


// We tolerate significant differences in final output file size, because as encoder performance
// changes in newer versions of ffmpeg, the resulting file size when reencoding may change
// significantly.
fn check_file_size_approx(p: &PathBuf, expected: u64) {
    let meta = fs::metadata(p).unwrap();
    let ratio = meta.len() as f64 / expected as f64;
    assert!(0.9 < ratio && ratio < 1.1, "File sizes: expected {}, got {}", expected, meta.len());
}


#[test]
fn test_dl_mp4 () {
    if env::var("CI").is_ok() {
        return;
    }
    let mpd = "https://cloudflarestream.com/31c9291ab41fac05471db4e73aa11717/manifest/video.mpd";
    let outpath = env::temp_dir().join("cf.mp4");
    let cli = Command::new("cargo")
        .args(["run", "--no-default-features", "--",
               "-v",
               "--quality", "worst",
               "--write-subs",
               "-o", &outpath.to_string_lossy(), mpd])
        .output()
        .expect("failed spawning cargo run / dash-mpd-cli");
    assert!(cli.status.success());
    check_file_size_approx(&outpath, 60_939);
    let format = FileFormat::from_file(outpath.clone()).unwrap();
    assert_eq!(format, FileFormat::Mpeg4Part14Video);
}


#[test]
fn test_dl_mp4a () {
    if env::var("CI").is_ok() {
        return;
    }
    let mpd = "https://dash.akamaized.net/dash264/TestCases/3a/fraunhofer/aac-lc_stereo_without_video/Sintel/sintel_audio_only_aaclc_stereo_sidx.mpd";
    let outpath = env::temp_dir().join("sintel-audio.mp4");
    let cli = Command::new("cargo")
        .args(["run", "--no-default-features", "--",
               "-o", &outpath.to_string_lossy(), mpd])
        .output()
        .expect("failed spawning cargo run / dash-mpd-cli");
    assert!(cli.status.success());
    check_file_size_approx(&outpath, 7_456_334);
    let format = FileFormat::from_file(outpath.clone()).unwrap();
    assert_eq!(format, FileFormat::Mpeg4Part14Audio);
    let meta = ffprobe(outpath.clone()).unwrap();
    assert_eq!(meta.streams.len(), 1);
    let audio = &meta.streams[0];
    assert_eq!(audio.codec_type, Some(String::from("audio")));
    assert_eq!(audio.codec_name, Some(String::from("aac")));
    assert!(audio.width.is_none());
}


#[test]
fn test_dl_audio_flac () {
    if env::var("CI").is_ok() {
        return;
    }
    let mpd = "http://rdmedia.bbc.co.uk/testcard/vod/manifests/radio-flac-en.mpd";
    let outpath = env::temp_dir().join("bbcradio-flac.mp4");
    let cli = Command::new("cargo")
        .args(["run", "--no-default-features", "--",
               "-v",
               "-o", &outpath.to_string_lossy(), mpd])
        .output()
        .expect("failed spawning cargo run / dash-mpd-cli");
    assert!(cli.status.success());
    check_file_size_approx(&outpath, 81_603_640);
    let format = FileFormat::from_file(outpath.clone()).unwrap();
    assert_eq!(format, FileFormat::Mpeg4Part14Audio);
    let meta = ffprobe(outpath.clone()).unwrap();
    assert_eq!(meta.streams.len(), 1);
    let audio = &meta.streams[0];
    assert_eq!(audio.codec_type, Some(String::from("audio")));
    assert_eq!(audio.codec_name, Some(String::from("flac")));
    assert!(audio.width.is_none());
}