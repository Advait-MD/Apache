use std::fs;

pub fn battery_percentage() -> Option<u8> {
    let path = "/sys/class/power_supply/BAT0/capacity";

    let contents = fs::read_to_string(path).ok()?;

    contents.trim().parse::<u8>().ok()
}
