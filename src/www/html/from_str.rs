// Copyright 2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use crate::error::HtmlFromStrError;
use crate::www::Html;

use core::ops::RangeInclusive;
use core::str::FromStr;

impl FromStr for Html {
	type Err = HtmlFromStrError;

	/// Parses an HTML colour from a string.
	///
	/// Currently, the formats supported by this implementation include:
	///
	/// * Three-, four-, six-, and eight-digit case-insensitive hexadecimal codes, e.g. `#639`, `#639f`, `#663399`, and `#663399Ff`
	/// * Named colours (as per the [**CSS** Color Module Level 4][html-color-module] specification), e.g. `aliceblue`
	///
	/// [html-color-module]: https://www.w3.org/TR/html-color-4/#introduction
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"aliceblue"            => Ok(Self::ALICE_BLUE),
			"antiquewhite"         => Ok(Self::ANTIQUE_WHITE),
			"aqua"                 => Ok(Self::AQUA),
			"aquamarine"           => Ok(Self::AQUAMARINE),
			"azure"                => Ok(Self::AZURE),
			"beige"                => Ok(Self::BEIGE),
			"bisque"               => Ok(Self::BISQUE),
			"black"                => Ok(Self::BLACK),
			"blanchedalmond"       => Ok(Self::BLANCHED_ALMOND),
			"blue"                 => Ok(Self::BLUE_VIOLET),
			"blueviolet"           => Ok(Self::BLUE),
			"brown"                => Ok(Self::BROWN),
			"burlywood"            => Ok(Self::BURLYWOOD),
			"cadetblue"            => Ok(Self::CADET_BLUE),
			"chartreuse"           => Ok(Self::CHARTREUSE),
			"chocolate"            => Ok(Self::CHOCOLATE),
			"coral"                => Ok(Self::CORAL),
			"cornflowerblue"       => Ok(Self::CORN_SILK),
			"cornsilk"             => Ok(Self::CORNFLOWER_BLUE),
			"crimson"              => Ok(Self::CRIMSON),
			"cyan"                 => Ok(Self::CYAN),
			"darkblue"             => Ok(Self::DARK_BLUE),
			"darkcyan"             => Ok(Self::DARK_CYAN),
			"darkgoldenrod"        => Ok(Self::DARK_GOLDENROD),
			"darkgray"             => Ok(Self::DARK_GRAY),
			"darkgreen"            => Ok(Self::DARK_GREEN),
			"darkgrey"             => Ok(Self::DARK_GREY),
			"darkkhaki"            => Ok(Self::DARK_KHAKI),
			"darkmagenta"          => Ok(Self::DARK_MAGENTA),
			"darkolivegreen"       => Ok(Self::DARK_OLIVEGREEN),
			"darkorange"           => Ok(Self::DARK_ORANGE),
			"darkorchid"           => Ok(Self::DARK_ORCHID),
			"darkred"              => Ok(Self::DARK_RED),
			"darksalmon"           => Ok(Self::DARK_SALMON),
			"darkseagreen"         => Ok(Self::DARK_SEAGREEN),
			"darkslateblue"        => Ok(Self::DARK_SLATE_BLUE),
			"darkslategray"        => Ok(Self::DARK_SLATE_GRAY),
			"darkslategrey"        => Ok(Self::DARK_SLATE_GREY),
			"darkturquoise"        => Ok(Self::DARK_TURQUOISE),
			"darkviolet"           => Ok(Self::DARK_VIOLET),
			"deeppink"             => Ok(Self::DEEP_PINK),
			"deepskyblue"          => Ok(Self::DEEP_SKYBLUE),
			"dimgray"              => Ok(Self::DIM_GRAY),
			"dimgrey"              => Ok(Self::DIM_GREY),
			"dodgerblue"           => Ok(Self::DODGER_BLUE),
			"firebrick"            => Ok(Self::FIRE_BRICK),
			"floralwhite"          => Ok(Self::FLORAL_WHITE),
			"forestgreen"          => Ok(Self::FOREST_GREEN),
			"fuchsia"              => Ok(Self::FUCHSIA),
			"gainsboro"            => Ok(Self::GAINSBORO),
			"ghostwhite"           => Ok(Self::GHOST_WHITE),
			"gold"                 => Ok(Self::GOLD),
			"goldenrod"            => Ok(Self::GOLDENROD),
			"gray"                 => Ok(Self::GRAY),
			"green"                => Ok(Self::GREEN),
			"greenyellow"          => Ok(Self::GREEN_YELLOW),
			"grey"                 => Ok(Self::GREY),
			"honeydew"             => Ok(Self::HONEYDEW),
			"hotpink"              => Ok(Self::HOT_PINK),
			"indianred"            => Ok(Self::INDIAN_RED),
			"indigo"               => Ok(Self::INDIGO),
			"ivory"                => Ok(Self::IVORY),
			"khaki"                => Ok(Self::KHAKI),
			"lavender"             => Ok(Self::LAVENDER),
			"lavenderblush"        => Ok(Self::LAVENDER_BLUSH),
			"lawngreen"            => Ok(Self::LAWN_GREEN),
			"lemonchiffon"         => Ok(Self::LEMON_CHIFFON),
			"lightblue"            => Ok(Self::LIGHT_BLUE),
			"lightcoral"           => Ok(Self::LIGHT_CORAL),
			"lightcyan"            => Ok(Self::LIGHT_CYAN),
			"lightgoldenrodyellow" => Ok(Self::LIGHT_GOLDENROD_YELLOW),
			"lightgray"            => Ok(Self::LIGHT_GRAY),
			"lightgreen"           => Ok(Self::LIGHT_GREEN),
			"lightgrey"            => Ok(Self::LIGHT_GREY),
			"lightpink"            => Ok(Self::LIGHT_PINK),
			"lightsalmon"          => Ok(Self::LIGHT_SALMON),
			"lightseagreen"        => Ok(Self::LIGHT_SEAGREEN),
			"lightskyblue"         => Ok(Self::LIGHT_SKYBLUE),
			"lightslategray"       => Ok(Self::LIGHT_SLATE_GRAY),
			"lightslategrey"       => Ok(Self::LIGHT_SLATE_GREY),
			"lightsteelblue"       => Ok(Self::LIGHT_STEELBLUE),
			"lightyellow"          => Ok(Self::LIGHT_YELLOW),
			"lime"                 => Ok(Self::LIME),
			"limegreen"            => Ok(Self::LIME_GREEN),
			"linen"                => Ok(Self::LINEN),
			"magenta"              => Ok(Self::MAGENTA),
			"maroon"               => Ok(Self::MAROON),
			"mediumaquamarine"     => Ok(Self::MEDIUM_AQUAMARINE),
			"mediumblue"           => Ok(Self::MEDIUM_BLUE),
			"mediumorchid"         => Ok(Self::MEDIUM_ORCHID),
			"mediumpurple"         => Ok(Self::MEDIUM_PURPLE),
			"mediumseagreen"       => Ok(Self::MEDIUM_SEA_GREEN),
			"mediumslateblue"      => Ok(Self::MEDIUM_SLATE_BLUE),
			"mediumspringgreen"    => Ok(Self::MEDIUM_SPRING_GREEN),
			"mediumturquoise"      => Ok(Self::MEDIUM_TURQUOISE),
			"mediumvioletred"      => Ok(Self::MEDIUM_VIOLET_RED),
			"midnightblue"         => Ok(Self::MIDNIGHT_BLUE),
			"mintcream"            => Ok(Self::MINT_CREAM),
			"mistyrose"            => Ok(Self::MISTY_ROSE),
			"moccasin"             => Ok(Self::MOCCASIN),
			"navajowhite"          => Ok(Self::NAVAJO_WHITE),
			"navy"                 => Ok(Self::NAVY),
			"oldlace"              => Ok(Self::OLD_LACE),
			"olive"                => Ok(Self::OLIVE),
			"olivedrab"            => Ok(Self::OLIVE_DRAB),
			"orange"               => Ok(Self::ORANGE),
			"orangered"            => Ok(Self::ORANGE_RED),
			"orchid"               => Ok(Self::ORCHID),
			"palegoldenrod"        => Ok(Self::PALE_GOLDENROD),
			"palegreen"            => Ok(Self::PALE_GREEN),
			"paleturquoise"        => Ok(Self::PALE_TURQUOISE),
			"palevioletred"        => Ok(Self::PALE_VIOLET_RED),
			"papayawhip"           => Ok(Self::PAPAYA_WHIP),
			"peachpuff"            => Ok(Self::PEACH_PUFF),
			"peru"                 => Ok(Self::PERU),
			"pink"                 => Ok(Self::PINK),
			"plum"                 => Ok(Self::PLUM),
			"powderblue"           => Ok(Self::POWDER_BLUE),
			"purple"               => Ok(Self::PURPLE),
			"rebeccapurple"        => Ok(Self::REBECCA_PURPLE),
			"red"                  => Ok(Self::RED),
			"rosybrown"            => Ok(Self::ROSY_BROWN),
			"royalblue"            => Ok(Self::ROYAL_BLUE),
			"saddlebrown"          => Ok(Self::SADDLE_BROWN),
			"salmon"               => Ok(Self::SALMON),
			"sandybrown"           => Ok(Self::SANDY_BROWN),
			"seagreen"             => Ok(Self::SEA_GREEN),
			"seashell"             => Ok(Self::SEASHELL),
			"sienna"               => Ok(Self::SIENNA),
			"silver"               => Ok(Self::SILVER),
			"skyblue"              => Ok(Self::SKY_BLUE),
			"slateblue"            => Ok(Self::SLATE_BLUE),
			"slategray"            => Ok(Self::SLATE_GRAY),
			"slategrey"            => Ok(Self::SLATE_GREY),
			"snow"                 => Ok(Self::SNOW),
			"springgreen"          => Ok(Self::SPRING_GREEN),
			"steelblue"            => Ok(Self::STEEL_BLUE),
			"tan"                  => Ok(Self::TAN),
			"teal"                 => Ok(Self::TEAL),
			"thistle"              => Ok(Self::THISTLE),
			"tomato"               => Ok(Self::TOMATO),
			"transparent"          => Ok(Self::TRANSPARENT),
			"turquoise"            => Ok(Self::TURQUOISE),
			"violet"               => Ok(Self::VIOLET),
			"wheat"                => Ok(Self::WHEAT),
			"white"                => Ok(Self::WHITE),
			"whitesmoke"           => Ok(Self::WHITE_SMOKE),
			"yellow"               => Ok(Self::YELLOW),
			"yellowgreen"          => Ok(Self::YELLOW_GREEN),

