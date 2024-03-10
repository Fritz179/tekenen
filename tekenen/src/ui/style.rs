use std::{marker::PhantomData};

use crate::{colors, math::{clamp, IndefRange, Vec2}, shapes::{rect::Rect, Sides}, Pixel};


#[derive(Debug, Clone)]
pub struct FormattingInfo {
    pub containing_block: Rect,
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/length
// https://www.w3.org/TR/css-values-4/#lengths
#[derive(Debug, Clone)]
pub enum CSSLength<const POSITIVE_ONLY: bool = false> {
    Pixels(i32),
    Em(i32),
}

impl<const POSITIVE_ONLY: bool> CSSLength<POSITIVE_ONLY> {
    pub fn new_pixels(value: i32) -> Self {
        if POSITIVE_ONLY {
            assert!(value >= 0, "Value must be positive");
        }

        Self::Pixels(value)
    }

    pub fn new_rem(value: i32) -> Self {
        if POSITIVE_ONLY {
            assert!(value >= 0, "Value must be positive");
        }

        Self::Em(value)
    }

    pub fn solve(&self, context: &FormattingInfo) -> i32 {
        match self {
            Self::Pixels(pixels) => *pixels,
            Self::Em(rem) => todo!(),
        }
    }
}

impl<const POSITIVE_ONLY: bool> From<i32> for CSSLength<POSITIVE_ONLY> {
    fn from(value: i32) -> Self {
        Self::Pixels(value)
    }
}

// Used to determine what the percentage is relative to
pub trait PercentSolver {
    fn solve(value: f32, context: &FormattingInfo) -> i32;
}

/// refer to logical width of containing block 
#[derive(Debug, Clone)]
pub struct WidthOfContainingBlock;
impl PercentSolver for WidthOfContainingBlock {
    fn solve(value: f32, context: &FormattingInfo) -> i32 {
        (context.containing_block.size.x as f32 * value) as i32
    }
}

/// refer to logical height of containing block 
#[derive(Debug, Clone)]
pub struct HeightOfContainingBlock;
impl PercentSolver for HeightOfContainingBlock {
    fn solve(value: f32, context: &FormattingInfo) -> i32 {
        todo!()
    }
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/percentage
// https://drafts.csswg.org/css-values/#percentages
#[derive(Debug, Clone)]
pub struct CSSPercentage<Solver: PercentSolver, const POSITIVE_ONLY: bool = false> {
    value: f32,
    phantom: PhantomData<Solver>
}

impl<Solver: PercentSolver, const POSITIVE_ONLY: bool> CSSPercentage<Solver, POSITIVE_ONLY> {
    fn new(value: f32) -> Self {
        if POSITIVE_ONLY {
            assert!(value >= 0.0, "Value must be between positive");
        }

        Self {
            value,
            phantom: PhantomData
        }
    }

