use std::{path::absolute, thread::sleep, time::Duration};

use win32_notif::{
  NotificationBuilder, NotificationDataSet, notification::visual::{Image, Placement, Text, image::{AdaptiveImageAlign, ImageCrop}, text::HintStyle}, notifier::ToastsNotifier
};

fn main() {
  let path = absolute("./examples/ahq.png").unwrap();
  let path = path.to_string_lossy();

  let notifier = ToastsNotifier::new(Some("Microsoft.Windows.Explorer")).unwrap();

  let notif = NotificationBuilder::new()
    .visual(
      Text::create(0, "Welcome to \"win32_notif\"!! 👋")
        .with_align_center(true)
        .with_wrap(true)
        .with_style(HintStyle::Title)
    )
    .visual(
      Text::create_binded(1, "desc")
        .with_align_center(true)
        .with_wrap(true)
        .with_style(HintStyle::Body)
    )
    .visual(
      Image::create(2, format!("file:///{path}").as_str())
        .with_align(AdaptiveImageAlign::Default)
        .with_alt("AHQ Logo")
        .with_crop(ImageCrop::Circle)
        .with_placement(Placement::AppLogoOverride)
    )
    .value("desc", "Data binding works as well {WOW}!")
    .build(0, &notifier, "01", "readme")
    .unwrap();

  notif.show()
    .unwrap();

  sleep(Duration::from_secs(1));

  let data = NotificationDataSet::new().unwrap();

  data.insert("desc", "Hello, the message is edited").unwrap();

  notifier.update(&data, "readme", "01").unwrap();
}