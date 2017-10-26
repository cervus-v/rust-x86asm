use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn minsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MINSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 93, 197], OperandSize::Dword)
}

#[test]
fn minsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MINSD, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 93, 7], OperandSize::Dword)
}

#[test]
fn minsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MINSD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 93, 229], OperandSize::Qword)
}

#[test]
fn minsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MINSD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(RDI, 659214753, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 93, 159, 161, 209, 74, 39], OperandSize::Qword)
}
