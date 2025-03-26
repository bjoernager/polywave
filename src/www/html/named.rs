// Copyright 2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

macro_rules! def_named_colour {
	{
		$($rust_name:ident: $html_name:ident = $value:expr;)*
	} => {
		impl ::polywave::www::Html {
			$(
				#[doc = ::core::concat!(
					"The `",
					::core::stringify!($html_name),
					"` <span style=\"aspect-ratio: 1 / 1; background-color: ",
					::core::stringify!($html_name),
					"; border-radius: calc(1em / 3); display: inline-block; height: 1em; text-align: center; vertical-align: middle;\"",
					"></span> HTML named colour.\n",
				)]
				pub const $rust_name: Self = $value;
			)*
		}
	};
}

def_named_colour! {
	ALICE_BLUE:             aliceblue            = Self::from_u32(0xF0F8FFFF);
	ANTIQUE_WHITE:          antiquewhite         = Self::from_u32(0xFAEBD7FF);
	AQUA:                   aqua                 = Self::from_u32(0x00FFFFFF);
	AQUAMARINE:             aquamarine           = Self::from_u32(0x7FFFD4FF);
	AZURE:                  azure                = Self::from_u32(0xF0FFFFFF);
	BEIGE:                  beige                = Self::from_u32(0xF5F5DCFF);
	BISQUE:                 bisque               = Self::from_u32(0xFFE4C4FF);
	BLACK:                  black                = Self::from_u32(0x000000FF);
	BLANCHED_ALMOND:        blanchedalmond       = Self::from_u32(0xFFEBCDFF);
	BLUE_VIOLET:            blue                 = Self::from_u32(0x8A2BE2FF);
	BLUE:                   blueviolet           = Self::from_u32(0x0000FFFF);
	BROWN:                  brown                = Self::from_u32(0xA52A2AFF);
	BURLYWOOD:              burlywood            = Self::from_u32(0xDEB887FF);
	CADET_BLUE:             cadetblue            = Self::from_u32(0x5F9EA0FF);
	CHARTREUSE:             chartreuse           = Self::from_u32(0x7FFF00FF);
	CHOCOLATE:              chocolate            = Self::from_u32(0xD2691EFF);
	CORAL:                  coral                = Self::from_u32(0xFF7F50FF);
	CORN_SILK:              cornflowerblue       = Self::from_u32(0xFFF8DCFF);
	CORNFLOWER_BLUE:        cornsilk             = Self::from_u32(0x6495EDFF);
	CRIMSON:                crimson              = Self::from_u32(0xDC143CFF);
	CYAN:                   cyan                 = Self::AQUA;
	DARK_BLUE:              darkblue             = Self::from_u32(0x00008BFF);
	DARK_CYAN:              darkcyan             = Self::from_u32(0x008B8BFF);
	DARK_GOLDENROD:         darkgoldenrod        = Self::from_u32(0xB8860BFF);
	DARK_GRAY:              darkgray             = Self::from_u32(0xA9A9A9FF);
	DARK_GREEN:             darkgreen            = Self::from_u32(0x006400FF);
	DARK_GREY:              darkgrey             = Self::GREY;
	DARK_KHAKI:             darkkhaki            = Self::from_u32(0xBDB76BFF);
	DARK_MAGENTA:           darkmagenta          = Self::from_u32(0x8B008BFF);
	DARK_OLIVEGREEN:        darkolivegreen       = Self::from_u32(0x556B2FFF);
	DARK_ORANGE:            darkorange           = Self::from_u32(0xFF8C00FF);
	DARK_ORCHID:            darkorchid           = Self::from_u32(0x9932CCFF);
	DARK_RED:               darkred              = Self::from_u32(0x8B0000FF);
	DARK_SALMON:            darksalmon           = Self::from_u32(0xE9967AFF);
	DARK_SEAGREEN:          darkseagreen         = Self::from_u32(0x8FBC8FFF);
	DARK_SLATE_BLUE:        darkslateblue        = Self::from_u32(0x483D8BFF);
	DARK_SLATE_GRAY:        darkslategray        = Self::from_u32(0x2F4F4FFF);
	DARK_SLATE_GREY:        darkslategrey        = Self::DARK_SLATE_GRAY;
	DARK_TURQUOISE:         darkturquoise        = Self::from_u32(0x00CED1FF);
	DARK_VIOLET:            darkviolet           = Self::from_u32(0x9400D3FF);
	DEEP_PINK:              deeppink             = Self::from_u32(0xFF1493FF);
	DEEP_SKYBLUE:           deepskyblue          = Self::from_u32(0x00BFFFFF);
	DIM_GRAY:               dimgray              = Self::from_u32(0x696969FF);
	DIM_GREY:               dimgrey              = Self::DIM_GRAY;
	DODGER_BLUE:            dodgerblue           = Self::from_u32(0x1E90FFFF);
	FIRE_BRICK:             firebrick            = Self::from_u32(0xB22222FF);
	FLORAL_WHITE:           floralwhite          = Self::from_u32(0xFFFAF0FF);
	FOREST_GREEN:           forestgreen          = Self::from_u32(0x228B22FF);
	FUCHSIA:                fuchsia              = Self::from_u32(0xFF00FFFF);
	GAINSBORO:              gainsboro            = Self::from_u32(0xDCDCDCFF);
	GHOST_WHITE:            ghostwhite           = Self::from_u32(0xF8F8FFFF);
	GOLD:                   gold                 = Self::from_u32(0xFFD700FF);
	GOLDENROD:              goldenrod            = Self::from_u32(0xDAA520FF);
	GRAY:                   gray                 = Self::from_u32(0x808080FF);
	GREEN:                  green                = Self::from_u32(0x008000FF);
	GREEN_YELLOW:           greenyellow          = Self::from_u32(0xADFF2FFF);
	GREY:                   grey                 = Self::from_u32(0x808080FF);
	HONEYDEW:               honeydew             = Self::from_u32(0xF0FFF0FF);
	HOT_PINK:               hotpink              = Self::from_u32(0xFF69B4FF);
	INDIAN_RED:             indianred            = Self::from_u32(0xCD5C5CFF);
	INDIGO:                 indigo               = Self::from_u32(0x4B0082FF);
	IVORY:                  ivory                = Self::from_u32(0xFFFFF0FF);
	KHAKI:                  khaki                = Self::from_u32(0xF0E68CFF);
	LAVENDER:               lavender             = Self::from_u32(0xE6E6FAFF);
	LAVENDER_BLUSH:         lavenderblush        = Self::from_u32(0xFFF0F5FF);
	LAWN_GREEN:             lawngreen            = Self::from_u32(0x7CFC00FF);
	LEMON_CHIFFON:          lemonchiffon         = Self::from_u32(0xFFFACDFF);
	LIGHT_BLUE:             lightblue            = Self::from_u32(0xADD8E6FF);
	LIGHT_CORAL:            lightcoral           = Self::from_u32(0xF08080FF);
	LIGHT_CYAN:             lightcyan            = Self::from_u32(0xE0FFFFFF);
	LIGHT_GOLDENROD_YELLOW: lightgoldenrodyellow = Self::from_u32(0xFAFAD2FF);
	LIGHT_GRAY:             lightgray            = Self::from_u32(0xD3D3D3FF);
	LIGHT_GREEN:            lightgreen           = Self::from_u32(0x90EE90FF);
	LIGHT_GREY:             lightgrey            = Self::from_u32(0xD3D3D3FF);
	LIGHT_PINK:             lightpink            = Self::from_u32(0xFFB6C1FF);
	LIGHT_SALMON:           lightsalmon          = Self::from_u32(0xFFA07AFF);
	LIGHT_SEAGREEN:         lightseagreen        = Self::from_u32(0x20B2AAFF);
	LIGHT_SKYBLUE:          lightskyblue         = Self::from_u32(0x87CEFAFF);
	LIGHT_SLATE_GRAY:       lightslategray       = Self::from_u32(0x778899FF);
	LIGHT_SLATE_GREY:       lightslategrey       = Self::LIGHT_SLATE_GRAY;
	LIGHT_STEELBLUE:        lightsteelblue       = Self::from_u32(0xB0C4DEFF);
	LIGHT_YELLOW:           lightyellow          = Self::from_u32(0xFFFFE0FF);
	LIME:                   lime                 = Self::from_u32(0x00FF00FF);
	LIME_GREEN:             limegreen            = Self::from_u32(0x32CD32FF);
	LINEN:                  linen                = Self::from_u32(0xFAF0E6FF);
	MAGENTA:                magenta              = Self::FUCHSIA;
	MAROON:                 maroon               = Self::from_u32(0x800000FF);
	MEDIUM_AQUAMARINE:      mediumaquamarine     = Self::from_u32(0x66CDAAFF);
	MEDIUM_BLUE:            mediumblue           = Self::from_u32(0x0000CDFF);
	MEDIUM_ORCHID:          mediumorchid         = Self::from_u32(0xBA55D3FF);
	MEDIUM_PURPLE:          mediumpurple         = Self::from_u32(0x9370DBFF);
	MEDIUM_SEA_GREEN:       mediumseagreen       = Self::from_u32(0x3CB371FF);
	MEDIUM_SLATE_BLUE:      mediumslateblue      = Self::from_u32(0x7B68EEFF);
	MEDIUM_SPRING_GREEN:    mediumspringgreen    = Self::from_u32(0x00FA9AFF);
	MEDIUM_TURQUOISE:       mediumturquoise      = Self::from_u32(0x48D1CCFF);
	MEDIUM_VIOLET_RED:      mediumvioletred      = Self::from_u32(0xC71585FF);
	MIDNIGHT_BLUE:          midnightblue         = Self::from_u32(0x191970FF);
	MINT_CREAM:             mintcream            = Self::from_u32(0xF5FFFAFF);
	MISTY_ROSE:             mistyrose            = Self::from_u32(0xFFE4E1FF);
	MOCCASIN:               moccasin             = Self::from_u32(0xFFE4B5FF);
	NAVAJO_WHITE:           navajowhite          = Self::from_u32(0xFFDEADFF);
	NAVY:                   navy                 = Self::from_u32(0x000080FF);
	OLD_LACE:               oldlace              = Self::from_u32(0xFDF5E6FF);
	OLIVE:                  olive                = Self::from_u32(0x808000FF);
	OLIVE_DRAB:             olivedrab            = Self::from_u32(0x6B8E23FF);
	ORANGE:                 orange               = Self::from_u32(0xFFA500FF);
	ORANGE_RED:             orangered            = Self::from_u32(0xFF4500FF);
	ORCHID:                 orchid               = Self::from_u32(0xDA70D6FF);
	PALE_GOLDENROD:         palegoldenrod        = Self::from_u32(0xEEE8AAFF);
	PALE_GREEN:             palegreen            = Self::from_u32(0x98FB98FF);
	PALE_TURQUOISE:         paleturquoise        = Self::from_u32(0xAFEEEEFF);
	PALE_VIOLET_RED:        palevioletred        = Self::from_u32(0xDB7093FF);
	PAPAYA_WHIP:            papayawhip           = Self::from_u32(0xFFEFD5FF);
	PEACH_PUFF:             peachpuff            = Self::from_u32(0xFFDAB9FF);
	PERU:                   peru                 = Self::from_u32(0xCD853FFF);
	PINK:                   pink                 = Self::from_u32(0xFFC0CBFF);
	PLUM:                   plum                 = Self::from_u32(0xDDA0DDFF);
	POWDER_BLUE:            powderblue           = Self::from_u32(0xB0E0E6FF);
	PURPLE:                 purple               = Self::from_u32(0x800080FF);
	REBECCA_PURPLE:         rebeccapurple        = Self::from_u32(0x663399FF);
	RED:                    red                  = Self::from_u32(0xFF0000FF);
	ROSY_BROWN:             rosybrown            = Self::from_u32(0xBC8F8FFF);
	ROYAL_BLUE:             royalblue            = Self::from_u32(0x4169E1FF);
	SADDLE_BROWN:           saddlebrown          = Self::from_u32(0x8B4513FF);
	SALMON:                 salmon               = Self::from_u32(0xFA8072FF);
	SANDY_BROWN:            sandybrown           = Self::from_u32(0xF4A460FF);
	SEA_GREEN:              seagreen             = Self::from_u32(0x2E8B57FF);
	SEASHELL:               seashell             = Self::from_u32(0xFFF5EEFF);
	SIENNA:                 sienna               = Self::from_u32(0xA0522DFF);
	SILVER:                 silver               = Self::from_u32(0xC0C0C0FF);
	SKY_BLUE:               skyblue              = Self::from_u32(0x87CEEBFF);
	SLATE_BLUE:             slateblue            = Self::from_u32(0x6A5ACDFF);
	SLATE_GRAY:             slategray            = Self::from_u32(0x708090FF);
	SLATE_GREY:             slategrey            = Self::SLATE_GRAY;
	SNOW:                   snow                 = Self::from_u32(0xFFFAFAFF);
	SPRING_GREEN:           springgreen          = Self::from_u32(0x00FF7FFF);
	STEEL_BLUE:             steelblue            = Self::from_u32(0x4682B4FF);
	TAN:                    tan                  = Self::from_u32(0xD2B48CFF);
	TEAL:                   teal                 = Self::from_u32(0x008080FF);
	THISTLE:                thistle              = Self::from_u32(0xD8BFD8FF);
	TOMATO:                 tomato               = Self::from_u32(0xFF6347FF);
	TRANSPARENT:            transparent          = Self::from_u32(0x00000000);
	TURQUOISE:              turquoise            = Self::from_u32(0x40E0D0FF);
	VIOLET:                 violet               = Self::from_u32(0xEE82EEFF);
	WHEAT:                  wheat                = Self::from_u32(0xF5DEB3FF);
	WHITE:                  white                = Self::from_u32(0xFFFFFFFF);
	WHITE_SMOKE:            whitesmoke           = Self::from_u32(0xF5F5F5FF);
	YELLOW:                 yellow               = Self::from_u32(0xFFFF00FF);
	YELLOW_GREEN:           yellowgreen          = Self::from_u32(0x9ACD32FF);
}
