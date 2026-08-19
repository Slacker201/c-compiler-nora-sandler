use slk_tokenstream::{Mark, TokenstreamSpan};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Span {
    start: usize,
    end: usize,
}

impl Span {
    #[inline]
    pub fn new(mut start: usize, mut end: usize) -> Span {
        if start > end {
            core::mem::swap(&mut start, &mut end);
        }
        Self { start, end }
    }
    #[inline]
    pub fn from_tuple((start, end): (usize, usize)) -> Self {
        Self::new(start, end)
    }
    #[inline]
    pub fn start(&self) -> usize {
        self.start
    }
    #[inline]
    pub fn end(&self) -> usize {
        self.end
    }
    #[inline]
    pub fn len(&self) -> usize {
        self.end - self.start
    }
    #[inline]
    pub fn from_tokenstream_mark(mut start: Mark, mut end: Mark) -> Self {
        if start.position() > end.position() {
            core::mem::swap(&mut start, &mut end);
        }

        Self::new(start.position(), end.position())
    }
}

impl From<TokenstreamSpan> for Span {
    fn from(value: TokenstreamSpan) -> Self {
        Self {
            start: value.start().position(),
            end: value.end().position(),
        }
    }
}
