pub fn reply(message: &str) -> &str {
	let trimmed_message = message.trim();

	let is_question = trimmed_message.ends_with('?');
	let is_all_caps = trimmed_message.chars().any(|c| c.is_alphabetic()) // any letters to check for capitalization?
		&& trimmed_message
		.chars()
		.all(|c| !c.is_alphabetic() || c.is_ascii_uppercase()); // existing letters are all caps?

	match (is_question, is_all_caps) {
		(true, true) => "Calm down, I know what I'm doing!",
		(true, false) => "Sure.",
		(false, true) => "Whoa, chill out!",
		_ if trimmed_message.is_empty() => "Fine. Be that way!",
		_ => "Whatever."
	}
}
