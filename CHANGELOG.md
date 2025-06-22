# Changelog

## [0.2.27] - Unreleased

- The support for downloading certain dynamic streams (“live” manifests) has been improved. More
  specifically, for `$Number$`-based dynamic streams the calculation of segment numbers now accounts
  for the difference between `@availabilityStartTime` and the current time, so will download content
  starting from when the download is started.

- New support for decrypting streams with ContentProtection using the MP4Box commandline application
  from GPAC. This provides an alternative to using mp4decrypt and Shaka packager. The MP4Box
  application does not support decryption of content in WebM containers, and sometimes rejects
  content which is accepted by mp4decrypt and Shaka packager, but may sometimes be more convenient
  for users if it’s already installed.


## [0.2.26] - 2025-03-30

- New commandline option `--base-url` that allows you to specify a Base URL to be used for all
  segment downloads. This overrides any `BaseURL` element provided in the DASH MPD. This option may
  be useful when downloading from a manifest specified as a file:// URL, which does not contain a
  `BaseURL` element.

- The reported download bandwidth, and the updating of the progress bar, should be more reliable for
  streams that are composed of a large number of very small segments.


## [0.2.25] - 2025-03-16

- HTTP requests will now try to establish HTTP/2 connections if the functionality is advertised by
  an HTTP server, using the `Upgrade` header. Disable this by building without the `http2` feature
  on our `dash-mpd-rs` dependency.

- ffmpeg muxing support supports the use of the `DASHMPD_PERSIST_FILES` environment variable to retain
  the temporary files created during muxing.

- The ffmpeg demuxer concat helper uses absolute paths in the ffconcat file, rather than relative
  paths, because ffmpeg interprets relative paths with respect to the location of the ffconcat file,
  rather than with respect to CWD. Reported by @Cocalus.


## [0.2.24] - 2025-01-12

- This release only includes updates to our crate dependencies. It should not lead to any
  user-visible changes in behaviour.

- Cargo.lock file committed to our GitHub repository for packages, as requested by @al3xtjames.


## [0.2.23] - 2024-09-08

- Add the ability to download from file:// URLs. This requires the MPD manifest to specify an
  absolute BaseURL element at the MPD, Period or Representation level, or to use absolute URLS for
  all media segments.

- Add support for the ffmpeg “concat demuxer” as a concatenation helper for multiperiod manifests,
  as an alternative to the existing ffmpeg “concat filter” support. To use this concatenation
  helper, all Periods must use the same encoding (same codecs, same time base, etc.), though they
  may be wrapped in different container formats. This concatenation helper is very fast because it
  copies the media streams, rather than reencoding them.

  In a typical use case of a multi-period DASH manifest with DAI (where Periods containing
  advertising have been intermixed with Periods of content), for which it is possible to drop the
  advertising segments by using `--minimum_period_duration` or using an XSLT/XPath filter on Period
  elements, the content segments are likely to all use the same codecs and encoding parameters, so
  this concat helper should work well.

  Use it with `--concat-preference mp4:ffmpegdemuxer` for example.

- Fix an additional possible off-by-one error in calculating the segment count for `$Time$`-based
  SegmentTemplate manifests.

- Fix duplicated merge of BaseURLS for video segments. Patch from @jonasgrosch.

- Log additional diagnostics information when the verbosity level is greater than 0 or 1 for
  external commands run for muxing, concatenating, subtitle extraction/merging, and decrypting. The
  logged information includes the full commandline.

- Fixed: the concatenation of multiperiod manifests using the ffmpeg concat filter was erroneously
  adding an empty audio track when none of the input Periods contained audio.


## [0.2.22] - 2024-08-26

Improvements to the handling of subtitles: we make additional efforts to extract STPP subtitles from
a sequence of fMP4 segments, as a `.ttml` file. ffmpeg does not currently seem to be able to extract
this in the more commonly supported SRT format. When saving to a Matroska container (`.mkv` or
`.webm` output files), we attempt to embed subtitle tracks with mkvmerge instead of with MP4Box
(which fails).

Fix off-by-one bug in the calculation of the number of media fragments to download when using
SegmentTemplate addressing with `$Number$`. The initialiation segment was being counted towards the
number of segments, but should not be. Bug reported by @vidasi.

