use std::{env::current_exe, path::absolute, thread::sleep, time::Duration};

use win32_notif::{
  notification::visual::Text,
  registration::{unregister_aumid, RegistrationBuilder},
  NotificationBuilder, ToastsNotifier,
};

fn main() {
  let mut icon_path = current_exe().expect("Unable to find exe location");

  icon_path.pop();
  icon_path.pop();
  icon_path.pop();
  icon_path.pop();
  icon_path.push("examples");
  icon_path.push("ahq.png");

  let icon_path = absolute(icon_path).unwrap();
  let icon = icon_path.to_string_lossy();
  let icon = icon.as_ref();

  RegistrationBuilder::new("com.ahqsoftwares.ahqlogo")
    .with_display_name("Akshanabha Chakraborty")
    .with_icon_path(icon)
    .register()
    .expect("Unable to register");

  {
    let notifier = ToastsNotifier::new(Some("com.ahqsoftwares.ahqlogo")).unwrap();

    NotificationBuilder::new()
      .visual(Text::create(0, "Hello everyone!"))
      .build(0, &notifier, "main", "main")
      .unwrap()
      .show()
      .unwrap();

    sleep(Duration::from_secs(5));
  }

  unregister_aumid("com.ahqsoftwares.ahqlogo").expect("Unable to cleanup!");
}
