pub fn yes_no(value: bool) -> &'static str {
    if value {
        "yes"
    } else {
        "no"
    }
}

pub fn on_off(value: bool) -> &'static str {
    if value {
        "on"
    } else {
        "off"
    }
}

pub fn enabled_disabled(value: bool) -> &'static str {
    if value {
        "enabled"
    } else {
        "disabled"
    }
}

pub fn true_false(value: bool) -> &'static str {
    if value {
        "true"
    } else {
        "false"
    }
}

pub fn active_inactive(value: bool) -> &'static str {
    if value {
        "active"
    } else {
        "inactive"
    }
}

pub fn up_down(value: bool) -> &'static str {
    if value {
        "up"
    } else {
        "down"
    }
}

pub fn open_closed(value: bool) -> &'static str {
    if value {
        "open"
    } else {
        "closed"
    }
}

pub fn connected_disconnected(value: bool) -> &'static str {
    if value {
        "connected"
    } else {
        "disconnected"
    }
}

pub fn pass_fail(value: bool) -> &'static str {
    if value {
        "pass"
    } else {
        "fail"
    }
}

pub fn is_true(string: &str) -> bool {
    matches!(
        string.to_lowercase().as_str(),
        "true" | "y" | "yes" | "on" | "enabled" | "active" | "up" | "open" | "connected" | "pass"
    )
}