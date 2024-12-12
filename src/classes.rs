use std::{collections::HashSet, hash::Hash};

use crate::utilities::align_self::align_self_css;
use crate::utilities::bg_img::bg_img_css;
use crate::utilities::flex::*;
use crate::utilities::font_size::font_size_css;
use crate::utilities::h::h_css;
use crate::utilities::hide::*;
use crate::utilities::line_height::line_height_css;
use crate::utilities::p::*;
use crate::utilities::p_child::*;
use crate::utilities::p_recursive::*;
use crate::utilities::ratio::ratio_css;
use crate::utilities::relative::relative_css;
use crate::utilities::w::w_css;
use crate::utilities::z_index::z_index_css;

#[derive(Debug, PartialEq, Hash, Eq)]
pub enum LayoutClass<'a> {
    // Component Classes
    MaxWidth(&'a str),
    MinCellWidth(&'a str),
    MinCols(&'a str),
    MaxCols(&'a str),
    Recursive,
    Screen,
    TwinWidth,
    NoWrap,
    HideBar,
    AndText,
    Grow,
    Gap(&'a str),
    GapX(&'a str),
    GapY(&'a str),
    GapDir(&'a str),
    Scale(&'a str),
    Align(&'a str),
    Position(&'a str),
    Top(&'a str),
    Bottom(&'a str),
    Left(&'a str),
    Right(&'a str),
    Height(&'a str),
    ItemWidth(&'a str),
    MinHeight(&'a str),
    MaxHeight(&'a str),
    SideWidth(&'a str),
    Side(&'a str),
    ContentMin(&'a str),
    Threshold(&'a str),
    Limit(&'a str),
    KeepP,
    KeepPL,
    KeepPR,
    KeepCenter,
    Shrink,
    Cols(&'a str),
    Rows(&'a str),

    // Utility Classes
    AlignSelf(&'a str),
    BgImg(&'a str),
    FlexBasis(&'a str),
    FlexGrow(&'a str),
    FlexShrink(&'a str),
    FontSize(&'a str),
    H(&'a str),
    HideOver(&'a str),
    HideUnder(&'a str),
    LineHeight(&'a str),
    Justify(&'a str),
    P(&'a str),
    PT(&'a str),
    PB(&'a str),
    PL(&'a str),
    PR(&'a str),
    PX(&'a str),
    PY(&'a str),
    PChild(&'a str),
    PTChild(&'a str),
    PBChild(&'a str),
    PLChild(&'a str),
    PRChild(&'a str),
    PXChild(&'a str),
    PYChild(&'a str),
    PRecursive(&'a str),
    PTRecursive(&'a str),
    PBRecursive(&'a str),
    PLRecursive(&'a str),
    PRRecursive(&'a str),
    PXRecursive(&'a str),
    PYRecursive(&'a str),
    Ratio(&'a str),
    Relative,
    W(&'a str),
    ZIndex(&'a str),
}

impl<'a> TryFrom<&'a str> for LayoutClass<'a> {
    type Error = ();

    fn try_from(input: &'a str) -> Result<LayoutClass<'a>, Self::Error> {
        let colon_index = input.find(':');
        let (class_name, class_value) = if let Some(i) = colon_index {
            //In the case where their is ':' but nothing after
            // this syntax is wrong so we return an error
            if input.len() <= i + 1 {
                return Err(());
            }
            (Some(&input[..i]), Some(&input[i + 1..]))
        } else {
            (Some(&input[..]), None)
        };
        match (class_name, class_value) {
            // all layout classes with value
            (Some(class), Some(value)) => match class {
                "max-width" => Ok(LayoutClass::MaxWidth(value)),
                "min-cell-width" => Ok(LayoutClass::MinCellWidth(value)),
                "min-cols" => Ok(LayoutClass::MinCols(value)),
                "max-cols" => Ok(LayoutClass::MaxCols(value)),
                "gap" => Ok(LayoutClass::Gap(value)),
                "gap-x" => Ok(LayoutClass::GapX(value)),
                "gap-y" => Ok(LayoutClass::GapY(value)),
                "gap-dir" => Ok(LayoutClass::GapDir(value)),
                "scale" => Ok(LayoutClass::Scale(value)),
                "align" => Ok(LayoutClass::Align(value)),
                "position" => Ok(LayoutClass::Position(value)),
                "top" => Ok(LayoutClass::Top(value)),
                "bottom" => Ok(LayoutClass::Bottom(value)),
                "left" => Ok(LayoutClass::Left(value)),
                "right" => Ok(LayoutClass::Right(value)),
                "height" => Ok(LayoutClass::Height(value)),
                "item-width" => Ok(LayoutClass::ItemWidth(value)),
                "min-height" => Ok(LayoutClass::MinHeight(value)),
                "max-height" => Ok(LayoutClass::MaxHeight(value)),
                "side" => Ok(LayoutClass::Side(value)),
                "side-width" => Ok(LayoutClass::SideWidth(value)),
                "content-min" => Ok(LayoutClass::ContentMin(value)),
                "threshold" => Ok(LayoutClass::Threshold(value)),
                "limit" => Ok(LayoutClass::Limit(value)),

                "align-self" => Ok(LayoutClass::AlignSelf(value)),
                "bg-img" => Ok(LayoutClass::BgImg(value)),
                "flex-basis" => Ok(LayoutClass::FlexBasis(value)),
                "flex-grow" => Ok(LayoutClass::FlexGrow(value)),
                "flex-shrink" => Ok(LayoutClass::FlexShrink(value)),
                "font-size" => Ok(LayoutClass::FontSize(value)),
                "h" => Ok(LayoutClass::H(value)),
                "hide-over" => Ok(LayoutClass::HideOver(value)),
                "hide-under" => Ok(LayoutClass::HideUnder(value)),
                "line-height" => Ok(LayoutClass::LineHeight(value)),
                "p" => Ok(LayoutClass::P(value)),
                "pt" => Ok(LayoutClass::PT(value)),
                "pb" => Ok(LayoutClass::PB(value)),
                "pl" => Ok(LayoutClass::PL(value)),
                "pr" => Ok(LayoutClass::PR(value)),
                "px" => Ok(LayoutClass::PX(value)),
                "py" => Ok(LayoutClass::PY(value)),
                "p-child" => Ok(LayoutClass::PChild(value)),
                "pt-child" => Ok(LayoutClass::PTChild(value)),
                "pb-child" => Ok(LayoutClass::PBChild(value)),
                "pl-child" => Ok(LayoutClass::PLChild(value)),
                "pr-child" => Ok(LayoutClass::PRChild(value)),
                "px-child" => Ok(LayoutClass::PXChild(value)),
                "py-child" => Ok(LayoutClass::PYChild(value)),

                "p-recursive" => Ok(LayoutClass::PRecursive(value)),
                "pt-recursive" => Ok(LayoutClass::PTRecursive(value)),
                "pb-recursive" => Ok(LayoutClass::PBRecursive(value)),
                "pl-recursive" => Ok(LayoutClass::PLRecursive(value)),
                "pr-recursive" => Ok(LayoutClass::PRRecursive(value)),
                "px-recursive" => Ok(LayoutClass::PXRecursive(value)),
                "py-recursive" => Ok(LayoutClass::PYRecursive(value)),


                "justify" => Ok(LayoutClass::Justify(value)),


                "ratio" => Ok(LayoutClass::Ratio(value)),
                "w" => Ok(LayoutClass::W(value)),
                "z-index" => Ok(LayoutClass::ZIndex(value)),

                c if c.starts_with("rows-") => Ok(LayoutClass::Rows(input)),
                c if c.starts_with("cols-") => Ok(LayoutClass::Cols(input)),

                _ => Err(()),
            },
            // all layout class without value
            (Some(class), None) => match class {
                "recursive" => Ok(LayoutClass::Recursive),
                "screen" => Ok(LayoutClass::Screen),
                "twin-width" => Ok(LayoutClass::TwinWidth),
                "nowrap" => Ok(LayoutClass::NoWrap),
                "hide-bar" => Ok(LayoutClass::HideBar),
                "relative" => Ok(LayoutClass::Relative),
                "grow" => Ok(LayoutClass::Grow),
                "keep-p" => Ok(LayoutClass::KeepP),
                "keep-pl" => Ok(LayoutClass::KeepPL),
                "keep-pr" => Ok(LayoutClass::KeepPR),
                "keep-center" => Ok(LayoutClass::KeepCenter),
                "shrink" => Ok(LayoutClass::Shrink),
                "and-text" => Ok(LayoutClass::AndText),
                _ => Err(()),
            },
            _ => Err(()),
        }
    }
}

impl<'a> LayoutClass<'a> {
    /// Generate the css of the layout class which are utilities
    /// and insert it inside the Hashset passed.
    /// This method consumes the LayoutClass, because we dont need it anymore
    /// after we get the css.
    pub fn insert_css(self, harmonic_ratio: f64, set: &mut HashSet<String>) {
        match self {
            Self::AlignSelf(value) => align_self_css(value, set),
            Self::BgImg(value) => bg_img_css(value, set),
            Self::FlexBasis(value) => flex_basis_css(value, set),
            Self::FlexGrow(value) => flex_grow_css(value, set),
            Self::FlexShrink(value) => flex_shrink_css(value, set),
            Self::FontSize(value) => font_size_css(value, harmonic_ratio, set),
            Self::H(value) => h_css(value, harmonic_ratio, set),
            Self::HideOver(value) => hide_over_css(value, set),
            Self::HideUnder(value) => hide_under_css(value, set),
            Self::LineHeight(value) => line_height_css(value, set),
            Self::P(value) => p_css(value, harmonic_ratio, set),
            Self::PT(value) => pt_css(value, harmonic_ratio, set),
            Self::PB(value) => pb_css(value, harmonic_ratio, set),
            Self::PL(value) => pl_css(value, harmonic_ratio, set),
            Self::PR(value) => pr_css(value, harmonic_ratio, set),
            Self::PX(value) => px_css(value, harmonic_ratio, set),
            Self::PY(value) => py_css(value, harmonic_ratio, set),
            Self::PTChild(value) => pt_child_css(value, harmonic_ratio, set),
            Self::PBChild(value) => pb_child_css(value, harmonic_ratio, set),
            Self::PLChild(value) => pl_child_css(value, harmonic_ratio, set),
            Self::PRChild(value) => pr_child_css(value, harmonic_ratio, set),
            Self::PXChild(value) => px_child_css(value, harmonic_ratio, set),
            Self::PYChild(value) => py_child_css(value, harmonic_ratio, set),
            Self::PTRecursive(value) => pt_recursive_css(value, harmonic_ratio, set),
            Self::PBRecursive(value) => pb_recursive_css(value, harmonic_ratio, set),
            Self::PLRecursive(value) => pl_recursive_css(value, harmonic_ratio, set),
            Self::PRRecursive(value) => pr_recursive_css(value, harmonic_ratio, set),
            Self::PXRecursive(value) => px_recursive_css(value, harmonic_ratio, set),
            Self::PYRecursive(value) => py_recursive_css(value, harmonic_ratio, set),
            Self::Ratio(value) => ratio_css(value, set),
            Self::Relative => relative_css(set),
            Self::W(value) => w_css(value, harmonic_ratio, set),
            Self::ZIndex(value) => z_index_css(value, set),
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_layout_class_for_rows_area() {
        let layout_class = "cols-3:440px";
        let cols_variant = LayoutClass::try_from(layout_class);
        //assert_eq!(cols_variant, LayoutClass::Cols("cols-"));
        assert_eq!(cols_variant, Ok(LayoutClass::Cols("cols-3:440px")));
    }

    #[test]
    fn create_layout_class_when_value_expected_but_no_set_with_colon() {
        let layout_class = "max-width:";
        let max_width_variant = LayoutClass::try_from(layout_class);
        assert_eq!(max_width_variant, Err(()));
    }

    #[test]
    fn create_layout_class_when_value_expected_but_no_set_without_colon() {
        let layout_class = "max-width";
        let max_width_variant = LayoutClass::try_from(layout_class);
        assert_eq!(max_width_variant, Err(()));
    }

    #[test]
    fn create_layout_class_from_text() {
        let layout_class = "max-width:440px";
        let max_width_variant = LayoutClass::try_from(layout_class);
        assert_eq!(max_width_variant, Ok(LayoutClass::MaxWidth("440px")));
    }
}
