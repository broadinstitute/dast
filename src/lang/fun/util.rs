use jati::trees::symbols::ArgsFailure;
use jati::trees::types::Type;

pub(crate) fn check_n_args(arg_types: &[Type], n_expected: usize) -> Result<(), ArgsFailure> {
    let n_actual = arg_types.len();
    if n_actual == n_expected {
        Ok(())
    } else {
        Err(ArgsFailure::new_wrong_number(n_actual, n_expected))
    }
}