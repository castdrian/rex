use colored::Colorize;

pub fn assign_color(t: &str) -> colored::ColoredString {
	match t {
		"Normal" => t.truecolor(168, 168, 120),
		"Fire" => t.truecolor(240, 128, 48),
		"Water" => t.truecolor(104, 144, 240),
		"Electric" => t.truecolor(248, 208, 48),
		"Grass" => t.truecolor(120, 200, 80),
		"Ice" => t.truecolor(152, 216, 216),
		"Fighting" => t.truecolor(192, 48, 40),
		"Poison" => t.truecolor(160, 64, 160),
		"Ground" => t.truecolor(224, 192, 104),
		"Flying" => t.truecolor(168, 144, 240),
		"Psychic" => t.truecolor(248, 88, 136),
		"Bug" => t.truecolor(168, 184, 32),
		"Rock" => t.truecolor(184, 160, 56),
		"Ghost" => t.truecolor(112, 88, 152),
		"Dragon" => t.truecolor(112, 56, 248),
		"Dark" => t.truecolor(112, 88, 72),
		"Steel" => t.truecolor(184, 184, 208),
		"Fairy" => t.truecolor(238, 153, 172),
		_ => t.white(),
	}
}