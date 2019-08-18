
pub mod fonthelper;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_fonts() {
        assert_eq!(fonthelper::get_fonts(), "Hello cargo!");
    }
}
