extern crate winapi;
extern crate kernel32;

use super::{LanguageRange,Locale};

use std::fmt::Write;

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

// Locale information types from winnls.h
const LOCALE_ILANGUAGE: winapi::c_ulong =              0x0001;
const LOCALE_SLANGUAGE: winapi::c_ulong =              0x0002;
const LOCALE_SENGLANGUAGE: winapi::c_ulong =           0x1001;
const LOCALE_SENGLISHLANGUAGENAME: winapi::c_ulong =   0x1001;
const LOCALE_SABBREVLANGNAME: winapi::c_ulong =        0x0003;
const LOCALE_SNATIVELANGNAME: winapi::c_ulong =        0x0004;
const LOCALE_SNATIVELANGUAGENAME: winapi::c_ulong =    0x0004;
const LOCALE_ICOUNTRY: winapi::c_ulong =               0x0005;
const LOCALE_SCOUNTRY: winapi::c_ulong =               0x0006;
const LOCALE_SLOCALIZEDCOUNTRYNAME: winapi::c_ulong =  0x0006;
const LOCALE_SENGCOUNTRY: winapi::c_ulong =            0x1002;
const LOCALE_SENGLISHCOUNTRYNAME: winapi::c_ulong =    0x1002;
const LOCALE_SABBREVCTRYNAME: winapi::c_ulong =        0x0007;
const LOCALE_SNATIVECTRYNAME: winapi::c_ulong =        0x0008;
const LOCALE_SNATIVECOUNTRYNAME: winapi::c_ulong =     0x0008;
const LOCALE_IDEFAULTLANGUAGE: winapi::c_ulong =       0x0009;
const LOCALE_IDEFAULTCOUNTRY: winapi::c_ulong =        0x000A;
const LOCALE_IDEFAULTCODEPAGE: winapi::c_ulong =       0x000B;
const LOCALE_IDEFAULTANSICODEPAGE: winapi::c_ulong =   0x1004;
const LOCALE_IDEFAULTMACCODEPAGE: winapi::c_ulong =    0x1011;
const LOCALE_SLIST: winapi::c_ulong =                  0x000C;
const LOCALE_IMEASURE: winapi::c_ulong =               0x000D;
const LOCALE_SDECIMAL: winapi::c_ulong =               0x000E;
const LOCALE_STHOUSAND: winapi::c_ulong =              0x000F;
const LOCALE_SGROUPING: winapi::c_ulong =              0x0010;
const LOCALE_IDIGITS: winapi::c_ulong =                0x0011;
const LOCALE_ILZERO: winapi::c_ulong =                 0x0012;
const LOCALE_INEGNUMBER: winapi::c_ulong =             0x1010;
const LOCALE_SNATIVEDIGITS: winapi::c_ulong =          0x0013;
const LOCALE_SCURRENCY: winapi::c_ulong =              0x0014;
const LOCALE_SINTLSYMBOL: winapi::c_ulong =            0x0015;
const LOCALE_SMONDECIMALSEP: winapi::c_ulong =         0x0016;
const LOCALE_SMONTHOUSANDSEP: winapi::c_ulong =        0x0017;
const LOCALE_SMONGROUPING: winapi::c_ulong =           0x0018;
const LOCALE_ICURRDIGITS: winapi::c_ulong =            0x0019;
const LOCALE_IINTLCURRDIGITS: winapi::c_ulong =        0x001A;
const LOCALE_ICURRENCY: winapi::c_ulong =              0x001B;
const LOCALE_INEGCURR: winapi::c_ulong =               0x001C;
const LOCALE_SDATE: winapi::c_ulong =                  0x001D;
const LOCALE_STIME: winapi::c_ulong =                  0x001E;
const LOCALE_SSHORTDATE: winapi::c_ulong =             0x001F;
const LOCALE_SLONGDATE: winapi::c_ulong =              0x0020;
const LOCALE_STIMEFORMAT: winapi::c_ulong =            0x1003;
const LOCALE_IDATE: winapi::c_ulong =                  0x0021;
const LOCALE_ILDATE: winapi::c_ulong =                 0x0022;
const LOCALE_ITIME: winapi::c_ulong =                  0x0023;
const LOCALE_ITIMEMARKPOSN: winapi::c_ulong =          0x1005;
const LOCALE_ICENTURY: winapi::c_ulong =               0x0024;
const LOCALE_ITLZERO: winapi::c_ulong =                0x0025;
const LOCALE_IDAYLZERO: winapi::c_ulong =              0x0026;
const LOCALE_IMONLZERO: winapi::c_ulong =              0x0027;
const LOCALE_S1159: winapi::c_ulong =                  0x0028;
const LOCALE_S2359: winapi::c_ulong =                  0x0029;
const LOCALE_ICALENDARTYPE: winapi::c_ulong =          0x1009;
const LOCALE_IOPTIONALCALENDAR: winapi::c_ulong =      0x100B;
const LOCALE_IFIRSTDAYOFWEEK: winapi::c_ulong =        0x100C;
const LOCALE_IFIRSTWEEKOFYEAR: winapi::c_ulong =       0x100D;
const LOCALE_SDAYNAME1: winapi::c_ulong =              0x002A;
const LOCALE_SDAYNAME2: winapi::c_ulong =              0x002B;
const LOCALE_SDAYNAME3: winapi::c_ulong =              0x002C;
const LOCALE_SDAYNAME4: winapi::c_ulong =              0x002D;
const LOCALE_SDAYNAME5: winapi::c_ulong =              0x002E;
const LOCALE_SDAYNAME6: winapi::c_ulong =              0x002F;
const LOCALE_SDAYNAME7: winapi::c_ulong =              0x0030;
const LOCALE_SABBREVDAYNAME1: winapi::c_ulong =        0x0031;
const LOCALE_SABBREVDAYNAME2: winapi::c_ulong =        0x0032;
const LOCALE_SABBREVDAYNAME3: winapi::c_ulong =        0x0033;
const LOCALE_SABBREVDAYNAME4: winapi::c_ulong =        0x0034;
const LOCALE_SABBREVDAYNAME5: winapi::c_ulong =        0x0035;
const LOCALE_SABBREVDAYNAME6: winapi::c_ulong =        0x0036;
const LOCALE_SABBREVDAYNAME7: winapi::c_ulong =        0x0037;
const LOCALE_SMONTHNAME1: winapi::c_ulong =            0x0038;
const LOCALE_SMONTHNAME2: winapi::c_ulong =            0x0039;
const LOCALE_SMONTHNAME3: winapi::c_ulong =            0x003A;
const LOCALE_SMONTHNAME4: winapi::c_ulong =            0x003B;
const LOCALE_SMONTHNAME5: winapi::c_ulong =            0x003C;
const LOCALE_SMONTHNAME6: winapi::c_ulong =            0x003D;
const LOCALE_SMONTHNAME7: winapi::c_ulong =            0x003E;
const LOCALE_SMONTHNAME8: winapi::c_ulong =            0x003F;
const LOCALE_SMONTHNAME9: winapi::c_ulong =            0x0040;
const LOCALE_SMONTHNAME10: winapi::c_ulong =           0x0041;
const LOCALE_SMONTHNAME11: winapi::c_ulong =           0x0042;
const LOCALE_SMONTHNAME12: winapi::c_ulong =           0x0043;
const LOCALE_SMONTHNAME13: winapi::c_ulong =           0x100E;
const LOCALE_SABBREVMONTHNAME1: winapi::c_ulong =      0x0044;
const LOCALE_SABBREVMONTHNAME2: winapi::c_ulong =      0x0045;
const LOCALE_SABBREVMONTHNAME3: winapi::c_ulong =      0x0046;
const LOCALE_SABBREVMONTHNAME4: winapi::c_ulong =      0x0047;
const LOCALE_SABBREVMONTHNAME5: winapi::c_ulong =      0x0048;
const LOCALE_SABBREVMONTHNAME6: winapi::c_ulong =      0x0049;
const LOCALE_SABBREVMONTHNAME7: winapi::c_ulong =      0x004A;
const LOCALE_SABBREVMONTHNAME8: winapi::c_ulong =      0x004B;
const LOCALE_SABBREVMONTHNAME9: winapi::c_ulong =      0x004C;
const LOCALE_SABBREVMONTHNAME10: winapi::c_ulong =     0x004D;
const LOCALE_SABBREVMONTHNAME11: winapi::c_ulong =     0x004E;
const LOCALE_SABBREVMONTHNAME12: winapi::c_ulong =     0x004F;
const LOCALE_SABBREVMONTHNAME13: winapi::c_ulong =     0x100F;
const LOCALE_SPOSITIVESIGN: winapi::c_ulong =          0x0050;
const LOCALE_SNEGATIVESIGN: winapi::c_ulong =          0x0051;
const LOCALE_IPOSSIGNPOSN: winapi::c_ulong =           0x0052;
const LOCALE_INEGSIGNPOSN: winapi::c_ulong =           0x0053;
const LOCALE_IPOSSYMPRECEDES: winapi::c_ulong =        0x0054;
const LOCALE_IPOSSEPBYSPACE: winapi::c_ulong =         0x0055;
const LOCALE_INEGSYMPRECEDES: winapi::c_ulong =        0x0056;
const LOCALE_INEGSEPBYSPACE: winapi::c_ulong =         0x0057;
const LOCALE_FONTSIGNATURE: winapi::c_ulong =          0x0058;
const LOCALE_SISO639LANGNAME: winapi::c_ulong =        0x0059;
const LOCALE_SISO3166CTRYNAME: winapi::c_ulong =       0x005A;
const LOCALE_IGEOID: winapi::c_ulong =                 0x005B;
const LOCALE_SNAME: winapi::c_ulong =                  0x005C;
const LOCALE_SDURATION: winapi::c_ulong =              0x005D;
const LOCALE_SKEYBOARDSTOINSTALL: winapi::c_ulong =    0x005E;
const LOCALE_SSHORTESTDAYNAME1: winapi::c_ulong =      0x0060;
const LOCALE_SSHORTESTDAYNAME2: winapi::c_ulong =      0x0061;
const LOCALE_SSHORTESTDAYNAME3: winapi::c_ulong =      0x0062;
const LOCALE_SSHORTESTDAYNAME4: winapi::c_ulong =      0x0063;
const LOCALE_SSHORTESTDAYNAME5: winapi::c_ulong =      0x0064;
const LOCALE_SSHORTESTDAYNAME6: winapi::c_ulong =      0x0065;
const LOCALE_SSHORTESTDAYNAME7: winapi::c_ulong =      0x0066;
const LOCALE_SISO639LANGNAME2: winapi::c_ulong =       0x0067;
const LOCALE_SISO3166CTRYNAME2: winapi::c_ulong =      0x0068;
const LOCALE_SNAN: winapi::c_ulong =                   0x0069;
const LOCALE_SPOSINFINITY: winapi::c_ulong =           0x006A;
const LOCALE_SNEGINFINITY: winapi::c_ulong =           0x006B;
const LOCALE_SSCRIPTS: winapi::c_ulong =               0x006C;
const LOCALE_SPARENT: winapi::c_ulong =                0x006D;
const LOCALE_SCONSOLEFALLBACKNAME: winapi::c_ulong =   0x006E;
const LOCALE_SLANGDISPLAYNAME: winapi::c_ulong =       0x006F;
const LOCALE_SLOCALIZEDLANGUAGENAME: winapi::c_ulong = 0x006F;
const LOCALE_IREADINGLAYOUT: winapi::c_ulong =         0x0070;
const LOCALE_INEUTRAL: winapi::c_ulong =               0x0071;
const LOCALE_SENGLISHDISPLAYNAME: winapi::c_ulong =    0x0072;
const LOCALE_SNATIVEDISPLAYNAME: winapi::c_ulong =     0x0073;
const LOCALE_INEGATIVEPERCENT: winapi::c_ulong =       0x0074;
const LOCALE_IPOSITIVEPERCENT: winapi::c_ulong =       0x0075;
const LOCALE_SPERCENT: winapi::c_ulong =               0x0076;
const LOCALE_SPERMILLE: winapi::c_ulong =              0x0077;
const LOCALE_SMONTHDAY: winapi::c_ulong =              0x0078;
const LOCALE_SSHORTTIME: winapi::c_ulong =             0x0079;
const LOCALE_SOPENTYPELANGUAGETAG: winapi::c_ulong =   0x007A;
const LOCALE_SSORTLOCALE: winapi::c_ulong =            0x007B;

const LOCALE_IDEFAULTEBCDICCODEPAGE: winapi::c_ulong = 0x1012;
const LOCALE_IPAPERSIZE: winapi::c_ulong =             0x100A;
const LOCALE_SENGCURRNAME: winapi::c_ulong =           0x1007;
const LOCALE_SNATIVECURRNAME: winapi::c_ulong =        0x1008;
const LOCALE_SYEARMONTH: winapi::c_ulong =             0x1006;
const LOCALE_SSORTNAME: winapi::c_ulong =              0x1013;
const LOCALE_IDIGITSUBSTITUTION: winapi::c_ulong =     0x1014;

fn get_user_default_locale() -> super::Result<LanguageRange<'static>> {
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
    return Err(super::Error::NotWellFormed);
}

fn get_system_default_locale() -> super::Result<LanguageRange<'static>> {
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
    return Err(super::Error::NotWellFormed);
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
