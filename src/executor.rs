use std::fmt::{Display, Formatter};

use miden_vm::{
    AdviceInputs, Assembler, DefaultHost, ExecutionProof, Program, ProgramInfo, ProvingOptions,
    StackInputs, StackOutputs, VerificationError, prove, verify,
};

#[derive(Debug, Clone)]
pub struct ExecutionArtifacts {
    pub stack_outputs: Vec<u64>,
    pub proof_bytes: Vec<u8>,
    pub program_info: ProgramInfo,
    pub stack_inputs: StackInputs,
}

#[derive(Debug)]
pub enum ExecutorError {
    Assembly(String),
    Input(String),
    Execution(String),
    ProofDeserialization(String),
    Verification(String),
}

impl Display for ExecutorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Assembly(msg) => write!(f, "assembly error: {msg}"),
            Self::Input(msg) => write!(f, "input error: {msg}"),
            Self::Execution(msg) => write!(f, "execution/proving error: {msg}"),
            Self::ProofDeserialization(msg) => write!(f, "proof deserialization error: {msg}"),
            Self::Verification(msg) => write!(f, "verification error: {msg}"),
        }
    }
}

impl std::error::Error for ExecutorError {}

pub fn assemble_program(masm: &str) -> Result<Program, ExecutorError> {
    Assembler::default()
        .assemble_program(masm)
        .map_err(|e| ExecutorError::Assembly(e.to_string()))
}

pub fn execute_with_proof(masm: &str, inputs: &[u64]) -> Result<ExecutionArtifacts, ExecutorError> {
    let program = assemble_program(masm)?;
    let stack_inputs = StackInputs::try_from_ints(inputs.iter().copied())
        .map_err(|e| ExecutorError::Input(e.to_string()))?;

    let mut host = DefaultHost::default();
    let (stack_outputs, proof) = prove(
        &program,
        stack_inputs.clone(),
        AdviceInputs::default(),
        &mut host,
        ProvingOptions::default(),
    )
    .map_err(|e| ExecutorError::Execution(e.to_string()))?;

    Ok(ExecutionArtifacts {
        stack_outputs: stack_outputs.as_int_vec(),
        proof_bytes: proof.to_bytes(),
        program_info: program.clone().into(),
        stack_inputs,
    })
}

pub fn verify_execution(
    program_info: ProgramInfo,
    stack_inputs: StackInputs,
    stack_outputs: StackOutputs,
    proof_bytes: &[u8],
) -> Result<u32, ExecutorError> {
    let proof = ExecutionProof::from_bytes(proof_bytes)
        .map_err(|e| ExecutorError::ProofDeserialization(e.to_string()))?;

    verify(program_info, stack_inputs, stack_outputs, proof).map_err(map_verification_error)
}

fn map_verification_error(err: VerificationError) -> ExecutorError {
    ExecutorError::Verification(err.to_string())
}

pub fn stack_outputs_from_ints(outputs: &[u64]) -> Result<StackOutputs, ExecutorError> {
    StackOutputs::try_from_ints(outputs.iter().copied())
        .map_err(|e| ExecutorError::Input(e.to_string()))
}
