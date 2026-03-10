//! Registration
//!
//! `win32_notif` also allows you to register your AUMID as a user-space application
//!
//! ## Caveats
//! This is only meant for registering applications under `HKEY_CURRENT_USER` and is only suitable
//! for portable applications.
//!
//! Please refrain from using this

use windows_registry::CURRENT_USER;

use crate::NotifError;

pub fn unregister_aumid<T: AsRef<str>>(aumid: T) -> Result<(), NotifError> {
  let aumid = aumid.as_ref().trim();

  if aumid.is_empty() {
    return Err(NotifError::EmptyAUMID);
  }

  let path = format!("Software\\Classes\\AppUserModelId\\{}", aumid);
  CURRENT_USER.remove_tree(path)?;

  Ok(())
}

pub struct RegistrationBuilder<'a> {
  app_id: &'a str,
  display_name: Option<&'a str>,
  icon_path: Option<&'a str>,
  icon_background: Option<&'a str>,
  com_activator_guid: Option<&'a str>,
}

macro_rules! methods {
  (
    $(
      $x:ident => $y:ident
    ),*
  ) => {
    $(
      pub fn $x(mut self, $y: &'a str) -> Self {
        self.$y = Some($y.as_ref());
        self
      }
    )*
  };
}

impl<'a> RegistrationBuilder<'a> {
  /// Creates a new [RegistrationBuilder]
  ///
  /// Your `aumid` is the AppUserModelId that windows uses to identify
  pub fn new(aumid: &'a str) -> Self {
    Self {
      app_id: aumid.trim(),
      display_name: None,
      icon_path: None,
      com_activator_guid: None,
      icon_background: None,
    }
  }

  methods! {
    with_display_name => display_name,
    with_icon_path => icon_path,
    with_icon_background => icon_background,
    with_com_activator_guid => com_activator_guid
  }

  /// Registers the app in HKCU (i.e. HKEY_CURRENT_USER)
  pub fn register(self) -> Result<(), NotifError> {
    let path = format!("Software\\Classes\\AppUserModelId\\{}", self.app_id);
    let key = CURRENT_USER.create(&path)?;

    if let Some(name) = self.display_name {
      key.set_string("DisplayName", &name)?;
    }

    if let Some(name) = self.icon_path {
      key.set_string("IconUri", &name)?;
    }

    if let Some(name) = self.icon_background {
      key.set_string("IconBackgroundColor", &name)?;
    }

    if let Some(name) = self.com_activator_guid {
      key.set_string("CustomActivator", &name)?;
    }

    Ok(())
  }
}
