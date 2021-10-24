pub trait HidingPlace {
    fn has_marmot(&self) -> bool;
}

#[derive(Default)]
pub struct NormalHole {}

impl HidingPlace for NormalHole {
    fn has_marmot(&self) -> bool {
        false
    }
}

#[derive(Default)]
pub struct MarmotHole {}

impl HidingPlace for MarmotHole {
    fn has_marmot(&self) -> bool {
        true
    }
}

/// This is an example of doctest that will be executed when running
/// test target BUT ONLY if we build a library
///
/// #Examples
///
/// ```
/// let marmot_found = find_marmot(21);
/// assert_eq!(marmot_found, true);
/// ```
pub fn find_marmot(depth: u32) -> bool {
    if depth > 20 {
        return true;
    }
    false
}

pub fn find_marmot_in(hiding_place: &dyn HidingPlace) -> bool {
    return hiding_place.has_marmot();
}

pub fn call_marmot(call: &str) -> &'static str {
    match call {
        "sqeek sqeek" => "squeeq",
        "yoohoo" => "squeeq",
        "kra kra" => "pip",
        _ => ""
    }
}

pub fn chance_to_find_marmot(day_of_weeek: u8) -> f32 {
    match day_of_weeek {
        1 => 1.0,
        2 | 3 | 4 => 1.0 / 2.0,
        5 | 6 | 7 => 1.0 / 3.0,
        _ => panic!("Not a day of week")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;
    use mockall::*;
    use mockall::predicate::*;
    use assert_approx_eq::assert_approx_eq;

    mock! {
        pub Hole {}
        impl HidingPlace for Hole{
            fn has_marmot(&self) -> bool;
        }
    }

    #[test]
    fn when_search_not_deep_then_no_marmot_found() {
        assert_eq!(find_marmot(19), false);
    }

    #[test]
    fn when_search_deep_then_marmot_found() {
        assert_eq!(find_marmot(21), true);
    }

    #[test]
    fn when_search_in_and_not_there_then_no_marmot_found() {
        let mut mock = MockHole::new();
        mock.expect_has_marmot()
            .times(1)
            .returning(||false);
        assert_eq!(find_marmot_in(&mock), false);
    }

    #[test]
    fn when_search_in_and_there_then_marmot_found() {
        let mut mock = MockHole::new();
        mock.expect_has_marmot()
            .times(1)
            .returning(||true);
        assert_eq!(find_marmot_in(&mock), true);
    }

    #[test]
    fn when_call_kra_kra_then_marmot_pip() {
        let res = call_marmot("kra kra");
        assert_eq!(res, "pip");
    }

    #[test]
    fn when_friday_than_0_33_chance_of_fiding_marmot() {
        let res = chance_to_find_marmot(5);
        assert_approx_eq!(res, 0.333, 0.01)
    }

    #[test]
    #[should_panic]
    fn when_invalid_day_than_panic() {
        chance_to_find_marmot(9);
    }

    #[rstest]
    #[case(1, 1.0)]
    #[case(2, 0.5)]
    #[case(3, 0.5)]
    #[case(4, 0.5)]
    #[case(5, 0.33)]
    #[case(6, 0.33)]
    #[case(7, 0.33)]
    fn check_all_days(#[case] input: u8,#[case] expected: f32) {
        assert_approx_eq!(expected, chance_to_find_marmot(input), 0.01)
    }
}