use trident_client::fuzzing::*;
mod fuzz_instructions;
use fuzz_instructions::FuzzInstruction;
use fuzz_instructions::*;
use vault::entry as entry_vault;
use vault::ID as PROGRAM_ID_VAULT;
const PROGRAM_NAME_VAULT: &str = "vault";
struct InstructionsSequence;
/// Define instruction sequences for invocation.
/// `pre` runs at the start, `middle` in the middle, and `post` at the end.
/// For example, to call `InitializeFn`, `UpdateFn` and then `WithdrawFn` during
/// each fuzzing iteration:
/// ```
/// use fuzz_instructions::{InitializeFn, UpdateFn, WithdrawFn};
/// impl FuzzDataBuilder<FuzzInstruction> for InstructionsSequence {
///     pre_sequence!(InitializeFn,UpdateFn);
///     middle_sequence!(WithdrawFn);
///}
/// ```
/// For more details, see: https://ackee.xyz/trident/docs/latest/features/instructions-sequences/#instructions-sequences
impl FuzzDataBuilder<FuzzInstruction> for InstructionsSequence {
    pre_sequence!(NewVaultType, NewVault);
    middle_sequence!(
        LockVaultType,
        RollOverVaultType,
        Deposit,
        TransferVaultTypeToken,
        Withdraw,
        Deactivate,
        Activate,
        NominateVaultTypeAuthority,
        AcceptVaultTypeAuthority
    );
    post_sequence!(CloseVault, CloseVaultType);
}
/// `fn fuzz_iteration` runs during every fuzzing iteration.
/// Modification is not required.
fn fuzz_iteration<T: FuzzTestExecutor<U> + std::fmt::Display, U>(
    fuzz_data: FuzzData<T, U>,
    config: &Config,
) {
    let fuzzing_program_vault = FuzzingProgram::new(
        PROGRAM_NAME_VAULT,
        &PROGRAM_ID_VAULT,
        processor!(convert_entry!(entry_vault)),
    );
    let mut client = ProgramTestClientBlocking::new(&[fuzzing_program_vault], config).unwrap();
    let _ = fuzz_data.run_with_runtime(&mut client, config);
}
fn main() {
    let config = Config::new();
    fuzz_trident ! (fuzz_ix : FuzzInstruction , | fuzz_data : InstructionsSequence | { fuzz_iteration (fuzz_data , & config) ; });
}
