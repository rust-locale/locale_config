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

const MUI_LANGUAGE_NAME: winapi::c_ulong = 0x8; // from winnls.h

fn get_user_preferred_languages() -> Vec<LanguageRange<'static>> {
    let mut buf = [0u16; 5 * 85 + 1];
    let mut n_langs = 0;
    let mut len = buf.len() as winapi::c_ulong;
    let res = unsafe {
        kernel32::GetUserPreferredUILanguages(MUI_LANGUAGE_NAME, &mut n_langs, buf.as_mut_ptr(), &mut len)
    };
    if res != 0 && len > 1 {
        let s = String::from_utf16_lossy(&buf[..(len as usize - 2)]);
        return s.split('\0')
            .filter_map(|x| LanguageRange::new(x).ok())
            .map(|x| x.into_static())
            .collect();
    }
    return Vec::new();
}

// User default language is the primary language. If user preferred UI languages returns anything, the
// first item is considered override for messages and the rest is fallbacks. And last fallback is
// system default language.
pub fn system_locale() -> Option<Locale> {
    let mut loc =
        if let Ok(lr) = get_user_default_locale() {
            Some(Locale::from(lr))
        } else {
            None
        };
    let mut msg_locs = get_user_preferred_languages();
    if !msg_locs.is_empty() {
        if loc.is_none() {
            loc = Some(Locale::from(msg_locs.remove(0)));
        } else {
            loc.as_mut().unwrap().add_category("messages", &msg_locs.remove(0));
        }
        debug_assert!(!loc.is_none());
        for l in msg_locs {
            loc.as_mut().unwrap().add(&l);
        }
    }
    if let Ok(lr) = get_system_default_locale() {
        if loc.is_none() {
            loc = Some(Locale::from(lr));
        } else {
            loc.as_mut().unwrap().add(&lr);
        }
    }
    loc
}
