// usage of termions 'Color' module is awkward
// see https://gitlab.redox-os.org/redox-os/termion/-/issues/123

// see https://stackoverflow.com/questions/4842424/list-of-ansi-color-escape-sequences

pub fn color(bg_color: u8, fg_color: u8) -> String {
    format!("\x1b[38;5;{fg_color};48;5;{bg_color}m")
}

pub fn reset() -> String {
    String::from("\x1b[0m")
}
