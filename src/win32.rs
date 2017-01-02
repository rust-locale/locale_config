extern crate winapi;
extern crate kernel32;

use super::{LanguageRange,Locale};

use std::fmt::Write;

const LOCALE_SDECIMAL: winapi::c_ulong = 0x000E;
const LOCALE_STHOUSAND: winapi::c_ulong = 0x000F;
const LOCALE_SNATIVEDIGITS: winapi::c_ulong = 0x0013;

fn get_user_default_locale() -> Result<LanguageRange<'static>, &'static str> {
    let mut buf = [0u16; 85];
    let len = unsafe {
        kernel32::GetUserDefaultLocaleName(buf.as_mut_ptr(), buf.len() as i32)
    };
    if len > 0 {
        let mut s = String::from_utf16_lossy(&buf[..(len as usize - 1)]);
        let mut u_ext = String::new();
        let mut x_ext = String::new();
        if_locale_info_differs(LOCALE_SDECIMAL, |s| {
            x_ext.push_str("-ds");
            for c in s.chars() {
                write!(&mut x_ext, "-{:04x}", c as u32).unwrap(); // shouldn't fail unless OOM
            }
        });
        if_locale_info_differs(LOCALE_STHOUSAND, |s| {
            x_ext.push_str("-gs");
            for c in s.chars() {
                write!(&mut x_ext, "-{:04x}", c as u32).unwrap(); // shouldn't fail unless OOM
            }
        });
        if_locale_info_differs(LOCALE_SNATIVEDIGITS, |s| {
            u_ext.push_str(match s.as_ref() {
                // basic plane numeric numberingSystems from CLDR
                "٠١٢٣٤٥٦٧٨٩" => "-nu-arab",
                "۰۱۲۳۴۵۶۷۸۹" => "-nu-arabext",
                "᭐᭑᭒᭓᭔᭕᭖᭗᭘᭙" => "-nu-bali",
                "০১২৩৪৫৬৭৮৯" => "-nu-beng",
                "꩐꩑꩒꩓꩔꩕꩖꩗꩘꩙" => "-nu-cham",
                "०१२३४५६७८९" => "-nu-deva",
                "０１２３４５６７８９" => "-nu-fullwide",
                "૦૧૨૩૪૫૬૭૮૯" => "-nu-gujr",
                "੦੧੨੩੪੫੬੭੮੯" => "-nu-guru",
                "〇一二三四五六七八九" => "-nu-hanidec",
                "꧐꧑꧒꧓꧔꧕꧖꧗꧘꧙" => "-nu-java",
                "꤀꤁꤂꤃꤄꤅꤆꤇꤈꤉" => "-nu-kali",
                "០១២៣៤៥៦៧៨៩" => "-nu-khmr",
                "೦೧೨೩೪೫೬೭೮೯" => "-nu-knda",
                "᪀᪁᪂᪃᪄᪅᪆᪇᪈᪉" => "-nu-lana",
                "᪐᪑᪒᪓᪔᪕᪖᪗᪘᪙" => "-nu-lanatham",
                "໐໑໒໓໔໕໖໗໘໙" => "-nu-laoo",
                "0123456789" => "-nu-latn",
                "᱀᱁᱂᱃᱄᱅᱆᱇᱈᱉" => "-nu-lepc",
                "᥆᥇᥈᥉᥊᥋᥌᥍᥎᥏" => "-nu-limb",
                "൦൧൨൩൪൫൬൭൮൯" => "-nu-mlym",
                "᠐᠑᠒᠓᠔᠕᠖᠗᠘᠙" => "-nu-mong",
                "꯰꯱꯲꯳꯴꯵꯶꯷꯸꯹" => "-nu-mtei",
                "၀၁၂၃၄၅၆၇၈၉" => "-nu-mymr",
                "႐႑႒႓႔႕႖႗႘႙" => "-nu-mymrshan",
                "߀߁߂߃߄߅߆߇߈߉" => "-nu-nkoo",
                "᱐᱑᱒᱓᱔᱕᱖᱗᱘᱙" => "-nu-olck",
                "୦୧୨୩୪୫୬୭୮୯" => "-nu-orya",
                "꣐꣑꣒꣓꣔꣕꣖꣗꣘꣙" => "-nu-saur",
                "᮰᮱᮲᮳᮴᮵᮶᮷᮸᮹" => "-nu-sund",
                "᧐᧑᧒᧓᧔᧕᧖᧗᧘᧙" => "-nu-talu",
                "௦௧௨௩௪௫௬௭௮௯" => "-nu-tamldec",
                "౦౧౨౩౪౫౬౭౮౯" => "-nu-telu",
                "๐๑๒๓๔๕๖๗๘๙" => "-nu-thai",
                "༠༡༢༣༤༥༦༧༨༩" => "-nu-tibt",
                "꘠꘡꘢꘣꘤꘥꘦꘧꘨꘩" => "-nu-vaii",
                // I don't think Windows can configure anything else, but just in case
                _ => "",
            })
        });
        // TODO: Other items configurable in Country & Language control panel.
        if !u_ext.is_empty() {
            s.push_str("-u");
            s.push_str(&*u_ext);
        }
        if !x_ext.is_empty() {
            s.push_str("-x");
            s.push_str(&*x_ext);
        }
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

fn if_locale_info_differs<F: FnOnce(&str)>(lc_type: winapi::c_ulong, func: F) {
    #[allow(non_snake_case)] // would be const if it wasn't for the fact const functions are still unstable
    let LOCALE_NAME_USER_DEFAULT: *mut u16 = ::std::ptr::null_mut();
    const LOCALE_NOUSEROVERRIDE: winapi::c_ulong = 0x80000000;
    let mut buf_user = [0u16; 86];
    let mut buf_def = [0u16; 86];
    let len_user = unsafe {
        kernel32::GetLocaleInfoEx(LOCALE_NAME_USER_DEFAULT, lc_type,
                                  buf_user.as_mut_ptr(), buf_user.len() as winapi::c_long)
    };
    let len_def = unsafe {
        kernel32::GetLocaleInfoEx(LOCALE_NAME_USER_DEFAULT, lc_type | LOCALE_NOUSEROVERRIDE,
                                  buf_def.as_mut_ptr(), buf_user.len() as winapi::c_long)
    };
    if buf_user[0..(len_user as usize - 1)] != buf_def[0..(len_def as usize - 1)] {
        let s = &*String::from_utf16_lossy(&buf_user[0..(len_user as usize - 1)]);
        func(&*s);
    }
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
