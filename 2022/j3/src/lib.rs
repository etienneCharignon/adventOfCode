use std::str;

#[allow(dead_code)]
const EXAMPLE:&str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

#[allow(dead_code)]
const INPUT:&str = "LdHVLDLDdHdtLMhcqCqGWcWg
ZZQZSZnnJrQrJQJbfzfnWGWPWMcChMMPcqMnhFcF
ZrzpWzfbpQpWbzvZWZpdtVtDNmRHNVptNDHt
gzCjffWZCtCfZZVdqVSqJdvJndSt
hMHLcmGLMLhHmsRMsSvsQSqrsrlJTTdV
NPNGRGHGHNLczNzzZFWSFFCC
VSBpcvNNbNWWSfGRwtJnRtrzzGzGGn
jZlhTlQLHFLLZbwrzQQsttDtbs
hmmPFlhLmhLMgFMFLbMBBgcNVcfNCcfSVSSNBq
jRDSzjCjjMRMrHzMRCDHMDjBnlQbbnQwLwrNLPwnTPQrlc
sZBqdZqsWgFBpgppGJqllPllwnbQwTwsswQPwN
gJgtJJBtqJqWBGzjRCHDDzRmDtVV
GRBWbbWnGNhbwSsfPfmrlslWsS
HLVCgCLpMgcLVDcDCgmlvstSlsstPtSSSlscvl
LzQDQzMMzJzFQVDJgCzJHJZRZBNbqGFhNFwBGhbhBhBm
JNsHhdPZSdZJjSHzzNwvwGgBFmBmvptJbFvm
WrDrwqqqQWVMCvMvGbGbbFFbFp
rqqLncqnlCncwQWCwWlLfSZPsZddfzzNNdhdPlZz
rwfrwHqSdRcgwdZrDcrqDDdgNzjNjPzPJNJPtbNPbGsHNGHv
hllLMTLLQMCzJssPsTvtFG
QtQnpWVVMVWnVQpRZSSwZDggrcSq
FDlqPMBdmbqMrdDqqFdFDwjHZChHHZfZWZNBsZjhfhHf
VQJVgncpTQRJfsssGhsNWC
NtpLLSpcLVRzzRRtpgVcLgglMwwlPlzdmDlFmFPDmqqwlF
PqwwSqNWPqwSWqjNBwpTPpffhJfZfstRtZJRRdJsdR
VFLFzQVgVnDVjhdJHsvhZjfg
VVFVDDnmnzLFCzVmznFzrFlrjSCPNMBCSGSGwwwGBjPNWPwC
zzbCGrfgbzfzCtvqdjSrvSjnvS
cNVTLnJZRNNmQmhJNptvBlBtVjlljqqldtlB
ZhpcJHLQhRcLZLmNQJzfFwDzzgHPFngDPPnP
ttntdMMBZzbZZtjPfwjwTwBvvVmh
llSllRRNsGCrCDTPfgVgTfgvsjPH
CFclcCWGrJNDSnnWMbQTnWtLQd
bccfdSfwFsswcbbdJFGQVGnCJLnBLnDnQLDj
ThvHZNhZmqNWMNqvDVjcjjBQVhQBBVLB
vWMZZNRmNMWqtWqmqHclfFgdlwRwrzzSfwbFRd
WNzRWWZNmtNZnssNRPtCJFQJCffVJVffCvlF
cBqBLgHHBcgqBbCJjrvrVQJfsHlr
DhDShwchBSbdwBsqgGcbgTTRmzdNWmMRMmTdNpTMmW
FHWZDbbPZDFHgGGPdPbJNLpJSlNjcjSphcJjFN
ZCCrBBRwwCjcNwcljppN
CmTTsrMfBCRsTMnnCRTmVZbgdbdGfWHPDVvdbbPV
cgSNBScMgjBBPtBCNLVvVVvhhJJhvsMhVfWq
TFlDrHPZHTTFmwTQHZDZTrqvvzqlJppVJvpffhqqVfJp
DbPwrTZDQDRQQwQZrDrbbQwRRnNSBtCjtcNGjSgSLjLccC
pdcVCpdZnZgcZgdcDWBDNcNwvBWJwc
RRmHspRmmQfLwwJJbFBFFJNR
rGlpfrQrqGhGqdGP
THhNsHhdFjcDthDcjDhLBqWcLLQQJvvBbvBJbl
GfrCSZGCzfVMrVCCzGVfSMQBBFlJlJBFFZqvWBqbbQWl
rSrFwCwzMMzRfGrGMwPfGCVdgHDttthNPssHjmNNdDjgdD
shmhggDsZCZWBDmsQTcTqrLPTbNbwQQrrN
lzpFVfdjfFFGnVVHpjJGJVlprNMPNtPctTNwLtcTbwctwTnN
jVFfzVlFSpzpFCshWmgSRgmBRb
zZGFNPmdPdszdWddGWZlpLMLpbQbLDpblzQbtv
wTwgwSSCHhhCSghJbpcpDglbbQbMVMpV
BHnBRrJCHmRNfGDmfm
CffgvfCRGngRrvGvgdnRVpPQQPSqbVZZDPDVPzFSSQ
cMlWJTMlcTLTLtHHJlpqDLwqSzSSDDqDZDDS
tmHTtlMmWpmsMJsCRdCNsrRGfrnffN
lNrzNHNwzZlHmwNBpMqqnqGDZSpMTGnb
ddjgFjjsRvGvQhQvvFjqbVTFpbVpFDJSbJVpTS
cjsQhsjvRvGcgNtzczmtmwCCcr
NJMJvBmBJPtMtRDnDDwDMFFWDWHG
hZsrzshpSdjZZqSSfgpDwnwFnGCCLHDbjnGCWG
zfgSpScsrdpmllttGvJGcm
GLsnLVLZGZmcGVpgZLmTjTsDDTlDTHPPHWlHwD
dNJhCCdtNJSvdDzwPlvTlQ
BCbBrPPfRMfMJSffrMCMChrSqVcmLLFpqLFFcggLVnqgmbgc
hMdjMndZLRnRnjclszLclQlzGwcr
FPCCSCSpCwStJVGzsPQGslGzPbsQ
TSSJTtfTFTwtmTFFVFDTWRvdgndjjRhnjnvHjdfR
ZwgsnsWsWvWQHqJhGhJVCJHhCJ
cdjlMWRRMhJLCpLL
jjSRmRmNRNWBmdmcWjlDWFPswsPtnZQsnBsnZvvTTnvT
vvbjLTPbQzrQQjpLzLbflfjfRDDsDlRfgDnnVD
ZGCtHHFGzFHhMlCnDqwlgggsqf
FJHMhHZGmNHtJhMhBJZZtZdrbSbSzbmWLLQrQTbPdbrS
ZClGVCvLZzCLBVbdGGzVVBvVrqQMDWMHrgmgQLWrWmTgqqHH
tNpNspcQPfHmqfgHmHHg
nNttnwPRjFtPcccsFptPlllQGlQZbvGzVVVFzhCd
lflmVWpDVsMmmVPlHVbbGSSbGnSHJcncnLZn
NvTFzNwzTQvhFGSZnvgrbgJCgr
jtRwTQFzjBNGGQQBdDsDqqlsVqRflMlPsP
rCSJPCrBwwMdBJRCrwMTGWTWNbcjGZNGGZTb
qzgqfgFghsHzfgHgmmfWGNGGWcNNFClbWlcCNW
DqsnzsHLhddBDwCRJR
sZpHjZrVQmcrbhbthzhFHzhH
QDPMDMndqqQGqQfCDDbhFtzzLtbWzlBLLPBW
TDMDqfGnJSnfnfvgjsjNgJvsjVQv
bDZQbZHdQQggZfttJjGnplSnldsG
FTrFCvWBWzTNSPNvRBGJsGjpsJjstmCLtjCt
rhBvzRVRBBSVHDHcHMHq
FSbSNZbZbzGzGGbNzGgcZPwlDPvlNmLLLwLLlLvvvl
pqnqpVCrpshqmChsQnnRpRCldDHvDHLjDlvPwddlwPtwDV
rRTCnnpCWCChTrWsrBTfbcFFmFSSmfBBGg
QdhdWDsHhHWzPrLPSCPGvs
gpZZmNmtjZwpBZBZgSnvFcGPrrvmncnvmC
RVVRjZJfJVfVBZVtBNBVppZVDqMHhqTbMGlWHQhhWldRMHWd
BPWQrRRNNMhrHhLqqGgjDJjH
TzVmmpmtCNwscTzszcNzDCfCHJqDfGjDJJgfGCDq
wdwmspTsVdlTcpbmVMPWBbBWRPNnFnBWMr
VZTnVnsgrjjsqPzPwWgWPghz
mcFdQGPMBdMSBdWbhRzzWqwLwcWt
GQPBvpfvNvFPBvTnfjnZDHDDjsrr
CZssCNFJBmBNFmFBNwBFCJFTtthGrrSThtSgSRtSfRTGtRrg
LDpDbnjjDGpggGrvGg
PLMWnWQgbQWnWbnbjqDbszHwwzwmNsFZMBmwJFZF
hQSjFLhFLLMSSFgdWTMdGgNbNbWv
JmPlltJBJqmzpbrrwTwrvvGqww
HBzztRBRplzlmHmRmmsplRJZcfFcLFfHSLjSZTcfVVcLVH
nBSQMnVQqJBGnfVfDgCrjbVbtC
FNcPPHdTdhmBdHBvwlZjbClClfcZgjclgj
vFdHWhTLHvnBRRqBsWSR
TRsNNTTHRRZRRsRzJQddSpJLcQdpjs
DMVPVVGmMGWMGtMgGtDlmMWwLLLpJfSfpjzpdQddLSmSSJdc
tVMgGMDwMgMWDBWMttjRBZNnvNCNZrZZRbZNvZ
wlJPVMJPPBShSlhgfTvgNNzzgNMCTg
FLtRnDDSrvdNdrng
FcSmpFZFFmmjWqPWJbmhGqqm
hlBqqTlSfvNhpbfb
fRVsVDDRtnRVfbDNCCNCNQGwNZ
nrrRPPnHzntRrPsRVrtJVBMjlzWfFWdMjjWMqdBBlT
zDNcnRsNNfRFFNNzRzLbRWgMZMMZcdhcBdMrBpZmmZ
PVHHVlPDGPPtjDmmdrrGBBMpWGWd
VVqTTlQtDCqFNzsnbLbCSJ
ndSGSZZGwSZTBdwnwdwmWCzPQCQLffZzRgMZRggMzf
mvqVmqrmcDqllNNtbcNcMCQMRCMCCMQfHLgvRgMg
FhNNcrrVljFcqmTJhwnsmGdJsT
wlmLmZLwzvVmVWVmQWzZSFJFDSqFHSSFJHhDqZ
RsgpMNcMdRgjDcRFqCSrHSHBCFJr
jdncssDNPsbmmwvvlPLw
wQGHMrHGgwgVTQrrMGgGQrTtWzzPJhsfhZztWssQWbZCWh
FjvBFSqqDbljFvSbnvFltszfWPPfWzJZBCsPtJft
qLlDbpjFRbpdGTgLGLGTTV
hrVJsBrpwbsMZtTLlwnqtqdc
QDDmHWmffHCQWHjRQjCWczTTjtlzdldqVtTnTqLt
HmfGfRNWfNWmQCRsgbsMFMhMGvpBVs
CRzzVCZhvGQqNmcWrgpgwQFSmF
BJsttjDtjbdLMHHsBTqBbBHMrprDSrFnFnSgrnnrpDSmWWnw
TbMBMPPdLTbHTjHMtPzZGvzlvqCPGNlNVRVP
QcmcrCVcdTCGRRLT
zzgWFWVBTSWLPdMP
JbhnBVzzfVhgztVDvqcqHwncZHNqnsnccQ
JJVBFfJjNNNsJTwVfZJNffFRpRzRzRptRWtCtSSHWsWzCD
rmrnhgclhQGcGnhrPjqgGMHbWRHRbRCWbzRbMSSpHWCD
mhhjQGGjQgggqnmQnmghdQdJFZBNvZBBLNTvvTNNTLfZ
DrBgwMCMRvMrvDgPCzdpdNtzqqlHNNtp
jWSSZGgfGjcLfdNjFzqqFFzzFF
nGZhTmZLLZhGPVVTgQgMRrrb
ppqZvppdJmSLHdSfZRrrtbscgRVVgwVrHt
hFFFzQPhNWzNhnhGVggrcbwVgBnvbwgR
CNQWFMzWWhCflpjvZJMJdj
pfpfmQMWmcBVfMBBmpfVQMbDGGNPDTcSNTTsSNPCCNhC
ZZrZwvvzZrvZlZlwhwswhNSsgbDssC
ttvdtzRzFDqRJWLVLWJJpQ
CZZPTQPTPTJhTQTrHCBbvtLbbbRWtjbDvb
cGfsVSVcLdSgSwBWRNNGwRNRbD
spSffnccsgcdnnJJQlZZqJLhpMJh
TwGGdWwdddtTsbzPzbbnTLnPLP
gqNSMvtvcSDLLfnMnnPzFM
NvDNDqtvRcjQVGZZGZZhwpQB
jtgFmnqjqttQpsphzNllblzlNH
GRMRDMGCVCHzSCbSbNNl
LMTJRTGRLBJBwLRRHmBFQPvqmPBvtgtc
jDjjwRDpPqqsMsDLJbJzVB
lMNMNddvMltNfFVWbVVWJrrVLfgL
NQQtmtFGFlGZPZcMmmcjjn
CgCNjvSCgSQQzVZNWVnTBPTcsTVBnpPs
bFbbLfbfdRBFhLwqFmblBJfRHtWcttcttlDpspcPWDcDptPn
mdbFhfJrmJwfbmmFFFvBZCNCzMGrNjMQjCCZ
TPDNHHSTNNmRfTrRMZSqwwttdbBvBMth
VVnnFGgnQcBvMqvnhNBN
GLzjjzGscssJGJCHljmfmTWPTCDN
BZZNcMQjBNjNtDJgstjgtwqGRQfhGhSvPfThfqvPhfhf
CCndrnmnnWbrnHrFbWbpbbVmGGPqLfTGhvGSPhqRLRdfSGsf
bFssVbbblFHzrmFlMNMtcNgDtJDzZgtw
smjMtSqQQSjtSfmDVVFHFhnHBHmbNPPH
TgvCCJcZdwdgNvbHvPbbvBNq
JLqRWTgLqJLCJcclgCJdWjfsSSpsfRrsQjDtspptQQ
hNwztzgzJnnNTVFwNTNhwVhZlrpLMLZZlpZlQndLPLpQLZ
vRDvqSSqjbqSWDvjbvBdLWspPLddZPQQLMllLp
SfGfRmSGCSfBfjTcNFgzwMFJzwgm
lhVBhZjjPHbThwFGrNrdvNNwFV
DRrDLfMLSgpCdCJcfmcJCm
LQtnprtqSRtZjHzTthlb
GrGsqfbtsWGWWntnrrwWWWGSSDSMDcSSSwTDzPzJSJzPcT
lmQhhVCgmffCNgmNNmCmBNRRPvDzDMhJvSSDJzzcTzvvPvMT
VBllNBpfQgQmpLBpRBtnqWLFFnZZWWGrZrjq
NRJdngMVwfgnwJtvlblcWLlLDHfccDbW
PFJzBmhmjPFpJrFqLcQHLlHGDlHDQbGz
ShJZJmPFpwdMvCCZRd
WQDqSVWqpBCsPqPWWNscfrHfhrhrHhGFGs
MmLRmLTjmTzTzlhGHfprhvfFhHfT
mLZLRdgMRjtdddmdgwmtMwQSCPbnDSSCqBDwpWPQqn
rNHwMMGDrggWwsvWMPMWWwjbCqjCBlZqvfjBqCJhfffj
FbtFmRTpzBBZqCClpJ
ztbzFtnzVNnNNPPDGD
PLPFcwdLdFcbgdfSwFtWhGWGRMWMJMGCblJR
qTpszVVjRlCHtWCT
qzvrDqQrqznzggFZwFwQScdW
LWLjLNjNjTwlwLZVcBVcVVZcBVQcZZ
JhGhFdmBRdGGDnQtbPvVVdnccS
zDrrFGFFRgRHmDNWTpjTBNTHWNjW
fwfBVLhmwfhHsgBstWCWQnDQnlldWW
hZvFTNJrZjZbFvNvttqWWDtcWqCtFDWn
rbjjrjpRzRzgBLzwLgmzLh
ZqqqWVzdSPnwBJBfwJfZTs
FHGgjRLMJFsJTsBw
DHRcDgHvLhDWPSCzwqnq
LZGZLLRLZpRQBtPTjTffrHljjmsB
wNVVwcCgNCCScwggmjHjTPmQPsTHmlSs
gbbwbqhNCQcbqqVchWhtRZDJWJDtZLWL
CmTmvvmvzCCCgzzVQmTQvTjjGRGShwSHwRrRSSSSDNHSFN
PqZqWdqlplsqBJMMsMMnGRJRbbNwNhrrhShGShFD
fBWBWdZppqpqDMBdlfcTCTLtLtLCQfQvcmgv
pntdtdHHWHqnptGpqHqNgMQwPPPnZMZZZZcfgc
LFmLSVBRTSBBRrffTQgMfQMtJZQT
bmRCSSSjRCtSrRChjqqGqpppGhqDGp
dGGhhfNfgRTGLcpL
BmCCwQMQqmQrBCBJLpbVTFbHcgcbLTMc
JrpqJJmqqqqmzqqwmwNlzfvltDPltfshlhNN
VCCbMJfJlgRCnNGVNnvFvVBF
STsgcZdghZsqSttBnsGnBtBtHt
qDcjgDphjhSghZTQgCJWQWWfwfRzWlwJzJ
JHMVMvmvRcdbmrRHQBBGjcjfFQfChSfj
NltNtZllgZtgtnpnqNWpgCrCBQzBGzFhQrGSSBCzWh
pDrwnqLlvDVmPbss
dbrpbSrwBjswsSjCwqllLqFtqLcrGqqFtF
RvfJDQnRpHvvQfRvvQRJFDqzcWltFFlzcLttWltW
ZpnRVZHmvHnTnPZZPHfHmVwdjVghwgVSBgdBBCwgdC
WRCBGWvNgHnMcFwnpC
ltlstrjlJNlfrZZqDJtNLsHnmwwpcHphhFPMFjwhmnFp
TStJssLstJLtqTsNgvvSBNzzvWvGRz
VBjdWdGcqWdBVCFRmHwfCRRV
DLzNpqbDzDNbrJvltMLJLRRmtRFTSRmTmFwfRHRTFf
zvvJNLgNqGcnjgnP
JjdnFfbdbdQMbQzjtRcwcCvbvBqRBCwt
LlNHlWGprPCVVBsVzqNR
hLmgTlrpPPHrLprHrTTGggHWzhZFSJDfhMdnjjZfFfdFMjFz
bDbwRpCSRgqqMfMf
HzzPcPnhzlhsQzHhHnTggBBqTQTgVQqBqjZW
tnsrFccnzsDvGpNGqNtq
GmPsPrsSlswNmcLzMvnpnmMpLBCf
glDTZRDqRTjRCvjvfBpfCzvp
DHlJVhJRDTbqZDqSNVrNwtVrQwSSGs
nNnDwqDwFVgDwDnCgLnLpCVWdBMRpsPdMPPjRHRHHRdBWj
tQtfTtJtJmlTQrTtTlhfzrmdHzMMRMsBPPddjddBPPdWsB
bbhtQTfTTsmmbStnqGFGNDbFDgFVnw
dsVpDPBMHVdHpplpvdHjRjmmjRTMTFFrrTTFQq
LzzWZLGCzCWNjfmRfBhmQjZq
zSSSwJwSBzNtzLBbwbSGLzWVvcvpHdssDllVJgVHVcdDPv
RWfQBDTBLQWpDLNRZjZwHHddjHNhZdtv
ScCCzSszFzJccPHHvmjHvjhpmHsj
FPclgFVCbcngVgnpWQqqRfLBDBrR
cRLLVwcsctwmbVcszztwtRMvNrCpTggqFrTvvhCVpghBqh
PdSDGdnZQfGDfDjWjWWgvCqFhpqvpNZgCTTvrp
dGnDHWnSQdJPDSFLLcJmRzzLLLRRcl
lCSqlcCcBqBCCwGwnNWnnFwBHF
WMZLMPbPhQddRbMpbbLbRLLHDFgjFGDmFNZgNnDGNHGGjD
dTVPPQbPbMdQMzvVrWvczrCJqv
vzscdHcHZzHzCCHlQTTTCcslMGPStmSlpDDSSSgSPDNBmNtl
FWVMFhFMMqWhFVFbDBDDhpmpGtPSDpGG
fRLbFfwWWLnVjMdzzQHQJnnvQs
SmPdRbWZdSqqzSPmbdWFFQgcQnvncgQGQMMT
BfBLmVNjprVVNlVBrpBlHpNrgQFHGCGgvTQTMGFFgMCvgQcQ
BjjJfVLBfNffJbZDqtDsdzzm
NLgtLsSggjqgqpLLDjsjmcJfvpmFmmJmvPpwhBJB
lMnlZMtdCMrRRnRbTddWbVwcmPfFmhJwPfwJmvfwFvPl
MnRrnGWRbgQqtNGDjt
dSdrTbTtLJCcttcFVw
PhsgQQGPZshvpQZGgsrBllVFlHVpFllJJrFH
gqsGPgMZhgvQbzrzTfSzMTLf
pqbDdQWqCgBfbbfFfB
vtjnmzLcmhBdzTFgTsRP
LZGmjvJGGctnLtvcchSjmhcLqNHCwVdQZwDwWDNpCwqHdDwQ
wlMWSSHWShSMbDSwVhCrNjJmcrDmGRRCGCjN
FHZdHftFFQnqsQqsQttjvGrJccmdGGcrNdRNmG
pHpzPpQHpsPzPlzlbSgSSMLwzh
fCQDLlDQTSjbHDqH
ZhrsrZZZhcclwNswGGwbwF
rcWhlhlpMJpMZmgtBCzCttCCRfdp
zLnCMLNTvtGNpNvNjhRHgZhHvZdZHdjD
fSsWWqScTfJJqfJFFJwswhdHhhhdhDdjbjZbhhDj
WsWmfcqBWfTfsrntrLmplCLttm
ljssbqMMPbHPlsbcWZNLLsWJWRFvvZfW
SzgggDDwTzrQmDQgdSSWvdJLFGffRvZG
zCzCDCrznnTTmCbbpvlPHtCPtb
TZSwNPpcgpNPbwbhhbwrwJqh
BlCDtvvgLWGCLffGfLzLrMqnnbDDHbmnnnJrhnVJ
lzBjdCjCGCjfGjjLGBGGjlCSsRppcdpRNdRSPQcRPQZTgT
TsFTrvGmZGfvZfZFzNNZrhClmRcBgCMwQwQPCPMPRP
bpnnVVJtSDgRBwbQRwlR
jpSnqLpqDJDJLDjWDWLWvvzfZZvqvNsGTHGGFfZl
bzbzznqfCpzvhCSMfbCbpCFhtHGHHJdtHJGhFsmshJJG
DLWRLjRrmNPQjZZlQPsFGFggVcWcFddggdsg
rjrZPwwDRlLLBjQlRRlPDpmbqzpqnnCSCfTMwMqSvC
FmcGcjLRPjQwQjMQrwHQ
btJzJbVNdBJJtzTdGBbdBztGrQhhQWhMwHrhrHSHgHQfhMVS
JJDpdDTtCtzNptnTJBznnvLCCvcFqsRqFcvZclLGRR";

