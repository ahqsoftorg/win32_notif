use std::{env::args, thread::sleep, time::Duration};

use win32_notif::{
  notification::{
    actions::{
      action::{ActivationType, AfterActivationBehavior},
      ActionButton,
    },
    visual::{
      text::{HintAlign, HintStyle},
      Text,
    },
  },
  NotificationBuilder, ToastsNotifier,
};

const _GUID: u128 = 23885548255760334674942869530154890271u128;

pub fn main() {
  let notifier = ToastsNotifier::new("Microsoft.Windows.Explorer").unwrap();

  let mut argv = args();

  argv.next();
  argv.next();
  argv.next();

  if let Some(_) = argv.next() {
    let notification = NotificationBuilder::new()
      .with_use_button_style(true)
      .visual(
        Text::create_binded(0, "hi")
          .with_style(HintStyle::Header)
          .with_align(HintAlign::Right),
      )
      .value("hi", "This is binded string")
      .action(
        ActionButton::create("test")
          .with_tooltip("Answer")
          .with_id("answer")
          .with_activation_type(ActivationType::Background)
          .with_after_activation_behavior(AfterActivationBehavior::PendingUpdate),
      )
      .value("test", "Hello World")
      .build(1, &notifier, "a", "ahq")
      .expect("Error");

    notification.show().expect("Not Sent");
  }

  loop {
    sleep(Duration::from_millis(10));
  }
}
