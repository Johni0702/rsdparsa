extern crate rsdparsa;

#[test]
fn it_works() {
    assert_eq!(true, rsdparsa::parse_sdp("v=0\r\n", true));
}

#[test]
fn zero_length_string_fails() {
    assert_eq!(false, rsdparsa::parse_sdp("", true));
}

#[test]
fn unsupported_fails() {
    assert_eq!(false, rsdparsa::parse_sdp("v=0\r\na=unsupported", true));
}

#[test]
fn unsupported_warns() {
    assert_eq!(true, rsdparsa::parse_sdp("v=0\r\na=unsupported", false));
}

#[test]
fn version_number_one() {
    assert_eq!(false, rsdparsa::parse_sdp("v=1\r\n", true));
}

#[test]
fn version_unsupported_char() {
    assert_eq!(false, rsdparsa::parse_sdp("v=a\r\n", true));
}

#[test]
fn version_to_long() {
    assert_eq!(false, rsdparsa::parse_sdp("v=11\r\n", true));
}

#[test]
fn version_empty() {
    assert_eq!(false, rsdparsa::parse_sdp("v=\r\n", true));
}

#[test]
fn origin_works_ip4() {
    assert_eq!(true, rsdparsa::parse_sdp("v=0\r\no=mozilla 506705521068071134 0 IN IP4 0.0.0.0\r\n", true));
}

#[test]
fn origin_works_ip6() {
    assert_eq!(true, rsdparsa::parse_sdp("v=0\r\no=mozilla 506705521068071134 0 IN IP6 ::1\r\n", true));
}

#[test]
fn origin_empty() {
    assert_eq!(false, rsdparsa::parse_sdp("v=0\r\no=\r\n", true));
}

#[test]
fn origin_missing_token() {
    assert_eq!(false, rsdparsa::parse_sdp("v=0\r\no=a b c d e\r\n", true));
}

#[test]
fn origin_too_many_tokens() {
    assert_eq!(false, rsdparsa::parse_sdp("v=0\r\no=a b c d e f g\r\n", true));
}

#[test]
fn origin_unsupported_nettype() {
    assert_eq!(false, rsdparsa::parse_sdp("v=0\r\no=mozilla 506705521068071134 0 UNSUPPORTED IP4 0.0.0.0\r\n", true));
}

#[test]
fn origin_unsupported_addrtpe() {
    assert_eq!(false, rsdparsa::parse_sdp("v=0\r\no=mozilla 506705521068071134 0 IN IP1 0.0.0.0\r\n", true));
}

#[test]
fn origin_broken_ip4_addr() {
    assert_eq!(false, rsdparsa::parse_sdp("v=0\r\no=mozilla 506705521068071134 0 IN IP4 1.1.1.256\r\n", true));
}

#[test]
fn origin_broken_ip6_addr() {
    assert_eq!(false, rsdparsa::parse_sdp("v=0\r\no=mozilla 506705521068071134 0 IN IP6 ::g\r\n", true));
}

#[test]
fn origin_addr_type_mismatch() {
    assert_eq!(false, rsdparsa::parse_sdp("v=0\r\no=mozilla 506705521068071134 0 IN IP4 ::1\r\n", true));
}

#[test]
fn session_works() {
    assert_eq!(true, rsdparsa::parse_sdp("v=0\r\ns=-\r\n", true));
}

#[test]
fn session_empty() {
    assert_eq!(false, rsdparsa::parse_sdp("v=0\r\ns=\r\n", true));
}

#[test]
fn information_works() {
    assert_eq!(true, rsdparsa::parse_sdp("v=0\r\ni=foobar\r\n", true));
}

#[test]
fn information_empty() {
    assert_eq!(false, rsdparsa::parse_sdp("v=0\r\ni=\r\n", true));
}

#[test]
fn uri_works() {
    assert_eq!(true, rsdparsa::parse_sdp("v=0\r\nu=http://www.mozilla.org\r\n", true));
}

#[test]
fn uri_empty() {
    assert_eq!(false, rsdparsa::parse_sdp("v=0\r\nu=\r\n", true));
}

#[test]
fn email_works() {
    assert_eq!(true, rsdparsa::parse_sdp("v=0\r\ne=nils@mozilla.com\r\n", true));
}

#[test]
fn email_empty() {
    assert_eq!(false, rsdparsa::parse_sdp("v=0\r\ne=\r\n", true));
}

#[test]
fn phone_works() {
    assert_eq!(true, rsdparsa::parse_sdp("v=0\r\np=+123456789\r\n", true));
}