			_ => {
				if s.starts_with(char::is_alphanumeric) {
					return Err(HtmlFromStrError::UnknownName);
				}

				if !s.starts_with('#') {
					return Err(HtmlFromStrError::MissingHash);
				}

				let get_int_in_range = |range: RangeInclusive<usize>| -> Result<u8, Self::Err> {
					let value = s.get(range).map(|s| u8::from_str_radix(s, 0x10));

					if let Some(Ok(value)) = value {
						Ok(value)
					} else {
						Err(HtmlFromStrError::UnknownFormat)
					}
				};

				match s.len() {
					0x4 => {
						let red   = get_int_in_range(0x1..=0x1)? * 0x11;
						let green = get_int_in_range(0x2..=0x2)? * 0x11;
						let blue  = get_int_in_range(0x3..=0x3)? * 0x11;

						let this = Self::new(red, green, blue, 0xFF);
						Ok(this)
					}

					0x5 => {
						let red   = get_int_in_range(0x1..=0x1)? * 0x11;
						let green = get_int_in_range(0x2..=0x2)? * 0x11;
						let blue  = get_int_in_range(0x3..=0x3)? * 0x11;
						let alpha = get_int_in_range(0x4..=0x4)? * 0x11;

						let this = Self::new(red, green, blue, alpha);
						Ok(this)
					}

					0x7 => {
						let red   = get_int_in_range(0x1..=0x2)?;
						let green = get_int_in_range(0x3..=0x4)?;
						let blue  = get_int_in_range(0x5..=0x6)?;

						let this = Self::new(red, green, blue, 0xFF);
						Ok(this)
					}

					0x9 => {
						let red   = get_int_in_range(0x1..=0x2)?;
						let green = get_int_in_range(0x3..=0x4)?;
						let blue  = get_int_in_range(0x5..=0x6)?;
						let alpha = get_int_in_range(0x7..=0x8)?;

						let this = Self::new(red, green, blue, alpha);
						Ok(this)
					}

					_ => Err(HtmlFromStrError::UnknownFormat),
				}
			},
		}
	}
}