    fn solve(&self, context: &FormattingInfo) -> i32 {
        Solver::solve(self.value, context)
    }
}

impl<T: PercentSolver, const POSITIVE_ONLY: bool> From<f32> for CSSPercentage<T, POSITIVE_ONLY> {
    fn from(value: f32) -> Self {
        Self::new(value)
    }
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/length-percentage
#[derive(Debug, Clone)]
pub enum CSSLengthPercentage<Solver: PercentSolver, const POSITIVE_ONLY: bool = false> {
    Length(CSSLength<POSITIVE_ONLY>),
    Percentage(CSSPercentage<Solver, POSITIVE_ONLY>),
}

impl<Solver: PercentSolver, const POSITIVE_ONLY: bool> CSSLengthPercentage<Solver, POSITIVE_ONLY> {
    fn solve(&self, context: &FormattingInfo) -> i32 {
        match self {
            Self::Length(length) => length.solve(context),
            Self::Percentage(percent) => percent.solve(context),
        }
    }
}

impl<Solver: PercentSolver, const POSITIVE_ONLY: bool> From<CSSLength<POSITIVE_ONLY>> for CSSLengthPercentage<Solver, POSITIVE_ONLY> {
    fn from(value: CSSLength<POSITIVE_ONLY>) -> Self {
        Self::Length(value)
    }
}

impl<Solver: PercentSolver, const POSITIVE_ONLY: bool> From<CSSPercentage<Solver, POSITIVE_ONLY>> for CSSLengthPercentage<Solver, POSITIVE_ONLY> {
    fn from(value: CSSPercentage<Solver, POSITIVE_ONLY>) -> Self {
        Self::Percentage(value)
    }
}


// https://developer.mozilla.org/en-US/docs/Web/CSS/margin
// https://drafts.csswg.org/css-box/#margin

// Name:	margin-top, margin-right, margin-bottom, margin-left
// Value:	<length-percentage> | auto
// Initial:	0
// Applies to:	all elements except internal table elements
// Inherited:	no
// Percentages:	refer to logical width of containing block
// Computed value:	the keyword auto or a computed <length-percentage> value
// Canonical order:	per grammar
// Animation type:	by computed value type
// Logical property group:	margin

/// The margin CSS shorthand property sets the margin area on all four sides of an element.
#[derive(Debug, Clone)]
pub enum CSSMargin {
    LengthPercentage(CSSLengthPercentage<WidthOfContainingBlock>),
    Auto,
}

impl Default for CSSMargin {
    fn default() -> Self {
        Self::LengthPercentage(CSSLengthPercentage::Length(CSSLength::Pixels(0)))
    }
}

impl CSSMargin {
    pub fn new_length(value: CSSLength) -> Self {
        Self::LengthPercentage(value.into())
    }

    // pub fn new_percent(value: f32) -> Self {
    //     assert!(value >= 0.0 && value <= 1.0, "Value must be between 0 and 1");

    //     Self::LengthPercentage(value.into())
    // }

    pub fn new_auto() -> Self {
        Self::Auto
    }

    pub fn solve_or_auto(&self, context: &FormattingInfo) -> Option<i32> {
        match self {
            Self::LengthPercentage(value) => Some(value.solve(context)),
            _ => None
        }
    }
}

impl Sides<CSSMargin> {
    pub fn get_total_height(&self, context: &FormattingInfo) -> i32 {
        self.top.solve_or_auto(context).unwrap() + self.bottom.solve_or_auto(context).unwrap()
    }

    pub fn get_total_width(&self, context: &FormattingInfo) -> i32 {
        self.left.solve_or_auto(context).unwrap() + self.right.solve_or_auto(context).unwrap()
    }

    pub fn set(&mut self, sides: CSSLength) {
        self.top = CSSMargin::new_length(sides.clone());
        self.right = CSSMargin::new_length(sides.clone());
        self.bottom = CSSMargin::new_length(sides.clone());
        self.left = CSSMargin::new_length(sides);
    }

    pub fn set_2(&mut self, top_bottom: CSSLength, left_right: CSSLength) {
        self.top = CSSMargin::new_length(top_bottom.clone());
        self.right = CSSMargin::new_length(left_right.clone());
        self.bottom = CSSMargin::new_length(top_bottom);
        self.left = CSSMargin::new_length(left_right);
    }
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/padding
// https://drafts.csswg.org/css-box/#paddings

// Name:	padding-top, padding-right, padding-bottom, padding-left
// Value:	<length-percentage [0,∞]>
// Initial:	0
// Applies to:	all elements except: internal table elements other than table cells
// Inherited:	no
// Percentages:	refer to logical width of containing block
// Computed value:	a computed <length-percentage> value
// Canonical order:	per grammar
// Animation type:	by computed value type
// Logical property group:	padding

/// The padding CSS shorthand property sets the padding area on all four sides of an element at once.
#[derive(Debug, Clone)]
pub struct CSSPadding {
    value: CSSLengthPercentage<WidthOfContainingBlock, true>
}

impl Default for CSSPadding {
    fn default() -> Self {
        Self {
            value: CSSLengthPercentage::Length(CSSLength::Pixels(0))
        }
    }
}

impl CSSPadding {
    pub fn new_length(value: CSSLength<true>) -> Self {
        Self {
            value: value.into()
        }
    }

    pub fn solve(&self, context: &FormattingInfo) -> i32 {
        self.value.solve(context)
    }

