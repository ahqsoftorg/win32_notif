use win32_notif::{
  notification::{
    actions::ActionButton,
    group::{Group, SubGroup},
    visual::{
      text::{HintAlign, HintStyle},
      Text,
    },
    Scenario,
  },
  NotificationBuilder, ToastsNotifier,
};

pub fn main() {
  let notifier = ToastsNotifier::new::<&str>(None).unwrap();

  let notification = NotificationBuilder::new()
    .with_scenario(Scenario::IncomingCall)
    .with_use_button_style(true)
    .visual(
      Group::new()
        .with_subgroup(
          SubGroup::new().with_visual(Text::create(0, "Hello World").with_style(HintStyle::Title)),
        )
        .with_subgroup(
          SubGroup::new().with_visual(
            Text::create(0, "Hello World x2")
              .with_style(HintStyle::Header)
              .with_align(HintAlign::Right),
          ),
        ),
    )
    .action(
      ActionButton::create("Answer")
        .with_tooltip("Answer")
        .with_id("answer"),
    )
    .build(1, &notifier, "a", "ahq")
    .expect("Error");

  notification.show().expect("Not Sent");
}
