/// The traditional lorem ipsum text as given in [Wikipedia]. Using
/// this text alone for a Markov chain of order two doesn't work very
/// well since each bigram (two consequtive words) is followed by just
/// one other word. In other words, the Markov chain will always
/// produce the same output and recreate the lorem ipsum text
/// precisely.
///
/// [Wikipedia]: https://en.wikipedia.org/wiki/Lorem_ipsum
const LOREM_IPSUM: &str = include_str!("lorem-ipsum.txt");

/// The first book in Cicero's work De finibus bonorum et malorum ("On
/// the ends of good and evil"). The lorem ipsum text in
/// [`LOREM_IPSUM`] is derived from part of this text.
///
/// [`LOREM_IPSUM`]: constant.LOREM_IPSUM.html
const LIBER_PRIMUS: &str = include_str!("liber-primus.txt");

/// Generate a standard lorem ipsum text.
pub fn lipsum() -> String {
    String::from(LOREM_IPSUM)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_with_lorem_ipsum() {
        assert_eq!(&lipsum()[..11], "Lorem ipsum");
    }
}