#[test]
fn phone_empty() {
    assert_eq!(false, rsdparsa::parse_sdp("v=0\r\np=\r\n", true));
}

#[test]
fn connection_works() {
    assert_eq!(true, rsdparsa::parse_sdp("v=0\r\nc=IN IP4 127.0.0.1\r\n", true));
}

#[test]
fn connection_lots_of_whitespace() {
    assert_eq!(true, rsdparsa::parse_sdp("v=0\r\nc=IN   IP4   127.0.0.1\r\n", true));
}

#[test]
fn connection_empty() {
    assert_eq!(false, rsdparsa::parse_sdp("v=0\r\nc=\r\n", true));
}

#[test]
fn connection_missing_ip() {
    assert_eq!(false, rsdparsa::parse_sdp("v=0\r\nc=IN IP4\r\n", true));
}

#[test]
fn connection_too_many_tokens() {
    assert_eq!(false, rsdparsa::parse_sdp("v=0\r\nc=IN IP4 0.0.0.0 foobar\r\n", true));
}

#[test]
fn connection_unsupported_nettype() {
    assert_eq!(false, rsdparsa::parse_sdp("v=0\r\nc=UNSUPPORTED IP4 0.0.0.0\r\n", true));
}

#[test]
fn connection_unsupported_addrtpe() {
    assert_eq!(false, rsdparsa::parse_sdp("v=0\r\nc=IN IP1 0.0.0.0\r\n", true));
}

#[test]
fn connection_broken_ip4_addr() {
    assert_eq!(false, rsdparsa::parse_sdp("v=0\r\nc=IN IP4 1.1.1.256\r\n", true));
}

#[test]
fn connection_broken_ip6_addr() {
    assert_eq!(false, rsdparsa::parse_sdp("v=0\r\nc=IN IP6 ::g\r\n", true));
}

#[test]
fn connection_addr_type_mismatch() {
    assert_eq!(false, rsdparsa::parse_sdp("v=0\r\nc=IN IP4 ::1\r\n", true));
}

#[test]
fn bandwidth_works() {
    assert_eq!(true, rsdparsa::parse_sdp("v=0\r\nb=TIAS:12345\r\n", true));
}

#[test]
fn bandwidth_empty() {
    assert_eq!(false, rsdparsa::parse_sdp("v=0\r\nb=\r\n", true));
}

#[test]
fn bandwidth_missing_bw() {
    assert_eq!(false, rsdparsa::parse_sdp("v=0\r\nb=TIAS\r\n", true));
}

#[test]
fn bandwidth_too_many_tokens() {
    assert_eq!(false, rsdparsa::parse_sdp("v=0\r\nb=TIAS:12345:xyz\r\n", true));
}

#[test]
fn bandwidth_unsupported_type() {
    assert_eq!(false, rsdparsa::parse_sdp("v=0\r\nb=UNSUPPORTED:12345\r\n", true));
}

#[test]
fn timing_works() {
    assert_eq!(true, rsdparsa::parse_sdp("v=0\r\nt=0 0\r\n", true));
}

#[test]
fn timing_empty() {
    assert_eq!(false, rsdparsa::parse_sdp("v=0\r\nt=\r\n", true));
}

#[test]
fn timing_missing_stop_time() {
    assert_eq!(false, rsdparsa::parse_sdp("v=0\r\nt=0\r\n", true));
}

#[test]
fn timing_too_many_tokens() {
    assert_eq!(false, rsdparsa::parse_sdp("v=0\r\nt=0 0 0\r\n", true));
}

#[test]
fn repeat_works() {
    // FIXME use a proper r value here
    assert_eq!(true, rsdparsa::parse_sdp("v=0\r\nr=0 0\r\n", true));
}

#[test]
fn repeat_empty() {
    assert_eq!(false, rsdparsa::parse_sdp("v=0\r\nr=\r\n", true));
}

#[test]
fn z_works() {
    // FIXME use a proper z value here
    assert_eq!(true, rsdparsa::parse_sdp("v=0\r\nz=0 0\r\n", true));
}

#[test]
fn z_empty() {
    assert_eq!(false, rsdparsa::parse_sdp("v=0\r\nz=\r\n", true));
}

#[test]
fn keys_works() {
    // FIXME use a proper k value here
    assert_eq!(true, rsdparsa::parse_sdp("v=0\r\nk=12345\r\n", true));
}

#[test]
fn keys_empty() {
    assert_eq!(false, rsdparsa::parse_sdp("v=0\r\nk=\r\n", true));
}

#[test]
fn media_audio_works() {
    assert_eq!(true, rsdparsa::parse_sdp("v=0\r\nm=audio 9 UDP/TLS/RTP/SAVPF 109\r\n", true));
}

