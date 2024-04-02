use core::panic;

use chrono::prelude::*;

#[derive(Debug)]
#[allow(dead_code)]
enum InsuranceMarket {
    Admitted,
    AllMarkets,
    Australia,
    Canada,
    Germany,
    Surplus,
    UnitedKingdom,
}

#[derive(Debug)]
#[allow(dead_code)]
enum InsuranceProgram {
    UsCyberTechEoSme,
    UsCyberTechEoMm,
    UsCyberTechEoExcessSme,
    UsCyberTechEoExcessMm,
    CaCyberPrimary,
    CaCyberExcess,
    GbCyberPrimary,
    GbCyberExcess,
    AuCyberPrimary,
    DeCyberPrimary,
    UsCyberTechEoLm,
    UsCyberTechEoExcessLm,
    UsCyberMplSme,
    UsCyberMplExcessSme,
    UsCyberMplExcessMm,
}

#[allow(dead_code)]
enum ModelName {
    CICCyberAdmittedModel,
    CoalitionConsolidatedSurplusModel,
    CoalitionConsolidatedExcessModel,
}

#[allow(dead_code)]
struct CoverageInstance {
    sublimit: i32,
    subretention: i32,
}

fn pricing_model_router(
    insurance_market: InsuranceMarket,
    insurance_program: InsuranceProgram,
    is_renewal: bool,
    company_address_state: &str,
    effective_date: NaiveDate,
    original_created_at_date: NaiveDate,
    coverage_instances: Vec<CoverageInstance>,
    company_revenue: i32,
) -> ModelName {
    match insurance_market {
        InsuranceMarket::Admitted => ModelName::CICCyberAdmittedModel,

        InsuranceMarket::Surplus => ModelName::CoalitionConsolidatedSurplusModel,

        // TODO: one thing that can be done here is that we could define a macro that would
        // allow us to very easily develop the market specific switch statements when it is simple
        InsuranceMarket::Canada => match insurance_program {
            InsuranceProgram::CaCyberExcess => ModelName::CoalitionConsolidatedExcessModel,
            InsuranceProgram::CaCyberPrimary => ModelName::CoalitionConsolidatedSurplusModel,
            _ => panic!(
                "No suitable '{:#?}' model for insurance program '{:#?}'.",
                insurance_market, insurance_program
            ),
        },

        InsuranceMarket::UnitedKingdom => match insurance_program {
            InsuranceProgram::GbCyberExcess => ModelName::CoalitionConsolidatedExcessModel,
            InsuranceProgram::GbCyberPrimary => ModelName::CoalitionConsolidatedSurplusModel,
            _ => panic!(
                "No suitable '{:#?}' model for insurance program '{:#?}'.",
                insurance_market, insurance_program
            ),
        },

        InsuranceMarket::Australia => match insurance_program {
            InsuranceProgram::AuCyberPrimary => ModelName::CoalitionConsolidatedSurplusModel,
            _ => panic!(
                "No suitable '{:#?}' model for insurance program '{:#?}'.",
                insurance_market, insurance_program
            ),
        },

        InsuranceMarket::Germany => match insurance_program {
            InsuranceProgram::DeCyberPrimary => ModelName::CoalitionConsolidatedSurplusModel,
            _ => panic!(
                "No suitable '{:#?}' model for insurance program '{:#?}'.",
                insurance_market, insurance_program
            ),
        },

        _ => panic!(
            "No suitable model for insruance market '{:#?}'.",
            insurance_market
        ),
    }
}
