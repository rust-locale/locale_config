extern crate winapi;
extern crate kernel32;

use super::{LanguageRange,Locale};

fn get_user_default_locale() -> Result<LanguageRange<'static>, &'static str> {
    let mut buf = [0u16; 85];
    let len = unsafe {
        kernel32::GetUserDefaultLocaleName(buf.as_mut_ptr(), buf.len() as i32)
    };
    if len > 0 {
        let s = String::from_utf16_lossy(&buf[..(len as usize - 1)]);
        return LanguageRange::new(&*s).map(|x| x.into_static());
    }
    // TODO: Fall back to GetUserDefaultLCID and/or GetLocaleInfoW
    // GetLocaleInfo(Ex) with LOCALE_SISO639LANGNAME and LOCALE_SISO3166CTRYNAME might be of some
    // utility too
    return Err("Could not detect locale");
}

fn get_system_default_locale() -> Result<LanguageRange<'static>, &'static str> {
    let mut buf = [0u16; 85];
    let len = unsafe {
        kernel32::GetSystemDefaultLocaleName(buf.as_mut_ptr(), buf.len() as i32)
    };
    if len > 0 {
        let s = String::from_utf16_lossy(&buf[..(len as usize - 1)]);
        return LanguageRange::new(&*s).map(|x| x.into_static());
    }
    // TODO: Fall back to GetUserDefaultLCID and/or GetLocaleInfoW
    // GetLocaleInfo(Ex) with LOCALE_SISO639LANGNAME and LOCALE_SISO3166CTRYNAME might be of some
    // utility too
    return Err("Could not detect locale");
}

pub fn system_locale() -> Option<Locale> {
    let mut loc =
        if let Ok(lr) = get_user_default_locale() {
            Some(Locale::from(lr))
        } else {
            None
        };
    if let Ok(lr) = get_system_default_locale() {
        if loc.is_none() {
            loc = Some(Locale::from(lr));
        } else {
            loc.iter_mut().next().unwrap().add(&lr);
        }
    }
    loc
}