#[test]
fn media_video_works() {
    assert_eq!(true, rsdparsa::parse_sdp("v=0\r\nm=video 9 UDP/TLS/RTP/SAVPF 126\r\n", true));
}

#[test]
fn media_application_works() {
    assert_eq!(true, rsdparsa::parse_sdp("v=0\r\nm=audio 9 UDP/TLS/RTP/SAVPF 109 9 0 8\r\n", true));
    assert_eq!(true, rsdparsa::parse_sdp("v=0\r\nm=application 9 DTLS/SCTP 5000\r\n", true));
    // FIXME
    //assert_eq!(true, rsdparsa::parse_sdp("v=0\r\nm=application 9 UDP/DTLS/SCTP webrtc-datachannel\r\n", true));
}

#[test]
fn media_empty() {
    assert_eq!(false, rsdparsa::parse_sdp("v=0\r\nm=\r\n", true));
}

#[test]
fn media_missing_token() {
    assert_eq!(false, rsdparsa::parse_sdp("v=0\r\nm=video 9 UDP/TLS/RTP/SAVPF\r\n", true));
}

#[test]
fn media_invalid_port_number() {
    assert_eq!(false, rsdparsa::parse_sdp("v=0\r\nm=video 75123 UDP/TLS/RTP/SAVPF 8\r\n", true));
}

#[test]
fn media_audio_multiple_payload() {
    assert_eq!(true, rsdparsa::parse_sdp("v=0\r\nm=audio 9 UDP/TLS/RTP/SAVPF 109 9 0 8\r\n", true));
}

#[test]
fn media_line_rejected() {
    assert_eq!(true, rsdparsa::parse_sdp("v=0\r\nm=audio 0 UDP/TLS/RTP/SAVPF 8\r\n", true));
}

#[test]
fn media_invalid_type() {
    assert_eq!(false, rsdparsa::parse_sdp("v=0\r\nm=invalid 9 UDP/TLS/RTP/SAVPF 8\r\n", true));
}

#[test]
fn media_invalid_transport() {
    assert_eq!(false, rsdparsa::parse_sdp("v=0\r\nm=audio 9 invalid/invalid 8\r\n", true));
}

#[test]
fn media_invalid_payload() {
    assert_eq!(false, rsdparsa::parse_sdp("v=0\r\nm=audio 9 UDP/TLS/RTP/SAVPF 300\r\n", true));
}

#[test]
fn parse_firefox_audio_offer() {
    assert_eq!(true, rsdparsa::parse_sdp("v=0\r\no=mozilla...THIS_IS_SDPARTA-52.0a1 506705521068071134 0 IN IP4 0.0.0.0\r\ns=-\r\nt=0 0\r\na=fingerprint:sha-256 CD:34:D1:62:16:95:7B:B7:EB:74:E2:39:27:97:EB:0B:23:73:AC:BC:BF:2F:E3:91:CB:57:A9:9D:4A:A2:0B:40\r\na=group:BUNDLE sdparta_0 sdparta_1 sdparta_2\r\na=ice-options:trickle\r\na=msid-semantic:WMS *\r\nm=audio 9 UDP/TLS/RTP/SAVPF 109 9 0 8\r\nc=IN IP4 0.0.0.0\r\na=sendrecv\r\na=extmap:1/sendonly urn:ietf:params:rtp-hdrext:ssrc-audio-level\r\na=fmtp:109 maxplaybackrate=48000;stereo=1;useinbandfec=1\r\na=ice-pwd:e3baa26dd2fa5030d881d385f1e36cce\r\na=ice-ufrag:58b99ead\r\na=mid:sdparta_0\r\na=msid:{5a990edd-0568-ac40-8d97-310fc33f3411} {218cfa1c-617d-2249-9997-60929ce4c405}\r\na=rtcp-mux\r\na=rtpmap:109 opus/48000/2\r\na=rtpmap:9 G722/8000/1\r\na=rtpmap:0 PCMU/8000\r\na=rtpmap:8 PCMA/8000\r\na=setup:actpass\r\na=ssrc:2655508255 cname:{735484ea-4f6c-f74a-bd66-7425f8476c2e}\r\n", true));
}