When the audio and video tracks are unsynchronized due to a difference in their startTime attribute,
we attempt to fix this desynchronization during muxing. This is a rare problem in the wild and has
not been heavily tested. The fix is currently only implemented when using ffmpeg as a muxer
application (uses the `-itsoffset` commandline option).


## [0.2.21] - 2024-07-27

- The progress bar will be updated more frequently, and more reliably when segment sizes are small
  and network speeds are high (suggestion from @filiptibell).

- Move from the unmaintained `bench_scraper` crate to the `decrypt-cookies` crate for access to cookies
  from your web browser. This crate also supports a wider range of web browsers.

- Move to v0.23 of the rustls crate for various performance and security improvements to TLS
  connection handling.

- Update the `protobuf-src` crate which was causing builds to fail, and move to current versions of
  crates in the axum ecosystem used for tests.


## [0.2.20] - 2024-06-08

- Add support for concatenating streams in multi-period manifests using mkvmerge, as an alternative
  to the existing support for concatenation using ffmpeg. The preference ordering for concatenation
  helpers is specified by commandline argument `--concat-preference`, which works similarly to the
  existing `--muxer-preference` commandline argument.

  Concatenation using mkvmerge works at least with MP4 and with Matroska (.mkv) containers. It tends
  to be much faster than using ffmpeg but also less robust (less tolerant of the variety of media
  encoding specificities found in the wild). You can specify multiple concatenation helpers, in
  which case they will be called sequentially until one of them returns a success code.

- Allow the user to specify a preference for streams based on the value of the `Role` element in an
  `AdaptationSet`. Streaming services sometimes publish various additional streams marked with roles
  such as `alternate` or `supplementary` or `description`, in addition to the main stream which is
  generally labelled with a role of `main`. The user can specify a preference order for these role
  annotations, which is applied after the language preference and before the width/height/quality
  preference.

- Fix a bug in concatenation for multiperiod manifests that occurred when one of the Periods does not
  contain any audio.

- Accomodate manifests which declare that a Representation has `mimeType="video/mp4"` and
  `codecs="mp4a"` (or some other audio codec). These Representations are now detected as audio
  streams rather than as video streams.


## [0.2.19] - 2024-05-21

- A new commandline argument `--minimum_period_duration` whose argument is a number of seconds.
  Periods in the manifest whose duration is less than this value will not be downloaded. This may be
  useful to avoid overloading the servers that deliver advertising segments that are sliced into the
  content of interest.

- Fix a bug in the concatenation of multiperiod manifests. When per-Period files contained both
  audio and video content, the content was being included twice in the concatenated file.

- MacOS release binaries are now universal binaries, rather than Aarch64 ("Apple Silicon") binaries.


## [0.2.18] - 2024-05-09

- Fix bug in filename handling when using the ffmpeg concatenation filter, which is used for
  multiperiod manifests when the technical characteristics of the different periods make it possible
  to concatenate them without reencoding. Filenames were not properly escaped when passed as
  arguments to the `filter_complex` commandline argument.

- Add support for subtitles that use SegmentBase addressing.

- Subtitles in STPP format (a data stream in MP4 fragments) are now converted to TTML format. The
  XML data stream is extracted using ffmpeg. If the conversion is successful it will be saved to a
  file with the same name as the output file, but with a `.ttml` extension.


## [0.2.17] - 2024-04-15

- Network requests for media fragments that fail are retried a certain number of times. The number
  of retries for each fragment request can be set using the `--fragment-retries` commandline
  argument (default is 10). Network errors that are identified as being transient (for
  example, network timeouts) do not count towards this retry count. Network requests were previously
  retried only if they were identified as transient, but anecdotally it seems that the internet and
  CDN servers are not set up in a way that allows transient errors reliably to be distinguished from
  non-transient errors. Non-transient retries still count towards the `max_error_count`, whose default
  value is increased to 30.

- Error messages include additional contextual information. For example, a network connection error
  caused by a TLS configuration error will include information on the underlying issue.

- The prebuilt software container `ghcr.io/emarsden/dash-mpd-cli` is now built also for
  Linux/ppc64le.


## [0.2.16] - 2024-03-30

- Improvements to error reporting. Network timeouts and connection errors will now be reported as
  different error types. Additional contextual information regarding the underlying source of an
  error will be printed. For example, a network connection error caused by a TLS configuration error
  will include information on the underlying issue.

- Updates to our dependencies (in particular the reqwest crate used for HTTP requests) should allow
  improved compatibility and performance.