    pub fn set(&mut self, value: CSSLength<true>) {
        self.value = value.into();
    }

    pub fn set_percent(&mut self, value: CSSPercentage<WidthOfContainingBlock, true>) {
        self.value = value.into();
    }
}

impl Sides<CSSPadding> {
    pub fn get_total_height(&self, context: &FormattingInfo) -> i32 {
        self.top.solve(context) + self.bottom.solve(context)
    }

    pub fn get_total_width(&self, context: &FormattingInfo) -> i32 {
        self.left.solve(context) + self.right.solve(context)
    }

    pub fn set(&mut self, sides: CSSLength<true>) {
        self.top.set(sides.clone());
        self.right.set(sides.clone());
        self.bottom.set(sides.clone());
        self.left.set(sides);
    }
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/line-style
// https://drafts.csswg.org/css-backgrounds/#typedef-line-style

/// <line-style> = none | hidden | dotted | dashed | solid | double | groove | ridge | inset | outset
#[derive(Debug, Clone)]
enum CSSLineStyle {
    /// No border. Color and width are ignored (i.e., the border has width 0). Note this means that the initial value of border-image-width will also resolve to zero.
    None,
    /// Same as none, but has different behavior in the border conflict resolution rules for border-collapsed tables [CSS2].
    Hidden,
    /// A series of round dots.
    Dotted,
    /// A series of square-ended dashes.
    Dashed,
    /// A single line segment.
    Solid,
    /// Two parallel solid lines with some space between them. (The thickness of the lines is not specified, but the sum of the lines and the space must equal border-width.)
    Double,
    /// Looks as if it were carved in the canvas. (This is typically achieved by creating a “shadow” from two colors that are slightly lighter and darker than the border-color.)
    Groove,
    /// Looks as if it were coming out of the canvas.
    Ridge,
    /// Looks as if the content on the inside of the border is sunken into the canvas. Treated as ridge in the collapsing border model. [CSS2]
    Inset,
    /// Looks as if the content on the inside of the border is coming out of the canvas. Treated as groove in the collapsing border model. [CSS2]
    Outset,
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/border-style
// https://drafts.csswg.org/css-backgrounds/#border-style

// Name:	border-top-style, border-right-style, border-bottom-style, border-left-style
// Value:	<line-style>
// Initial:	none
// Applies to:	all elements except ruby base containers and ruby annotation containers
// Inherited:	no
// Percentages:	N/A
// Computed value:	specified keyword
// Canonical order:	per grammar
// Animation type:	discrete
// Logical property group:	border-style

/// The border-style shorthand CSS property sets the line style for all four sides of an element's border.
#[derive(Debug, Clone)]
pub struct CSSBorderStyle {
    value: CSSLineStyle
}

impl Default for CSSBorderStyle {
    fn default() -> Self {
        Self {
            value: CSSLineStyle::None
        }
    }
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/color_value
// https://drafts.csswg.org/css-color/#color-syntax

// <color> = <color-base> | currentColor | <system-color> 

// <color-base> = <hex-color> | <color-function> | <named-color> | transparent
// <color-function> = <rgb()> | <rgba()> |
//               <hsl()> | <hsla()> | <hwb()> |
//               <lab()> | <lch()> | <oklab()> | <oklch()> |
//               <color()>

#[derive(Debug, Clone)]
enum CSSColor {
    /// A color value. See <color> values for the syntax of individual color values.
    ColorBase(Pixel),
    /// The value of the 'color' property. The computed value of the 'currentColor' keyword is the computed value of the 'color' property. If the 'currentColor' keyword is set on the 'color' property itself, it is treated as 'color: inherit'.
    CurrentColor,
    /// A system color. See <system-color> for the list of possible system colors.
    SystemColor
}

impl CSSColor {
    fn get_color(&self, context: &FormattingInfo) -> Pixel {
        match self {
            Self::ColorBase(pixel) => *pixel,
            Self::CurrentColor => todo!(),
            Self::SystemColor => todo!(),
        }
    }
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/border-width#line-width
// https://drafts.csswg.org/css-backgrounds/#typedef-line-width

// <line-width> = <length [0,∞]> | thin | medium | thick

#[derive(Debug, Clone)]
pub enum CSSLineWidth {
    Length(CSSLength<true>),
    /// 1px
    Thin,
    /// 3px
    Medium,
    /// 5px
    Thick
}

impl CSSLineWidth {
    pub fn solve(&self, context: &FormattingInfo) -> i32 {
        match self {
            Self::Length(length) => length.solve(context),
            Self::Thin => 1,
            Self::Medium => 3,
            Self::Thick => 5
        }
    }
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/border-width
// https://drafts.csswg.org/css-backgrounds/#the-border-width

// Name:	border-top-width, border-right-width, border-bottom-width, border-left-width
// Value:	<line-width>
// Initial:	medium
// Applies to:	all elements except ruby base containers and ruby annotation containers
// Inherited:	no
// Percentages:	N/A
// Computed value:	absolute length, snapped as a border width; zero if the border style is none or hidden
// Canonical order:	per grammar
// Animation type:	by computed value
// Logical property group:	border-width

#[derive(Debug, Clone)]
pub struct CSSBorderWidth {
    value: CSSLineWidth
}

impl Default for CSSBorderWidth {
    fn default() -> Self {
        Self {
            value: CSSLineWidth::Medium
        }
    }
}

impl CSSBorderWidth {
    pub fn new_length(value: CSSLength<true>) -> Self {
        Self {
            value: CSSLineWidth::Length(value)
        }
    }

