/// Transparently collapses "`\r\n`" into "`\n`" when wrapping a character iterator.
///
/// This iterator standardizes the different line endings used by various platforms. Most notably,
/// Unix-like systems utilize "`\n`" as their line ending, and Windows sysems utilize "`\r\n`".
///
/// The lexer provided by this crate only recognizes the "`\n`" Unix-style line ending, so it is
/// recommended that input streams are first wrapped by this iterator before being passed to the
/// lexer.
///
/// # Example
/// ```
/// use ruby_lexer::plugins::NewlinesHandler;
/// let input = "This is \r\n an \r input \n string.\r\n";
/// let output = "This is \n an \r input \n string.\n";
/// let iter = NewlinesHandler::new(input.chars());
/// assert_eq!(output, iter.collect::<String>());
/// ```
pub struct NewlinesHandler<T: Iterator<Item = char>> {
    source: T,
    chr0: Option<char>,
    chr1: Option<char>,
}

impl<T> NewlinesHandler<T>
where
    T: Iterator<Item = char>,
{
    pub fn new(source: T) -> Self {
        let mut nlh = NewlinesHandler {
            source,
            chr0: None,
            chr1: None,
        };
        nlh.shift();
        nlh.shift();
        nlh
    }

    fn shift(&mut self) -> Option<char> {
        let result = self.chr0;
        self.chr0 = self.chr1;
        self.chr1 = self.source.next();
        result
    }
}

impl<T> Iterator for NewlinesHandler<T>
where
    T: Iterator<Item = char>,
{
    type Item = char;

    /// Collapses \r\n sequences into \n before returning the next character
    fn next(&mut self) -> Option<Self::Item> {
        if self.chr0 == Some('\r') && self.chr1 == Some('\n') {
            self.shift();
        }
        self.shift()
    }
}
