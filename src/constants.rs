use colored::Colorize;

pub fn assign_typecolor(t: &str) -> colored::ColoredString {
	match t {
		"Normal" => t.black().on_truecolor(168, 168, 120),
		"Fire" => t.black().on_truecolor(240, 128, 48),
		"Water" => t.black().on_truecolor(104, 144, 240),
		"Electric" => t.black().on_truecolor(248, 208, 48),
		"Grass" => t.black().on_truecolor(120, 200, 80),
		"Ice" => t.black().on_truecolor(152, 216, 216),
		"Fighting" => t.black().on_truecolor(192, 48, 40),
		"Poison" => t.black().on_truecolor(160, 64, 160),
		"Ground" => t.black().on_truecolor(224, 192, 104),
		"Flying" => t.black().on_truecolor(168, 144, 240),
		"Psychic" => t.black().on_truecolor(248, 88, 136),
		"Bug" => t.black().on_truecolor(168, 184, 32),
		"Rock" => t.black().on_truecolor(184, 160, 56),
		"Ghost" => t.black().on_truecolor(112, 88, 152),
		"Dragon" => t.black().on_truecolor(112, 56, 248),
		"Dark" => t.black().on_truecolor(112, 88, 72),
		"Steel" => t.black().on_truecolor(184, 184, 208),
		"Fairy" => t.black().on_truecolor(238, 153, 172),
		_ => t.white(),
	}
}

pub fn get_colored_space(t: &str) -> colored::ColoredString {
	match t {
		"Normal" => "\u{2009}".on_truecolor(168, 168, 120),
		"Fire" => "\u{2009}".on_truecolor(240, 128, 48),
		"Water" => "\u{2009}".on_truecolor(104, 144, 240),
		"Electric" => "\u{2009}".on_truecolor(248, 208, 48),
		"Grass" => "\u{2009}".on_truecolor(120, 200, 80),
		"Ice" => "\u{2009}".on_truecolor(152, 216, 216),
		"Fighting" => "\u{2009}".on_truecolor(192, 48, 40),
		"Poison" => "\u{2009}".on_truecolor(160, 64, 160),
		"Ground" => "\u{2009}".on_truecolor(224, 192, 104),
		"Flying" => "\u{2009}".on_truecolor(168, 144, 240),
		"Psychic" => "\u{2009}".on_truecolor(248, 88, 136),
		"Bug" => "\u{2009}".on_truecolor(168, 184, 32),
		"Rock" => "\u{2009}".on_truecolor(184, 160, 56),
		"Ghost" => "\u{2009}".on_truecolor(112, 88, 152),
		"Dragon" => "\u{2009}".on_truecolor(112, 56, 248),
		"Dark" => "\u{2009}".on_truecolor(112, 88, 72),
		"Steel" => "\u{2009}".on_truecolor(184, 184, 208),
		"Fairy" => "\u{2009}".on_truecolor(238, 153, 172),
		_ => "\u{2009}".white(),
	}
}