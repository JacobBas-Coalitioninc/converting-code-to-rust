use core::panic;

struct FullTermComponentsStruct {
    non_prorated_additive: f64,
    non_prorated_multiplicative: f64,
    prorated_additive: f64,
    prorated_multiplicative: f64,
}

fn combine_prorated_and_non_prorated(
    non_prorated_component: f64,
    prorated_component: f64,
    prorate_factor: f64,
) -> f64 {
    non_prorated_component + (prorated_component * prorate_factor)
}

fn get_implied_expected_multiplier(
    original_full_term_components: FullTermComponentsStruct,
    new_full_term_components: FullTermComponentsStruct,
    prorate_factor: f64,
    expected_additive_factor: f64,
) -> f64 {
    // getting the additive component deltas
    let non_prorated_additive_delta = new_full_term_components.non_prorated_additive
        - original_full_term_components.non_prorated_additive;
    let prorated_additive_delta = new_full_term_components.prorated_additive
        - original_full_term_components.prorated_additive;
    let sum_of_additive_deltas = combine_prorated_and_non_prorated(
        non_prorated_additive_delta,
        prorated_additive_delta,
        prorate_factor,
    );

    // getting the required values for the multiplicative components
    let sum_of_original_total_multiplicative = combine_prorated_and_non_prorated(
        original_full_term_components.non_prorated_multiplicative,
        original_full_term_components.prorated_multiplicative,
        prorate_factor,
    );
    let sum_of_new_total_multiplicative = combine_prorated_and_non_prorated(
        new_full_term_components.non_prorated_multiplicative,
        new_full_term_components.prorated_multiplicative,
        prorate_factor,
    );

    if sum_of_new_total_multiplicative == 0.0 {
        panic!("There is no available multiplicative component to allow for an override.")
    }

    // calcualting the implied multiplicative factor
    let rounding = 10_000_000.0;
    let raw_factor = (expected_additive_factor - sum_of_additive_deltas
        + sum_of_original_total_multiplicative)
        / sum_of_new_total_multiplicative;

    // returning out the rounded factor
    (raw_factor * rounding).round() / rounding
}

fn calculate_override(
    original_full_term_components: FullTermComponentsStruct,
    new_full_term_components: FullTermComponentsStruct,
    prorate_factor: f64,
    expected_additive_factor: f64,
) -> f64 {
    let response = get_implied_expected_multiplier(
        original_full_term_components,
        new_full_term_components,
        prorate_factor,
        expected_additive_factor,
    );
    response
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success_case_1() {
        let org_full_term_comp = FullTermComponentsStruct {
            non_prorated_additive: 100.0,
            prorated_additive: 100.0,
            non_prorated_multiplicative: 100.0,
            prorated_multiplicative: 100.0,
        };
        let new_full_term_comp = FullTermComponentsStruct {
            non_prorated_additive: 100.0,
            prorated_additive: 100.0,
            non_prorated_multiplicative: 100.0,
            prorated_multiplicative: 100.0,
        };
        let prorate_factor = 1.0;
        let expected_additive_factor = -0.99;
        let expected_response = 0.99505;

        assert_eq!(
            calculate_override(
                org_full_term_comp,
                new_full_term_comp,
                prorate_factor,
                expected_additive_factor,
            ),
            expected_response
        );
    }

    #[test]
    #[should_panic(
        expected = "There is no available multiplicative component to allow for an override."
    )]
    fn failure_case_1() {
        let org_full_term_comp = FullTermComponentsStruct {
            non_prorated_additive: 0.0,
            prorated_additive: 0.0,
            non_prorated_multiplicative: 0.0,
            prorated_multiplicative: 0.0,
        };
        let new_full_term_comp = FullTermComponentsStruct {
            non_prorated_additive: 0.0,
            prorated_additive: 0.0,
            non_prorated_multiplicative: 0.0,
            prorated_multiplicative: 0.0,
        };
        let prorate_factor = 1.0;
        let expected_additive_factor = -0.99;

        calculate_override(
            org_full_term_comp,
            new_full_term_comp,
            prorate_factor,
            expected_additive_factor,
        );
    }
}
