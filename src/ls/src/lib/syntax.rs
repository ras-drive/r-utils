
use std::fmt::format;
use std::fs;
use std::fs::Metadata;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::time::SystemTime;

pub fn get_metadata(name: &str) -> Metadata {
    fs::metadata(&name).unwrap()
}

fn get_perms(mut meta: &Metadata) -> String {
    let perms = meta.permissions().mode();
    let mut string_buf = String::new();

    for i in perms.to_string().chars() {
        match i.to_string().parse().unwrap() {
            7 => string_buf.push_str(&*format!("{}{}{}", "r", "w", "x")), // "rwx"
            6 => string_buf.push_str(&*format!("{}{}{}", "r", "w", "-")), // "rw-"
            5 => string_buf.push_str(&*format!("{}{}{}", "r", "-", "x")), // "r-x"
            4 => string_buf.push_str(&*format!("{}{}{}", "r", "-", "-")), // "r--"
            3 => string_buf.push_str(&*format!("{}{}{}", "-", "w", "x")), // "-wx"
            2 => string_buf.push_str(&*format!("{}{}{}", "-", "w", "-")), // "-w-"
            1 => string_buf.push_str(&*format!("{}{}{}", "-", "-", "x")), // "--x"
            _ => break,
        }
    }
    string_buf
}

fn get_owner(meta: &Metadata) -> u32 {
    meta.uid()
}

fn get_date(meta: &Metadata) -> String {
    time_to_http_date_string(meta.modified().unwrap())
}

fn time_to_http_date_string(time: SystemTime) -> String {
    const MINUTE: u64 = 60;
    const HOUR: u64 = 3600;
    const DAY: u64 = 86400;
    const WEEKDAYS: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
    const MONTH: u64 = 2629743;
    const MONTHS: [&str; 12] = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    const YEAR: u64 = 31556926;

    let time = time
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let year = 1970 + (time / YEAR);
    let month = MONTHS[(time % YEAR / MONTH) as usize];
    let weekday = WEEKDAYS[((time / DAY + 4) % 7) as usize];

    // FIXME: Slightly wrong as time distance from the UNIX epoch increases.
    let day = (time % MONTH / DAY) + 1;

    let hour = time % DAY / HOUR;
    let minute = time % HOUR / MINUTE;
    let second = time % MINUTE;

    format!(
        "{weekday}, {day:02} {month} {year} {hour:02}:{minute:02}:{second:02}",
        weekday = weekday,
        day = day,
        month = month,
        year = year,
        hour = hour,
        minute = minute,
        second = second,
    )
}

fn type_of_file(meta: &Metadata) -> &'static str {
    if meta.is_file() {
        "."
    } else if meta.is_dir() {
        "d"
    } else if meta.is_symlink() {
        "l"
    } else {
        eprintln!("this shouldn't happen!");
        ""
    }
}

fn get_size(meta: &Metadata) -> u64 {
    meta.size()
}

pub fn get_long(meta: Metadata, name: &str) -> String {
    format!(
        "{}{} {} {} {} {}",
        type_of_file(&meta),
        &*get_perms(&meta),
        &*get_size(&meta).to_string(),
        &*get_owner(&meta).to_string(),
        &*get_date(&meta),
        name
    )
    .to_string()
}