#[cfg(test)]
mod tests {
    use super::*;

    fn split_in_compartments(rucksack: &str) -> Vec<String> {
        rucksack.chars()
                 .collect::<Vec<char>>()
                 .chunks(rucksack.len() / 2)
                 .map(|c| c.iter().collect::<String>())
                 .collect::<Vec<String>>()
    }

    fn find_item(compartments: Vec<String>) -> char {
        for c in compartments[0].chars() {
            if compartments[1].contains(c) {
                return c;
            }
        }
        panic!("no item found")
    }

    fn find_badge(rucksacks: &[&str]) -> char {
        for c in rucksacks[0].chars() {
            if rucksacks[1].contains(c) && rucksacks[2].contains(c) {
                return c;
            }
        }
        panic!("no item found")
    }

    fn priority(item: char) -> usize {
        ('a'..='z').chain('A'..='Z').position(|c| c == item).unwrap() + 1
        // " abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".find(item).unwrap()
        // String::from_utf8(
        //     (b'a'..=b'z').chain(b'A'..=b'Z').collect()
        // ).unwrap().find(item).unwrap() + 1
        // let code = item as u16;
        // if code > 96 { code - 96 } else { code - 64 + 26 }
    }

    fn solve_p1(rucksacks: &str) -> usize {
        rucksacks.lines()
                 .map(|rucksack| priority(find_item(split_in_compartments(rucksack))))
                 .sum()
    }