    pub fn solve(&self, context: &FormattingInfo) -> i32 {
        self.value.solve(context)
    }
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/border-color
// https://drafts.csswg.org/css-backgrounds/#border-color

// Name:	border-top-color, border-right-color, border-bottom-color, border-left-color
// Value:	<color>
// Initial:	currentColor
// Applies to:	all elements except ruby base containers and ruby annotation containers
// Inherited:	no
// Percentages:	N/A
// Computed value:	computed color
// Canonical order:	per grammar
// Animation type:	by computed value
// Logical property group:	border-color

#[derive(Debug, Clone)]
pub struct CSSBorderColor {
    value: CSSColor
}

impl Default for CSSBorderColor {
    fn default() -> Self {
        Self {
            value: CSSColor::CurrentColor
        }
    }
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/border
// https://drafts.csswg.org/css-backgrounds/#propdef-border


// Name:	border-top, border-right, border-bottom, border-left
// Value:	<line-width> || <line-style> || <color>
// Initial:	See individual properties
// Applies to:	all elements except ruby base containers and ruby annotation containers
// Inherited:	no
// Percentages:	N/A
// Computed value:	see individual properties
// Canonical order:	per grammar
// Animation type:	see individual properties

#[derive(Debug, Clone, Default)]
pub struct CSSBorder {
    pub line_width: CSSBorderWidth,
    pub line_style: CSSBorderStyle,
    pub color: CSSBorderColor
}

impl CSSBorder {
    pub fn solve(&self, context: &FormattingInfo) -> i32 {
        match self.line_style.value {
            CSSLineStyle::None | CSSLineStyle::Hidden => 0,
            _ => self.line_width.solve(context)
        }
    }
}

impl Sides<CSSBorder> {
    pub fn get_total_height(&self, context: &FormattingInfo) -> i32 {
        self.top.solve(context) + self.bottom.solve(context)
    }

    pub fn get_total_width(&self, context: &FormattingInfo) -> i32 {
        self.left.solve(context) + self.right.solve(context)
    }

