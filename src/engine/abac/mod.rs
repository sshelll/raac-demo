mod talent;

pub use talent::CHECKER as TALENT_CHECKER;

pub  fn init() {
    use crate::util::cedar::*;
    talent::CHECKER.get_or_init(|| talent::Checker::new(policy_path("talent.cedar"), None));
}
