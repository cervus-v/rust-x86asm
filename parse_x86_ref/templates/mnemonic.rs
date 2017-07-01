#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Mnemonic {
    AAA,
    AAD,
    AAM,
    AAS,
    ADC,
    ADD,
    ADDPD,
    ADDPS,
    ADDSD,
    ADDSS,
    ADDSUBPD,
    ADDSUBPS,
    AMX,
    AND,
    ANDNPD,
    ANDNPS,
    ANDPD,
    ANDPS,
    ARPL,
    BLENDPD,
    BLENDPS,
    BLENDVPD,
    BLENDVPS,
    BOUND,
    BSF,
    BSR,
    BSWAP,
    BT,
    BTC,
    BTR,
    BTS,
    CALL,
    CALLF,
    CBW,
    CDQ,
    CDQE,
    CLC,
    CLD,
    CLFLUSH,
    CLI,
    CLTS,
    CMC,
    CMOVA,
    CMOVAE,
    CMOVB,
    CMOVBE,
    CMOVC,
    CMOVE,
    CMOVG,
    CMOVGE,
    CMOVL,
    CMOVLE,
    CMOVNA,
    CMOVNAE,
    CMOVNB,
    CMOVNBE,
    CMOVNC,
    CMOVNE,
    CMOVNG,
    CMOVNGE,
    CMOVNL,
    CMOVNLE,
    CMOVNO,
    CMOVNP,
    CMOVNS,
    CMOVNZ,
    CMOVO,
    CMOVP,
    CMOVPE,
    CMOVPO,
    CMOVS,
    CMOVZ,
    CMP,
    CMPPD,
    CMPPS,
    CMPS,
    CMPSB,
    CMPSD,
    CMPSQ,
    CMPSS,
    CMPSW,
    CMPXCHG,
    CMPXCHG16B,
    CMPXCHG8B,
    COMISD,
    COMISS,
    CPUID,
    CQO,
    CRC32,
    CVTDQ2PD,
    CVTDQ2PS,
    CVTPD2DQ,
    CVTPD2PI,
    CVTPD2PS,
    CVTPI2PD,
    CVTPI2PS,
    CVTPS2DQ,
    CVTPS2PD,
    CVTPS2PI,
    CVTSD2SI,
    CVTSD2SS,
    CVTSI2SD,
    CVTSI2SS,
    CVTSS2SD,
    CVTSS2SI,
    CVTTPD2DQ,
    CVTTPD2PI,
    CVTTPS2DQ,
    CVTTPS2PI,
    CVTTSD2SI,
    CVTTSS2SI,
    CWD,
    CWDE,
    DAA,
    DAS,
    DEC,
    DIV,
    DIVPD,
    DIVPS,
    DIVSD,
    DIVSS,
    DPPS,
    EMMS,
    ENTER,
    EXTRACTPS,
    F2XM1,
    FABS,
    FADD,
    FADDP,
    FBLD,
    FBSTP,
    FCHS,
    FCLEX,
    FCMOVB,
    FCMOVBE,
    FCMOVE,
    FCMOVNB,
    FCMOVNBE,
    FCMOVNE,
    FCMOVNU,
    FCMOVU,
    FCOM,
    FCOM2,
    FCOMI,
    FCOMIP,
    FCOMP,
    FCOMP3,
    FCOMP5,
    FCOMPP,
    FCOS,
    FDECSTP,
    FDISI,
    FDIV,
    FDIVP,
    FDIVR,
    FDIVRP,
    FENI,
    FFREE,
    FFREEP,
    FIADD,
    FICOM,
    FICOMP,
    FIDIV,
    FIDIVR,
    FILD,
    FIMUL,
    FINCSTP,
    FINIT,
    FIST,
    FISTP,
    FISTTP,
    FISUB,
    FISUBR,
    FLD,
    FLD1,
    FLDCW,
    FLDENV,
    FLDL2E,
    FLDL2T,
    FLDLG2,
    FLDLN2,
    FLDPI,
    FLDZ,
    FMUL,
    FMULP,
    FNCLEX,
    FNDISI,
    FNENI,
    FNINIT,
    FNOP,
    FNSAVE,
    FNSETPM,
    FNSTCW,
    FNSTENV,
    FNSTSW,
    FPATAN,
    FPREM,
    FPREM1,
    FPTAN,
    FRNDINT,
    FRSTOR,
    FSAVE,
    FSCALE,
    FSETPM,
    FSIN,
    FSINCOS,
    FSQRT,
    FST,
    FSTCW,
    FSTENV,
    FSTP,
    FSTP1,
    FSTP8,
    FSTP9,
    FSTSW,
    FSUB,
    FSUBP,
    FSUBR,
    FSUBRP,
    FTST,
    FUCOM,
    FUCOMI,
    FUCOMIP,
    FUCOMP,
    FUCOMPP,
    FWAIT,
    FXAM,
    FXCH,
    FXCH4,
    FXCH7,
    FXRSTOR,
    FXSAVE,
    FXTRACT,
    FYL2X,
    FYL2XP1,
    GETSEC,
    HADDPD,
    HADDPS,
    HLT,
    HSUBPD,
    HSUBPS,
    ICEBP,
    IDIV,
    IMUL,
    IN,
    INC,
    INS,
    INSB,
    INSD,
    INSERTPS,
    INSW,
    INT,
    INT1,
    INTO,
    INVD,
    INVEPT,
    INVLPG,
    INVVPID,
    IRET,
    IRETD,
    IRETQ,
    JA,
    JAE,
    JB,
    JBE,
    JC,
    JCXZ,
    JE,
    JECXZ,
    JG,
    JGE,
    JL,
    JLE,
    JMP,
    JMPF,
    JNA,
    JNAE,
    JNB,
    JNBE,
    JNC,
    JNE,
    JNG,
    JNGE,
    JNL,
    JNLE,
    JNO,
    JNP,
    JNS,
    JNZ,
    JO,
    JP,
    JPE,
    JPO,
    JRCXZ,
    JS,
    JZ,
    LAHF,
    LAR,
    LDDQU,
    LDMXCSR,
    LDS,
    LEA,
    LEAVE,
    LES,
    LFENCE,
    LFS,
    LGDT,
    LGS,
    LIDT,
    LLDT,
    LMSW,
    LOADALL,
    LODS,
    LODSB,
    LODSD,
    LODSQ,
    LODSW,
    LOOP,
    LOOPE,
    LOOPNE,
    LOOPNZ,
    LOOPZ,
    LSL,
    LSS,
    LTR,
    MASKMOVDQU,
    MASKMOVQ,
    MAXPD,
    MAXPS,
    MAXSD,
    MAXSS,
    MFENCE,
    MINPD,
    MINPS,
    MINSD,
    MINSS,
    MONITOR,
    MOV,
    MOVAPD,
    MOVAPS,
    MOVBE,
    MOVD,
    MOVDDUP,
    MOVDQ2Q,
    MOVDQA,
    MOVDQU,
    MOVHLPS,
    MOVHPD,
    MOVHPS,
    MOVLHPS,
    MOVLPD,
    MOVLPS,
    MOVMSKPD,
    MOVMSKPS,
    MOVNTDQ,
    MOVNTDQA,
    MOVNTI,
    MOVNTPD,
    MOVNTPS,
    MOVNTQ,
    MOVQ,
    MOVQ2DQ,
    MOVS,
    MOVSB,
    MOVSD,
    MOVSHDUP,
    MOVSLDUP,
    MOVSQ,
    MOVSS,
    MOVSW,
    MOVSX,
    MOVSXD,
    MOVUPD,
    MOVUPS,
    MOVZX,
    MPSADBW,
    MUL,
    MULPD,
    MULPS,
    MULSD,
    MULSS,
    MWAIT,
    NEG,
    NOP,
    NOT,
    OR,
    ORPD,
    ORPS,
    OUT,
    OUTS,
    OUTSB,
    OUTSD,
    OUTSW,
    PABSB,
    PABSD,
    PABSW,
    PACKSSDW,
    PACKSSWB,
    PACKUSDW,
    PACKUSWB,
    PADDB,
    PADDD,
    PADDQ,
    PADDSB,
    PADDSW,
    PADDUSB,
    PADDUSW,
    PADDW,
    PALIGNR,
    PAND,
    PANDN,
    PAUSE,
    PAVGB,
    PAVGW,
    PBLENDVB,
    PBLENDW,
    PCMPEQB,
    PCMPEQD,
    PCMPEQQ,
    PCMPEQW,
    PCMPESTRI,
    PCMPESTRM,
    PCMPGTB,
    PCMPGTD,
    PCMPGTQ,
    PCMPGTW,
    PCMPISTRI,
    PCMPISTRM,
    PEXTRB,
    PEXTRD,
    PEXTRQ,
    PEXTRW,
    PHADDD,
    PHADDSW,
    PHADDW,
    PHMINPOSUW,
    PHSUBD,
    PHSUBSW,
    PHSUBW,
    PINSRB,
    PINSRD,
    PINSRQ,
    PINSRW,
    PMADDUBSW,
    PMADDWD,
    PMAXSB,
    PMAXSD,
    PMAXSW,
    PMAXUB,
    PMAXUD,
    PMAXUW,
    PMINSB,
    PMINSD,
    PMINSW,
    PMINUB,
    PMINUD,
    PMINUW,
    PMOVMSKB,
    PMOVSXBD,
    PMOVSXBQ,
    PMOVSXBW,
    PMOVSXDQ,
    PMOVSXWD,
    PMOVSXWQ,
    PMOVZXBD,
    PMOVZXBQ,
    PMOVZXBW,
    PMOVZXDQ,
    PMOVZXWD,
    PMOVZXWQ,
    PMULDQ,
    PMULHRSW,
    PMULHUW,
    PMULHW,
    PMULLD,
    PMULLW,
    PMULUDQ,
    POP,
    POPA,
    POPAD,
    POPCNT,
    POPF,
    POPFD,
    POPFQ,
    POR,
    PREFETCHNTA,
    PREFETCHT0,
    PREFETCHT1,
    PREFETCHT2,
    PSADBW,
    PSHUFB,
    PSHUFD,
    PSHUFHW,
    PSHUFLW,
    PSHUFW,
    PSIGNB,
    PSIGND,
    PSIGNW,
    PSLLD,
    PSLLDQ,
    PSLLQ,
    PSLLW,
    PSRAD,
    PSRAW,
    PSRLD,
    PSRLDQ,
    PSRLQ,
    PSRLW,
    PSUBB,
    PSUBD,
    PSUBQ,
    PSUBSB,
    PSUBSW,
    PSUBUSB,
    PSUBUSW,
    PSUBW,
    PTEST,
    PUNPCKHBW,
    PUNPCKHDQ,
    PUNPCKHQDQ,
    PUNPCKHWD,
    PUNPCKLBW,
    PUNPCKLDQ,
    PUNPCKLQDQ,
    PUNPCKLWD,
    PUSH,
    PUSHA,
    PUSHAD,
    PUSHF,
    PUSHFD,
    PUSHFQ,
    PXOR,
    RCL,
    RCPPS,
    RCPSS,
    RCR,
    RDMSR,
    RDPMC,
    RDTSC,
    RDTSCP,
    RETF,
    RETN,
    ROL,
    ROR,
    ROUNDPD,
    ROUNDPS,
    ROUNDSD,
    ROUNDSS,
    RSM,
    RSQRTPS,
    RSQRTSS,
    SAHF,
    SAL,
    SALC,
    SAR,
    SBB,
    SCAS,
    SCASB,
    SCASD,
    SCASQ,
    SCASW,
    SETA,
    SETAE,
    SETALC,
    SETB,
    SETBE,
    SETC,
    SETE,
    SETG,
    SETGE,
    SETL,
    SETLE,
    SETNA,
    SETNAE,
    SETNB,
    SETNBE,
    SETNC,
    SETNE,
    SETNG,
    SETNGE,
    SETNL,
    SETNLE,
    SETNO,
    SETNP,
    SETNS,
    SETNZ,
    SETO,
    SETP,
    SETPE,
    SETPO,
    SETS,
    SETZ,
    SFENCE,
    SGDT,
    SHL,
    SHLD,
    SHR,
    SHRD,
    SHUFPD,
    SHUFPS,
    SIDT,
    SLDT,
    SMSW,
    SQRTPD,
    SQRTPS,
    SQRTSD,
    SQRTSS,
    STC,
    STD,
    STI,
    STMXCSR,
    STOS,
    STOSB,
    STOSD,
    STOSQ,
    STOSW,
    STR,
    SUB,
    SUBPD,
    SUBPS,
    SUBSD,
    SUBSS,
    SWAPGS,
    SYSCALL,
    SYSENTER,
    SYSEXIT,
    SYSRET,
    TEST,
    UCOMISD,
    UCOMISS,
    UNPCKHPD,
    UNPCKHPS,
    UNPCKLPD,
    UNPCKLPS,
    VERR,
    VERW,
    VMCALL,
    VMCLEAR,
    VMLAUNCH,
    VMPTRLD,
    VMPTRST,
    VMREAD,
    VMRESUME,
    VMWRITE,
    VMXOFF,
    VMXON,
    WAIT,
    WBINVD,
    WRMSR,
    XADD,
    XCHG,
    XGETBV,
    XLAT,
    XLATB,
    XOR,
    XORPD,
    XORPS,
    XRSTOR,
    XSAVE,
    XSETBV,
}

