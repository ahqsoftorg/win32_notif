use std::time::Duration;

use win32_notif::{
  notification::{
    actions::{action::ActivationType, input::Selection, ActionButton, Input},
    visual::Text,
  },
  NotificationActivatedEventHandler, NotificationBuilder, ToastsNotifier,
};
// Input Selection Example

fn main() {
  let notifier = ToastsNotifier::new(Some("Microsoft.Windows.Explorer")).unwrap();

  let notification = NotificationBuilder::new()
    .visual(Text::create(0, "Enter your name"))
    .action(Input::create_selection_input(
      "0",
      "Name",
      "AHQ",
      vec![
        Selection::new("1", "AHQ Novel"),
        Selection::new("2", "AHQ Soft"),
        Selection::new("3", "Wings of Fire"),
      ],
    ))
    .action(
      ActionButton::create("Submit")
        .with_input_id("0")
        // Required or else it would go to file explorer
        .with_activation_type(ActivationType::Foreground),
    )
    .on_activated(NotificationActivatedEventHandler::new(|_notif, data| {
      let data = data.unwrap();

      println!("{:#?}", data);

      Ok(())
    }))
    // Notification will be gone after 5 secs
    .with_expiry(Duration::from_secs(5))
    // sequence: a custom defined id (never used)
    // tag, group: Unique notification identifier
    // Notifier, ofc you its the notifier
    .build(0, &notifier, "01", "input")
    .unwrap();

  notification.show().unwrap();

  // App should be running
  loop {
    std::thread::sleep(Duration::from_secs(2));
  }
}
