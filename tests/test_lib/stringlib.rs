use pretty_assertions::assert_eq;
use rstest::rstest;

use pad::{
    Alignment,
    PadStr
};

use core_dev::stringlib::align_left;
use core_dev::stringlib::align_right;

#[cfg(test)]
mod test_align_center {

    use core_dev::stringlib::align_center;
    use super::assert_eq;
    use super::rstest;

    #[rstest]
    #[case("content", 20, "      content       ")]
    #[case("c", 3, " c ")]
    fn against_str(
        #[case] content: &str,
        #[case] width: usize,
        #[case] expected: &str
    ) {
        let result = align_center(content, width);
        println!("'{}'", result);
        assert_eq!(result, expected);
    }
}

#[cfg(test)]
mod test_align_string_builder {
    use core_dev::stringlib::AlignString;
    use super::Alignment;
    use super::rstest;

    #[rstest]
    #[case("content", 20, Alignment::Middle, "      content       ")]
    #[case("c", 3, Alignment::Middle, " c ")]
    #[case("c", 3, Alignment::Left, "c  ")]
    #[case("c", 3, Alignment::Right, "  c")]
    fn against_str(
        #[case] content: &str,
        #[case] width: usize,
        #[case] alignment: Alignment,
        #[case] expected: &str
    ) {
        let mut builder = AlignString::new(content);
        let mut builder = builder.width(width).alignment(alignment);

        let result = builder.build();
        println!("'{}'", result);
        assert_eq!(result, expected);
    }
}
