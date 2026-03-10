use std::time::Duration;

use win32_notif::{
  notification::{
    audio::{Audio, Src},
    visual::Text,
    ToastDuration,
  },
  NotificationBuilder, ToastsNotifier,
};

fn main() {
  let notifier = ToastsNotifier::new(Some("Microsoft.Windows.Explorer")).unwrap();

  let notif = NotificationBuilder::new()
    .audio(Audio::new(Src::Reminder, true, false))
    .visual(Text::create(
      1,
      "This will automatically vanish (exp: 10secs)!",
    ))
    .with_duration(ToastDuration::Short)
    .with_expiry(Duration::from_secs(10))
    .build(0, &notifier, "01", "ahq")
    .expect("Unable to build notification");

  _ = notif.show();
}