#[test]
fn parse_firefox_video_offer() {
    assert_eq!(true, rsdparsa::parse_sdp("v=0\r\no=mozilla...THIS_IS_SDPARTA-52.0a1 506705521068071134 0 IN IP4 0.0.0.0\r\ns=-\r\nt=0 0\r\na=fingerprint:sha-256 CD:34:D1:62:16:95:7B:B7:EB:74:E2:39:27:97:EB:0B:23:73:AC:BC:BF:2F:E3:91:CB:57:A9:9D:4A:A2:0B:40\r\na=group:BUNDLE sdparta_0 sdparta_1 sdparta_2\r\na=ice-options:trickle\r\na=msid-semantic:WMS *\r\nm=video 9 UDP/TLS/RTP/SAVPF 126 120 97\r\nc=IN IP4 0.0.0.0\r\na=recvonly\r\na=fmtp:126 profile-level-id=42e01f;level-asymmetry-allowed=1;packetization-mode=1\r\na=fmtp:120 max-fs=12288;max-fr=60\r\na=fmtp:97 profile-level-id=42e01f;level-asymmetry-allowed=1\r\na=ice-pwd:e3baa26dd2fa5030d881d385f1e36cce\r\na=ice-ufrag:58b99ead\r\na=mid:sdparta_2\r\na=rtcp-fb:126 nack\r\na=rtcp-fb:126 nack pli\r\na=rtcp-fb:126 ccm fir\r\na=rtcp-fb:126 goog-remb\r\na=rtcp-fb:120 nack\r\na=rtcp-fb:120 nack pli\r\na=rtcp-fb:120 ccm fir\r\na=rtcp-fb:120 goog-remb\r\na=rtcp-fb:97 nack\r\na=rtcp-fb:97 nack pli\r\na=rtcp-fb:97 ccm fir\r\na=rtcp-fb:97 goog-remb\r\na=rtcp-mux\r\na=rtpmap:126 H264/90000\r\na=rtpmap:120 VP8/90000\r\na=rtpmap:97 H264/90000\r\na=setup:actpass\r\na=ssrc:2709871439 cname:{735484ea-4f6c-f74a-bd66-7425f8476c2e}", true));
}

#[test]
fn parse_firefox_datachannel_offer() {
    assert_eq!(true, rsdparsa::parse_sdp("v=0\r\no=mozilla...THIS_IS_SDPARTA-52.0a2 3327975756663609975 0 IN IP4 0.0.0.0\r\ns=-\r\nt=0 0\r\na=sendrecv\r\na=fingerprint:sha-256 AC:72:CB:D6:1E:A3:A3:B0:E7:97:77:25:03:4B:5B:FF:19:6C:02:C6:93:7D:EB:5C:81:6F:36:D9:02:32:F8:23\r\na=ice-options:trickle\r\na=msid-semantic:WMS *\r\nm=application 49760 DTLS/SCTP 5000\r\nc=IN IP4 172.16.156.106\r\na=candidate:0 1 UDP 2122252543 172.16.156.106 49760 typ host\r\na=sendrecv\r\na=end-of-candidates\r\na=ice-pwd:24f485c580129b36447b65df77429a82\r\na=ice-ufrag:4cba30fe\r\na=mid:sdparta_0\r\na=sctpmap:5000 webrtc-datachannel 256\r\na=setup:active\r\na=ssrc:3376683177 cname:{62f78ee0-620f-a043-86ca-b69f189f1aea}\r\n", true));
}

