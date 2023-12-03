use std::collections::HashSet;

use itertools::Itertools;

fn priority(item: u8) -> u32 {
    if item >= b'a' {
        u32::from(item - b'a' + 1)
    } else {
        u32::from(item - b'A' + 27)
    }
}

pub fn star1(input: &str) -> String {
    input
        .lines()
        .map(|l| {
            let line_bytes = l.trim().as_bytes();
            let (a, b) = line_bytes.split_at(line_bytes.len() / 2);

            let a_set: HashSet<_> = a.iter().copied().collect();
            priority(*b.iter().find(|&c| a_set.contains(c)).unwrap())
        })
        .sum::<u32>()
        .to_string()
}

fn line_to_set(line: &str) -> HashSet<u8> {
    line.trim().as_bytes().iter().copied().collect()
}

pub fn star2(input: &str) -> String {
    input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|mut chunk| {
            let mut set: HashSet<_> = line_to_set(chunk.next().unwrap());

            for line in chunk {
                set = line_to_set(line).intersection(&set).copied().collect();
            }

            priority(set.into_iter().exactly_one().unwrap())
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    star_test!(example1a, star1, IN1, "157");
    star_test!(me1, star1, ME, "7980");

    star_test!(example1b, star2, IN1, "70");
    star_test!(me2, star2, ME, "2881");

    const IN1: &str = indoc! {"
        vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw
    "};

    const ME: &str = indoc! {"
        GbccTtTSGGbgrcWBGGrdgTnVQnCmNpCJlNnNPVfClcnN
        vMzvZhzhwDLVmQnClwwNQp
        FRsZFzjQFsqRzRRjDZbdtTgdHBBWGrdBdHHs
        HCLTmbCLgzNBNPSSlT
        JJGMWRJMrrdwWWGjGWMLRGLjBzNQsBzPPfflzDPBsBffDrQz
        pwJdLMjdMddWjLtwZWMMwGtHhnvnCnhvqVFFZnvbgbqVCZ
        tvMCDCSVVvDDBQFRbqWMMsWgFWgc
        BLLPTpBmfLPrHLLfLsbhRqbzRRcRHgqssR
        dfdNLmPTdNZmZdZPfpmTJLPPSvQjtSGVwQSDJSjSwDQBVCGw
        wZWTWNFqzwZbWNpSgGMVMtTHsgGs
        nlnPnPvLQjzdtsjBHBMMGSHg
        LdnrrLnhRdLLmLDRPvmdQnJDJWNqcCqZJZqfFqfcfzcq
        vPTbfWggzvGVqjsVqV
        dDcJHZcZHmMFQQMshsjcRqVChjNtqh
        dDHJDmFnrJmQFnBdMdQHJdZZlWTTPPjTLWbTzLWlTTfwjzBT
        VfmDHDfZzfLcZLLLHBFQtRJTcdjgdTgFjjtR
        WRPhMlGSshPRGgvFMtddTjjCQt
        SPhWPsbNWShsWllswGpzLmzfZwmZfqLVRrDBZB
        MtZgRgJJbbGjgDDgbjRjRbGcNdWwncBFdLBBMhFBQwnWnw
        vlpsNVsCzzfHpvTpzlSSSvppcQdfnwnWhndWndwfQLhnhcFQ
        pCsCCTHVvSzvPvHvzpPVTVHHRJJDgJZjJjqNmjZmDtJRZPNR
        GTGTbhhPjJJjBhhZsGzmfHSNsmHHgSdL
        FcFpMDFDRFfsRHSjmLjR
        CpFjqcCjwjnpwhTPTWBQZZTb
        zdzzwDlnTDQQQQnqQqqsFqnrSBSprbpjNJJBJPPdpJfbZb
        hgMcRVGMtHgRcNSPrpfgfjJpBp
        CLRHVHhtvtvGPWFFDqPDLnqP
        jssjjjHCSGCMNJTWWCJT
        DvcvBtVrrDSNWcMfLRfM
        ppDZSpBhBvBmvDHnFsHHPnGPGbQp
        VVVLsLWnjVVBGgScjtBjjLLgmJdPmJmfmhmGmmmNmJGhPNJP
        QbqlZqQTZvMHmshvFhPfffFp
        CsTRrQrwbCbrZqQTQlRjtDBSBtwBjgVWLBtgBt
        FLsSFRTPscHZmGRGGc
        npNNptgttCNpgLbnQMgnQnMNqVhGqZrmrmmqrmcrqmCVZwqH
        WMWbtpjLgnLNvSfPPzjvjPdv
        FcFFhZlhlMrHlSFSrHZMJZSVmmLmVLLCsBtLBCzCBVDRcV
        PMGPbndvGfGstLzDCmLB
        NwjPqdvPpvgddqgwHrZhJlTlThpWJMZp
        SdjStScTWTwwvwwfjRhQPQQQDlLBGpLrPrLrLc
        gCqJbNsVsNMgzMJnnqzNlCLGGlZZPrLLlLLtpPDl
        JsFJFMmbJqqnJbhSHdjWwjtmHWvS
        zBFDGGbNzDWRbDccsWslHlWWsJcS
        zMVqTwzPfVfVMwmlcZTZZlSmTlmc
        MrMvMCnrMVMCPrPnDFFGgQdFgRdzznbd
        rJtJnrnSShJgcCsjjNNMSSDzRmzm
        HWDWPBPDBfFVBffqplvlmNlQllvzQNQqlN
        FVGHHFVbwBpBPwFFGfBpHVDgrcJCbZcLdCgtcCcJLJrd
        MGHGGFFqbFTGmFwLmQsQflFN
        WcvBdpjhdZdNwdZwLZ
        vWtgVcpvjthtNcjntDhhpSJMSqHzqTzqCVHTSqHPGT
        cVHZfjfZMcrSDQMJRCBCQw
        PtGddtslsWQDBdwCDDdw
        WsvTFnshPTGhGhhlPNGTCnsjgFVfmgfHZfVHgcHZVVFmFV
        JFFqfJBgrHBffVHlsBFqfWNgjTtztNnttWWvWNwzwt
        hGZbcBcZZSQmZLQRTbvNttTzjtTvbpzv
        cmmhRchPZhZSSmdmGPDDdJdBVMffHlqlslfF
        qWwTNwNHMHNNMRqMdRMQQMHLmmvzrTmrzPvzJvZvZlvzjZ
        FphBpnBhVBSFvLljzZPpmrPL
        nGsBbssbcbdlwggdNl
        RLSRTLSFFFLPSWpzzTJdzsQpbd
        DvqqcwVMDDcfrrnwDcwnvCdpQQphJhJjhdhpzsJhMQ
        fGcvDZffcGGZDHGrGrtJRPlPmJSlPLRgNBHg
        QlFFmGQFDQrrWlRlWGrnQVCLNvvPwLCwBvCcCcJCLCCm
        tHtfsjSMCNPwzvCf
        MjqStqMHsMSgjShjTttgphsTlrbDWGDrGlRTlNDbQrQRWRbD
        QbChcCJCbHQCjbGCjQfsdsrtTqrfTLrcFftd
        DwRzVzzZnzZRwvgRhRWtqsLLWtRdqLLdqd
        NMwMzBVVPPSGQhBl
        ttTPHWdrJjCdjnFMtLLtLNvQltLh
        pSDBwZRBBsgfDGGsGpBVMFPQQlFMFQQFQfNvLNfF
        PsppZzBVzwgDwBwwgpSSBssWjqrdCnjjHdmCTHznWCJJWn
        WcdHdPcdZrLPDPBQDg
        pMjMMqfmJlqNflMlFNRfLBwnLzTTTDJJwDTTGTLJ
        hlhbqpbNNbVVdbZtSB
        RDBWGRDnzBWBJDNBttSLlclldtQQcTTLFF
        rTPVjZZsCZrVhdFMcgLgwFSgQh
        PsjHVffbsTCHrCvTPfDJGJHNzzNJWnnnmzDB
        LQdFgTLdQjVsQFTRBjMZrmBjWGMGSW
        flvJJlJpbNnppCpMGGfBBZSZRfFmGr
        NNbNDNlbDpHlbDDplvzvnCbzqsQPFQsTTccsqdQqLgdHLwQw
        DnGDNDTFdFwDzCZZRmhThCRRRv
        SgrPLrrLsBPbHBCmtVZVCdCcctHH
        rSbgBrsqgsPppMBqfpPsLpPGlNGGDwNFNJWDldlllDwJMG
        PWbvNWvpvJPnWDGqDjDczj
        QwfFFVVQSMlDlQfFZhsHrBrhHHTcjnczqjzqrG
        mwMSgfmDmSSFgfFNbmLpbRbJbvbRpC
        lsggLLLDGldGTGBBhNTCwRwVnJnNCCnbRV
        QQpWrpHtrHrpNRRJNtfbJCVR
        PvQQFPzccvBglclNscls
        NsszMMNGWLcWBhMF
        gTtwvbqfnDTdpvqDftpnnntDZvLFQFBLmRWFRhJZJhLLBRQB
        DwrpDbngprPWGllSNrSS
        nCqdLPZPMMZLNvtGhRmGhGPmtW
        TSrVZVSZVwFTgSVtrtchvWRRrtWtcr
        gjbjBjgTjfgfVfHHppBLMnqLMDnqClsZJLLD
        hrqShCPCpHHBVBGWQFVQGFGnzQDf
        tgvZsbwsbcMbRsgccjDGFvGFfWJLLzFFQJ
        TTZmMcgmbmWZMctbbtsHrrqqSHHrCrPBBSCPrT
        HHHNZLGLpBpRSvWlGlqhPghqDGnnFr
        QCNCMTJdjMjdjsQTbdQmmCQDngFqnggPFcPcnPFcDqcbDn
        MNJfzNsfJdJjdzwMNjjTJttSHVStRtZVwHvWRWtZHt
        DSbvDdDbbwHgCSgZPwpbPgmTTJhsTTChqTJssQssFmJJ
        zzjMNNGMMRcNNhvnvFqmtJJv
        WWffvlVrcGzGlcjLvfrVRLHgHgpDPSbPpwwHbWbBbPPH
        FCCjjFlFtCjzlpTHtJsQTTcpTT
        DWLhWSgDWWdSWLwmmpHHQTHcBTBvvwHvHl
        mgGRhrLLgWqnjrfCNlzP
        cLsslBlsqNNTHlTVNbLZZLRCQbZZdQdpbP
        JGfJhhwfwBBSJPRdZddpZRQbfR
        hWmWGgDhJrFhBcWsssWHvHll
        lmmvlJFtMHFtQzVSRbPGzLJRgG
        BcTcrNBrrwwqDBqNqwcrhLpLPVzRhPRPPPgSGVPLbS
        TrDqcnsTcsmnvHtdGtMW
        CcnDQpSDcnFcPBrmbPQGBsGB
        gCtCfRZTBWbjPRbr
        qgvHqgJhMfZTtvHgfTghJgMJDpdppFSLLCcSDvLLcdDwvLcw
        ffFgGRMWSTGcnDgllDDpDp
        dvSdHBrVSLNVLjdlsllcsDqpsZ
        SHHHNrLJLvtNQJVvmMfGRGGRCJWJRwzWMh
        JNpNDfDBDHVzwHHzpzBWVBPsvsFNCbmbqsFFNsjCmvsmNC
        rnnrtLhnrrQZMvtFbWmqtllcFb
        GQRdGQLLhMSQhZLZdgdwwHzPDzSVWzVDwJDVpz
        LdcGjgdcrMDSFGVfnnGG
        HNsCCQFCPvFFBJnnSBJVfDVJwf
        HHFRqHPpNppmQPcpLjzrdgtbgztT
        GlZZbclGZsDvlGhsShRnCnMQtjtQjnCQsQRM
        PggFVcdFNFNNVVFLPdPdrwpWBMMnqMpnttJMnjMnQqtqQtqq
        FdgcdcLwfTmSGTmhlbzG
        RGvhGrLhhRhlpChZrGSprBdPPHJJSBgSSHqBWBBffH
        mQmjmwtTMTVLzHnTPWffPHHJBf
        jMmmwMcVcFLFrlRshZbCrF
        SnNgNgBlNZSZdZtMrlnSnnQtjpwFwpvFJwFqpwSbqjjqGRpv
        CLCcWHLhLTzsDPcCWMLGpFJbGFwsbvGwJwjpRv
        zMCTPhmHWzfhQQmndNllNrQg
        dbdBdZrQsrdrGslrrSpLvwHmlTmmwScTHv
        FgnJqLDLWqNnNpppmpCSSmCJTw
        NNhhnRNfzMhgnMDFfGdGLbBVVdQRtPVZGt
        BBQJNTTzTcfRhtjhffqDDWCC
        vZnsLsVLSvPwPFFnwPlSPgZWqGjChgWCCWWCMCgGMh
        srLLnLmlPwrrPwmwwvlRqzQRJBmppQTTQpTdBN
        PLDpZGpWbNGWLDfQmsQDwwsmhm
        vTzMMbgCfgHQsmQt
        VMlRznlzVnTcFzbMcrpJcNrJdjdpZrLdcZ
        SftvFcDSvDHsFtctMSvbdjbpqpRRpRTJrMdrrb
        QzQZWZnQgQZwBBwsJdqPjdjrnTpJjs
        NGmwmgszhZwwGGgZGmggWLVSVHlNVVtDcDltFVVVlVHt
        WCfFBfBHHjHHjgHBjJFVcVRwQMbVrRhrJbRRJM
        sZMsDqzZPRrRrVswdc
        vvTzDzpDTvpDvZPvSnNZZlSHMCHjjFtWmlttlCjmCF
        hJZwhrvhBJRrPQPwRRZLllgLqfcqpTggpcTWMTff
        HHDzMztbVgTzNpgf
        nGbmtjDMFjDjCHbbbHHHdBQQPBrZvJQRwvwRPZQJGs
        NNSrMSHRqWpWNNrNMvLffTBBDmsvcmcJLM
        lwPPhcddcGPlBDTDlmDvJJsv
        bZhzCdPGGFzVVPwVwbNtcqHrpnpZptSZqRrt
        GvvSWhmhWBNcBDNc
        FzlRRTljjRTjRRmZfbflRTlFFrrMrcBcDVqBVsNDDJsMFr
        bttRfzfRHzjlmlnCbTtzbRShgwHGGvppLdpvwLLGLLhd
        MHGMWdBFFNsFFHpWSFddMmqVmVBggmlVfbVffjgZml
        hcJsTTscvsLDzDJmqVgfqbqnbmfJbJ
        PvRTzsPwLcwCprSdwdNW
        qfJnJdLpJzrcqCrCzcGfpRSSVBPRSjSSllTNRBdTRS
        DbsbtggsbbsghhgvnWWSlVjPSjmmPBtjPNlTmS
        vHHHHDHvZHQvWbWsZDgWhDwWzpfGfzfcpFJzczwFJrfffnGC
        sQvsRQsFZvfpGhjhQqjpZvjGJHgngPBNHnCBJBCmSBmBNG
        HTHwbtdTDDnCTPTT
        zwMlVdzbzLzMWvQZRQZfZZlHsR
        QhzWwRBPHgFrWWrH
        SDgJCCDCsVpMMqTtFpfpqG
        gJNCCddSZNSlljQzPPNBzR
        dLzVVjfLGCCdRPrdmBtwWttr
        NnbNsbTHJnbHbSHlNQsNtwrJRwBMMBhrPJWZRRtM
        QSslpFvpSSsQPFCDqqgzcjCj
        fcpGshsfNcNZsmRjNqCtnFgbCgHrrggmrn
        QvzBlBBQBdJTBzBwVVMgbrwwLFtLtgLFHCHrbF
        dlQQMBSSTPZfPcfssZNC
        gNGVMzVpVVTdPDWdRdNT
        BfjbnCBjBzffHrbrzBDddQWTZZQTTJTQTHHS
        zrFncfBjcjnrrlCLwFgpmvFmwGmVLh
        MbngccTfWgbWcTTzZghmLshhLRttpthRDLtf
        CdFdJHCJjBvBSCNCNJBjjdjpsPDwDtwvptRPmLmzRwhhLR
        qJSCCBFHQBFFldrVZZbggnGTzcZQ
        lPrpppllcwwpHprppNdfLbQJnWdLJnncdN
        tSjjjSSDGgghRbbSTfTbTFTLQn
        jCBgDMbBMGghZzCZmmlrrpwp
        FhCDFvvPwCjcLhDjhnvjnsdfZTlflQlflLsppdQfld
        zPNSmmHrSSHWBNSMMVGzfGfZTZQZzdpdRGZR
        WSNVVMMVtHSVbMNWBHqmwcvhcgwgvwtPvgtPDjtw
        jSSSjzZMmgSzzwmZBtHcHmtNdncHtnpNcn
        VsLsRsJJsTfRVfLRLJlfLlWqNbDcddncvpvbdvcnpqdpdtHq
        GQQTsJGGJLlRGJFWffWLhgZwrZBZFZrhtBjrjjZw
        ZfzJPvPnLvRJRfZLDfjfrBcqrgsgDBrcrGgslsms
        SNhpqSNhpVTNQSMNgWmrlccVGBBmwrBw
        QHHFhhNdTNHHfZPFqtqPRtjq
        PMZSPSZZGMspsLhLRqRVzfGjvF
        tcwwgcgbcbCrtbbtmQQcCqRffFLhRgqjFjRfhFqhqz
        CLQtcbcmwmbdrbBrlrCwQTsTPsPsZNBPNWJpZWpTss
        lMTrcHrhChWnRzJrznnr
        DDJbPwjLJpfBQjPVBpbsGVGVWnZnsqnZsnzsqZ
        wJQLbpPJDLfgPbDNCHNlghNCMhcNcl
        tlVZhlVWtnBltVtssZBBbPbcpdPwbPWfvcbLvbbb
        NwCTFNFDNdSNPpLpfN
        CGwRjwDjzFFGRGjjFRjlBtZqMzVVtVqhMZMBZn
        HhFdMFHhgrdjcZtZjr
        zvvQQvzwzDMjZTjtcrTDtt
        BwMwSvQSVlzQlMQzwzNgGHPGGHFCCCgGhsHLCS
        zMVtBhhVhhDhtzBtMTTfDrPbmRRmPbQmrQbNQGRQtR
        vLlJHgnLpDvHHvHvmPbSQbQRGmJmPRrb
        pspwHClCwqplsHqDsMMTFWsWfjzszf
        CmmjLwWSWGCHCjwSmStJBgQcccBhwgQtgthQ
        WZVFTpqWsMsZpFddzszbVzJQBnRtrQthchdBgtgRtdrc
        bbsMTWsMVsZqNZMpqWDqbMsCjvlCfjGCPlLLPLCmSCfCLN
        VzsjjVGhpjJrJHCppprt
        WtMnqtWdSQDtMRSnLNHHwHwQvrJrJCPN
        tTtWSScTddBqdRMmlsbFBfhVBhfjjF
        gLMWzdTgLFQHdlMgMRwcwhqqvPcPhVFRDF
        tBnGrSCZNZCrtGBsSNGtBPhcgfchqqDPwVPRvNRqwN
        CmrgstjZngtBzbjJlQWWHjjM
        qttwGWHtVPzJJPqbmb
        NrRvfTTghNrpLrrpLTrNrRrhvJmzmzlbbVVbdbdZlDdvzMPb
        rfcprNcfgpLrVNnnCcnnscstFGCF
        ZZhTfggZsbshGrfshMrNMCSRMMWqCqMNRq
        TVTJPDTFccqMCcJw
        BBLBmLTLDHFvsQpfgnZhbQvG
        ffSrFvVVmVCQSfVDFzDvDDmmnGWCRqGRWNNqlttnRsNtGnWW
        PgZQgPJJpTpTHRGtNRGWqZMWWR
        wdTdwgbPJTJgTgLSFBbrQvSrFrVS
        ppssshsscCVCHhVWVpznnQRBnZnBbzczFPRS
        dqqfJGWttfWGlwwPSbFbZnRFPFtFZS
        wGwdGdddLfGgMTJfwLMlJMpTCChjHhjTHpjjhmsDHmHW
        PZQBhRPQBQrWHFHqHFHCqh
        STQSvvvppzSVHJJFWjHC
        TTgTvbsbszcNnnvbncvRGPBRtRgQrDPLfftPPR
        dMltttpQhmQVZdmhsdrvNCHvlWbHWvHCWrlr
        PzzLTGpGPDzFBzqFGFqFvHWHvRJbbrbWvCvjJCLv
        BpqTBzpzfGGTTPZtSddtQmVffSst
        bwHbRZldhQQfDWWGDjBf
        CzvgpsNMsvCvFvpszpnMsFgBTDBDWPnPVJJZZDJnfjDTff
        CpcLsFrMZbhRcdmt
        HgjpWlhzpWjhWTQPFdPBRQzTMQ
        JsfwrqLttwJVLGhRQGGPBd
        CttfrqDmDDtCsbZCHjhZHSHNlgcW
        QSdCWlCRhWRdlrlZrDssZsGDbv
        pjPrpjqFNrZNGnBbsNDG
        wjjVHjfLQRCgdLrC
        PjMpRdBdjMSGsjpdprqtwCrNGrrNlthhrG
        WQzDzLZDgzZcqlqqrtJclJnh
        zbWHQHDfDWZHfLZHfffWVZpRSPpdVvBSPMqVMPjdvspS
        TMBJLTJlFHBjFFtMGngpvvpgvQmtNSNngv
        bVhsZswRCbbVZWVfVZwVSpmSQPPvNHwPHmgmSSNN
        dCCVZZcbWVVcCbbfsLrdjFMJdDDHBTMrjr
        vNWcTWnCqNCPPjhhHsQrfgszrTJRQsfRQD
        wLdwMBLFBBQJpszJBqzB
        lLwVmMSmttVMlSNqcbcbSbNcvHbh
        PVfJfDWrPVPPLcPPFWcjPrqlqqQsljRpplqBQpRvSQvs
        NdggMTCChMgdChNmdtTbtmsQSRhQslhlpFRpFwllwQvw
        CdnGzbGbgMGMdTCZZDDJcZFDDnWrPH
        fsshhnfLZSvcVbdcZVJj
        RCCSmDFFpRqHQDgWvbGjgjDdbG
        pFmFtCSBCSMBBLwrPsBPNlNB
        fWWcwbbwbWfGCPgPfvbwgvgcQQqQLsGLJQTZHHrZRsrLqlJs
        VnszmsDBpMFpzNFlrlRLRRHZqRHr
        VpdzDMNzNDjpsdzdnzDcPCwtWCjhbthvfgtgwP
        SPQtSWDLLltQQctHLSBSWHlWgFwhMRsgwggrFJPgdgwwGJhJ
        nCqmfVqfVjTznCMhsGRRRgGFMffw
        mCnNTVzVvjmqNtlDtbttDlBM
        LjctjtppFWmgthgs
        nBrNvzTqlDJlbbZgvmhMZVZb
        JJrnTrrgGDqDPwwSPHPpfjRdPc
        sTQmCmmVqmJHSTjGdMMfMNNvNHvc
        rFbzlLLWWPzwlWrlbwzrWbRvdfFjdjpvjfFNNMccphCvhN
        rrWzrwzPBBBCZTJgZg
        gffvjftWddzZtbvdNvgZLwBBMJLSWMDMDDBRWRmS
        PqPqpqcCnCpVqlClTQQmPMDwPRJJBLLLhS
        VCGqlHmmHsjtHNsZ
        mmMlVllWmhmmBzzLGMWlBmpstptPRRZpPMFJSpRsFRFs
        DgjnndQcNTCCCDNcdSRSdtZPfwfwJSJJ
        gQjCQqQjHNnjDCgHNcZGZLhHzrLVLGzlrGmb
        JfwfJpBgJSMphZqtqDDG
        QcQrssrGCcMCVcMc
        RnljPRnPjWbGRbjnjbvmSzwHfHgwfJHzdLFSwBFW
        sBjbHCBCnjvsJCHBsbvwwJGfRNFFFfFGTcrVFffNHRTP
        zDqdpqMgMtgzthgDtQmzGPTVSTVrVGTFSVFFqNRF
        zDLdphmmLhMhDhQdlzgLLbjCnWswWWlrZJBswCJJZl
        FMNrQFgrVwmrpJMwMTMPflbsHPTtlSbftSjCbC
        zGnGnhnGzDqRLnZLHNHbbbHDlltNSjCl
        nzddcRzzBnRRvRRhvnQFpmpgggJVcVmMQmgN
        CCpMlhwwpJpdBlsdcjvtZDFrtmRqmDrsmv
        gzVPbjSPfSPTTTPnWVSbbvDQZZDZrFWDvFDvmvQQmZ
        LTbLTPgnTzLVPNNGnNTgVNPlGdHHCphMwHMwjMphlpjccl
        rMMrqcrmJqJqmCsTPWWGGPzPlPPrGL
        fnwqwwZwRnVlWWnzWBWlDP
        jvVHvqfpJmhtHJtH
        NLMVQjRNTJCTJtZTJc
        DlGlGHvFHGDgcFCtfhCJFtZc
        DBHGGGSDvGDPHWBGdBbSvgWDNVMjLLRnmNmjPLNPNcRQVnjj
        tsGdTJdJtNllzjGRzm
        HZvvDLLWqbBBMRMRNjVhHRmn
        vZDCvqqgBDZZjbZDrWqBvpdpFpcdpCJcPTSJJtptpP
    "};
}
