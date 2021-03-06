// Added with later Core 2
// todo annotate, sort
pub enum SSE41Instr {
    MPSADBW,
    PHMINPOSUW,
    PMULLD,
    PMULDQ,
    DPPS,
    DPPD,
    BLENDPS,
    BLENDPD,
    BLENDVPS,
    BLENDVPD,
    PBLENDVB,
    PBLENDW,
    PMINSB,
    PMAXSB,
    PMINUW,
    PMAXUW,
    PMINUD,
    PMAXUD,
    PMINSD,
    PMAXSD,
    ROUNDPS,
    ROUNDSS,
    ROUNDPD,
    ROUNDSD,
    INSERTPS,
    PINSRB,
    PINSRD,
    PINSRQ,
    EXTRACTPS,
    PEXTRB,
    PEXTRW,
    PEXTRD,
    PEXTRQ,
    PMOVSXBW,
    PMOVZXBW,
    PMOVSXBD,
    PMOVZXBD,
    PMOVSXBQ,
    PMOVZXBQ,
    PMOVSXWD,
    PMOVZXWD,
    PMOVSXWQ,
    PMOVZXWQ,
    PMOVSXDQ,
    PMOVZXDQ,
    PTEST,
    PCMPEQQ,
    PACKUSDW,
    MOVNTDQA
}