impl Mnemonic {
    pub fn parse(val: &str) -> Result<Mnemonic, ()> {
        match val {
            "AAA" => Ok(Mnemonic::AAA),
            "AAD" => Ok(Mnemonic::AAD),
            "AAM" => Ok(Mnemonic::AAM),
            "AAS" => Ok(Mnemonic::AAS),
            "ADC" => Ok(Mnemonic::ADC),
            "ADD" => Ok(Mnemonic::ADD),
            "ADDPD" => Ok(Mnemonic::ADDPD),
            "ADDPS" => Ok(Mnemonic::ADDPS),
            "ADDSD" => Ok(Mnemonic::ADDSD),
            "ADDSS" => Ok(Mnemonic::ADDSS),
            "ADDSUBPD" => Ok(Mnemonic::ADDSUBPD),
            "ADDSUBPS" => Ok(Mnemonic::ADDSUBPS),
            "AMX" => Ok(Mnemonic::AMX),
            "AND" => Ok(Mnemonic::AND),
            "ANDNPD" => Ok(Mnemonic::ANDNPD),
            "ANDNPS" => Ok(Mnemonic::ANDNPS),
            "ANDPD" => Ok(Mnemonic::ANDPD),
            "ANDPS" => Ok(Mnemonic::ANDPS),
            "ARPL" => Ok(Mnemonic::ARPL),
            "BLENDPD" => Ok(Mnemonic::BLENDPD),
            "BLENDPS" => Ok(Mnemonic::BLENDPS),
            "BLENDVPD" => Ok(Mnemonic::BLENDVPD),
            "BLENDVPS" => Ok(Mnemonic::BLENDVPS),
            "BOUND" => Ok(Mnemonic::BOUND),
            "BSF" => Ok(Mnemonic::BSF),
            "BSR" => Ok(Mnemonic::BSR),
            "BSWAP" => Ok(Mnemonic::BSWAP),
            "BT" => Ok(Mnemonic::BT),
            "BTC" => Ok(Mnemonic::BTC),
            "BTR" => Ok(Mnemonic::BTR),
            "BTS" => Ok(Mnemonic::BTS),
            "CALL" => Ok(Mnemonic::CALL),
            "CALLF" => Ok(Mnemonic::CALLF),
            "CBW" => Ok(Mnemonic::CBW),
            "CDQ" => Ok(Mnemonic::CDQ),
            "CDQE" => Ok(Mnemonic::CDQE),
            "CLC" => Ok(Mnemonic::CLC),
            "CLD" => Ok(Mnemonic::CLD),
            "CLFLUSH" => Ok(Mnemonic::CLFLUSH),
            "CLI" => Ok(Mnemonic::CLI),
            "CLTS" => Ok(Mnemonic::CLTS),
            "CMC" => Ok(Mnemonic::CMC),
            "CMOVA" => Ok(Mnemonic::CMOVA),
            "CMOVAE" => Ok(Mnemonic::CMOVAE),
            "CMOVB" => Ok(Mnemonic::CMOVB),
            "CMOVBE" => Ok(Mnemonic::CMOVBE),
            "CMOVC" => Ok(Mnemonic::CMOVC),
            "CMOVE" => Ok(Mnemonic::CMOVE),
            "CMOVG" => Ok(Mnemonic::CMOVG),
            "CMOVGE" => Ok(Mnemonic::CMOVGE),
            "CMOVL" => Ok(Mnemonic::CMOVL),
            "CMOVLE" => Ok(Mnemonic::CMOVLE),
            "CMOVNA" => Ok(Mnemonic::CMOVNA),
            "CMOVNAE" => Ok(Mnemonic::CMOVNAE),
            "CMOVNB" => Ok(Mnemonic::CMOVNB),
            "CMOVNBE" => Ok(Mnemonic::CMOVNBE),
            "CMOVNC" => Ok(Mnemonic::CMOVNC),
            "CMOVNE" => Ok(Mnemonic::CMOVNE),
            "CMOVNG" => Ok(Mnemonic::CMOVNG),
            "CMOVNGE" => Ok(Mnemonic::CMOVNGE),
            "CMOVNL" => Ok(Mnemonic::CMOVNL),
            "CMOVNLE" => Ok(Mnemonic::CMOVNLE),
            "CMOVNO" => Ok(Mnemonic::CMOVNO),
            "CMOVNP" => Ok(Mnemonic::CMOVNP),
            "CMOVNS" => Ok(Mnemonic::CMOVNS),
            "CMOVNZ" => Ok(Mnemonic::CMOVNZ),
            "CMOVO" => Ok(Mnemonic::CMOVO),
            "CMOVP" => Ok(Mnemonic::CMOVP),
            "CMOVPE" => Ok(Mnemonic::CMOVPE),
            "CMOVPO" => Ok(Mnemonic::CMOVPO),
            "CMOVS" => Ok(Mnemonic::CMOVS),
            "CMOVZ" => Ok(Mnemonic::CMOVZ),
            "CMP" => Ok(Mnemonic::CMP),
            "CMPPD" => Ok(Mnemonic::CMPPD),
            "CMPPS" => Ok(Mnemonic::CMPPS),
            "CMPS" => Ok(Mnemonic::CMPS),
            "CMPSB" => Ok(Mnemonic::CMPSB),
            "CMPSD" => Ok(Mnemonic::CMPSD),
            "CMPSQ" => Ok(Mnemonic::CMPSQ),
            "CMPSS" => Ok(Mnemonic::CMPSS),
            "CMPSW" => Ok(Mnemonic::CMPSW),
            "CMPXCHG" => Ok(Mnemonic::CMPXCHG),
            "CMPXCHG16B" => Ok(Mnemonic::CMPXCHG16B),
            "CMPXCHG8B" => Ok(Mnemonic::CMPXCHG8B),
            "COMISD" => Ok(Mnemonic::COMISD),
            "COMISS" => Ok(Mnemonic::COMISS),
            "CPUID" => Ok(Mnemonic::CPUID),
            "CQO" => Ok(Mnemonic::CQO),
            "CRC32" => Ok(Mnemonic::CRC32),
            "CVTDQ2PD" => Ok(Mnemonic::CVTDQ2PD),
            "CVTDQ2PS" => Ok(Mnemonic::CVTDQ2PS),
            "CVTPD2DQ" => Ok(Mnemonic::CVTPD2DQ),
            "CVTPD2PI" => Ok(Mnemonic::CVTPD2PI),
            "CVTPD2PS" => Ok(Mnemonic::CVTPD2PS),
            "CVTPI2PD" => Ok(Mnemonic::CVTPI2PD),
            "CVTPI2PS" => Ok(Mnemonic::CVTPI2PS),
            "CVTPS2DQ" => Ok(Mnemonic::CVTPS2DQ),
            "CVTPS2PD" => Ok(Mnemonic::CVTPS2PD),
            "CVTPS2PI" => Ok(Mnemonic::CVTPS2PI),
            "CVTSD2SI" => Ok(Mnemonic::CVTSD2SI),
            "CVTSD2SS" => Ok(Mnemonic::CVTSD2SS),
            "CVTSI2SD" => Ok(Mnemonic::CVTSI2SD),
            "CVTSI2SS" => Ok(Mnemonic::CVTSI2SS),
            "CVTSS2SD" => Ok(Mnemonic::CVTSS2SD),
            "CVTSS2SI" => Ok(Mnemonic::CVTSS2SI),
            "CVTTPD2DQ" => Ok(Mnemonic::CVTTPD2DQ),
            "CVTTPD2PI" => Ok(Mnemonic::CVTTPD2PI),
            "CVTTPS2DQ" => Ok(Mnemonic::CVTTPS2DQ),
            "CVTTPS2PI" => Ok(Mnemonic::CVTTPS2PI),
            "CVTTSD2SI" => Ok(Mnemonic::CVTTSD2SI),
            "CVTTSS2SI" => Ok(Mnemonic::CVTTSS2SI),
            "CWD" => Ok(Mnemonic::CWD),
            "CWDE" => Ok(Mnemonic::CWDE),
            "DAA" => Ok(Mnemonic::DAA),
            "DAS" => Ok(Mnemonic::DAS),
            "DEC" => Ok(Mnemonic::DEC),
            "DIV" => Ok(Mnemonic::DIV),
            "DIVPD" => Ok(Mnemonic::DIVPD),
            "DIVPS" => Ok(Mnemonic::DIVPS),
            "DIVSD" => Ok(Mnemonic::DIVSD),
            "DIVSS" => Ok(Mnemonic::DIVSS),
            "DPPS" => Ok(Mnemonic::DPPS),
            "EMMS" => Ok(Mnemonic::EMMS),
            "ENTER" => Ok(Mnemonic::ENTER),
            "EXTRACTPS" => Ok(Mnemonic::EXTRACTPS),
            "F2XM1" => Ok(Mnemonic::F2XM1),
            "FABS" => Ok(Mnemonic::FABS),
            "FADD" => Ok(Mnemonic::FADD),
            "FADDP" => Ok(Mnemonic::FADDP),
            "FBLD" => Ok(Mnemonic::FBLD),
            "FBSTP" => Ok(Mnemonic::FBSTP),
            "FCHS" => Ok(Mnemonic::FCHS),
            "FCLEX" => Ok(Mnemonic::FCLEX),
            "FCMOVB" => Ok(Mnemonic::FCMOVB),
            "FCMOVBE" => Ok(Mnemonic::FCMOVBE),
            "FCMOVE" => Ok(Mnemonic::FCMOVE),
            "FCMOVNB" => Ok(Mnemonic::FCMOVNB),
            "FCMOVNBE" => Ok(Mnemonic::FCMOVNBE),
            "FCMOVNE" => Ok(Mnemonic::FCMOVNE),
            "FCMOVNU" => Ok(Mnemonic::FCMOVNU),
            "FCMOVU" => Ok(Mnemonic::FCMOVU),
            "FCOM" => Ok(Mnemonic::FCOM),
            "FCOM2" => Ok(Mnemonic::FCOM2),
            "FCOMI" => Ok(Mnemonic::FCOMI),
            "FCOMIP" => Ok(Mnemonic::FCOMIP),
            "FCOMP" => Ok(Mnemonic::FCOMP),
            "FCOMP3" => Ok(Mnemonic::FCOMP3),
            "FCOMP5" => Ok(Mnemonic::FCOMP5),
            "FCOMPP" => Ok(Mnemonic::FCOMPP),
            "FCOS" => Ok(Mnemonic::FCOS),
            "FDECSTP" => Ok(Mnemonic::FDECSTP),
            "FDISI" => Ok(Mnemonic::FDISI),
            "FDIV" => Ok(Mnemonic::FDIV),
            "FDIVP" => Ok(Mnemonic::FDIVP),
            "FDIVR" => Ok(Mnemonic::FDIVR),
            "FDIVRP" => Ok(Mnemonic::FDIVRP),
            "FENI" => Ok(Mnemonic::FENI),
            "FFREE" => Ok(Mnemonic::FFREE),
            "FFREEP" => Ok(Mnemonic::FFREEP),
            "FIADD" => Ok(Mnemonic::FIADD),
            "FICOM" => Ok(Mnemonic::FICOM),
            "FICOMP" => Ok(Mnemonic::FICOMP),
            "FIDIV" => Ok(Mnemonic::FIDIV),
            "FIDIVR" => Ok(Mnemonic::FIDIVR),
            "FILD" => Ok(Mnemonic::FILD),
            "FIMUL" => Ok(Mnemonic::FIMUL),
            "FINCSTP" => Ok(Mnemonic::FINCSTP),
            "FINIT" => Ok(Mnemonic::FINIT),
            "FIST" => Ok(Mnemonic::FIST),
            "FISTP" => Ok(Mnemonic::FISTP),
            "FISTTP" => Ok(Mnemonic::FISTTP),
            "FISUB" => Ok(Mnemonic::FISUB),
            "FISUBR" => Ok(Mnemonic::FISUBR),
            "FLD" => Ok(Mnemonic::FLD),
            "FLD1" => Ok(Mnemonic::FLD1),
            "FLDCW" => Ok(Mnemonic::FLDCW),
            "FLDENV" => Ok(Mnemonic::FLDENV),
            "FLDL2E" => Ok(Mnemonic::FLDL2E),
            "FLDL2T" => Ok(Mnemonic::FLDL2T),
            "FLDLG2" => Ok(Mnemonic::FLDLG2),
            "FLDLN2" => Ok(Mnemonic::FLDLN2),
            "FLDPI" => Ok(Mnemonic::FLDPI),
            "FLDZ" => Ok(Mnemonic::FLDZ),
            "FMUL" => Ok(Mnemonic::FMUL),
            "FMULP" => Ok(Mnemonic::FMULP),
            "FNCLEX" => Ok(Mnemonic::FNCLEX),
            "FNDISI" => Ok(Mnemonic::FNDISI),
            "FNENI" => Ok(Mnemonic::FNENI),
            "FNINIT" => Ok(Mnemonic::FNINIT),
            "FNOP" => Ok(Mnemonic::FNOP),
            "FNSAVE" => Ok(Mnemonic::FNSAVE),
            "FNSETPM" => Ok(Mnemonic::FNSETPM),
            "FNSTCW" => Ok(Mnemonic::FNSTCW),
            "FNSTENV" => Ok(Mnemonic::FNSTENV),
            "FNSTSW" => Ok(Mnemonic::FNSTSW),
            "FPATAN" => Ok(Mnemonic::FPATAN),
            "FPREM" => Ok(Mnemonic::FPREM),
            "FPREM1" => Ok(Mnemonic::FPREM1),
            "FPTAN" => Ok(Mnemonic::FPTAN),
            "FRNDINT" => Ok(Mnemonic::FRNDINT),
            "FRSTOR" => Ok(Mnemonic::FRSTOR),
            "FSAVE" => Ok(Mnemonic::FSAVE),
            "FSCALE" => Ok(Mnemonic::FSCALE),
            "FSETPM" => Ok(Mnemonic::FSETPM),
            "FSIN" => Ok(Mnemonic::FSIN),
            "FSINCOS" => Ok(Mnemonic::FSINCOS),
            "FSQRT" => Ok(Mnemonic::FSQRT),
            "FST" => Ok(Mnemonic::FST),
            "FSTCW" => Ok(Mnemonic::FSTCW),
            "FSTENV" => Ok(Mnemonic::FSTENV),
            "FSTP" => Ok(Mnemonic::FSTP),
            "FSTP1" => Ok(Mnemonic::FSTP1),
            "FSTP8" => Ok(Mnemonic::FSTP8),
            "FSTP9" => Ok(Mnemonic::FSTP9),
            "FSTSW" => Ok(Mnemonic::FSTSW),
            "FSUB" => Ok(Mnemonic::FSUB),
            "FSUBP" => Ok(Mnemonic::FSUBP),
            "FSUBR" => Ok(Mnemonic::FSUBR),
            "FSUBRP" => Ok(Mnemonic::FSUBRP),
            "FTST" => Ok(Mnemonic::FTST),
            "FUCOM" => Ok(Mnemonic::FUCOM),
            "FUCOMI" => Ok(Mnemonic::FUCOMI),
            "FUCOMIP" => Ok(Mnemonic::FUCOMIP),
            "FUCOMP" => Ok(Mnemonic::FUCOMP),
            "FUCOMPP" => Ok(Mnemonic::FUCOMPP),
            "FWAIT" => Ok(Mnemonic::FWAIT),
            "FXAM" => Ok(Mnemonic::FXAM),
            "FXCH" => Ok(Mnemonic::FXCH),
            "FXCH4" => Ok(Mnemonic::FXCH4),
            "FXCH7" => Ok(Mnemonic::FXCH7),
            "FXRSTOR" => Ok(Mnemonic::FXRSTOR),
            "FXSAVE" => Ok(Mnemonic::FXSAVE),
            "FXTRACT" => Ok(Mnemonic::FXTRACT),
            "FYL2X" => Ok(Mnemonic::FYL2X),
            "FYL2XP1" => Ok(Mnemonic::FYL2XP1),
            "GETSEC" => Ok(Mnemonic::GETSEC),
            "HADDPD" => Ok(Mnemonic::HADDPD),
            "HADDPS" => Ok(Mnemonic::HADDPS),
            "HLT" => Ok(Mnemonic::HLT),
            "HSUBPD" => Ok(Mnemonic::HSUBPD),
            "HSUBPS" => Ok(Mnemonic::HSUBPS),
            "ICEBP" => Ok(Mnemonic::ICEBP),
            "IDIV" => Ok(Mnemonic::IDIV),
            "IMUL" => Ok(Mnemonic::IMUL),
            "IN" => Ok(Mnemonic::IN),
            "INC" => Ok(Mnemonic::INC),
            "INS" => Ok(Mnemonic::INS),
            "INSB" => Ok(Mnemonic::INSB),
            "INSD" => Ok(Mnemonic::INSD),
            "INSERTPS" => Ok(Mnemonic::INSERTPS),
            "INSW" => Ok(Mnemonic::INSW),
            "INT" => Ok(Mnemonic::INT),
            "INT1" => Ok(Mnemonic::INT1),
            "INTO" => Ok(Mnemonic::INTO),
            "INVD" => Ok(Mnemonic::INVD),
            "INVEPT" => Ok(Mnemonic::INVEPT),
            "INVLPG" => Ok(Mnemonic::INVLPG),
            "INVVPID" => Ok(Mnemonic::INVVPID),
            "IRET" => Ok(Mnemonic::IRET),
            "IRETD" => Ok(Mnemonic::IRETD),
            "IRETQ" => Ok(Mnemonic::IRETQ),
            "JA" => Ok(Mnemonic::JA),
            "JAE" => Ok(Mnemonic::JAE),
            "JB" => Ok(Mnemonic::JB),
            "JBE" => Ok(Mnemonic::JBE),
            "JC" => Ok(Mnemonic::JC),
            "JCXZ" => Ok(Mnemonic::JCXZ),
            "JE" => Ok(Mnemonic::JE),
            "JECXZ" => Ok(Mnemonic::JECXZ),
            "JG" => Ok(Mnemonic::JG),
            "JGE" => Ok(Mnemonic::JGE),
            "JL" => Ok(Mnemonic::JL),
            "JLE" => Ok(Mnemonic::JLE),
            "JMP" => Ok(Mnemonic::JMP),
            "JMPF" => Ok(Mnemonic::JMPF),
            "JNA" => Ok(Mnemonic::JNA),
            "JNAE" => Ok(Mnemonic::JNAE),
            "JNB" => Ok(Mnemonic::JNB),
            "JNBE" => Ok(Mnemonic::JNBE),
            "JNC" => Ok(Mnemonic::JNC),
            "JNE" => Ok(Mnemonic::JNE),
            "JNG" => Ok(Mnemonic::JNG),
            "JNGE" => Ok(Mnemonic::JNGE),
            "JNL" => Ok(Mnemonic::JNL),
            "JNLE" => Ok(Mnemonic::JNLE),
            "JNO" => Ok(Mnemonic::JNO),
            "JNP" => Ok(Mnemonic::JNP),
            "JNS" => Ok(Mnemonic::JNS),
            "JNZ" => Ok(Mnemonic::JNZ),
            "JO" => Ok(Mnemonic::JO),
            "JP" => Ok(Mnemonic::JP),
            "JPE" => Ok(Mnemonic::JPE),
            "JPO" => Ok(Mnemonic::JPO),
            "JRCXZ" => Ok(Mnemonic::JRCXZ),
            "JS" => Ok(Mnemonic::JS),
            "JZ" => Ok(Mnemonic::JZ),
            "LAHF" => Ok(Mnemonic::LAHF),
            "LAR" => Ok(Mnemonic::LAR),
            "LDDQU" => Ok(Mnemonic::LDDQU),
            "LDMXCSR" => Ok(Mnemonic::LDMXCSR),
            "LDS" => Ok(Mnemonic::LDS),
            "LEA" => Ok(Mnemonic::LEA),
            "LEAVE" => Ok(Mnemonic::LEAVE),
            "LES" => Ok(Mnemonic::LES),
            "LFENCE" => Ok(Mnemonic::LFENCE),
            "LFS" => Ok(Mnemonic::LFS),
            "LGDT" => Ok(Mnemonic::LGDT),
            "LGS" => Ok(Mnemonic::LGS),
            "LIDT" => Ok(Mnemonic::LIDT),
            "LLDT" => Ok(Mnemonic::LLDT),
            "LMSW" => Ok(Mnemonic::LMSW),
            "LOADALL" => Ok(Mnemonic::LOADALL),
            "LODS" => Ok(Mnemonic::LODS),
            "LODSB" => Ok(Mnemonic::LODSB),
            "LODSD" => Ok(Mnemonic::LODSD),
            "LODSQ" => Ok(Mnemonic::LODSQ),
            "LODSW" => Ok(Mnemonic::LODSW),
            "LOOP" => Ok(Mnemonic::LOOP),
            "LOOPE" => Ok(Mnemonic::LOOPE),
            "LOOPNE" => Ok(Mnemonic::LOOPNE),
            "LOOPNZ" => Ok(Mnemonic::LOOPNZ),
            "LOOPZ" => Ok(Mnemonic::LOOPZ),
            "LSL" => Ok(Mnemonic::LSL),
            "LSS" => Ok(Mnemonic::LSS),
            "LTR" => Ok(Mnemonic::LTR),
            "MASKMOVDQU" => Ok(Mnemonic::MASKMOVDQU),
            "MASKMOVQ" => Ok(Mnemonic::MASKMOVQ),
            "MAXPD" => Ok(Mnemonic::MAXPD),
            "MAXPS" => Ok(Mnemonic::MAXPS),
            "MAXSD" => Ok(Mnemonic::MAXSD),
            "MAXSS" => Ok(Mnemonic::MAXSS),
            "MFENCE" => Ok(Mnemonic::MFENCE),
            "MINPD" => Ok(Mnemonic::MINPD),
            "MINPS" => Ok(Mnemonic::MINPS),
            "MINSD" => Ok(Mnemonic::MINSD),
            "MINSS" => Ok(Mnemonic::MINSS),
            "MONITOR" => Ok(Mnemonic::MONITOR),
            "MOV" => Ok(Mnemonic::MOV),
            "MOVAPD" => Ok(Mnemonic::MOVAPD),
            "MOVAPS" => Ok(Mnemonic::MOVAPS),
            "MOVBE" => Ok(Mnemonic::MOVBE),
            "MOVD" => Ok(Mnemonic::MOVD),
            "MOVDDUP" => Ok(Mnemonic::MOVDDUP),
            "MOVDQ2Q" => Ok(Mnemonic::MOVDQ2Q),
            "MOVDQA" => Ok(Mnemonic::MOVDQA),
            "MOVDQU" => Ok(Mnemonic::MOVDQU),
            "MOVHLPS" => Ok(Mnemonic::MOVHLPS),
            "MOVHPD" => Ok(Mnemonic::MOVHPD),
            "MOVHPS" => Ok(Mnemonic::MOVHPS),
            "MOVLHPS" => Ok(Mnemonic::MOVLHPS),
            "MOVLPD" => Ok(Mnemonic::MOVLPD),
            "MOVLPS" => Ok(Mnemonic::MOVLPS),
            "MOVMSKPD" => Ok(Mnemonic::MOVMSKPD),
            "MOVMSKPS" => Ok(Mnemonic::MOVMSKPS),
            "MOVNTDQ" => Ok(Mnemonic::MOVNTDQ),
            "MOVNTDQA" => Ok(Mnemonic::MOVNTDQA),
            "MOVNTI" => Ok(Mnemonic::MOVNTI),
            "MOVNTPD" => Ok(Mnemonic::MOVNTPD),
            "MOVNTPS" => Ok(Mnemonic::MOVNTPS),
            "MOVNTQ" => Ok(Mnemonic::MOVNTQ),
            "MOVQ" => Ok(Mnemonic::MOVQ),
            "MOVQ2DQ" => Ok(Mnemonic::MOVQ2DQ),
            "MOVS" => Ok(Mnemonic::MOVS),
            "MOVSB" => Ok(Mnemonic::MOVSB),
            "MOVSD" => Ok(Mnemonic::MOVSD),
            "MOVSHDUP" => Ok(Mnemonic::MOVSHDUP),
            "MOVSLDUP" => Ok(Mnemonic::MOVSLDUP),
            "MOVSQ" => Ok(Mnemonic::MOVSQ),
            "MOVSS" => Ok(Mnemonic::MOVSS),
            "MOVSW" => Ok(Mnemonic::MOVSW),
            "MOVSX" => Ok(Mnemonic::MOVSX),
            "MOVSXD" => Ok(Mnemonic::MOVSXD),
            "MOVUPD" => Ok(Mnemonic::MOVUPD),
            "MOVUPS" => Ok(Mnemonic::MOVUPS),
            "MOVZX" => Ok(Mnemonic::MOVZX),
            "MPSADBW" => Ok(Mnemonic::MPSADBW),
            "MUL" => Ok(Mnemonic::MUL),
            "MULPD" => Ok(Mnemonic::MULPD),
            "MULPS" => Ok(Mnemonic::MULPS),
            "MULSD" => Ok(Mnemonic::MULSD),
            "MULSS" => Ok(Mnemonic::MULSS),
            "MWAIT" => Ok(Mnemonic::MWAIT),
            "NEG" => Ok(Mnemonic::NEG),
            "NOP" => Ok(Mnemonic::NOP),
            "NOT" => Ok(Mnemonic::NOT),
            "OR" => Ok(Mnemonic::OR),
            "ORPD" => Ok(Mnemonic::ORPD),
            "ORPS" => Ok(Mnemonic::ORPS),
            "OUT" => Ok(Mnemonic::OUT),
            "OUTS" => Ok(Mnemonic::OUTS),
            "OUTSB" => Ok(Mnemonic::OUTSB),
            "OUTSD" => Ok(Mnemonic::OUTSD),
            "OUTSW" => Ok(Mnemonic::OUTSW),
            "PABSB" => Ok(Mnemonic::PABSB),
            "PABSD" => Ok(Mnemonic::PABSD),
            "PABSW" => Ok(Mnemonic::PABSW),
            "PACKSSDW" => Ok(Mnemonic::PACKSSDW),
            "PACKSSWB" => Ok(Mnemonic::PACKSSWB),
            "PACKUSDW" => Ok(Mnemonic::PACKUSDW),
            "PACKUSWB" => Ok(Mnemonic::PACKUSWB),
            "PADDB" => Ok(Mnemonic::PADDB),
            "PADDD" => Ok(Mnemonic::PADDD),
            "PADDQ" => Ok(Mnemonic::PADDQ),
            "PADDSB" => Ok(Mnemonic::PADDSB),
            "PADDSW" => Ok(Mnemonic::PADDSW),
            "PADDUSB" => Ok(Mnemonic::PADDUSB),
            "PADDUSW" => Ok(Mnemonic::PADDUSW),
            "PADDW" => Ok(Mnemonic::PADDW),
            "PALIGNR" => Ok(Mnemonic::PALIGNR),
            "PAND" => Ok(Mnemonic::PAND),
            "PANDN" => Ok(Mnemonic::PANDN),
            "PAUSE" => Ok(Mnemonic::PAUSE),
            "PAVGB" => Ok(Mnemonic::PAVGB),
            "PAVGW" => Ok(Mnemonic::PAVGW),
            "PBLENDVB" => Ok(Mnemonic::PBLENDVB),
            "PBLENDW" => Ok(Mnemonic::PBLENDW),
            "PCMPEQB" => Ok(Mnemonic::PCMPEQB),
            "PCMPEQD" => Ok(Mnemonic::PCMPEQD),
            "PCMPEQQ" => Ok(Mnemonic::PCMPEQQ),
            "PCMPEQW" => Ok(Mnemonic::PCMPEQW),
            "PCMPESTRI" => Ok(Mnemonic::PCMPESTRI),
            "PCMPESTRM" => Ok(Mnemonic::PCMPESTRM),
            "PCMPGTB" => Ok(Mnemonic::PCMPGTB),
            "PCMPGTD" => Ok(Mnemonic::PCMPGTD),
            "PCMPGTQ" => Ok(Mnemonic::PCMPGTQ),
            "PCMPGTW" => Ok(Mnemonic::PCMPGTW),
            "PCMPISTRI" => Ok(Mnemonic::PCMPISTRI),
            "PCMPISTRM" => Ok(Mnemonic::PCMPISTRM),
            "PEXTRB" => Ok(Mnemonic::PEXTRB),
            "PEXTRD" => Ok(Mnemonic::PEXTRD),
            "PEXTRQ" => Ok(Mnemonic::PEXTRQ),
            "PEXTRW" => Ok(Mnemonic::PEXTRW),
            "PHADDD" => Ok(Mnemonic::PHADDD),
            "PHADDSW" => Ok(Mnemonic::PHADDSW),
            "PHADDW" => Ok(Mnemonic::PHADDW),
            "PHMINPOSUW" => Ok(Mnemonic::PHMINPOSUW),
            "PHSUBD" => Ok(Mnemonic::PHSUBD),
            "PHSUBSW" => Ok(Mnemonic::PHSUBSW),
            "PHSUBW" => Ok(Mnemonic::PHSUBW),
            "PINSRB" => Ok(Mnemonic::PINSRB),
            "PINSRD" => Ok(Mnemonic::PINSRD),
            "PINSRQ" => Ok(Mnemonic::PINSRQ),
            "PINSRW" => Ok(Mnemonic::PINSRW),
            "PMADDUBSW" => Ok(Mnemonic::PMADDUBSW),
            "PMADDWD" => Ok(Mnemonic::PMADDWD),
            "PMAXSB" => Ok(Mnemonic::PMAXSB),
            "PMAXSD" => Ok(Mnemonic::PMAXSD),
            "PMAXSW" => Ok(Mnemonic::PMAXSW),
            "PMAXUB" => Ok(Mnemonic::PMAXUB),
            "PMAXUD" => Ok(Mnemonic::PMAXUD),
            "PMAXUW" => Ok(Mnemonic::PMAXUW),
            "PMINSB" => Ok(Mnemonic::PMINSB),
            "PMINSD" => Ok(Mnemonic::PMINSD),
            "PMINSW" => Ok(Mnemonic::PMINSW),
            "PMINUB" => Ok(Mnemonic::PMINUB),
            "PMINUD" => Ok(Mnemonic::PMINUD),
            "PMINUW" => Ok(Mnemonic::PMINUW),
            "PMOVMSKB" => Ok(Mnemonic::PMOVMSKB),
            "PMOVSXBD" => Ok(Mnemonic::PMOVSXBD),
            "PMOVSXBQ" => Ok(Mnemonic::PMOVSXBQ),
            "PMOVSXBW" => Ok(Mnemonic::PMOVSXBW),
            "PMOVSXDQ" => Ok(Mnemonic::PMOVSXDQ),
            "PMOVSXWD" => Ok(Mnemonic::PMOVSXWD),
            "PMOVSXWQ" => Ok(Mnemonic::PMOVSXWQ),
            "PMOVZXBD" => Ok(Mnemonic::PMOVZXBD),
            "PMOVZXBQ" => Ok(Mnemonic::PMOVZXBQ),
            "PMOVZXBW" => Ok(Mnemonic::PMOVZXBW),
            "PMOVZXDQ" => Ok(Mnemonic::PMOVZXDQ),
            "PMOVZXWD" => Ok(Mnemonic::PMOVZXWD),
            "PMOVZXWQ" => Ok(Mnemonic::PMOVZXWQ),
            "PMULDQ" => Ok(Mnemonic::PMULDQ),
            "PMULHRSW" => Ok(Mnemonic::PMULHRSW),
            "PMULHUW" => Ok(Mnemonic::PMULHUW),
            "PMULHW" => Ok(Mnemonic::PMULHW),
            "PMULLD" => Ok(Mnemonic::PMULLD),
            "PMULLW" => Ok(Mnemonic::PMULLW),
            "PMULUDQ" => Ok(Mnemonic::PMULUDQ),
            "POP" => Ok(Mnemonic::POP),
            "POPA" => Ok(Mnemonic::POPA),
            "POPAD" => Ok(Mnemonic::POPAD),
            "POPCNT" => Ok(Mnemonic::POPCNT),
            "POPF" => Ok(Mnemonic::POPF),
            "POPFD" => Ok(Mnemonic::POPFD),
            "POPFQ" => Ok(Mnemonic::POPFQ),
            "POR" => Ok(Mnemonic::POR),
            "PREFETCHNTA" => Ok(Mnemonic::PREFETCHNTA),
            "PREFETCHT0" => Ok(Mnemonic::PREFETCHT0),
            "PREFETCHT1" => Ok(Mnemonic::PREFETCHT1),
            "PREFETCHT2" => Ok(Mnemonic::PREFETCHT2),
            "PSADBW" => Ok(Mnemonic::PSADBW),
            "PSHUFB" => Ok(Mnemonic::PSHUFB),
            "PSHUFD" => Ok(Mnemonic::PSHUFD),
            "PSHUFHW" => Ok(Mnemonic::PSHUFHW),
            "PSHUFLW" => Ok(Mnemonic::PSHUFLW),
            "PSHUFW" => Ok(Mnemonic::PSHUFW),
            "PSIGNB" => Ok(Mnemonic::PSIGNB),
            "PSIGND" => Ok(Mnemonic::PSIGND),
            "PSIGNW" => Ok(Mnemonic::PSIGNW),
            "PSLLD" => Ok(Mnemonic::PSLLD),
            "PSLLDQ" => Ok(Mnemonic::PSLLDQ),
            "PSLLQ" => Ok(Mnemonic::PSLLQ),
            "PSLLW" => Ok(Mnemonic::PSLLW),
            "PSRAD" => Ok(Mnemonic::PSRAD),
            "PSRAW" => Ok(Mnemonic::PSRAW),
            "PSRLD" => Ok(Mnemonic::PSRLD),
            "PSRLDQ" => Ok(Mnemonic::PSRLDQ),
            "PSRLQ" => Ok(Mnemonic::PSRLQ),
            "PSRLW" => Ok(Mnemonic::PSRLW),
            "PSUBB" => Ok(Mnemonic::PSUBB),
            "PSUBD" => Ok(Mnemonic::PSUBD),
            "PSUBQ" => Ok(Mnemonic::PSUBQ),
            "PSUBSB" => Ok(Mnemonic::PSUBSB),
            "PSUBSW" => Ok(Mnemonic::PSUBSW),
            "PSUBUSB" => Ok(Mnemonic::PSUBUSB),
            "PSUBUSW" => Ok(Mnemonic::PSUBUSW),
            "PSUBW" => Ok(Mnemonic::PSUBW),
            "PTEST" => Ok(Mnemonic::PTEST),
            "PUNPCKHBW" => Ok(Mnemonic::PUNPCKHBW),
            "PUNPCKHDQ" => Ok(Mnemonic::PUNPCKHDQ),
            "PUNPCKHQDQ" => Ok(Mnemonic::PUNPCKHQDQ),
            "PUNPCKHWD" => Ok(Mnemonic::PUNPCKHWD),
            "PUNPCKLBW" => Ok(Mnemonic::PUNPCKLBW),
            "PUNPCKLDQ" => Ok(Mnemonic::PUNPCKLDQ),
            "PUNPCKLQDQ" => Ok(Mnemonic::PUNPCKLQDQ),
            "PUNPCKLWD" => Ok(Mnemonic::PUNPCKLWD),
            "PUSH" => Ok(Mnemonic::PUSH),
            "PUSHA" => Ok(Mnemonic::PUSHA),
            "PUSHAD" => Ok(Mnemonic::PUSHAD),
            "PUSHF" => Ok(Mnemonic::PUSHF),
            "PUSHFD" => Ok(Mnemonic::PUSHFD),
            "PUSHFQ" => Ok(Mnemonic::PUSHFQ),
            "PXOR" => Ok(Mnemonic::PXOR),
            "RCL" => Ok(Mnemonic::RCL),
            "RCPPS" => Ok(Mnemonic::RCPPS),
            "RCPSS" => Ok(Mnemonic::RCPSS),
            "RCR" => Ok(Mnemonic::RCR),
            "RDMSR" => Ok(Mnemonic::RDMSR),
            "RDPMC" => Ok(Mnemonic::RDPMC),
            "RDTSC" => Ok(Mnemonic::RDTSC),
            "RDTSCP" => Ok(Mnemonic::RDTSCP),
            "RETF" => Ok(Mnemonic::RETF),
            "RETN" => Ok(Mnemonic::RETN),
            "ROL" => Ok(Mnemonic::ROL),
            "ROR" => Ok(Mnemonic::ROR),
            "ROUNDPD" => Ok(Mnemonic::ROUNDPD),
            "ROUNDPS" => Ok(Mnemonic::ROUNDPS),
            "ROUNDSD" => Ok(Mnemonic::ROUNDSD),
            "ROUNDSS" => Ok(Mnemonic::ROUNDSS),
            "RSM" => Ok(Mnemonic::RSM),
            "RSQRTPS" => Ok(Mnemonic::RSQRTPS),
            "RSQRTSS" => Ok(Mnemonic::RSQRTSS),
            "SAHF" => Ok(Mnemonic::SAHF),
            "SAL" => Ok(Mnemonic::SAL),
            "SALC" => Ok(Mnemonic::SALC),
            "SAR" => Ok(Mnemonic::SAR),
            "SBB" => Ok(Mnemonic::SBB),
            "SCAS" => Ok(Mnemonic::SCAS),
            "SCASB" => Ok(Mnemonic::SCASB),
            "SCASD" => Ok(Mnemonic::SCASD),
            "SCASQ" => Ok(Mnemonic::SCASQ),
            "SCASW" => Ok(Mnemonic::SCASW),
            "SETA" => Ok(Mnemonic::SETA),
            "SETAE" => Ok(Mnemonic::SETAE),
            "SETALC" => Ok(Mnemonic::SETALC),
            "SETB" => Ok(Mnemonic::SETB),
            "SETBE" => Ok(Mnemonic::SETBE),
            "SETC" => Ok(Mnemonic::SETC),
            "SETE" => Ok(Mnemonic::SETE),
            "SETG" => Ok(Mnemonic::SETG),
            "SETGE" => Ok(Mnemonic::SETGE),
            "SETL" => Ok(Mnemonic::SETL),
            "SETLE" => Ok(Mnemonic::SETLE),
            "SETNA" => Ok(Mnemonic::SETNA),
            "SETNAE" => Ok(Mnemonic::SETNAE),
            "SETNB" => Ok(Mnemonic::SETNB),
            "SETNBE" => Ok(Mnemonic::SETNBE),
            "SETNC" => Ok(Mnemonic::SETNC),
            "SETNE" => Ok(Mnemonic::SETNE),
            "SETNG" => Ok(Mnemonic::SETNG),
            "SETNGE" => Ok(Mnemonic::SETNGE),
            "SETNL" => Ok(Mnemonic::SETNL),
            "SETNLE" => Ok(Mnemonic::SETNLE),
            "SETNO" => Ok(Mnemonic::SETNO),
            "SETNP" => Ok(Mnemonic::SETNP),
            "SETNS" => Ok(Mnemonic::SETNS),
            "SETNZ" => Ok(Mnemonic::SETNZ),
            "SETO" => Ok(Mnemonic::SETO),
            "SETP" => Ok(Mnemonic::SETP),
            "SETPE" => Ok(Mnemonic::SETPE),
            "SETPO" => Ok(Mnemonic::SETPO),
            "SETS" => Ok(Mnemonic::SETS),
            "SETZ" => Ok(Mnemonic::SETZ),
            "SFENCE" => Ok(Mnemonic::SFENCE),
            "SGDT" => Ok(Mnemonic::SGDT),
            "SHL" => Ok(Mnemonic::SHL),
            "SHLD" => Ok(Mnemonic::SHLD),
            "SHR" => Ok(Mnemonic::SHR),
            "SHRD" => Ok(Mnemonic::SHRD),
            "SHUFPD" => Ok(Mnemonic::SHUFPD),
            "SHUFPS" => Ok(Mnemonic::SHUFPS),
            "SIDT" => Ok(Mnemonic::SIDT),
            "SLDT" => Ok(Mnemonic::SLDT),
            "SMSW" => Ok(Mnemonic::SMSW),
            "SQRTPD" => Ok(Mnemonic::SQRTPD),
            "SQRTPS" => Ok(Mnemonic::SQRTPS),
            "SQRTSD" => Ok(Mnemonic::SQRTSD),
            "SQRTSS" => Ok(Mnemonic::SQRTSS),
            "STC" => Ok(Mnemonic::STC),
            "STD" => Ok(Mnemonic::STD),
            "STI" => Ok(Mnemonic::STI),
            "STMXCSR" => Ok(Mnemonic::STMXCSR),
            "STOS" => Ok(Mnemonic::STOS),
            "STOSB" => Ok(Mnemonic::STOSB),
            "STOSD" => Ok(Mnemonic::STOSD),
            "STOSQ" => Ok(Mnemonic::STOSQ),
            "STOSW" => Ok(Mnemonic::STOSW),
            "STR" => Ok(Mnemonic::STR),
            "SUB" => Ok(Mnemonic::SUB),
            "SUBPD" => Ok(Mnemonic::SUBPD),
            "SUBPS" => Ok(Mnemonic::SUBPS),
            "SUBSD" => Ok(Mnemonic::SUBSD),
            "SUBSS" => Ok(Mnemonic::SUBSS),
            "SWAPGS" => Ok(Mnemonic::SWAPGS),
            "SYSCALL" => Ok(Mnemonic::SYSCALL),
            "SYSENTER" => Ok(Mnemonic::SYSENTER),
            "SYSEXIT" => Ok(Mnemonic::SYSEXIT),
            "SYSRET" => Ok(Mnemonic::SYSRET),
            "TEST" => Ok(Mnemonic::TEST),
            "UCOMISD" => Ok(Mnemonic::UCOMISD),
            "UCOMISS" => Ok(Mnemonic::UCOMISS),
            "UNPCKHPD" => Ok(Mnemonic::UNPCKHPD),
            "UNPCKHPS" => Ok(Mnemonic::UNPCKHPS),
            "UNPCKLPD" => Ok(Mnemonic::UNPCKLPD),
            "UNPCKLPS" => Ok(Mnemonic::UNPCKLPS),
            "VERR" => Ok(Mnemonic::VERR),
            "VERW" => Ok(Mnemonic::VERW),
            "VMCALL" => Ok(Mnemonic::VMCALL),
            "VMCLEAR" => Ok(Mnemonic::VMCLEAR),
            "VMLAUNCH" => Ok(Mnemonic::VMLAUNCH),
            "VMPTRLD" => Ok(Mnemonic::VMPTRLD),
            "VMPTRST" => Ok(Mnemonic::VMPTRST),
            "VMREAD" => Ok(Mnemonic::VMREAD),
            "VMRESUME" => Ok(Mnemonic::VMRESUME),
            "VMWRITE" => Ok(Mnemonic::VMWRITE),
            "VMXOFF" => Ok(Mnemonic::VMXOFF),
            "VMXON" => Ok(Mnemonic::VMXON),
            "WAIT" => Ok(Mnemonic::WAIT),
            "WBINVD" => Ok(Mnemonic::WBINVD),
            "WRMSR" => Ok(Mnemonic::WRMSR),
            "XADD" => Ok(Mnemonic::XADD),
            "XCHG" => Ok(Mnemonic::XCHG),
            "XGETBV" => Ok(Mnemonic::XGETBV),
            "XLAT" => Ok(Mnemonic::XLAT),
            "XLATB" => Ok(Mnemonic::XLATB),
            "XOR" => Ok(Mnemonic::XOR),
            "XORPD" => Ok(Mnemonic::XORPD),
            "XORPS" => Ok(Mnemonic::XORPS),
            "XRSTOR" => Ok(Mnemonic::XRSTOR),
            "XSAVE" => Ok(Mnemonic::XSAVE),
            "XSETBV" => Ok(Mnemonic::XSETBV),
            _ => Err(())
        }
    }
}