#[test]
fn parse_chrome_audio_video_offer() {
    assert_eq!(true, rsdparsa::parse_sdp("v=0\r\no=- 3836772544440436510 2 IN IP4 127.0.0.1\r\ns=-\r\nt=0 0\r\na=group:BUNDLE audio video\r\na=msid-semantic: WMS HWpbmTmXleVSnlssQd80bPuw9cxQFroDkkBP\r\nm=audio 9 UDP/TLS/RTP/SAVPF 111 103 104 9 0 8 106 105 13 126\r\nc=IN IP4 0.0.0.0\r\na=rtcp:9 IN IP4 0.0.0.0\r\na=ice-ufrag:A4by\r\na=ice-pwd:Gfvb2rbYMiW0dZz8ZkEsXICs\r\na=fingerprint:sha-256 15:B0:92:1F:C7:40:EE:22:A6:AF:26:EF:EA:FF:37:1D:B3:EF:11:0B:8B:73:4F:01:7D:C9:AE:26:4F:87:E0:95\r\na=setup:actpass\r\na=mid:audio\r\na=extmap:1 urn:ietf:params:rtp-hdrext:ssrc-audio-level\r\na=sendrecv\r\na=rtcp-mux\r\na=rtpmap:111 opus/48000/2\r\na=rtcp-fb:111 transport-cc\r\na=fmtp:111 minptime=10;useinbandfec=1\r\na=rtpmap:103 ISAC/16000\r\na=rtpmap:104 ISAC/32000\r\na=rtpmap:9 G722/8000\r\na=rtpmap:0 PCMU/8000\r\na=rtpmap:8 PCMA/8000\r\na=rtpmap:106 CN/32000\r\na=rtpmap:105 CN/16000\r\na=rtpmap:13 CN/8000\r\na=rtpmap:126 telephone-event/8000\r\na=ssrc:162559313 cname:qPTZ+BI+42mgbOi+\r\na=ssrc:162559313 msid:HWpbmTmXleVSnlssQd80bPuw9cxQFroDkkBP f6188af5-d8d6-462c-9c75-f12bc41fe322\r\na=ssrc:162559313 mslabel:HWpbmTmXleVSnlssQd80bPuw9cxQFroDkkBP\r\na=ssrc:162559313 label:f6188af5-d8d6-462c-9c75-f12bc41fe322\r\nm=video 9 UDP/TLS/RTP/SAVPF 100 101 107 116 117 96 97 99 98\r\nc=IN IP4 0.0.0.0\r\na=rtcp:9 IN IP4 0.0.0.0\r\na=ice-ufrag:A4by\r\na=ice-pwd:Gfvb2rbYMiW0dZz8ZkEsXICs\r\na=fingerprint:sha-256 15:B0:92:1F:C7:40:EE:22:A6:AF:26:EF:EA:FF:37:1D:B3:EF:11:0B:8B:73:4F:01:7D:C9:AE:26:4F:87:E0:95\r\na=setup:actpass\r\na=mid:video\r\na=extmap:2 urn:ietf:params:rtp-hdrext:toffset\r\na=extmap:3 http://www.webrtc.org/experiments/rtp-hdrext/abs-send-time\r\na=extmap:4 urn:3gpp:video-orientation\r\na=extmap:5 http://www.ietf.org/id/draft-holmer-rmcat-transport-wide-cc-extensions-01\r\na=extmap:6 http://www.webrtc.org/experiments/rtp-hdrext/playout-delay\r\na=sendrecv\r\na=rtcp-mux\r\na=rtcp-rsize\r\na=rtpmap:100 VP8/90000\r\na=rtcp-fb:100 ccm fir\r\na=rtcp-fb:100 nack\r\na=rtcp-fb:100 nack pli\r\na=rtcp-fb:100 goog-remb\r\na=rtcp-fb:100 transport-cc\r\na=rtpmap:101 VP9/90000\r\na=rtcp-fb:101 ccm fir\r\na=rtcp-fb:101 nack\r\na=rtcp-fb:101 nack pli\r\na=rtcp-fb:101 goog-remb\r\na=rtcp-fb:101 transport-cc\r\na=rtpmap:107 H264/90000\r\na=rtcp-fb:107 ccm fir\r\na=rtcp-fb:107 nack\r\na=rtcp-fb:107 nack pli\r\na=rtcp-fb:107 goog-remb\r\na=rtcp-fb:107 transport-cc\r\na=fmtp:107 level-asymmetry-allowed=1;packetization-mode=1;profile-level-id=42e01f\r\na=rtpmap:116 red/90000\r\na=rtpmap:117 ulpfec/90000\r\na=rtpmap:96 rtx/90000\r\na=fmtp:96 apt=100\r\na=rtpmap:97 rtx/90000\r\na=fmtp:97 apt=101\r\na=rtpmap:99 rtx/90000\r\na=fmtp:99 apt=107\r\na=rtpmap:98 rtx/90000\r\na=fmtp:98 apt=116\r\na=ssrc-group:FID 3156517279 2673335628\r\na=ssrc:3156517279 cname:qPTZ+BI+42mgbOi+\r\na=ssrc:3156517279 msid:HWpbmTmXleVSnlssQd80bPuw9cxQFroDkkBP b6ec5178-c611-403f-bbec-3833ed547c09\r\na=ssrc:3156517279 mslabel:HWpbmTmXleVSnlssQd80bPuw9cxQFroDkkBP\r\na=ssrc:3156517279 label:b6ec5178-c611-403f-bbec-3833ed547c09\r\na=ssrc:2673335628 cname:qPTZ+BI+42mgbOi+\r\na=ssrc:2673335628 msid:HWpbmTmXleVSnlssQd80bPuw9cxQFroDkkBP b6ec5178-c611-403f-bbec-3833ed547c09\r\na=ssrc:2673335628 mslabel:HWpbmTmXleVSnlssQd80bPuw9cxQFroDkkBP\r\na=ssrc:2673335628 label:b6ec5178-c611-403f-bbec-3833ed547c09\r\n", true));
}
