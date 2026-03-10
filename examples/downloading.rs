use std::{thread::sleep, time::Duration};

use win32_notif::{
  notification::{
    visual::{progress::ProgressValue, Progress, Text},
    AdaptiveText, Scenario,
  },
  NotificationBuilder, NotificationDataSet, ToastsNotifier,
};

pub fn main() {
  let notifier = ToastsNotifier::new(Some("Microsoft.Windows.Explorer")).unwrap();

  let notification = NotificationBuilder::new()
    .with_scenario(Scenario::IncomingCall)
    .with_use_button_style(true)
    .visual(Text::create_binded(1, "status"))
    .visual(Progress::create(
      AdaptiveText::BindTo("typeof"),
      ProgressValue::BindTo("value"),
    ))
    .value("status", "AHQ Store")
    .value("typeof", "Downloading...")
    .value("value", "indeterminate")
    .build(1, &notifier, "a", "ahq")
    .expect("Error");

  notification.show().expect("Not Sent");

  let data = NotificationDataSet::new().unwrap();
  for perc in 1..=100 {
    data
      .insert("value", format!("{}", perc as f32 / 100.0).as_str())
      .unwrap();

    _ = notifier.update(&data, "ahq", "a");

    sleep(Duration::from_millis(100));
  }
}
