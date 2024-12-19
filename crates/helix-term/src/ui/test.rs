use std::cmp::Ordering;
use std::iter::Peekable;

use anyhow::Result;

use crate::{
    compositor::{Context, EventResult},
    ctrl, key, shift,
};
use helix_core::unicode::width::UnicodeWidthStr;
use helix_view::{
    graphics::Rect,
    input::{Event, KeyEvent},
};
use tui::{buffer::Buffer as Surface, text::Spans};

pub trait TreeItem: Sized {
    type Params;

    fn text(&self, cx: &mut Context, selected: bool, params: &mut Self::Params) -> Spans;
    fn is_child(&self, other: &Self) -> bool;
    fn cmp(&self, other: &Self) -> Ordering;

    fn filter(&self, cx: &mut Context, s: &str, params: &mut Self::Params) -> bool {
        self.text(cx, false, params)
            .0
            .into_iter()
            .map(|s| s.content)
            .collect::<Vec<_>>()
            .concat()
            .contains(s)
    }

    fn get_childs(&self) -> Result<Vec<Self>> {
        Ok(vec![])
    }
}