## [0.2.15] - 2024-02-24

- Fix the handling of the `--referer` commandline option, which is now used in all network
  requests (bug reported by @yoyo890121).


## [0.2.14] - 2024-02-18

- The referer header specified using the `--referer` commandline option is now used in all network
  requests, including requests for media segments. Previously, the referer specified on the
  commandline was only used to retrieve the MPD manifest, and the referer header used in network
  requests for media segments was the URL of the MPD manifest, updated to account for any HTTP
  redirects and for use of the DASH `Location` redirect functionality. The new behaviour allows the
  user to mimic the behaviour of a web browser which is playing media embedded in a web page.

- Cookies set while retrieving the MPD manifest will be included in requests for media segments.
  In practice, media servers rarely check cookies, as doing so is expensive on CDN infrastructure,
  but this should help to mimic the behaviour of a web browser which is playing media embedded in a
  web page.

- Fix handling of XLinked elements when remote XML fragment contains multiple elements.


## [0.2.13] - 2024-02-04

- Fix the handling of XLinked elements. The Shaka heliocentrism test case now works correctly.

- Widevine and PlayReady initialization data will now be decoded and pretty printed, alongside their
  Base64 representation (uses the new `pssh-box` crate).

- Fix concatenation for multiperiod manifests in situations where one period has audio and another
  has no audio track.


## [0.2.12] - 2023-12-25

- The `tracing` crate is now used for all logging purposes. Logged messages can be controlled using
  the `RUST_LOG` environment variable. As previously, warning and error messages are printed to
  stderr, and other informative messages to stdout, but they will now be prefixed by a timestamp.

- The Docker container at `ghcr.io/emarsden/dash-mpd-cli` is now also available for linux/arm/v7
  (32-bit ARM) and linux/riscv64, in addition to linux/amd64 and linux/arm64.

- Fix bug in the handling of toplevel `Period.SegmentTemplate` elements (rarely present in the wild,
  but allowed by the DASH specification).

- When deciding whether downloaded video files can be concatenated using the ffmpeg concat muxer, we
  tolerate missing sar metainformation (not always present in MP4 containers in the wild).


## [0.2.11] - 2023-12-09

- New commandline argument `--drop-elements` which takes an XPath expression as argument. XML
  elements in the MPD manifest that match this XPath expression will be removed from the manifest
  before downloading. This may be useful to help select an audio track based on attributes such as
  its role or label, or to avoid overloading the servers that serve advertising content.

- Include the query component of the MPD URL in requests for media segments, to support the
  token-based authentication used by some streaming services. If the manifest URL is
  `https://example.com/manifest.mpd?token=foo`, requests to segments will look like
  `/segment/42.m4v?token=foo`, unless the manifest includes an explicit query component in the
  segment URLs.

- Muxing to a WebM container using the VLC external muxer should be fixed.


## [0.2.10] - 2023-11-28

