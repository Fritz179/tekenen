use super::Pixel;

// https://developer.mozilla.org/en-US/docs/Web/CSS/named-color
// https://drafts.csswg.org/css-color/#named-colors

pub const BLACK  : Pixel = [0x00, 0x00, 0x00, 0xff]; // #000000
pub const SILVER : Pixel = [0xc0, 0xc0, 0xc0, 0xff]; // #c0c0c0
pub const GRAY   : Pixel = [0x80, 0x80, 0x80, 0xff]; // #808080
pub const WHITE  : Pixel = [0xff, 0xff, 0xff, 0xff]; // #ffffff
pub const MAROON : Pixel = [0x80, 0x00, 0x00, 0xff]; // #800000
pub const RED    : Pixel = [0xff, 0x00, 0x00, 0xff]; // #ff0000
pub const PURPLE : Pixel = [0x80, 0x00, 0x80, 0xff]; // #800080
pub const FUCHSIA: Pixel = [0xff, 0x00, 0xff, 0xff]; // #ff00ff (same as magenta)
pub const MAGENTA: Pixel = [0xff, 0x00, 0xff, 0xff]; // #ff00ff (same as fuchsia)
pub const GREEN  : Pixel = [0x00, 0x80, 0x00, 0xff]; // #008000
pub const LIME   : Pixel = [0x00, 0xff, 0x00, 0xff]; // #00ff00
pub const OLIVE  : Pixel = [0x80, 0x80, 0x00, 0xff]; // #808000
pub const YELLOW : Pixel = [0xff, 0xff, 0x00, 0xff]; // #ffff00
pub const NAVY   : Pixel = [0x00, 0x00, 0x80, 0xff]; // #000080
pub const BLUE   : Pixel = [0x00, 0x00, 0xff, 0xff]; // #0000ff
pub const TEAL   : Pixel = [0x00, 0x80, 0x80, 0xff]; // #008080
pub const AQUA   : Pixel = [0x00, 0xff, 0xff, 0xff]; // #00ffff (same as cyan)
pub const CYAN   : Pixel = [0x00, 0xff, 0xff, 0xff]; // #00ffff (same as aqua)
pub const ORANGE : Pixel = [0xff, 0xa5, 0x00, 0xff]; // #ffa500

// My favorite color ;-)
pub const FRITZ_GRAY: Pixel = [0x33, 0x33, 0x33, 0xff]; // #333333

pub mod css {
    use super::Pixel;