    pub fn set(&mut self, width: CSSLength<true>) {
        let size = CSSBorderWidth::new_length(width);
        self.top.line_width = size.clone();
        self.right.line_width = size.clone();
        self.bottom.line_width = size.clone();
        self.left.line_width = size;
    }
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/width
// https://drafts.csswg.org/css-sizing-3/#sizing-properties


// Name:	width, height
// Value:	auto | <length-percentage [0,∞]> | min-content | max-content | fit-content(<length-percentage [0,∞]>)
// Initial:	auto
// Applies to:	all elements except non-replaced inlines
// Inherited:	no
// Percentages:	relative to width/height of containing block
// Computed value:	as specified, with <length-percentage> values computed
// Canonical order:	per grammar
// Animation type:	by computed value type, recursing into fit-content()
// Logical property group:	size

// NOTE: width, height, min-width, min-height, max-width, max-height are similar
#[derive(Debug, Clone, Default)]
pub enum CSSSize<Solver: PercentSolver> {
    LengthPercentage(CSSLengthPercentage<Solver, true>),
    MinContent,
    MaxContent,
    FitContent(CSSLengthPercentage<Solver, true>),
    #[default]
    /// Auto for 'width', 'height', 'min-width', 'min-height'. None for 'max-width', 'max-height'
    AutoNone
}

impl<Solver: PercentSolver> CSSSize<Solver> {
    pub fn solve(&self, context: &FormattingInfo) -> Option<i32> {
        match self {
            Self::LengthPercentage(value) => Some(value.solve(context)),
            Self::MinContent => todo!(),
            Self::MaxContent => todo!(),
            Self::FitContent(value) => Some(clamp(
                value.solve(context), 
                Self::solve(&Self::MinContent, context).unwrap(), 
                Self::solve(&Self::MaxContent, context).unwrap(), 
            )),
            Self::AutoNone => None
        }
    }

    pub fn set(&mut self, value: CSSLength<true>) {
        *self = Self::LengthPercentage(value.into());
    }
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/display
// https://drafts.csswg.org/css-display/#the-display-properties

// Name:	display
// Value:	[ <display-outside> || <display-inside> ] | <display-listitem> | <display-internal> | <display-box> | <display-legacy>
// Initial:	inline
// Applies to:	all elements
// Inherited:	no
// Percentages:	n/a
// Computed value:	a pair of keywords representing the inner and outer display types plus optional list-item flag, or a <display-internal> or <display-box> keyword; see prose in a variety of specs for computation rules
// Canonical order:	per grammar
// Animation type:	not animatable

// <display-outside>  = block | inline | run-in
// <display-inside>   = flow | flow-root | table | flex | grid | ruby
// <display-listitem> = <display-outside>? && [ flow | flow-root ]? && list-item
// <display-internal> = table-row-group | table-header-group |
//                      table-footer-group | table-row | table-cell |
//                      table-column-group | table-column | table-caption |
//                      ruby-base | ruby-text | ruby-base-container |
//                      ruby-text-container
// <display-box>      = contents | none
// <display-legacy>   = inline-block | inline-table | inline-flex | inline-grid

#[derive(Debug, Clone, Default)]
pub enum CSSDisplay {
    Block,
    #[default]
    Inline,
    None,
    Flex
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/flex-direction
// https://drafts.csswg.org/css-flexbox/#flex-direction-property

// Name:	flex-direction
// Value:	row | row-reverse | column | column-reverse
// Initial:	row
// Applies to:	flex containers
// Inherited:	no
// Percentages:	n/a
// Computed value:	specified keyword
// Canonical order:	per grammar
// Animation type:	discrete

#[derive(Debug, Clone, Default)]
pub enum CSSFlexDirection {
    #[default]
    Row,
    RowReverse,
    Column,
    ColumnReverse
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/background-color
// https://drafts.csswg.org/css-backgrounds/#background-color

// Name:	background-color
// Value:	<color>
// Initial:	transparent
// Applies to:	all elements
// Inherited:	no
// Percentages:	N/A
// Computed value:	computed color
// Canonical order:	per grammar
// Animation type:	by computed value

#[derive(Debug, Clone)]
pub struct CSSBackgroundColor {
    color: CSSColor
}

impl CSSBackgroundColor {
    pub fn set(&mut self, color: Pixel) {
        self.color = CSSColor::ColorBase(color);
    }

