// Copyright 2017 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[derive(Clone, Copy, Debug)]
pub struct GlyphRange {
    pub start: u16,
    pub end: u16,
}

impl GlyphRange {
    #[inline]
    pub fn iter(&self) -> GlyphRangeIter {
        GlyphRangeIter {
            start: self.start,
            end: self.end,
        }
    }
}

#[derive(Clone)]
pub struct GlyphRangeIter {
    start: u16,
    end: u16,
}

impl Iterator for GlyphRangeIter {
    type Item = u16;

    #[inline]
    fn next(&mut self) -> Option<u16> {
        if self.start > self.end {
            None
        } else {
            let item = self.start;
            self.start += 1;
            Some(item)
        }
    }
}

#[derive(Clone, Debug)]
pub struct FontGlyphRange {
    pub font_id: u32,
    pub ranges: Vec<GlyphRange>,
}

#[derive(Clone)]
pub struct FontGlyphRanges {
    pub fonts: Vec<FontGlyphRange>,
}

impl FontGlyphRanges {
    pub fn new() -> FontGlyphRanges {
        FontGlyphRanges {
            fonts: vec![],
        }
    }
}

