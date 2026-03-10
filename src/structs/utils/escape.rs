// Manage xml character escapes
//
// Courtesy: https://docs.rs/quick-xml/0.39.2/src/quick_xml/escape.rs.html
// The code is an updated version of the above. We are thankful for the implementation

use std::borrow::Cow;
use std::fmt::{self, Write};

pub fn escape<'a>(raw: impl Into<Cow<'a, str>>) -> Cow<'a, str> {
  _escape(raw, |ch| matches!(ch, b'<' | b'>' | b'&' | b'\'' | b'\"'))
}

#[inline(always)]
fn escape_char<W>(writer: &mut W, value: &str, from: usize, to: usize) -> fmt::Result
where
  W: fmt::Write,
{
  writer.write_str(&value[from..to])?;
  match value.as_bytes()[to] {
    b'<' => writer.write_str("&lt;")?,
    b'>' => writer.write_str("&gt;")?,
    b'\'' => writer.write_str("&apos;")?,
    b'&' => writer.write_str("&amp;")?,
    b'"' => writer.write_str("&quot;")?,

    // This set of escapes handles characters that should be escaped
    // in elements of xs:lists, because those characters works as
    // delimiters of list elements
    b'\t' => writer.write_str("&#9;")?,
    b'\n' => writer.write_str("&#10;")?,
    b'\r' => writer.write_str("&#13;")?,
    b' ' => writer.write_str("&#32;")?,
    _ => unreachable!("Only '<', '>','\', '&', '\"', '\\t', '\\r', '\\n', and ' ' are escaped"),
  }
  Ok(())
}

#[inline(always)]
fn _escape<'a, F: Fn(u8) -> bool>(raw: impl Into<Cow<'a, str>>, escape_chars: F) -> Cow<'a, str> {
  let raw = raw.into();
  let bytes = raw.as_bytes();
  let mut escaped = None;
  let mut iter = bytes.iter();
  let mut pos = 0;
  while let Some(i) = iter.position(|&b| escape_chars(b)) {
    if escaped.is_none() {
      escaped = Some(String::with_capacity(raw.len()));
    }
    let escaped = escaped.as_mut().expect("initialized");
    let new_pos = pos + i;
    // SAFETY: It should fail only on OOM
    escape_char(escaped, &raw, pos, new_pos).unwrap();
    pos = new_pos + 1;
  }

  if let Some(mut escaped) = escaped {
    if let Some(raw) = raw.get(pos..) {
      // SAFETY: It should fail only on OOM
      escaped.write_str(raw).unwrap();
    }
    Cow::Owned(escaped)
  } else {
    raw
  }
}
