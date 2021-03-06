use solana_account_decoder::parse_bpf_loader::{
    parse_bpf_upgradeable_loader, BpfUpgradeableLoaderAccountType,
};
use tracing::error;

use crate::{Instruction, InstructionFunction, InstructionProperty, InstructionSet};
use solana_account_decoder::parse_account_data::{ParseAccountError, ParsableAccount};

pub const PROGRAM_ADDRESS: &str = "BPFLoaderUpgradeab1e11111111111111111111111";

/// Extracts the contents of an instruction into small bits and pieces, or what we would call,
/// instruction_properties.
///
/// The function should return a list of instruction properties extracted from an instruction.
pub async fn fragment_instruction(
    // The instruction
    instruction: Instruction,
) -> Option<InstructionSet> {
    let bpf_loader_upgradeable_dr =
        parse_bpf_upgradeable_loader(instruction.data.as_slice());

    return match bpf_loader_upgradeable_dr {
        Ok(ref blu) => {
            let bpf_loader_upgradeable_i = blu.clone();

            match bpf_loader_upgradeable_i {
                BpfUpgradeableLoaderAccountType::Uninitialized => {
                    Some(InstructionSet {
                        function: InstructionFunction {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            program: instruction.program.clone(),
                            function_name: "uninitialized".to_string(),
                            timestamp: instruction.timestamp.clone()
                        },
                        properties: vec![]
                    })
                }
                BpfUpgradeableLoaderAccountType::Buffer(buffer) => {
                    Some(InstructionSet {
                        function: InstructionFunction {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            program: instruction.program.clone(),
                            function_name: "buffer".to_string(),
                            timestamp: instruction.timestamp.clone()
                        },
                        properties: vec![
                            InstructionProperty {
                                tx_instruction_id: instruction.tx_instruction_id.clone(),
                                transaction_hash: instruction.transaction_hash.clone(),
                                parent_index: instruction.parent_index.clone(),
                                key: "authority".to_string(),
                                value: if let Some(ba) = buffer.authority.clone() {
                                    ba
                                } else {
                                    "".to_string()
                                },
                                parent_key: "buffer".to_string(),
                                timestamp: instruction.timestamp.clone(),
                            },
                            InstructionProperty {
                                tx_instruction_id: instruction.tx_instruction_id.clone(),
                                transaction_hash: instruction.transaction_hash.clone(),
                                parent_index: instruction.parent_index.clone(),
                                key: "data".to_string(),
                                value: serde_json::to_string(&buffer.data).unwrap().to_string(),
                                parent_key: "buffer".to_string(),
                                timestamp: instruction.timestamp.clone(),
                            },
                        ]
                    })
                }
                BpfUpgradeableLoaderAccountType::Program(program) => {
                    Some(InstructionSet {
                        function: InstructionFunction {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            program: instruction.program.clone(),
                            function_name: "program".to_string(),
                            timestamp: instruction.timestamp.clone()
                        },
                        properties: vec![
                            InstructionProperty {
                                tx_instruction_id: instruction.tx_instruction_id.clone(),
                                transaction_hash: instruction.transaction_hash.clone(),
                                parent_index: instruction.parent_index.clone(),
                                key: "program_data".to_string(),
                                value: serde_json::to_string(&program.program_data).unwrap().to_string(),
                                parent_key: "program".to_string(),
                                timestamp: instruction.timestamp.clone(),
                            }
                        ]
                    })
                }
                BpfUpgradeableLoaderAccountType::ProgramData(program_data) => {
                    Some(InstructionSet {
                        function: InstructionFunction {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            program: instruction.program.clone(),
                            function_name: "program-data".to_string(),
                            timestamp: instruction.timestamp.clone()
                        },
                        properties: vec![
                            InstructionProperty {
                                tx_instruction_id: instruction.tx_instruction_id.clone(),
                                transaction_hash: instruction.transaction_hash.clone(),
                                parent_index: instruction.parent_index.clone(),
                                key: "authority".to_string(),
                                value: if let Some(auth) = program_data.authority.clone() {
                                    auth
                                } else {
                                    "".to_string()
                                },
                                parent_key: "program_data".to_string(),
                                timestamp: instruction.timestamp.clone(),
                            },
                            InstructionProperty {
                                tx_instruction_id: instruction.tx_instruction_id.clone(),
                                transaction_hash: instruction.transaction_hash.clone(),
                                parent_index: instruction.parent_index.clone(),
                                key: "data".to_string(),
                                value: serde_json::to_string(&program_data.data).unwrap().to_string(),
                                parent_key: "program_data".to_string(),
                                timestamp: instruction.timestamp.clone(),
                            },
                            InstructionProperty {
                                tx_instruction_id: instruction.tx_instruction_id.clone(),
                                transaction_hash: instruction.transaction_hash.clone(),
                                parent_index: instruction.parent_index.clone(),
                                key: "slot".to_string(),
                                value: program_data.slot.to_string(),
                                parent_key: "program_data".to_string(),
                                timestamp: instruction.timestamp.clone(),
                            },
                        ]
                    })
                }
            }
        }
        Err(instruction_err) => {
            // If the instruction parsing is failing, bail out
            match instruction_err {
                ParseAccountError::AccountNotParsable(parseable_account) => {
                    let account_involved = match parseable_account {
                        ParsableAccount::BpfUpgradeableLoader => "BpfUpgradeableLoader",
                        ParsableAccount::Config => "Config",
                        ParsableAccount::Nonce => "Nonce",
                        ParsableAccount::SplToken => "SplToken",
                        ParsableAccount::Stake => "Stake",
                        ParsableAccount::Sysvar => "Sysvar",
                        ParsableAccount::Vote => "Vote",
                    };

                    error!("[spi-wrapper/bpf_loader_upgradeable] Attempt to parse instruction from \
                program {} failed as the account was not parsable ({} was not parseable).",
                    instruction.program, account_involved);
                }
                ParseAccountError::ProgramNotParsable => {
                    error!("[spi-wrapper/bpf_loader_upgradeable] Attempt to parse instruction from \
                program {} failed as it was not parsable.", instruction.program);
                }
                ParseAccountError::AdditionalDataMissing(missing) => {
                    error!("[spi-wrapper/bpf_loader_upgradeable] Attempt to parse instruction from \
                program {} failed as it was missing data for {}.", instruction.program, missing);
                }
                ParseAccountError::InstructionError(_err) => {
                    // TODO: Tell us what instruction error it exactly is.
                    error!("[spi-wrapper/bpf_loader_upgradeable] Attempt to parse instruction from \
                program {} failed as there was an instruction error.", instruction.program);
                }
                ParseAccountError::SerdeJsonError(err) => {
                    error!("[spi-wrapper/bpf_loader_upgradeable] Attempt to parse instruction from \
                program {} failed as there was serde json error: {}.", instruction.program, err);
                }
            }

            None
        }
    }
}