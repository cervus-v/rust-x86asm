use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmaskmovdqu_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVDQU, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 247, 213], OperandSize::Dword)
}

#[test]
fn vmaskmovdqu_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVDQU, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 247, 198], OperandSize::Qword)
}
