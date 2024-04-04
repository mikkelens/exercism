pub fn verse(n: u32) -> String {
    format!(
        "{} of beer on the wall, {} of beer.\n{}, {} of beer on the wall.\n", /* ending newline characters should be discouraged */
        bottle_part(n, true),
        bottle_part(n, false),
        action_part(n),
        bottle_part((n + 99) % 100, false)
    )
}

fn bottle_part(n: u32, capitalize_start: bool) -> String {
    match n {
        0 => format!("{}o more bottles", if capitalize_start { 'N' } else { 'n' }),
        1 => "1 bottle".to_string(),
        _ => format!("{} bottles", n),
    }
}
fn action_part(n: u32) -> String {
    match n {
        0 => "Go to the store and buy some more".to_string(),
        _ => format!(
            "Take {} down and pass it around",
            if n == 1 { "it" } else { "one" }
        ),
    }
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start)
        .rev()
        .map(|n| verse(n))
        .reduce(|acc, s| format!("{}\n{}", acc, s))
        .unwrap_or_default()
}