- A [user manual](https://emarsden.github.io/dash-mpd-cli/) is available on GitHub pages.

- dash-mpd-cli can be run in a Podman/Docker container, packaged on the GitHub Container Registry at
  `ghcr.io/emarsden/dash-mpd-cli`. The container conveniently includes most of the external helper
  applications (ffmpeg, MP4Box, mkvmerge, shaka-packager, mp4decrypt, etc.). It’s a multiarch
  container, currently packaged for linux/amd64 and linux/arm64. See the user manual for details on
  running in a container.

- The current download bandwidth is displayed in the progress bar, if it is activated.

- Fix the calculation of audio segments to be downloaded for a live stream (dynamic manifest) for
  which `--force_duration` has been specified.


## [0.2.9] - 2023-11-18

- Add the possibility to use the Shaka packager application for decryption of media with Content Protection,
  as an alternative to mp4decrypt. The shaka-packager application is able to handle more media
  formats (e.g. WebM/Matroska containers) and is better maintained than mp4decrypt. See the
  commandline arguments `--decryption-application` and `--shaka-packager-location`.

- New commandline argument `--enable-live-streams` that makes it possible to attempt to download
  from a live (dynamic) manifest. Downloading from a genuinely live stream won’t work well, because
  we don’t implement the clock-related throttling needed to only download media segments when they
  become available. However, some media sources publish pseudo-live streams where all media segments
  are in fact available (they don’t update the manifest once the live is complete), which we will be
  able to download. You might also have some success in combination with the `--sleep-requests`
  commandline argument.

- New commandline argument `--force-duration` which makes it possible to specify the number of
  seconds of content to download from the DASH stream. This may be necessary when using
  `--enable-live-streams`, because live streams often don’t specify a duration. It can also be used
  to download only the first part of a normal (static) stream.

- Fix the selection of the desired Representation (according to the user’s quality/resolution
  preferences) for DASH manifests that include multiple AdaptationSets. This is the case on some
  manifests that offer media streams using different codecs. We were previously only examining
  Representation elements in the first AdaptationSet present in the manifest.


## [0.2.8] - 2023-11-04

- Add preliminary support for applying rewrite rules to the MPD manifest before downloading media
  segments. Rewrite rules are expressed as XSLT stylesheets that are applied to the manifest using
  the `xsltproc` commandline tool (which supports XSLT v1.0). This allows complex rewrite rules to
  be expressed using a standard (if a little finicky) stylesheet language. See the `--xslt-stylesheet`
  commandline option.

  This functionality and API are experimental, and may evolve to use a different XSLT processor, such as
  Saxon-HE (https://github.com/Saxonica/Saxon-HE/) which has support for XSLT v3.0, but is
  implemented in Java. Alternatively, a more general filtering functionality based on WASM bytecode
  might be implemented to allow the implementation of rewrite rules in a range of languages that can
  compile to WebAssembly.

- Change the default ordering of muxer applications when saving media to a .webm container to prefer
  VLC over ffmpeg. With the commandline arguments that we use, ffmpeg does not automatically
  reencode content to a codec that is allowed by the WebM specification, whereas VLC does do so.

- Some limited DASH conformity checks will be run on manifests before downloading, which may
  generate warnings written to stderr. A surprising number of manifests, including some
  generated by the most widely used commercial streaming software, feature non-conformities such as
  incorrect values of @maxWidth / @maxHeight or inserted advertising segments that don't respect
  @maxSegmentDuration).


## [0.2.7] - 2023-10-15

- Allow the user to specify the order in which muxer applications are tried, instead of using a
  hard-coded ordering per container type. The ordering is specified per container type ("mkv",
  "mp4", "avi", "ts", etc.). The user specifies an ordering such as "ffmpeg,vlc,mp4box" which means
  that ffmpeg is tried first, and if that fails vlc, and if that fails mp4box. The muxers currently
  available are ffmpeg, vlc, mkvmerge and mp4box. See commandline arguemnt `--muxer-preference`.

- Work around a bug in VLC, which does not correctly report failure to mux via a non-zero exit code.


## [0.2.6] - 2023-09-30

- New commandline argument `--auth-bearer` to specify the token to be used for Bearer authentication
  of network requests to retrieve the manifest and the media segments. This is the authentication
  method specified in RFC 6750, originally designed for OAuth 2.0, but also used in other settings
  such as JSON Web Tokens (JWT).

- Enable support for MPEG-4 Part 17 (Timed Text) subtitles (tx3g codec). They will be converted to
  SRT format if the MP4Box commandline application is installed.

- When printing the available media streams, print `Role` and `Label` information if they are
  specified on an `AdaptationSet` element.

- Fix handling of `MPD.Location` field (thanks to @nissy34).


## [0.2.5] - 2023-09-03

- New commandline arguments `--prefer-video-width` and `--prefer-video-height` which allow the user
  to specify the video stream to be downloaded, when multiple video streams with different
  resolutions are made available. The video stream with the horizontal (respectively vertical)
  resolution closest to the specified width (respectively height) is chosen. This preference only
  concerns the video stream; use the `--quality` commandline argument to specify the preferred audio
  stream when multiple audio streams with different quality levels are available. If a preference
  for both video width and video height is provided, the preferred width is used. A width or height
  preference overrides (for the video stream) a specified quality preference.

- New value `intermediate` recognized for the `--quality` commandline argument. If the DASH manifest
  specifies several Adaptations with different bitrates or quality levels (specified by the
  `@qualityRanking` attribute in the manifest -- quality ranking may differ from bandwidth
  ranking when different codecs are used), prefer the Adaptation with an intermediate bitrate
  (closest to the median value).

- New commandline arguments `--auth-username` and `--auth-password` to specify the username and
  password to be used for authentication with the server. Currently, only HTTP Basic authentication
  is supported.

- Improve support for selecting the output container format based on its filename extension.
  Selecting an output file with an `.mkv` extension will now produce an output file in Matroska
  container format, even in cases where the manifest only contains a video stream or only an audio
  stream (shortcircuiting the muxing functionality). In these cases, the stream will be copied if
  the output container requested is compatible with the downloaded stream format, and otherwise a
  new media container with the requested format will be created and the audio or video stream will
  be inserted (and reencoded if necessary) into the output file. This insertion and reencoding is
  undertaken by the same commandline applications used for muxing: ffmpeg, mkvmerge, mp4box
  (currently not vlc).


## [0.2.4] - 2023-08-14

- New commandline argument `--header` (alias `-H`) which is compatible with cURL. This can be
  convenient when using “Copy as cURL” functionality in Chromium DevTools. The syntax for the
  argument is slightly different from the existing `--add-header` commandline argument.

- On startup, check whether a newer version is available as a GitHub release, unless the
  `--no-version-check` commandline option is enabled.

- Improve support for multiperiod manifests. When the contents of the different periods
  can be joined into a single output container (because they share the same resolution, frame rate
  and aspect ratio), we concatenate them using ffmpeg (with reencoding in case the codecs in the
  various periods are different). If they cannot be joined, we save the content in output files
  named according to the requested output file (whose name is used for the first period). Names
  ressemble "output-p2.mp4" for the second period, and so on.

- New function `concatenate_periods` on `DashDownloader` to specify whether the concatenation using
  ffmpeg (which is very CPU-intensive due to the reencoding) of multi-period manifests should be
  attempted. The default behaviour is to concatenate when the media contents allow it.

- Improved support for certain addressing types on subtitles (AdaptationSet>SegmentList,
  Representation>SegmentList, SegmentTemplate+SegmentTimeline addressing modes).

- Significantly improved support for XLink semantics on elements (remote elements). In particular, a
  remote XLinked element may resolve to multiple elements (e.g. a Period with href pointing to a
  remote MPD fragment may resolve to three final Period elements), and a remote XLinked element may
  contain a remote XLinked element (the number of repeated resolutions is limited, to avoid DoS
  attacks).


## [0.2.3] - 2023-08-05

- New commandline argument `--referer` to specify the value of the Referer HTTP header. This is an
  alternative to the use of the `--add-header` commandline argument.

- Fix regression: restore printing of logged diagnostics.

- Add support for EIA-608 aka CEA-608 subtitles/closed captions.

- More diagnostics information is printed concerning the selected audio/video streams. In
  particular, pssh information will be printed for streams with ContentProtection whose pssh is
  embedded in the initialization segments rather than in the DASH manifest.


## [0.2.2] - 2023-07-16

- New commandline argument `--simulate` to retrieve the MPD manifest but not download any audio,
  video or subtitle content.

- Improve support for retrieving subtitles that are distributed in fragmented MP4 streams (in
  particular WebVTT/STPP formats).

- More diagnostics information is printed concerning the selected audio/video streams.


## [0.2.1] - 2023-07-08

- Support for decrypting encrypted media streams that use ContentProtection, via the Bento4
  mp4decrypt commandline application. See the `--key` commandline argument to allow kid:key pairs to
  be specified, and the `--mp4decrypt-location` commandline argument to specify a non-standard
  location for the mp4decrypt binary.

- Fix a bug in the handling of the `--add-header` commandline argument.


## [0.2.0] - 2023-06-25

- Incompatible change to the `--keep_audio` and `keep_video` commandline arguments, to allow
  the user to specify the path for the audio and video files. Instead of operating as flags, they
  allow the user to specify the filename to which the corresponding stream will be saved (and not
  deleted after muxing).

- New commandline argument `--client-identity-certificate` to provide a file containing a private
  key and certificate (both encoded in PEM format). These will be used to authenticate TLS network
  connections.

- Print information on the different media streams available (resolution, bitrate, codec) in a
  manifest when requested verbosity is non-zero.


## [0.1.14] - 2023-06-10

- New commandline argument `--add-root-certificate` to add an X.509 certificate to the list of root
  certificates used to check TLS connections to servers. Certificates should be provided in PEM format.

- New commandline argument `--no-proxy` to disable use of a proxy, even if related enviroment
  variables (`HTTP_PROXY` etc.) are set.

- Connection errors at the network level are handled as permanent, rather than transient, errors. In
  particular, TLS certificate verification errors will no longer be treated as transient errors.


## [0.1.13] - 2023-05-28

- New commandline argument `--cookies-from-browser` to load HTTP cookies from a web browser (support
  for Firefox, Chromium, Chrome, ChromeBeta, Edge and Safari on Linux, Windows and MacOS, via the
  bench_scraper crate). This support is gated by the `cookies` feature, which is enabled by default.
  Disable it (with `--no-default-features`) to build on platforms which are not supported by the
  bench_scraper crate, such as Android/Termux.


## [0.1.12] - 2023-05-12

- Add commandline argument `--save-fragments` to save media fragments (individual DASH audio and
  video segments) to the specified directory.

- Add commandline argument `--mp4box-location=<path>` to allow a non-standard location for the
  MP4Box binary (from the GPAC suite) to be specified.

- Update to version 0.9.0 of the dash-mpd crate, which is more tolerant of unexpected extensions to
  the DASH schema.


## [0.1.11] - 2023-05-08

- New commandline argument `--limit-rate` to throttle the network bandwidth used to download media
  segments, expressed in octets per second. The limit can be expressed with a k, M or G suffix to
  indicate kB/s, MB/s or GB/s (fractional suffixed quantities are allowed, such as `1.5M`). The
  default is not to throttle bandwidth.


## [0.1.10] - 2023-04-15

- New commandline argument `--max-error-count` to specify the maximum number of non-transient
  network errors that should be ignored before a download is aborted. This is useful in particular
  on some manifests using Time-based or Number-based SegmentLists for which the packager calculates
  a number of segments which is different to our calculation (in which case the last segment can
  generate an HTTP 404 error).

- Update to version 0.7.3 of the dash-mpd crate, which provides better handling of transient and
  non-transient network errors.

- Fix bug in the handling the value of the `--sleep-requests` commandline argument.


## [0.1.9] - 2023-03-19

- Update to version 0.7.2 of the dash-mpd crate. This provides support for downloading additional
  types of subtitles in DASH streams. This version also makes it possible to select between
  native-tls and rustls-tls TLS implementations. We build with rustls-tls in order to build static
  Linux binaries using musl-libc, and to simplify building on Android.


## [0.1.8] - 2023-01-29

- Move to async API used by version 0.7.0 of the dash-mpd crate. There should be no user-visible
  changes in this version.


## [0.1.7] - 2023-01-15

- Add commandline argument `--write-subs` to download subtitles, if they are available. Subtitles
  are downloaded to a file with the same name as the audio-video content, but a filename extension
  dependent on the subtitle format (`.vtt`, `.ttml`, `.srt`).


## [0.1.6] - 2022-11-27

- Add commandline arguments `--keep-video` and `--keep-audio` to retain the files containing video and
  audio content after muxing.

- Add commandline argument `--ignore-content-type` to disable checks that content-type of fragments
  is compatible with audio or video media (may be required for some poorly configured servers).


## [0.1.5] - 2022-10-26

- Produce release binaries for Linux/AMD64, Windows and MacOS/AMD64 using Github Actions.

- Update the version of the clap crate used to parse commandline arguments.


## [0.1.4] - 2022-09-10

- Add commandline arguments `--vlc-location=<path>` and `--mkvmerge-location=<path>` to allow
  specification of a non-standard location for the VLC and mkvmerge binaries, respectively.


## [0.1.3] - 2022-07-02

- Add commandline arguments `--audio-only` and `--video-only`, to retrieve only the audio stream, or
  only the video stream (for streams in which audio and video content are available separately).

- Add commmandline argument `--prefer-language` to allow the user to specify the preferred language
  when multiple audio streams with different languages are available. The argument must be in RFC
  5646 format (e.g. "fr" or "en-AU"). If a preference is not specified and multiple audio streams are
  present, the first one listed in the DASH manifest will be downloaded.


## [0.1.2] - 2022-06-01

- Add `--sleep-requests` commandline argument, a number of seconds to sleep between network
  requests. This provides a primitive mechanism for throttling bandwidth consumption.


## [0.1.1] - 2022-03-19

- Add `--ffmpeg-location` commandline argument, to use an ffmpeg which is not in the PATH.


## [0.1.0] - 2022-01-25

- Initial release.