    fn solve_p2(rucksacks: &str) -> usize {
        rucksacks.lines()
                 .collect::<Vec<&str>>()
                 .chunks(3)
                 .map(|rucksacks| priority(find_badge(rucksacks)))
                 .sum()
    }

    #[test]
    fn it_split_in_compartments() {
        assert_eq!(split_in_compartments("vJrwpWtwJgWrhcsFMMfFFhFp")[0], "vJrwpWtwJgWr");
    }

    #[test]
    fn it_find_item() {
        assert_eq!(find_item(vec![String::from("vJrwpWtwJgWr"), String::from("hcsFMMfFFhFp")]), 'p');
    }

    #[test]
    fn it_priority() {
        assert_eq!(priority('p'), 16);
        assert_eq!(priority('L'), 38);
    }

    #[test]
    fn it_solve_p1() {
        assert_eq!(solve_p1(EXAMPLE), 157);
        assert_eq!(solve_p1(INPUT), 8072);
    }

    #[test]
    fn it_find_badge() {
        assert_eq!(find_badge(&[
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "JqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg"
        ]), 'r');
    }

    #[test]
    fn it_solve_p2() {
        assert_eq!(solve_p2(EXAMPLE), 70);
        assert_eq!(solve_p2(INPUT), 2567);
    }
}
