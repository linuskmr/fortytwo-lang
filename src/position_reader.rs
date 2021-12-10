use crate::position_container::PositionContainer;

pub type Symbol = PositionContainer<char>;

pub struct PositionReader<S: Iterator<Item = char>> {
    /// The source to read from.
    source: S,

    index: usize,
}

impl<S: Iterator<Item = char>> PositionReader<S> {
    /// Creates a new [PositionReader] with the given source.
    pub fn new(source: S) -> Self {
        Self { source, index: 0 }
    }
}

impl<S: Iterator<Item = char>> Iterator for PositionReader<S> {
    type Item = Symbol;

    fn next(&mut self) -> Option<Self::Item> {
        // Read char and add position information
        let symbol = self.source.next().map(|read_char| Symbol {
            data: read_char,
            position: miette::SourceSpan::new(self.index.into(), 1.into()),
        });
        self.index += 1;
        symbol
    }
}