    pub const ALICEBLUE           : Pixel = [0xf0, 0xf8, 0xff, 0xff]; // #f0f8ff
    pub const ANTIQUEWHITE        : Pixel = [0xfa, 0xeb, 0xd7, 0xff]; // #faebd7
    pub const AQUA                : Pixel = [0x00, 0xff, 0xff, 0xff]; // #00ffff
    pub const AQUAMARINE          : Pixel = [0x7f, 0xff, 0xd4, 0xff]; // #7fffd4
    pub const AZURE               : Pixel = [0xf0, 0xff, 0xff, 0xff]; // #f0ffff
    pub const BEIGE               : Pixel = [0xf5, 0xf5, 0xdc, 0xff]; // #f5f5dc
    pub const BISQUE              : Pixel = [0xff, 0xe4, 0xc4, 0xff]; // #ffe4c4
    pub const BLACK               : Pixel = [0x00, 0x00, 0x00, 0xff]; // #000000
    pub const BLANCHEDALMOND      : Pixel = [0xff, 0xeb, 0xcd, 0xff]; // #ffebcd
    pub const BLUE                : Pixel = [0x00, 0x00, 0xff, 0xff]; // #0000ff
    pub const BLUEVIOLET          : Pixel = [0x8a, 0x2b, 0xe2, 0xff]; // #8a2be2
    pub const BROWN               : Pixel = [0xa5, 0x2a, 0x2a, 0xff]; // #a52a2a
    pub const BURLYWOOD           : Pixel = [0xde, 0xb8, 0x87, 0xff]; // #deb887
    pub const CADETBLUE           : Pixel = [0x5f, 0x9e, 0xa0, 0xff]; // #5f9ea0
    pub const CHARTREUSE          : Pixel = [0x7f, 0xff, 0x00, 0xff]; // #7fff00
    pub const CHOCOLATE           : Pixel = [0xd2, 0x69, 0x1e, 0xff]; // #d2691e
    pub const CORAL               : Pixel = [0xff, 0x7f, 0x50, 0xff]; // #ff7f50
    pub const CORNFLOWERBLUE      : Pixel = [0x64, 0x95, 0xed, 0xff]; // #6495ed
    pub const CORNSILK            : Pixel = [0xff, 0xf8, 0xdc, 0xff]; // #fff8dc
    pub const CRIMSON             : Pixel = [0xdc, 0x14, 0x3c, 0xff]; // #dc143c
    pub const CYAN                : Pixel = [0x00, 0xff, 0xff, 0xff]; // #00ffff (synonym of aqua)
    pub const DARKBLUE            : Pixel = [0x00, 0x00, 0x8b, 0xff]; // #00008b
    pub const DARKCYAN            : Pixel = [0x00, 0x8b, 0x8b, 0xff]; // #008b8b
    pub const DARKGOLDENROD       : Pixel = [0xb8, 0x86, 0x0b, 0xff]; // #b8860b
    pub const DARKGRAY            : Pixel = [0xa9, 0xa9, 0xa9, 0xff]; // #a9a9a9
    pub const DARKGREEN           : Pixel = [0x00, 0x64, 0x00, 0xff]; // #006400
    pub const DARKGREY            : Pixel = [0xa9, 0xa9, 0xa9, 0xff]; // #a9a9a9
    pub const DARKKHAKI           : Pixel = [0xbd, 0xb7, 0x6b, 0xff]; // #bdb76b
    pub const DARKMAGENTA         : Pixel = [0x8b, 0x00, 0x8b, 0xff]; // #8b008b
    pub const DARKOLIVEGREEN      : Pixel = [0x55, 0x6b, 0x2f, 0xff]; // #556b2f
    pub const DARKORANGE          : Pixel = [0xff, 0x8c, 0x00, 0xff]; // #ff8c00
    pub const DARKORCHID          : Pixel = [0x99, 0x32, 0xcc, 0xff]; // #9932cc
    pub const DARKRED             : Pixel = [0x8b, 0x00, 0x00, 0xff]; // #8b0000
    pub const DARKSALMON          : Pixel = [0xe9, 0x96, 0x7a, 0xff]; // #e9967a
    pub const DARKSEAGREEN        : Pixel = [0x8f, 0xbc, 0x8f, 0xff]; // #8fbc8f
    pub const DARKSLATEBLUE       : Pixel = [0x48, 0x3d, 0x8b, 0xff]; // #483d8b
    pub const DARKSLATEGRAY       : Pixel = [0x2f, 0x4f, 0x4f, 0xff]; // #2f4f4f
    pub const DARKSLATEGREY       : Pixel = [0x2f, 0x4f, 0x4f, 0xff]; // #2f4f4f
    pub const DARKTURQUOISE       : Pixel = [0x00, 0xce, 0xd1, 0xff]; // #00ced1
    pub const DARKVIOLET          : Pixel = [0x94, 0x00, 0xd3, 0xff]; // #9400d3
    pub const DEEPPINK            : Pixel = [0xff, 0x14, 0x93, 0xff]; // #ff1493
    pub const DEEPSKYBLUE         : Pixel = [0x00, 0xbf, 0xff, 0xff]; // #00bfff
    pub const DIMGRAY             : Pixel = [0x69, 0x69, 0x69, 0xff]; // #696969
    pub const DIMGREY             : Pixel = [0x69, 0x69, 0x69, 0xff]; // #696969
    pub const DODGERBLUE          : Pixel = [0x1e, 0x90, 0xff, 0xff]; // #1e90ff
    pub const FIREBRICK           : Pixel = [0xb2, 0x22, 0x22, 0xff]; // #b22222
    pub const FLORALWHITE         : Pixel = [0xff, 0xfa, 0xf0, 0xff]; // #fffaf0
    pub const FORESTGREEN         : Pixel = [0x22, 0x8b, 0x22, 0xff]; // #228b22
    pub const FUCHSIA             : Pixel = [0xff, 0x00, 0xff, 0xff]; // #ff00ff
    pub const GAINSBORO           : Pixel = [0xdc, 0xdc, 0xdc, 0xff]; // #dcdcdc
    pub const GHOSTWHITE          : Pixel = [0xf8, 0xf8, 0xff, 0xff]; // #f8f8ff
    pub const GOLD                : Pixel = [0xff, 0xd7, 0x00, 0xff]; // #ffd700
    pub const GOLDENROD           : Pixel = [0xda, 0xa5, 0x20, 0xff]; // #daa520
    pub const GRAY                : Pixel = [0x80, 0x80, 0x80, 0xff]; // #808080
    pub const GREEN               : Pixel = [0x00, 0x80, 0x00, 0xff]; // #008000
    pub const GREENYELLOW         : Pixel = [0xad, 0xff, 0x2f, 0xff]; // #adff2f
    pub const GREY                : Pixel = [0x80, 0x80, 0x80, 0xff]; // #808080 (synonym of gray)
    pub const HONEYDEW            : Pixel = [0xf0, 0xff, 0xf0, 0xff]; // #f0fff0
    pub const HOTPINK             : Pixel = [0xff, 0x69, 0xb4, 0xff]; // #ff69b4
    pub const INDIANRED           : Pixel = [0xcd, 0x5c, 0x5c, 0xff]; // #cd5c5c
    pub const INDIGO              : Pixel = [0x4b, 0x00, 0x82, 0xff]; // #4b0082
    pub const IVORY               : Pixel = [0xff, 0xff, 0xf0, 0xff]; // #fffff0
    pub const KHAKI               : Pixel = [0xf0, 0xe6, 0x8c, 0xff]; // #f0e68c
    pub const LAVENDER            : Pixel = [0xe6, 0xe6, 0xfa, 0xff]; // #e6e6fa
    pub const LAVENDERBLUSH       : Pixel = [0xff, 0xf0, 0xf5, 0xff]; // #fff0f5
    pub const LAWNGREEN           : Pixel = [0x7c, 0xfc, 0x00, 0xff]; // #7cfc00
    pub const LEMONCHIFFON        : Pixel = [0xff, 0xfa, 0xcd, 0xff]; // #fffacd
    pub const LIGHTBLUE           : Pixel = [0xad, 0xd8, 0xe6, 0xff]; // #add8e6
    pub const LIGHTCORAL          : Pixel = [0xf0, 0x80, 0x80, 0xff]; // #f08080
    pub const LIGHTCYAN           : Pixel = [0xe0, 0xff, 0xff, 0xff]; // #e0ffff
    pub const LIGHTGOLDENRODYELLOW: Pixel = [0xfa, 0xfa, 0xd2, 0xff]; // #fafad2
    pub const LIGHTGRAY           : Pixel = [0xd3, 0xd3, 0xd3, 0xff]; // #d3d3d3
    pub const LIGHTGREEN          : Pixel = [0x90, 0xee, 0x90, 0xff]; // #90ee90
    pub const LIGHTGREY           : Pixel = [0xd3, 0xd3, 0xd3, 0xff]; // #d3d3d3
    pub const LIGHTPINK           : Pixel = [0xff, 0xb6, 0xc1, 0xff]; // #ffb6c1
    pub const LIGHTSALMON         : Pixel = [0xff, 0xa0, 0x7a, 0xff]; // #ffa07a
    pub const LIGHTSEAGREEN       : Pixel = [0x20, 0xb2, 0xaa, 0xff]; // #20b2aa
    pub const LIGHTSKYBLUE        : Pixel = [0x87, 0xce, 0xfa, 0xff]; // #87cefa
    pub const LIGHTSLATEGRAY      : Pixel = [0x77, 0x88, 0x99, 0xff]; // #778899
    pub const LIGHTSLATEGREY      : Pixel = [0x77, 0x88, 0x99, 0xff]; // #778899
    pub const LIGHTSTEELBLUE      : Pixel = [0xb0, 0xc4, 0xde, 0xff]; // #b0c4de
    pub const LIGHTYELLOW         : Pixel = [0xff, 0xff, 0xe0, 0xff]; // #ffffe0
    pub const LIME                : Pixel = [0x00, 0xff, 0x00, 0xff]; // #00ff00
    pub const LIMEGREEN           : Pixel = [0x32, 0xcd, 0x32, 0xff]; // #32cd32
    pub const LINEN               : Pixel = [0xfa, 0xf0, 0xe6, 0xff]; // #faf0e6
    pub const MAGENTA             : Pixel = [0xff, 0x00, 0xff, 0xff]; // #ff00ff (synonym of fuchsia)
    pub const MAROON              : Pixel = [0x80, 0x00, 0x00, 0xff]; // #800000
    pub const MEDIUMAQUAMARINE    : Pixel = [0x66, 0xcd, 0xaa, 0xff]; // #66cdaa
    pub const MEDIUMBLUE          : Pixel = [0x00, 0x00, 0xcd, 0xff]; // #0000cd
    pub const MEDIUMORCHID        : Pixel = [0xba, 0x55, 0xd3, 0xff]; // #ba55d3
    pub const MEDIUMPURPLE        : Pixel = [0x93, 0x70, 0xdb, 0xff]; // #9370db
    pub const MEDIUMSEAGREEN      : Pixel = [0x3c, 0xb3, 0x71, 0xff]; // #3cb371
    pub const MEDIUMSLATEBLUE     : Pixel = [0x7b, 0x68, 0xee, 0xff]; // #7b68ee
    pub const MEDIUMSPRINGGREEN   : Pixel = [0x00, 0xfa, 0x9a, 0xff]; // #00fa9a
    pub const MEDIUMTURQUOISE     : Pixel = [0x48, 0xd1, 0xcc, 0xff]; // #48d1cc
    pub const MEDIUMVIOLETRED     : Pixel = [0xc7, 0x15, 0x85, 0xff]; // #c71585
    pub const MIDNIGHTBLUE        : Pixel = [0x19, 0x19, 0x70, 0xff]; // #191970
    pub const MINTCREAM           : Pixel = [0xf5, 0xff, 0xfa, 0xff]; // #f5fffa
    pub const MISTYROSE           : Pixel = [0xff, 0xe4, 0xe1, 0xff]; // #ffe4e1
    pub const MOCCASIN            : Pixel = [0xff, 0xe4, 0xb5, 0xff]; // #ffe4b5
    pub const NAVAJOWHITE         : Pixel = [0xff, 0xde, 0xad, 0xff]; // #ffdead
    pub const NAVY                : Pixel = [0x00, 0x00, 0x80, 0xff]; // #000080
    pub const OLDLACE             : Pixel = [0xfd, 0xf5, 0xe6, 0xff]; // #fdf5e6
    pub const OLIVE               : Pixel = [0x80, 0x80, 0x00, 0xff]; // #808000
    pub const OLIVEDRAB           : Pixel = [0x6b, 0x8e, 0x23, 0xff]; // #6b8e23
    pub const ORANGE              : Pixel = [0xff, 0xa5, 0x00, 0xff]; // #ffa500
    pub const ORANGERED           : Pixel = [0xff, 0x45, 0x00, 0xff]; // #ff4500
    pub const ORCHID              : Pixel = [0xda, 0x70, 0xd6, 0xff]; // #da70d6
    pub const PALEGOLDENROD       : Pixel = [0xee, 0xe8, 0xaa, 0xff]; // #eee8aa
    pub const PALEGREEN           : Pixel = [0x98, 0xfb, 0x98, 0xff]; // #98fb98
    pub const PALETURQUOISE       : Pixel = [0xaf, 0xee, 0xee, 0xff]; // #afeeee
    pub const PALEVIOLETRED       : Pixel = [0xdb, 0x70, 0x93, 0xff]; // #db7093
    pub const PAPAYAWHIP          : Pixel = [0xff, 0xef, 0xd5, 0xff]; // #ffefd5
    pub const PEACHPUFF           : Pixel = [0xff, 0xda, 0xb9, 0xff]; // #ffdab9
    pub const PERU                : Pixel = [0xcd, 0x85, 0x3f, 0xff]; // #cd853f
    pub const PINK                : Pixel = [0xff, 0xc0, 0xcb, 0xff]; // #ffc0cb
    pub const PLUM                : Pixel = [0xdd, 0xa0, 0xdd, 0xff]; // #dda0dd
    pub const POWDERBLUE          : Pixel = [0xb0, 0xe0, 0xe6, 0xff]; // #b0e0e6
    pub const PURPLE              : Pixel = [0x80, 0x00, 0x80, 0xff]; // #800080
    pub const REBECCAPURPLE       : Pixel = [0x66, 0x33, 0x99, 0xff]; // #663399
    pub const RED                 : Pixel = [0xff, 0x00, 0x00, 0xff]; // #ff0000
    pub const ROSYBROWN           : Pixel = [0xbc, 0x8f, 0x8f, 0xff]; // #bc8f8f
    pub const ROYALBLUE           : Pixel = [0x41, 0x69, 0xe1, 0xff]; // #4169e1
    pub const SADDLEBROWN         : Pixel = [0x8b, 0x45, 0x13, 0xff]; // #8b4513
    pub const SALMON              : Pixel = [0xfa, 0x80, 0x72, 0xff]; // #fa8072
    pub const SANDYBROWN          : Pixel = [0xf4, 0xa4, 0x60, 0xff]; // #f4a460
    pub const SEAGREEN            : Pixel = [0x2e, 0x8b, 0x57, 0xff]; // #2e8b57
    pub const SEASHELL            : Pixel = [0xff, 0xf5, 0xee, 0xff]; // #fff5ee
    pub const SIENNA              : Pixel = [0xa0, 0x52, 0x2d, 0xff]; // #a0522d
    pub const SILVER              : Pixel = [0xc0, 0xc0, 0xc0, 0xff]; // #c0c0c0
    pub const SKYBLUE             : Pixel = [0x87, 0xce, 0xeb, 0xff]; // #87ceeb
    pub const SLATEBLUE           : Pixel = [0x6a, 0x5a, 0xcd, 0xff]; // #6a5acd
    pub const SLATEGRAY           : Pixel = [0x70, 0x80, 0x90, 0xff]; // #708090
    pub const SLATEGREY           : Pixel = [0x70, 0x80, 0x90, 0xff]; // #708090
    pub const SNOW                : Pixel = [0xff, 0xfa, 0xfa, 0xff]; // #fffafa
    pub const SPRINGGREEN         : Pixel = [0x00, 0xff, 0x7f, 0xff]; // #00ff7f
    pub const STEELBLUE           : Pixel = [0x46, 0x82, 0xb4, 0xff]; // #4682b4
    pub const TAN                 : Pixel = [0xd2, 0xb4, 0x8c, 0xff]; // #d2b48c
    pub const TEAL                : Pixel = [0x00, 0x80, 0x80, 0xff]; // #008080
    pub const THISTLE             : Pixel = [0xd8, 0xbf, 0xd8, 0xff]; // #d8bfd8
    pub const TOMATO              : Pixel = [0xff, 0x63, 0x47, 0xff]; // #ff6347
    pub const TRANSPARENT         : Pixel = [0x00, 0x00, 0x00, 0x00]; // #00000000
    pub const TURQUOISE           : Pixel = [0x40, 0xe0, 0xd0, 0xff]; // #40e0d0
    pub const VIOLET              : Pixel = [0xee, 0x82, 0xee, 0xff]; // #ee82ee
    pub const WHEAT               : Pixel = [0xf5, 0xde, 0xb3, 0xff]; // #f5deb3
    pub const WHITE               : Pixel = [0xff, 0xff, 0xff, 0xff]; // #ffffff
    pub const WHITESMOKE          : Pixel = [0xf5, 0xf5, 0xf5, 0xff]; // #f5f5f5
    pub const YELLOW              : Pixel = [0xff, 0xff, 0x00, 0xff]; // #ffff00
    pub const YELLOWGREEN         : Pixel = [0x9a, 0xcd, 0x32, 0xff]; // #9acd32
}