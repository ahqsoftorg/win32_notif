#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(private_bounds)]

//! Win32 Notification
//!
//! This library implements UWP XML Toast Notification
//! This is a safe wrapper around the official WinRT apis
//!
//! # Example
//! ```rust
//! use win32_notif::{
//!  notification::{
//!   actions::ActionButton,
//!   group::{Group, SubGroup},
//!   visual::{
//!     text::{HintAlign, HintStyle},
//!     Text,
//!   },
//!   Scenario,
//!  },
//!  NotificationBuilder, ToastsNotifier,
//! };
//!
//! pub fn main() {
//!  let notifier = ToastsNotifier::new(Some("Microsoft.Windows.Explorer")).unwrap();
//!
//!  let notification = NotificationBuilder::new()
//!   .with_scenario(Scenario::IncomingCall)
//!   .with_use_button_style(true)
//!   .visual(
//!     Group::new()
//!      .with_subgroup(
//!       SubGroup::new().with_visual(Text::create(0, "Hello World").with_style(HintStyle::Title)),
//!     )
//!     .with_subgroup(
//!       SubGroup::new().with_visual(
//!         Text::create(0, "Hello World x2")
//!           .with_style(HintStyle::Header)
//!           .with_align(HintAlign::Right),
//!       ),
//!     ),
//!   )
//!   .action(
//!     ActionButton::create("Answer")
//!     .with_tooltip("Answer")
//!     .with_id("answer"),
//!   )
//!   .build(1, &notifier, "a", "ahq")
//!   .expect("Error");
//!
//!   notification.show().expect("Not Sent");
//! }
//! ```

#[macro_export]
///
/// Creates a reference to a value in notification
///
/// # Example
/// ```rust
/// use win32_notif::string;
///
/// fn main() {
///     let value = string!("status");
/// }
/// ```
macro_rules! string {
    ($($x:tt)*) => {
        format!($($x)*)
    };
}

mod structs;

use std::{error::Error, fmt::Display};

pub use structs::*;

#[cfg(feature = "registration")]
pub mod registration;

macro_rules! from_impl {
  ($x:ty => $y:ident) => {
    impl From<$x> for NotifError {
      fn from(value: $x) -> Self {
        Self::$y(value)
      }
    }
  };
}

#[derive(Debug)]
pub enum NotifError {
  WindowsCore(windows::core::Error),
  DurationTooLong,
  AUMIDRequired,
  UnknownAndImpossible,
  EmptyAUMID,
}

impl Display for NotifError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl Error for NotifError {}

from_impl!(windows::core::Error => WindowsCore);