    pub fn solve(&self, context: &FormattingInfo) -> Pixel {
        self.color.get_color(context)
    }
}

impl Default for CSSBackgroundColor {
    fn default() -> Self {
        Self {
            color: CSSColor::ColorBase(colors::css::TRANSPARENT)
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Style {
    pub margin: Sides<CSSMargin>,
    pub padding: Sides<CSSPadding>,
    pub border: Sides<CSSBorder>,
    pub width: CSSSize<WidthOfContainingBlock>,
    pub min_width: CSSSize<WidthOfContainingBlock>,
    pub max_width: CSSSize<WidthOfContainingBlock>,
    pub height: CSSSize<HeightOfContainingBlock>,
    pub min_height: CSSSize<HeightOfContainingBlock>,
    pub max_height: CSSSize<HeightOfContainingBlock>,
    pub display: CSSDisplay,
    pub flex_direction: CSSFlexDirection,
    pub background_color: CSSBackgroundColor
}

impl Style {
    // pub fn get_width_from_height(&self, target: &Ref<'_, dyn DomElement>, height: i32, context: &LayoutContext) -> i32 {
    //     let inner_height = height - self.get_total_bounding_height(context);
    //     let inner_width = target.get_width_from_height(inner_height, context);
    //     inner_width + self.get_total_bounding_width(context)
    // }

    // pub fn get_height_from_width(&self, target: &Ref<'_, dyn DomElement>, width: i32, context: &LayoutContext) -> i32 {
    //     let inner_width = width - self.get_total_bounding_width(context);
    //     let inner_height = target.get_height_from_width(inner_width, context);
    //     inner_height + self.get_total_bounding_height(context)
    // }

    pub fn get_computed_padding(&self, context: &FormattingInfo) -> Sides {
        Sides {
            top: self.padding.top.solve(context),
            right: self.padding.right.solve(context),
            bottom: self.padding.bottom.solve(context),
            left: self.padding.left.solve(context),
        }
    }

    pub fn get_computed_border(&self, context: &FormattingInfo) -> Sides {
        Sides {
            top: self.border.top.solve(context),
            right: self.border.right.solve(context),
            bottom: self.border.bottom.solve(context),
            left: self.border.left.solve(context),
        }
    }

    pub fn get_computed_margin(&self, context: &FormattingInfo) -> Sides {
        Sides {
            top: self.margin.top.solve_or_auto(context).unwrap_or(0),
            right: self.margin.right.solve_or_auto(context).unwrap_or(0),
            bottom: self.margin.bottom.solve_or_auto(context).unwrap_or(0),
            left: self.margin.left.solve_or_auto(context).unwrap_or(0),
        }
    }

    pub fn get_total_computed_boudning(&self, context: &FormattingInfo) -> Sides {
        self.get_computed_margin(context) + self.get_computed_padding(context) + self.get_computed_border(context)
    }

    pub fn get_height_constrains(&self, context: &FormattingInfo) -> IndefRange {
        if let Some(height) = self.height.solve(context) {
            return IndefRange::new_definite(height)
        }

        IndefRange::new_option(self.min_height.solve(context), self.max_height.solve(context))
    }

    pub fn get_width_constrains(&self, context: &FormattingInfo) -> IndefRange {
        if let Some(width) = self.width.solve(context) {
            return IndefRange::new_definite(width)
        }

        IndefRange::new_option(self.min_width.solve(context), self.max_width.solve(context))
    }

    pub fn get_size_contraint(&self, context: &FormattingInfo) -> Vec2<IndefRange> {
        Vec2::new(self.get_width_constrains(context), self.get_height_constrains(context))
    }

    // pub fn get_min_max_margin_area(&self, target: &Ref<'_, dyn DomElement>, context: &LayoutContext) -> Vec2<IndefRange> {
    //     let mut base = target.get_inner_min_max_content(context);

    //     base.x += self.get_total_bounding_width(context);
    //     base.y += self.get_total_bounding_height(context);

    //     base
    // }

    // pub fn get_min_max_border_area(&self, target: &Ref<'_, dyn DomElement>, context: &LayoutContext) -> (Vec2<IndefRange>, Sides) {
    //     let mut base = target.get_inner_min_max_content(context);

    //     base.x += self.padding.get_total_width(context) + self.border.get_total_width(context);
    //     base.y += self.padding.get_total_height(context) + self.border.get_total_height(context);

    //     (base, self.get_computed_margin(context))
    // }
}