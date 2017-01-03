#[cfg(windows)]
mod windows {
    extern crate winapi;
    extern crate kernel32;

    const ITEMS: &'static [(&'static str, winapi::c_ulong)] = &[
        ("ILANGUAGE", 0x0001),
        ("SLANGUAGE", 0x0002),
        ("SENGLANGUAGE", 0x1001),
        ("SENGLISHLANGUAGENAME", 0x1001),
        ("SABBREVLANGNAME", 0x0003),
        ("SNATIVELANGNAME", 0x0004),
        ("SNATIVELANGUAGENAME", 0x0004),
        ("ICOUNTRY", 0x0005),
        ("SCOUNTRY", 0x0006),
        ("SLOCALIZEDCOUNTRYNAME", 0x0006),
        ("SENGCOUNTRY", 0x1002),
        ("SENGLISHCOUNTRYNAME", 0x1002),
        ("SABBREVCTRYNAME", 0x0007),
        ("SNATIVECTRYNAME", 0x0008),
        ("SNATIVECOUNTRYNAME", 0x0008),
        ("IDEFAULTLANGUAGE", 0x0009),
        ("IDEFAULTCOUNTRY", 0x000A),
        ("IDEFAULTCODEPAGE", 0x000B),
        ("IDEFAULTANSICODEPAGE", 0x1004),
        ("IDEFAULTMACCODEPAGE", 0x1011),
        ("SLIST", 0x000C),
        ("IMEASURE", 0x000D),
        ("SDECIMAL", 0x000E),
        ("STHOUSAND", 0x000F),
        ("SGROUPING", 0x0010),
        ("IDIGITS", 0x0011),
        ("ILZERO", 0x0012),
        ("INEGNUMBER", 0x1010),
        ("SNATIVEDIGITS", 0x0013),
        ("SCURRENCY", 0x0014),
        ("SINTLSYMBOL", 0x0015),
        ("SMONDECIMALSEP", 0x0016),
        ("SMONTHOUSANDSEP", 0x0017),
        ("SMONGROUPING", 0x0018),
        ("ICURRDIGITS", 0x0019),
        ("IINTLCURRDIGITS", 0x001A),
        ("ICURRENCY", 0x001B),
        ("INEGCURR", 0x001C),
        ("SDATE", 0x001D),
        ("STIME", 0x001E),
        ("SSHORTDATE", 0x001F),
        ("SLONGDATE", 0x0020),
        ("STIMEFORMAT", 0x1003),
        ("IDATE", 0x0021),
        ("ILDATE", 0x0022),
        ("ITIME", 0x0023),
        ("ITIMEMARKPOSN", 0x1005),
        ("ICENTURY", 0x0024),
        ("ITLZERO", 0x0025),
        ("IDAYLZERO", 0x0026),
        ("IMONLZERO", 0x0027),
        ("S1159", 0x0028),
        ("S2359", 0x0029),
        ("ICALENDARTYPE", 0x1009),
        ("IOPTIONALCALENDAR", 0x100B),
        ("IFIRSTDAYOFWEEK", 0x100C),
        ("IFIRSTWEEKOFYEAR", 0x100D),
        ("SDAYNAME1", 0x002A),
        ("SDAYNAME2", 0x002B),
        ("SDAYNAME3", 0x002C),
        ("SDAYNAME4", 0x002D),
        ("SDAYNAME5", 0x002E),
        ("SDAYNAME6", 0x002F),
        ("SDAYNAME7", 0x0030),
        ("SABBREVDAYNAME1", 0x0031),
        ("SABBREVDAYNAME2", 0x0032),
        ("SABBREVDAYNAME3", 0x0033),
        ("SABBREVDAYNAME4", 0x0034),
        ("SABBREVDAYNAME5", 0x0035),
        ("SABBREVDAYNAME6", 0x0036),
        ("SABBREVDAYNAME7", 0x0037),
        ("SMONTHNAME1", 0x0038),
        ("SMONTHNAME2", 0x0039),
        ("SMONTHNAME3", 0x003A),
        ("SMONTHNAME4", 0x003B),
        ("SMONTHNAME5", 0x003C),
        ("SMONTHNAME6", 0x003D),
        ("SMONTHNAME7", 0x003E),
        ("SMONTHNAME8", 0x003F),
        ("SMONTHNAME9", 0x0040),
        ("SMONTHNAME10", 0x0041),
        ("SMONTHNAME11", 0x0042),
        ("SMONTHNAME12", 0x0043),
        ("SMONTHNAME13", 0x100E),
        ("SABBREVMONTHNAME1", 0x0044),
        ("SABBREVMONTHNAME2", 0x0045),
        ("SABBREVMONTHNAME3", 0x0046),
        ("SABBREVMONTHNAME4", 0x0047),
        ("SABBREVMONTHNAME5", 0x0048),
        ("SABBREVMONTHNAME6", 0x0049),
        ("SABBREVMONTHNAME7", 0x004A),
        ("SABBREVMONTHNAME8", 0x004B),
        ("SABBREVMONTHNAME9", 0x004C),
        ("SABBREVMONTHNAME10", 0x004D),
        ("SABBREVMONTHNAME11", 0x004E),
        ("SABBREVMONTHNAME12", 0x004F),
        ("SABBREVMONTHNAME13", 0x100F),
        ("SPOSITIVESIGN", 0x0050),
        ("SNEGATIVESIGN", 0x0051),
        ("IPOSSIGNPOSN", 0x0052),
        ("INEGSIGNPOSN", 0x0053),
        ("IPOSSYMPRECEDES", 0x0054),
        ("IPOSSEPBYSPACE", 0x0055),
        ("INEGSYMPRECEDES", 0x0056),
        ("INEGSEPBYSPACE", 0x0057),
        ("FONTSIGNATURE", 0x0058),
        ("SISO639LANGNAME", 0x0059),
        ("SISO3166CTRYNAME", 0x005A),
        ("IGEOID", 0x005B),
        ("SNAME", 0x005C), // crashes on wine, but I need to know what it does on actual Win7
        ("SDURATION", 0x005D),
        ("SKEYBOARDSTOINSTALL", 0x005E),
        ("SSHORTESTDAYNAME1", 0x0060),
        ("SSHORTESTDAYNAME2", 0x0061),
        ("SSHORTESTDAYNAME3", 0x0062),
        ("SSHORTESTDAYNAME4", 0x0063),
        ("SSHORTESTDAYNAME5", 0x0064),
        ("SSHORTESTDAYNAME6", 0x0065),
        ("SSHORTESTDAYNAME7", 0x0066),
        ("SISO639LANGNAME2", 0x0067),
        ("SISO3166CTRYNAME2", 0x0068),
        ("SNAN", 0x0069),
        ("SPOSINFINITY", 0x006A),
        ("SNEGINFINITY", 0x006B),
        ("SSCRIPTS", 0x006C),
        ("SPARENT", 0x006D),
        ("SCONSOLEFALLBACKNAME", 0x006E),
        ("SLANGDISPLAYNAME", 0x006F),
        ("SLOCALIZEDLANGUAGENAME", 0x006F),
        ("IREADINGLAYOUT", 0x0070),
        ("INEUTRAL", 0x0071),
        ("SENGLISHDISPLAYNAME", 0x0072),
        ("SNATIVEDISPLAYNAME", 0x0073),
        ("INEGATIVEPERCENT", 0x0074),
        ("IPOSITIVEPERCENT", 0x0075),
        ("SPERCENT", 0x0076),
        ("SPERMILLE", 0x0077),
        ("SMONTHDAY", 0x0078),
        ("SSHORTTIME", 0x0079),
        ("SOPENTYPELANGUAGETAG", 0x007A),
        ("SSORTLOCALE", 0x007B),

        ("IDEFAULTEBCDICCODEPAGE", 0x1012),
        ("IPAPERSIZE", 0x100A),
        ("SENGCURRNAME", 0x1007),
        ("SNATIVECURRNAME", 0x1008),
        ("SYEARMONTH", 0x1006),
        ("SSORTNAME", 0x1013),
        ("IDIGITSUBSTITUTION", 0x1014),
        ];

    fn print_item(item: &(&'static str, winapi::c_ulong)) -> bool {
        #[allow(non_snake_case)] // would be const if it wasn't for the fact const functions are still unstable
        let LOCALE_NAME_USER_DEFAULT: *mut u16 = ::std::ptr::null_mut();
        let mut buf = [0u16; 86];
        println!("LOCALE_{} …", item.0);
        let len = unsafe {
            kernel32::GetLocaleInfoEx(LOCALE_NAME_USER_DEFAULT, item.1,
                                      buf.as_mut_ptr(), buf.len() as winapi::c_long)
        };
        println!("  … {}", len);
        if len >= 0 && len <= buf.len() as i32 {
            let s = String::from_utf16_lossy(&buf[0..(len as usize - 1)]);
            println!("LOCALE_{} = “{}” ({:?})", item.0, s, s);
            true
        } else {
            println!("LOCALE_{} invalid len {}", item.0, len);
            false
        }
    }

    pub fn main() -> bool {
        ITEMS.iter().all(print_item)
    }
}
#[cfg(not(windows))]
mod windows {
    pub fn main() -> bool { true }
}

pub fn main() {
    if !windows::main() {
        std::process::exit(1);
    }
}
