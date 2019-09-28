/// Transparently collapses "`\r\n`" into "`\n`" when wrapping a character iterator.
///
/// # Example
/// ```
/// use ruby_lexer::plugins::NewlineHandler;
/// let nlh = NewlineHandler::new("This is \r\n an \r input \n string.\r\n".chars());
/// assert_eq!("This is \n an \r input \n string.\n", nlh.collect::<String>());
/// ```
pub struct NewlineHandler<T: Iterator<Item = char>> {
    source: T,
    chr0: Option<char>,
    chr1: Option<char>,
}

impl<T> NewlineHandler<T>
where
    T: Iterator<Item = char>,
{
    pub fn new(source: T) -> Self {
        let mut nlh = NewlineHandler {
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

impl<T> Iterator for NewlineHandler<T>
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
