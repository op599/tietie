//! Rich-text pasteboard read/write via NSPasteboard (objc2-app-kit).
//!
//! arboard only gives us text + image. macOS clipboard also carries
//! `public.rtf` (Rich Text Format) and `public.html` when the source
//! is Word / a browser / Notes etc. We snapshot all three so we can
//! restore formatting on paste-back.

use objc2::rc::Retained;
use objc2_app_kit::NSPasteboard;
use objc2_foundation::{NSData, NSString};

const TYPE_RTF: &str = "public.rtf";
const TYPE_HTML: &str = "public.html";
const TYPE_STRING: &str = "public.utf8-plain-text";

/// Snapshot of clipboard rich-text contents (plain is consumed via arboard
/// upstream, so we only carry the rich variants here).
pub struct RichSnapshot {
    pub html: Option<String>,
    pub rtf: Option<Vec<u8>>,
}

pub fn read_current() -> RichSnapshot {
    let pb = NSPasteboard::generalPasteboard();
    let html = pb
        .stringForType(&NSString::from_str(TYPE_HTML))
        .map(|s| s.to_string());
    let rtf: Option<Vec<u8>> = pb
        .dataForType(&NSString::from_str(TYPE_RTF))
        .map(|d| ns_data_to_vec(&d));
    RichSnapshot { html, rtf }
}

/// Write plain text + (optionally) RTF + HTML to the general pasteboard.
pub fn write_rich(plain: &str, html: Option<&str>, rtf: Option<&[u8]>) {
    let pb = NSPasteboard::generalPasteboard();
    pb.clearContents();

    if let Some(html) = html {
        pb.setString_forType(&NSString::from_str(html), &NSString::from_str(TYPE_HTML));
    }
    if let Some(rtf) = rtf {
        let data = NSData::with_bytes(rtf);
        pb.setData_forType(Some(&data), &NSString::from_str(TYPE_RTF));
    }
    // Always include plain text last so it's the canonical fallback.
    pb.setString_forType(&NSString::from_str(plain), &NSString::from_str(TYPE_STRING));
}

fn ns_data_to_vec(d: &Retained<NSData>) -> Vec<u8> {
    d.to_vec()
}
