fn interpolate(
    lower_bound: f32,
    lower_bound_factor: f32,
    upper_bound: f32,
    upper_bound_factor: f32,
    selected: f32,
) -> f32 {
    if !(lower_bound <= selected && selected <= upper_bound) {
        panic!(
            "Selected value '{}' is not between the lower '{}' and upper '{}' bounds.",
            selected, lower_bound, upper_bound
        )
    }

    if lower_bound == upper_bound && lower_bound_factor != upper_bound_factor {
        if lower_bound_factor != upper_bound_factor {
            panic!("Lower bound and upper bound are equal, but the corresponding factors are not.")
        } else {
            return lower_bound_factor;
        }
    }

    let percent_applied_lower_factor = (upper_bound - selected) / (upper_bound - lower_bound);
    let percent_applied_upper_factor = 1.0 - percent_applied_lower_factor;

    let interpolated_value = percent_applied_lower_factor * lower_bound_factor
        + percent_applied_upper_factor * upper_bound_factor;
    interpolated_value
}

// implement testing for this function; there are two different ways that we can actually
// paramtrize test functions; The first is that we can implement a testing function
// and then loop through different parameters within the the actual test function. The other
// thing that we can do is create a macro which will allow us to generate the code the test
// function code using meta programming
#[cfg(test)]
mod tests {
    use super::*;

    // testing macros for the success and fail cases
    macro_rules! test_interpolate_success {
        ($name:ident, $expected:expr, $lower_bound:expr, $lower_factor:expr, $upper_bound:expr, $upper_factor:expr, $selected_value:expr) => {
            #[test]
            fn $name() {
                let expected = $expected;
                let actual = interpolate(
                    $lower_bound,
                    $lower_factor,
                    $upper_bound,
                    $upper_factor,
                    $selected_value,
                );
                assert_eq!(expected, actual);
            }
        };
    }
    macro_rules! test_interpolate_fail {
        ($name:ident, $expected_panic_msg:expr, $lower_bound:expr, $lower_factor:expr, $upper_bound:expr, $upper_factor:expr, $selected_value:expr) => {
            #[test]
            #[should_panic(expected = $expected_panic_msg)]
            fn $name() {
                interpolate(
                    $lower_bound,
                    $lower_factor,
                    $upper_bound,
                    $upper_factor,
                    $selected_value,
                );
            }
        };
    }

    test_interpolate_success!(success_case_1, 7.5, 0.0, 0.0, 10.0, 10.0, 7.5);
    test_interpolate_fail!(
        fail_case_not_between_bounds,
        "Selected value '7.5' is not between the lower '10' and upper '0' bounds.",
        10.0,
        10.0,
        0.0,
        0.0,
        7.5
    );
    test_interpolate_fail!(
        fail_case_factors_not_equal_but_bounds_are,
        "Lower bound and upper bound are equal, but the corresponding factors are not.",
        0.0,
        10.0,
        0.0,
        0.0,
        0.0
    );
}
