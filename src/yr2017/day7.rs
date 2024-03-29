use std::collections::HashMap;

/// Structure representing an tower and its children
#[derive(Debug)]
struct Tower {
    name: String,
    weight: i32,
    children: Vec<Tower>,
}

/// Type used for towers without their children resolved
#[derive(Debug)]
struct UnresolvedTower {
    name: String,
    weight: i32,
    children: Vec<String>,
}

/// Parses a single unresolved tower line
fn parse_unresolved_tower(line: &str) -> UnresolvedTower {
    // Split up the different parts of the string
    let head_tail: Vec<&str> = line.split("->").collect();
    assert!(head_tail.len() == 1 || head_tail.len() == 2);

    let name_weight: Vec<&str> = head_tail[0].split('(').collect();
    assert_eq!(name_weight.len(), 2);

    let name = name_weight[0].trim();
    let weight = name_weight[1].trim_end().trim_end_matches(')').trim();
    let children: Vec<&str> = match head_tail.get(1) {
        Some(value) => value.split(',').map(|s| s.trim()).collect(),
        None => vec![],
    };

    // Return generated structure
    UnresolvedTower {
        name: String::from(name),
        weight: weight.parse().unwrap(),
        children: children.iter().map(|s| s.to_string()).collect(),
    }
}

/// Parses a list of towers into a hashmap containing unresolved towers
fn parse_tower_list(input: &str) -> HashMap<String, UnresolvedTower> {
    let mut map = HashMap::new();

    for line in input.lines() {
        let tower = parse_unresolved_tower(line);
        map.insert(tower.name.clone(), tower);
    }

    map
}

/// Resolved a map of unresolved towers into a single tower tree
///  Panics if cycles or multiple separate trees are found
fn resolve_towers(mut input: HashMap<String, UnresolvedTower>) -> Tower {
    // A tower is resolved when all its children are now Towers
    //  Only towers without parents are in this map
    let mut resolved: HashMap<String, Tower> = HashMap::new();

    // We repeatedly scan the input map for resolvable towers and move them
    // across into the resolved map. Eventually the input map should be empty
    // and the resolved map will contain one final root tower.
    while !input.is_empty() {
        // Find a resolvable tower
        let resolvable = input
            .iter()
            .find(|&(_, value)| value.children.iter().all(|s| resolved.contains_key(s)))
            .unwrap()
            .0
            .clone();

        // Remove it from the input and convert to a resolved tower
        let value = input.remove(&resolvable).unwrap();
        let new_tower = Tower {
            name: value.name,
            weight: value.weight,
            children: value
                .children
                .iter()
                .map(|s| resolved.remove(s).unwrap())
                .collect(),
        };

        resolved.insert(resolvable, new_tower);
    }

    // Extract last entry in hashmap
    assert!(resolved.len() == 1);
    let result = resolved.drain().next().unwrap().1;
    result
}

/// Checks a tower to ensure all the weights are correct
///  Returns: Ok(tower weight), Err(fixed weight)
fn check_tower(tower: &Tower) -> Result<i32, i32> {
    // Recurse to children and return if an error result is found
    let child_results_err: Result<Vec<i32>, i32> = tower.children.iter().map(check_tower).collect();
    let child_results = child_results_err?;

    // Check that all children are balanced
    if child_results.len() >= 3 {
        // Choose the "correct" weight
        let good_weight = if child_results[0] == child_results[1] {
            child_results[0]
        } else {
            assert!(child_results[2] == child_results[0] || child_results[2] == child_results[1]);
            child_results[2]
        };

        // Find child which doesn't match this weight
        if let Some((i, value)) = child_results
            .iter()
            .enumerate()
            .find(|&(_, &value)| value != good_weight)
        {
            // Return corrected weight
            return Err(tower.children[i].weight + good_weight - value);
        }
    } else if child_results.len() == 2 && child_results[0] != child_results[1] {
        // There are two children with different weights
        //  Solving this is complex, so we pretend it doesn't exist :)
        panic!("tower with 2 children of different weights found");
    }

    // Return total weight
    let child_sum: i32 = child_results.iter().sum();
    Ok(tower.weight + child_sum)
}

/// Calculate bottom element of a tree given each item and its children
pub fn star1(input: &str) -> String {
    resolve_towers(parse_tower_list(input)).name
}

/// Check weights of the tower and return the corrected weight
pub fn star2(input: &str) -> String {
    match check_tower(&resolve_towers(parse_tower_list(input))) {
        Ok(_) => panic!("tower was correct !?"),
        Err(value) => value.to_string(),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    star_test!(example1a, star1, IN1, "tknk");
    star_test!(me1, star1, ME, "vgzejbd");

    star_test!(example1b, star2, IN1, "60");
    star_test!(me2, star2, ME, "1226");

    const IN1: &str = indoc! {"
        pbga (66)
        xhth (57)
        ebii (61)
        havc (66)
        ktlj (57)
        fwft (72) -> ktlj, cntj, xhth
        qoyq (66)
        padx (45) -> pbga, havc, qoyq
        tknk (41) -> ugml, padx, fwft
        jptl (61)
        ugml (68) -> gyxo, ebii, jptl
        gyxo (61)
        cntj (57)
    "};

    const ME: &str = indoc! {"
        wdysq (135) -> sxldvex, wiasj
        vjwuuft (33) -> inuci, neddz, rwamq
        oislgqy (77)
        lphki (233)
        wgbviwb (417)
        vikip (136) -> eofyk, dkexo, xzsxx
        elmieqh (19) -> dbziu, spefs, krtxpw
        tmzef (79)
        ectlgy (232) -> zmstcy, ncobxr
        sdatyo (91)
        uisri (11)
        smqimxg (132) -> husor, olzys
        pltzthr (82)
        szaqj (188) -> ptnndxj, fljpye
        jqdngi (58)
        uazwsu (15)
        xrrhso (79)
        gxeehd (68) -> iweii, rnqlzmv, hpmtom, vfzwqfr, xfzxrd, sgqhelx, hibjkps
        evkoenr (43) -> oecxbyt, qbthgst
        qivuzn (52)
        udeev (389) -> lphki, qthzk, hpgsb
        izgqzs (96) -> vbxzk, ubrrdtd
        naxtp (65)
        mvtkwn (42)
        sxldvex (34)
        tnlpmw (49)
        rzbrbmy (31) -> dvnqv, helyy
        esavxwq (81)
        yqgru (465) -> gfuyuz, elmieqh, xckzut, tmbhjxf
        ygypj (1303) -> ohcuki, ejdjxu, ytabct
        yggqq (855) -> gowlaq, ebtoxi, xpljwl
        ubaxya (92)
        pjkzokv (23)
        lvarp (76)
        yrysmsi (14)
        nofepy (23)
        apjeywv (132) -> kclmmu, exsugls
        licrtwb (56)
        gspffet (84) -> wqzxa, lptaikg
        gkrqba (82)
        mqreb (126) -> jvdoo, paykww
        xtidu (12)
        kjauagn (88)
        vvafqjs (56) -> qbvhefh, vfhgfb, iqtyv, ebdva, uxqkau, tydtcxc
        snuewn (118024) -> vvafqjs, mhaucon, kikva, mqnbmre
        gtkii (163) -> txcon, vuvwa
        oaoisa (61)
        ssqrs (24)
        ybditq (21)
        xgqxofa (119) -> bedzc, hdbkw, zwgsh
        wxfir (71)
        pxvse (19)
        xemcuk (23)
        paunv (28) -> zzvhmse, mubkcmk, vksephm, cbsdget, mellhhn
        couhjsv (11)
        tuzpls (58)
        puomwl (90) -> wtxjnc, jdjbnc
        wudlze (78)
        ibslyw (153) -> ksncpee, npvuz
        zzmczwl (54)
        kidvt (3136) -> tuzzzct, qtqhwon
        patlvg (20)
        yvvxtg (90)
        epxwjhy (22)
        epvyxld (17)
        xhecnbf (85) -> yituewe, wwhyd
        mqsrwsz (26)
        bhyzi (73)
        batky (18815) -> woctu, iykjtnw, tpiwftj
        sdponx (44)
        qhotyqx (95) -> vdwfz, agthd, tuwedv, ixkkdyc, lnlkwq, bvmhqar, rlwmfp
        nubqteb (89)
        zygfdu (70)
        tkerz (18)
        solqvvf (10)
        akiaxs (86)
        jsfqyd (70)
        aialff (68)
        uiilkiq (73)
        adlyhbs (46)
        tnljdhr (20)
        frctm (471) -> isqnx, euhgw, nrfqc, cmmnjch, zevrg
        jjklma (105) -> xrrhso, uyjor
        lpmnwh (56) -> orrfh, bqfzqra
        owxski (23)
        gqxyh (18)
        lpyntz (91)
        ivvyjos (81)
        qssjswr (268)
        jkbks (225)
        jwbdb (83)
        axqrke (119) -> mgoic, yivgs, bxggi, xamfjfv
        uwizlce (24)
        wuwhup (50)
        suawycj (60)
        wazvn (99)
        wtxjnc (53)
        ebtoxi (64) -> ttbns, voxychz, umsdpa, ajjmsnw
        uuhoydu (82) -> gvucdh, hesyq, glwsf
        tshyvej (1373) -> ebiovn, uvzpp, pblqb
        bhakbq (991) -> izqxal, nhhytw, flmma, isgky, coqek
        hcsjun (60) -> inelfwo, qcnstq, eymmhh, lpyntz
        otjbn (67) -> szaqj, kkhlzm, jtzdqn
        vtwud (10735) -> jldhlug, ikfzj, tsevkec
        heztht (86)
        rqonynk (65) -> olzqxap, oyhqsok
        esbdjah (61)
        hdinwud (1515) -> bdiftdg, ydtuje, aambg
        rbbjcq (71)
        kimffjc (155)
        wifrwut (91)
        hrlgs (9)
        ieowdj (76)
        vpqsjbj (147) -> eziosif, rptcpf
        rlurjca (51) -> gtbjtpi, ohpkkhx, zkwnhn, cagvlf, zvlobk, ahsqaob, iactix
        yzykfd (43) -> atqfhm, kazwz
        kikva (6131) -> gkpalaj, jafrv, uekzf
        lsoen (51)
        uexqjw (10923) -> dxczurt, omqhf, ylfxp, qofsaux
        nlnphv (117) -> vzlzrt, ypiwnt
        bemxocu (78)
        etcsuuv (312)
        ihwbmt (27)
        oorvvpt (63)
        dvnqv (88)
        ntlawn (60)
        jhaow (45)
        kccpv (17)
        kbybvzk (89)
        ctcmfv (49)
        ujqjrz (89)
        xekft (62)
        wihtv (1278) -> qgbbhdv, adlyhbs
        agthd (158) -> ucnehpw, hnutx
        vuytzn (21)
        svcqds (7)
        emqqf (5)
        uqddsal (17) -> trait, sdatyo
        uhwlk (252) -> sxtto, qyvsaxn
        ttbgzv (46) -> rokri, pygoqsv
        oecxbyt (71)
        xunvu (33)
        mvbtomp (35)
        uxqkau (693) -> qubhi, gneut, htmcpcy, qmncyu, wdysq
        ycrnaf (177) -> ssqrs, twimkx
        nlalji (28)
        dgvvo (37)
        niydqsy (33)
        pprvv (89)
        tmbhjxf (135) -> jutep, vkhiz
        iaftm (63)
        zjnrzph (81) -> gsxmqnw, gnxfwv
        ggzqccl (5)
        wwtiby (121) -> kytha, dgzywn
        zismp (58)
        hjbrfba (28)
        xnlkkx (79) -> puomwl, xnbvupf, hgksz
        bptaz (38)
        ozaogat (15)
        hxxml (27) -> emazu, kproiw
        xpptslq (76)
        ilohfvn (10)
        hadddf (128) -> geapi, ocltuv
        gtprqg (17)
        qofsaux (209) -> bbnjml, fwskxq
        xpljwl (128) -> ufust, aialff
        thhbhrm (41) -> smqkli, ovtfd
        xamfjfv (36)
        jdjbnc (53)
        kecpcmj (72)
        ixkkdyc (80) -> wutzrk, antev
        ysnbr (35)
        cyvwwa (190) -> jtqxxa, pgmtcg, btvsj
        uyjor (79)
        oztxmy (76)
        cancxr (213) -> ivdvlvq, ykxalh
        zlads (14)
        peokkb (167) -> mqkqtjo, qpchzkg, epssvr, yzose
        iauxpc (81)
        qbodszw (945) -> rgdvqu, oktgjb, abstvp
        dmwutl (73) -> desuqj, etjwno
        fcaht (134) -> ajidd, kigxtt
        fkgkcwd (51) -> mqwagy, gvtve, djvbdm, ckapp, ldiqm, ezspaq, pyrvg
        dqzkic (1032) -> inglhm, dgxdydd, hxhhhu
        ydtykm (75)
        zatfy (35)
        rmenjck (89)
        nrfqc (238) -> zgxjjm, fwide
        schexn (97)
        lrplxk (22)
        nroecsv (44)
        kiawm (64)
        zpfuxux (73)
        mvdkbch (28)
        sufdte (71) -> gfjeud, rbbjcq
        xmjunhp (45)
        wsnsqx (7)
        etyaxa (13)
        cfrqn (57)
        tslujme (60)
        bebuqpx (33)
        wwyuud (11)
        gtotarq (95)
        vtopekt (65)
        hxhhhu (230)
        aubvpzf (73)
        kllsasv (73) -> enuvuv, jlhipos, hittl, smyzyqg
        mgiuz (94)
        dsytvn (296) -> mxfonyo, yihru
        icsxziz (256)
        agflbq (73)
        amtrbok (12)
        qcebc (20)
        eymmhh (91)
        pbpva (95)
        rodxm (199) -> rzvlc, vnmml, rrmlgct
        ekblpi (97) -> flwbwcq, ecwpa, usgpgi, pknsw, gkjwl, zzfom
        ltpmtif (86)
        bkfnip (4873) -> murnoa, qhzem, udeev
        yhiab (249) -> sdponx, yvattv
        hbaai (70) -> rmnkup, dgtsic
        anazjn (225)
        zjjyjj (101) -> viwez, ihwbmt, qnwwjwt
        husly (98)
        jlhipos (20)
        xflsc (2516) -> qodow, vdkhvyp, masvpfj
        cenqs (156) -> jhaow, mivmhi
        rgrcla (84) -> nroecsv, pexfst
        pdddwgk (30)
        upefkx (75)
        abstvp (46) -> wqgoxl, gbwaobo
        glzmsn (93)
        bgtzw (37)
        flvrfdl (146) -> ybditq, vuytzn, yqdaz
        jmzpxql (47)
        wulvcf (89)
        mxfonyo (20)
        aambg (69)
        jfjvw (32)
        uslhk (8)
        wwzjdv (45)
        zwtsg (248) -> bjjtifj, ogbov
        mbxyf (56)
        xckzut (157) -> gaalz, upefkx
        trseebo (18)
        hbcpm (32)
        ggoaczz (88)
        zangrh (66)
        ktbto (55) -> pdqzfg, mfoismz, mkirruz
        rhpcgg (197) -> evkoenr, klpzhup, cigew
        kbbcau (25)
        porttu (92) -> haaqbj, lttlp
        gfsgzni (73)
        pyaqqhu (134) -> kdkobz, szvocp
        eiyiuq (54)
        fjgjfpc (17)
        lvvkjx (56)
        kinah (22)
        ivdvlvq (31)
        njerlzh (35)
        zzdae (63)
        rrkbjhn (78)
        hnutx (50)
        oqokji (198)
        voxychz (50)
        zovlqz (56)
        ulood (81)
        yzttfu (15)
        htmcpcy (203)
        jmytuxi (221) -> dvuctqv, sgilmx, fqdolnu, rdrncg
        rehhbv (17)
        ixkpaf (59) -> jqbdh, atvtit, qcebc
        antev (89)
        jfikdf (20)
        ykxalh (31)
        vqwclk (19) -> ewldimp, wkeqlvj, ouxuh
        hxeqn (528) -> kimffjc, qpgphk, vqfiq
        umsdpa (50)
        aodfzpi (69)
        wutzrk (89)
        jyjvpeb (81)
        ttbns (50)
        wmewl (977) -> yhiab, iordzi, uuhoydu
        ydejjeb (159) -> yvcanc, jrrubn
        bykuuwe (80)
        iesjhoq (336) -> bhzfx, sxexfai, mjounwc
        kdrhj (1832) -> rrkbjhn, gzpgvqq
        smyzyqg (20)
        kigxtt (80)
        rokvo (7)
        wpcng (50)
        koevsvb (97)
        yihru (20)
        krtglj (18)
        cmfwwv (40)
        jsaujq (1397) -> rgrcla, vqwclk, rxilrp
        gmjxxi (97)
        ekvmhd (49)
        lttlp (66)
        takbkro (200) -> ngjipr, inhfutj
        whvpgmw (54)
        lqaveh (97)
        qthzk (187) -> wntmkg, pjkzokv
        haaqbj (66)
        vxufrto (40)
        zseldo (181) -> hdvtd, papbxya
        psmgm (95)
        clwtgt (34)
        bpwxvvw (89)
        pmyrysb (77)
        gjclsl (75)
        olzys (57)
        woqoljk (99)
        ocodtdz (30) -> gjclsl, ogioyhi
        nbybi (54)
        veiaf (139) -> vlqqgb, iaftm
        uvzpp (165) -> mxvba, jjtafg
        qodow (63) -> oqxnfd, rpxaf, sybnvtp
        kouye (58)
        dlhuell (84) -> schexn, otzinx
        nlrlj (145) -> veknoj, lfpjfv, yrlnu, ugelox
        sambc (138) -> mnoinr, pkordz
        bypqww (50)
        rgkfzk (35)
        kiatxq (1232) -> dispgy, irnjtjo, iqpoc
        gigtu (60)
        xzsrv (89)
        dhafpel (636) -> vxddev, nxkpnt, zbuftv
        bqfzqra (71)
        rcbqqok (243) -> ofrogun, wwyuud
        oaqkk (69)
        dgxdydd (132) -> mudewqe, bazwto
        vxddev (128) -> bzpjiss, qmjamoi
        iqtyv (800) -> lvtotof, zddmrx, yzykfd, nlnphv
        vdcmrrr (93) -> xuoyxmc, amrcopl, fcnsfy
        jutep (86)
        cyanr (23)
        cxyxa (97)
        tuzkv (153) -> iflll, tipewrj
        cdegn (52)
        mvijo (35)
        xnbvupf (60) -> frlgjzf, wtorpp
        rwopuzo (15)
        uxfsb (76)
        qedyqs (157) -> bjzbqzq, alfqryh
        rwhgw (76)
        csmjozb (79) -> sasym, wmewl, kdrhj, btvmyff, sqkfgo, jcputh
        jbujmr (294)
        xvjfw (8)
        ekfwu (17)
        ebiovn (225) -> mnufo, rokvo
        ohcuki (197) -> alsfpfg, xsfryrh
        zavisuv (15)
        glwsf (85)
        vurdlqx (196) -> acfdkr, aefjv
        ihksnmq (95)
        ecwpa (248) -> soyta, ytomr
        jldhlug (234)
        oazfz (61)
        rdrncg (16)
        rrrmaka (51)
        qhgzopn (66) -> tmzef, iiiupn
        ifetn (86)
        plgjg (99)
        okjds (73)
        algkwbg (99)
        ugmhzm (56)
        vvnhm (135) -> ntrtfv, ihuqmbd
        csuywzh (1788) -> wgvpotf, fwtxvo, vikip, bpzhj, bomuft, otjbn, zsotrv
        pawpvkj (47)
        zzvkwsb (148) -> hjfucl, gxcft
        aivbhtz (78)
        nyckm (23)
        qwqpht (68)
        kbuurtb (62)
        ahexrp (19)
        dlikiv (247) -> kinah, sthtydb
        udiai (50)
        lopnwz (91)
        xbtvux (73)
        gvtve (222) -> typbqmw, vdsaccd, sdlta, uisri
        vczyhcg (8)
        edswttn (58)
        kqaua (89)
        lmnuii (536) -> rlwoz, gjvmrh, ukapl, lthoz, boahhv, ylpxahm, rqonynk
        kwlyw (193) -> hcruo, kecpcmj
        trskdr (322) -> hdrfo, bimxf
        clbmn (33)
        scffzsr (48)
        ewldimp (51)
        qhixu (21) -> roorbg, oruhqn, psmgm
        ypcup (94)
        vrszijz (56)
        vksephm (157) -> ngkax, tkerz
        sxtto (30)
        tkojiz (12)
        elygt (569) -> bixtvg, xsqbapj, ocodtdz
        vdctvf (84) -> iauxpc, cuyweja
        rktxkyb (108) -> xjbot, flwjj
        zckdwxe (54)
        vfzwqfr (54) -> mphzxio, clbmn, donnc, niydqsy
        iqpoc (260) -> zlads, jhapr
        pojst (51)
        ueitm (169) -> emqqf, qqstwmn
        dgzywn (39)
        ldcmzd (130) -> mertvs, ggoaczz
        xtgmmc (306)
        mjounwc (9)
        ftbiwxy (62)
        ogioyhi (75)
        fkitgnx (69)
        gvucdh (85)
        opsrep (43)
        olsycb (59)
        jiizacd (49)
        qbthgst (71)
        nxqvwm (69)
        rvrzp (113) -> xrqhewm, ifetn
        pkordz (84)
        vempqu (6)
        ulmwqtm (230)
        zwgsh (43)
        srgob (561) -> plrig, djbtbrk
        rmnkup (47)
        btvmyff (1442) -> mrqtrkq, zjjyjj, uqkskjn
        nrfyua (11)
        nuozixg (1064) -> nnelc, zmzobfp, oggljxs
        icrvj (22)
        vksphlf (1423) -> kdykksn, wwtiby, uqddsal
        xmnbar (58)
        ukapl (195)
        lijszmh (44)
        zzonk (28)
        jhapr (14)
        mrqtrkq (88) -> zaxbfog, pawpvkj
        aibtig (109) -> ggzqccl, hyxtojc
        vuvwa (87)
        ahysv (90)
        mqwagy (88) -> uxlisdh, rmenjck
        xpttnwd (95)
        oullg (25) -> dlhuell, imrwgqw, hadddf, rktxkyb, zwtsg, wllrov
        bwsgfa (22)
        vmiykci (153)
        jcgvp (18) -> mvbtomp, zatfy
        wetluzb (33)
        zvnzrr (37)
        sgilmx (16)
        geabygu (96)
        kakvi (41)
        fuujiy (92) -> xnfuz, urrhqwc, fmdfm
        ppvsy (285)
        nyuxiu (122) -> nzvqjrt, dhzsao
        tjwynpa (75)
        gowlaq (138) -> uwvfga, bnvlydl
        snywwu (26) -> snqey, aodfzpi
        cvude (41)
        snqey (69)
        ejqam (17)
        pgmtcg (47)
        rtcqml (85) -> zangrh, dspug
        ezspaq (215) -> rehhbv, epvyxld, oguxu
        ytabct (89) -> frfotet, zerxp
        nxkpnt (248)
        qubhi (107) -> lnznpq, scffzsr
        tfotbwz (424)
        wkgyfpp (824) -> ccbie, iesjhoq, kdffrsb
        wjsxcmb (95)
        ztdbvxb (53) -> zismp, dqoagdt, yxfjglq, givaxj
        ouxuh (51)
        xfzxrd (22) -> kakvi, mjlvx, wridnv, zeexkde
        rekerpj (7)
        redkli (81)
        qdkhjb (53)
        yituewe (95)
        ocnkhq (10)
        iaayhc (300) -> fwhhoz, ycrnaf, jkbks, anazjn, yhzjjgc, pocwgw
        mjlzzq (212) -> hwycny, gmzabjk
        rhaoo (36) -> kdzhiq, ydkhmp, dsytvn, wzezz
        fqkzcq (293) -> gztng, oiaxzp
        essijo (1248) -> wuqapgg, msmxf
        ckapp (94) -> clktah, akiaxs
        nktwwvp (159) -> qivuzn, cdegn
        qxwdzl (50)
        lfpjfv (48)
        lmfclbd (14)
        cglyptu (240) -> icrvj, epxwjhy, tpmqmg
        dhzsao (24)
        bckfa (16) -> ntlawn, tslujme, ccjze, woklobt
        mivmhi (45)
        bfjxeyo (24)
        plrig (53)
        ijlooi (1429) -> jcgvp, zvsgd, fzafkj
        pyhcj (64)
        gdvbtjk (71)
        fxrkz (70)
        husor (57)
        riiaj (44) -> lvvkjx, mbxyf
        lxmvg (4738) -> msjbv, ijoaqyv, yname
        wntmkg (23)
        helyy (88)
        muxvdx (57) -> veiaf, rcbqqok, gfnmx, zezeds, ktbto, qedyqs
        rroub (41)
        hnwfe (89)
        enuvuv (20)
        wzzlsg (18)
        hpgsb (81) -> xpptslq, jtoryw
        woklobt (60)
        jicyq (18)
        lybwr (64)
        ujgpnsy (12)
        mbtweib (86)
        vgzejbd (10) -> vuoqao, vwkkml, kmpfxl, snuewn, jjgjvki, fiprusz
        vgfveov (198)
        lvtotof (49) -> pprvv, gsckda
        ylpxahm (33) -> jyjvpeb, tytsynv
        glximrw (53)
        iykjtnw (542) -> dizqtw, oiozpzq, dmwutl
        xuoyxmc (51)
        wpvale (18626) -> wihtv, isbdm, gxeehd
        jruttt (28)
        urrhqwc (84)
        yfyaho (7)
        klpzhup (11) -> befwz, mypula, xmnbar
        kclmmu (49)
        qcnstq (91)
        sasym (13) -> vimnt, eoign, lusub, peokkb, mlhaxv
        mgoxhns (130) -> edswttn, ioycu
        stzzli (89)
        ltesfzf (44) -> sufdte, vpqsjbj, oghyxz, hxxml, mnhrjdv
        yxfjglq (58)
        fuhbay (82)
        gxcft (25)
        rwamq (86)
        qxqwmye (68)
        ketopn (42)
        tmsmb (84) -> pkxrm, pbbkg
        zerxp (75)
        zpbrw (95)
        wuqapgg (66)
        nmsats (82)
        vvzctek (9)
        cmoty (91)
        qkkhj (88)
        pocct (99)
        xyfvxng (45)
        mqnbmre (2069) -> vecnb, azhddw, muxvdx, qbodszw, yggqq
        msjbv (918) -> nktwwvp, qijleb, ttnoqm, ulaio, furlcyu
        mkirruz (70)
        mubkcmk (163) -> solqvvf, ocnkhq, wntitz
        lvzzwy (89)
        ypiwnt (55)
        bveilrx (306) -> pxvse, ahexrp
        shiaqps (45)
        mlhaxv (297) -> ctcmfv, ekvmhd
        eiohuhm (72)
        isqnx (122) -> azgohsw, pltzthr
        qqstwmn (5)
        fiwvclv (80)
        zobti (87)
        qbvhefh (53) -> ylazd, cyvwwa, sfkpzr, zsjlevq, zseldo
        xqgxel (54)
        yrlnu (48)
        rrmlgct (19)
        jtzdqn (178) -> lsoen, pojst
        nffvt (37)
        axtepsa (13)
        ngkax (18)
        viwez (27)
        kdykksn (7) -> geabygu, cfiqn
        zvlobk (419) -> mdfkbxa, wzzlsg
        xsqbapj (164) -> xqxvktz, xvjfw
        nzvqjrt (24)
        gbqbg (18)
        hskko (38)
        bgmntpq (6)
        zddmrx (227)
        dgtsic (47)
        iijfq (73)
        qytxu (71)
        mklcsj (54)
        jnwdraz (63) -> aubvpzf, zpfuxux
        hudde (1072) -> wyrfplr, rzbrbmy, ujmfmiu
        jcoyk (18)
        dspug (66)
        qgbbhdv (46)
        neddz (86)
        boahhv (195)
        ajjmsnw (50)
        lusub (209) -> nrloleu, lfwpqb
        bkgise (17)
        xsgyar (568) -> fesce, vfteg, thhbhrm, vvnhm
        mudewqe (49)
        trait (91)
        fljpye (46)
        xobszr (61)
        pkxrm (57)
        qrynkpt (22) -> kswafh, oztxmy, zejcp
        frlgjzf (68)
        aomksx (70) -> pmyrysb, robpw
        gjvmrh (167) -> xxtyban, mfjdb
        atvtit (20)
        ihuqmbd (34)
        mfjdb (14)
        rptcpf (33)
        ljitswv (89)
        zkujvlv (51)
        ydtuje (69)
        emazu (93)
        sqkfgo (1451) -> ueitm, okbtg, cnzynyj
        flmma (86) -> ygglox, ocvrlia
        jnqgs (76)
        ctken (72)
        kxdbyp (20)
        ktlkx (112) -> etyaxa, axygpr, rqsyc, axtepsa
        hesyq (85)
        tydtcxc (919) -> swcsd, nkclzqq, dhcopz
        sqbxzl (12)
        dispgy (52) -> olsycb, ynecxzo, esqvkcm, xclsj
        djbtbrk (53)
        svjnd (47)
        fizbyu (78)
        vqfiq (61) -> zdrhuz, jmzpxql
        tqsrg (70)
        dulrvcq (2086) -> apjeywv, vurdlqx, bwemy, ulmwqtm, qkxtwct
        gfjeud (71)
        xdzgwl (220) -> jicyq, prbbfe
        jcputh (196) -> xdzgwl, rodxm, piutfin, ezmot, bckfa, zyisjsj, icsxziz
        gbwaobo (94)
        mlyar (44)
        zsotrv (751) -> wudlze, hjmreq
        chxgqe (47)
        junva (83)
        fmdfm (84)
        ewwab (70)
        hdrfo (51)
        qijleb (111) -> qngug, voxemk
        qmjamoi (60)
        pyrvg (230) -> shcavm, trseebo
        joszidy (95)
        rxilrp (138) -> gtprqg, xqgqp
        ptnndxj (46)
        typbqmw (11)
        fesce (39) -> fuhbay, jozav
        oeixw (65)
        gjhfseh (20)
        zeexkde (41)
        ynecxzo (59)
        oqfpo (78)
        keazynv (68)
        gnsdn (51) -> njerlzh, rgkfzk, lhyzwtf
        sqljeh (20)
        dclmckh (93)
        rqsyc (13)
        wvwjep (70)
        ncobxr (8)
        builgr (78)
        xkdjbxo (81)
        bzpjiss (60)
        vfomix (95)
        wtorpp (68)
        pruui (1276) -> glaosn, xgqxofa, ggkael
        monhu (76)
        piutfin (156) -> wpcng, qxwdzl
        wrwim (10)
        jrrubn (52)
        vkdytw (24)
        vdtdmy (26)
        pbbkg (57)
        uwviodb (208) -> bkxfso, evrtfzj
        vfhgfb (1084) -> gnsdn, ttbgzv, pyaqqhu, riiaj
        iordzi (223) -> cfrqn, nzuhp
        biffdx (42)
        bpsesr (50)
        cglfuq (199) -> vempqu, noogeuk, bgmntpq
        bkxfso (19)
        oopnu (88)
        wlavb (92)
        gmzabjk (41)
        ehrfv (57) -> pruui, eondv, ygypj, vksphlf
        lpwuzn (89)
        cnzynyj (89) -> wwzjdv, xyfvxng
        dgmtgrv (12)
        hpmtom (50) -> zzpwgcd, qxqwmye
        jafrv (719) -> qhgzopn, aomksx, porttu
        msmxf (66)
        ldiqm (266)
        llydxxz (37)
        vdsaccd (11)
        cbsdget (81) -> licrtwb, zovlqz
        qgwhbwh (62)
        epuqu (17)
        loojs (43)
        ovshy (95)
        qhayeqe (86) -> opsrep, loojs, fvukbun
        amrcopl (51)
        tsncd (108) -> zygfdu, jsfqyd
        oguxu (17)
        skyaki (69)
        ufxfx (25)
        mpzyapm (19409) -> elygt, ltesfzf, dhqrk
        lthoz (150) -> rwopuzo, uazwsu, ozaogat
        xxtyban (14)
        ggkael (54) -> gmjxxi, koevsvb
        yuyhys (12)
        qngug (76)
        kazwz (92)
        owysb (71)
        eondv (25) -> uxggaix, ngfxax, ztdbvxb, jmytuxi, hkxfb, rvrzp, ppvsy
        oyhqsok (65)
        iflll (97)
        xjbot (85)
        tytsynv (81)
        rsyxcdq (157)
        vuoqao (102055) -> kahduw, lxmvg, vtwud, ktcffyf, ggdoqnl
        ocvrlia (48)
        cpuxdo (75) -> rroub, cvude
        tpiwftj (482) -> cancxr, xhecnbf, ilcmx
        eaoxb (2777) -> cfrbz, vmiykci, kllsasv
        alfqryh (54)
        gfnmx (214) -> ekfwu, ejqam, kccpv
        daerl (14)
        nvahhge (14)
        rzvlc (19)
        ktcffyf (1777) -> dhafpel, jtyljl, nmcewof, rhaoo, xsgyar, otetl, essijo
        pntvfpd (49)
        hkxfb (285)
        ngfcc (87)
        yqdaz (21)
        qzwauo (49)
        kdkobz (11)
        tslbp (224) -> mvijo, qolgxnd
        vlqqgb (63)
        zsjlevq (175) -> bemxocu, builgr
        xzsxx (79) -> zugkqyx, jubeqci
        yvattv (44)
        nhhytw (44) -> skyaki, nxqvwm
        zmzobfp (75) -> ntxtwk, yhvdjl, xzsrv
        sretj (240) -> wsnsqx, rekerpj, knciro, svcqds
        awwhgzp (70)
        adqnigs (72)
        oipscid (85)
        xsyqdlv (26)
        eoign (134) -> aujeia, ngfcc, zobti
        nfulfov (14) -> aivbhtz, exlwxxi, oqfpo
        fqehxv (168) -> lgonp, vxufrto
        impzr (70) -> lybwr, pyhcj
        bnvlydl (63)
        olzqxap (65)
        frfotet (75)
        ajidd (80)
        qyvsaxn (30)
        rlwoz (19) -> kjauagn, qkkhj
        hysssy (12)
        ptmsqz (81)
        kahduw (31) -> qhotyqx, xhxek, bhakbq, ivgpv, lmnuii, frctm
        ejdjxu (41) -> wazvn, algkwbg
        ylfxp (99) -> heztht, mbtweib
        isgky (80) -> kecckss, obakqyf
        xclsj (59)
        qhzem (47) -> fnkgp, ujipv, tuzkv
        zaxbfog (47)
        uekzf (1315) -> qjxikz, bptaz
        gkjwl (50) -> pucdapo, zckdwxe, xqgxel, vcprhe
        xqgqp (17)
        ntrtfv (34)
        ufust (68)
        yvcanc (52)
        jtqxxa (47)
        rfumv (92)
        qjxikz (38)
        sybnvtp (59)
        pugxio (62)
        olfwppt (154) -> bwsgfa, lrplxk
        ugelox (48)
        oruhqn (95)
        rlwmfp (158) -> bypqww, wuwhup
        qnwwjwt (27)
        hjmreq (78)
        paykww (22)
        aefjv (17)
        hvpess (1924) -> vaxtfaa, wbnmb
        dbziu (96)
        xnfuz (84)
        tpmqmg (22)
        qrinzet (69)
        ldrpjd (88)
        pocwgw (13) -> glximrw, bhmirp, rcqkas, qdkhjb
        sxexfai (9)
        euhgw (208) -> xrbkzmk, xsyqdlv, vdtdmy
        ulaio (203) -> pdddwgk, sakmm
        vnmml (19)
        ksncpee (28)
        pucdapo (54)
        ucnehpw (50)
        btvsj (47)
        veknoj (48)
        papbxya (75)
        sdzhsq (94)
        orrfh (71)
        gnxfwv (68)
        grthhzv (244) -> dgmtgrv, tkojiz
        dxcozh (35) -> phrpja, ahysv
        vekndsc (94)
        ujmfmiu (39) -> vrszijz, ugmhzm, sdrcc
        jctafp (62)
        bhmirp (53)
        tmqlkof (28)
        wyrfplr (109) -> qzwauo, pntvfpd
        shcavm (18)
        xmkegiw (28)
        ydkhmp (290) -> cyanr, nofepy
        ivkcc (70)
        ujipv (183) -> gkrqba, nmsats
        lnznpq (48)
        xqxvktz (8)
        wzezz (264) -> bfjxeyo, vkdytw, uwizlce
        nvgohm (26)
        ydqtt (158) -> oeabdth, bkgise, fjgjfpc
        nmcewof (1182) -> plgjg, imtzl
        bbnjml (31)
        iweii (8) -> lvzzwy, nubqteb
        kmpfxl (134829) -> csuywzh, ehrfv, bkfnip
        dbkxe (34)
        sfkpzr (76) -> lsszcka, manlcoz, oipscid
        bbyoqzx (14)
        uuavydd (43)
        hjehgnc (489) -> ujqjrz, wulvcf
        qmncyu (17) -> ofveqm, qgwhbwh, ftbiwxy
        zzfom (210) -> jruttt, mvdkbch
        cigew (185)
        nozgflw (89)
        trfjf (174) -> hskko, evbeo
        idqhx (12)
        noogeuk (6)
        ccjze (60)
        bedzc (43)
        uymju (8) -> ectlgy, pchapba, tsncd
        jvwvi (86)
        xrqhewm (86)
        ifijr (28)
        tuzzzct (50)
        zwrjdk (82)
        ekvzyod (11)
        vivco (626) -> okhisbv, biffdx, rbascuz
        atujmm (250)
        bdiftdg (69)
        wqzxa (82)
        jtoryw (76)
        sttlowq (207) -> xmkegiw, tmqlkof
        wqkoep (90)
        sdloeq (14)
        hzruk (47)
        mgoic (36)
        mqkqtjo (57)
        gsxmqnw (68)
        pexfst (44)
        ycfbxe (254) -> tnljdhr, jfikdf
        kenxmax (91)
        yhvdjl (89)
        gzpgvqq (78)
        inglhm (94) -> qwqpht, keazynv
        uytmyv (91)
        wqgoxl (94)
        yfxbu (32) -> jbujmr, mjlzzq, hcrys, mwfksx, fcaht, ycfbxe, tslbp
        mnhrjdv (185) -> lmfclbd, sdloeq
        gcasp (17)
        jjgjvki (99205) -> suuppr, csmjozb, uexqjw, cckrzh, yuzzsk
        zvqvvyp (39)
        bgpab (1781) -> qmqpbm, rdwxvvp, vjwuuft, dlikiv, acaqfng
        prbbfe (18)
        gaalz (75)
        robpw (77)
        rcqkas (53)
        axsimnf (403) -> yfyaho, qeyjc
        mypula (58)
        mnahxn (52) -> cxyxa, lqaveh
        smqkli (81)
        nkclzqq (44) -> bhyzi, okjds, gfsgzni
        soyta (9)
        cagvlf (267) -> ypcup, mbffei
        dvuctqv (16)
        hdvtd (75)
        lexqf (96)
        cuyweja (81)
        vfteg (17) -> dclmckh, iihwb
        fzuosl (94)
        ipctg (80) -> zzyxzr, rqdtsp, esbdjah
        yuzzsk (3542) -> ijlooi, yqgru, oullg, ekblpi, hudde
        tpvhe (42) -> sretj, grthhzv, luqqu, uwqee, qssjswr, kbrwk
        gpbdzes (96) -> dgvvo, zvnzrr
        cmfwvem (23)
        befwz (58)
        mellhhn (166) -> vvzctek, hrlgs, evqbli
        bmxolxn (20) -> bpwxvvw, sfnix, fxnap
        rnqlzmv (88) -> tnlpmw, hrfbh
        qeyjc (7)
        jozav (82)
        gjahajj (62)
        qyfzonc (89)
        ngjipr (24)
        sepmxir (33) -> uuavydd, xocoiqa
        qizpmf (98)
        mroft (77)
        lyeyx (61)
        gtgovt (134) -> gqxyh, gbqbg
        hrihd (50)
        pygoqsv (55)
        dxczurt (89) -> rupxxhm, lopnwz
        mdfkbxa (18)
        hcruo (72)
        ytomr (9)
        bhzfx (9)
        mbgapsj (39)
        omqhf (105) -> jwbdb, junva
        wntitz (10)
        zugkqyx (89)
        ntxtwk (89)
        phrpja (90)
        ilcmx (129) -> xbtvux, agflbq
        hhcoc (63)
        bxggi (36)
        obakqyf (51)
        xrbkzmk (26)
        sthtydb (22)
        xvljr (21744) -> nfulfov, takbkro, gspffet, fqehxv
        hofirm (33)
        okhisbv (42)
        uldgij (64)
        sakmm (30)
        zbuftv (62) -> glzmsn, oacnj
        jxevsc (57) -> wvwjep, jilzdse, fxrkz, vkgfrb
        ioycu (58)
        ysuttai (72)
        ttnoqm (133) -> vtopekt, naxtp
        ereyjs (54)
        hgksz (174) -> msuvilq, couhjsv
        luqqu (112) -> yfaihrb, fizbyu
        bixtvg (180)
        kswafh (76)
        fwide (24)
        ghgou (12)
        aujeia (87)
        lgqad (75)
        szvocp (11)
        yivgs (36)
        lnlkwq (216) -> nvahhge, bbyoqzx, daerl
        qtcck (137) -> lgqad, ngtdwvu
        wkeqlvj (51)
        vbxzk (75)
        uqkskjn (132) -> kbbcau, ufxfx
        gacpvy (26)
        ccbie (90) -> cmoty, uytmyv, dmiccse
        suuppr (9751) -> uymju, rhpcgg, vivco
        fqdolnu (16)
        vcprhe (54)
        iihwb (93)
        swcsd (91) -> ltpmtif, jvwvi
        nbwaei (61)
        zdgyktc (84) -> jnqgs, uxfsb, xkzcttf
        bimxf (51)
        pdqzfg (70)
        bomuft (785) -> xobszr, idpttp
        kytha (39)
        zyisjsj (58) -> woqoljk, pocct
        hwycny (41)
        ofveqm (62)
        oqxnfd (59)
        pnpsnjp (803) -> zpbrw, pbpva
        ysekeoo (126) -> suawycj, gigtu
        gneut (203)
        zdrhuz (47)
        qmqpbm (7) -> owysb, gdvbtjk, qytxu, wxfir
        imrwgqw (170) -> yhncv, mklcsj
        dymzl (72)
        vzlzrt (55)
        lfwpqb (93)
        dirhtk (65) -> hrihd, udiai, bpsesr
        donnc (33)
        iactix (293) -> ulood, xkdjbxo
        acaqfng (271) -> wrwim, ilohfvn
        tuwedv (150) -> eiyiuq, ereyjs
        mjlvx (41)
        fwhhoz (72) -> rrrmaka, zkujvlv, fyaco
        msuvilq (11)
        mxvba (37)
        smvskd (1262) -> cglfuq, rtcqml, zjnrzph
        ggdoqnl (9436) -> hjehgnc, xnlkkx, srgob
        aifpyc (61)
        vwkkml (148790) -> hvpess, nuozixg, tshyvej, yfxbu, kiatxq
        tipewrj (97)
        pblqb (7) -> kouye, ibydam, jqdngi, tuzpls
        oktgjb (70) -> lcbgeev, zwrjdk
        dqoagdt (58)
        xkzcttf (76)
        zevrg (108) -> stzzli, kbybvzk
        lomrivn (92)
        grkzvkj (124) -> xemcuk, cmfwvem
        uwvfga (63)
        frxshon (95)
        zzvhmse (157) -> ujgpnsy, idqhx, hysssy
        ahsqaob (75) -> gtotarq, joszidy, frxshon, vfomix
        jqbdh (20)
        kvzmie (69)
        oiaxzp (22)
        spefs (96)
        fxnap (89)
        azhddw (909) -> smqimxg, mgoxhns, izgqzs
        epssvr (57)
        kxhvvgi (8) -> ihksnmq, wjsxcmb
        etjwno (91)
        tlwxovy (91)
        capuc (91)
        evbeo (38)
        djvbdm (50) -> zzmczwl, whvpgmw, szqdapo, nbybi
        mbffei (94)
        uwqee (90) -> qyfzonc, nozgflw
        bpzhj (251) -> hbaai, gfsnz, snywwu, ktlkx
        dhcopz (207) -> hjbrfba, nlalji
        jilzdse (70)
        lgonp (40)
        zvsgd (58) -> yzttfu, zavisuv
        rdwxvvp (203) -> mlyar, lijszmh
        wridnv (41)
        ovtfd (81)
        kttsf (35)
        murnoa (238) -> gpbdzes, gtgovt, mqreb, grkzvkj, nyuxiu
        otzinx (97)
        bjjtifj (15)
        irnjtjo (12) -> ubaxya, lomrivn, rlhmcf
        zezeds (79) -> kbuurtb, pugxio, gjahajj
        ofrogun (11)
        bjzbqzq (54)
        qolgxnd (35)
        vdkhvyp (52) -> kfkfyx, mgiuz
        rlhmcf (92)
        okbtg (35) -> dymzl, eiohuhm
        kkhlzm (96) -> wlavb, rfumv
        dmiccse (91)
        esqvkcm (59)
        zzyxzr (61)
        tegtsu (72)
        kbrwk (124) -> tegtsu, adqnigs
        mertvs (88)
        awmmdn (49)
        pknsw (74) -> lexqf, pxadg
        fwtxvo (71) -> ibslyw, jnwdraz, ydqtt, flvrfdl
        flwjj (85)
        asbmk (76)
        sdlta (11)
        vdwfz (234) -> amtrbok, brpgc
        exsugls (49)
        yfaihrb (78)
        oghyxz (213)
        manlcoz (85)
        mnoinr (84)
        ibydam (58)
        hjfucl (25)
        lcbgeev (82)
        ygglox (48)
        xhxek (425) -> uwviodb, cenqs, mnahxn, ysekeoo, vdcmrrr, vdctvf
        krtxpw (96)
        zgxjjm (24)
        vkhiz (86)
        inhfutj (24)
        qkxtwct (166) -> jfjvw, hbcpm
        mfoismz (70)
        voxemk (76)
        inuci (86)
        wllrov (200) -> nvgohm, mqsrwsz, gacpvy
        fyaco (51)
        ngtdwvu (75)
        kfkfyx (94)
        iiiupn (79)
        gkpalaj (76) -> jjklma, ydejjeb, sttlowq, axqrke, ipctg
        ubrrdtd (75)
        fwskxq (31)
        nzuhp (57)
        npvuz (28)
        fnkgp (65) -> vekndsc, sdzhsq, fzuosl
        roorbg (95)
        cfiqn (96)
        fcnsfy (51)
        idpttp (61)
        vaxtfaa (83)
        jubeqci (89)
        uxggaix (195) -> shiaqps, xmjunhp
        vvlimno (63)
        txcon (87)
        ocltuv (75)
        kdffrsb (187) -> ldrpjd, oopnu
        imtzl (99)
        jvdoo (22)
        yhzjjgc (93) -> xunvu, bebuqpx, wetluzb, hofirm
        clktah (86)
        givaxj (58)
        fiprusz (88) -> bgydix, mpzyapm, xvljr, bnywvsx, snmey, batky, wpvale
        wgvpotf (115) -> tmsmb, zzvkwsb, olfwppt, impzr
        ohpkkhx (385) -> kttsf, ysnbr
        eofyk (41) -> ctken, jyrzc, ysuttai
        ubllty (91) -> qizpmf, husly
        aqgumo (47)
        zezbp (91)
        mphzxio (33)
        gfuyuz (209) -> jiizacd, awmmdn
        rpxaf (59)
        aewlgu (62) -> oazfz, aifpyc, oaoisa, nbwaei
        coqek (166) -> uslhk, vczyhcg
        kecckss (51)
        jqlbenn (65)
        jyrzc (72)
        azgohsw (82)
        qtqhwon (50)
        furlcyu (85) -> lpwuzn, hnwfe
        evqbli (9)
        zwvhgce (121) -> chxgqe, hzruk
        xsfryrh (21)
        jjtafg (37)
        acfdkr (17)
        rupxxhm (91)
        nnelc (342)
        hdbkw (43)
        usgpgi (182) -> ketopn, mvtkwn
        thxqmj (81)
        bgydix (16136) -> tpvhe, excdqtl, yjvfni, iaayhc
        pxadg (96)
        uiryg (2879) -> aibtig, sepmxir, ixkpaf
        exlwxxi (78)
        vecnb (897) -> atujmm, qrynkpt, trfjf
        zkwnhn (329) -> oorvvpt, zzdae
        ezmot (96) -> bykuuwe, fiwvclv
        vrxkr (37) -> jxevsc, gtkii, nlrlj, kwlyw, fqkzcq
        zmstcy (8)
        axygpr (13)
        yzose (57)
        kproiw (93)
        lptaikg (82)
        cmmnjch (6) -> ewwab, awwhgzp, tqsrg, ivkcc
        qpgphk (33) -> lyeyx, cqfsg
        hittl (20)
        wbnmb (83)
        tsevkec (54) -> yvvxtg, wqkoep
        mwfksx (216) -> mbgapsj, zvqvvyp
        bvmhqar (224) -> epuqu, gcasp
        oeabdth (17)
        pchapba (70) -> kqaua, ljitswv
        gsckda (89)
        gtbjtpi (265) -> ovshy, xpttnwd
        xocoiqa (43)
        woctu (56) -> qrorhfv, axsimnf, wgbviwb
        foxvut (77)
        zerhdhs (150) -> xtidu, yuyhys, ghgou, sqbxzl
        rqdtsp (61)
        dizqtw (103) -> ieowdj, vgkhk
        ebdva (718) -> vgfveov, zerhdhs, kxhvvgi, oqokji, lpmnwh
        ikfzj (212) -> nrfyua, ekvzyod
        hcrys (63) -> foxvut, oislgqy, mroft
        vkgfrb (70)
        bwemy (92) -> oaqkk, kvzmie
        yname (1373) -> qhayeqe, dxcozh, dirhtk, zwvhgce
        desuqj (91)
        bnywvsx (19757) -> pnpsnjp, paunv, hxeqn
        wiasj (34)
        lhyzwtf (35)
        gztng (22)
        szqdapo (54)
        flwbwcq (142) -> xekft, jctafp
        ijoaqyv (91) -> ldcmzd, sambc, aewlgu, wrkoi, xtgmmc, cglyptu, qhixu
        knyvyft (81)
        ngfxax (139) -> uiilkiq, iijfq
        atqfhm (92)
        masvpfj (184) -> ifijr, zzonk
        yhncv (54)
        wwhyd (95)
        oggljxs (314) -> awrbrc, yrysmsi
        sdrcc (56)
        sgqhelx (60) -> hhcoc, vvlimno
        snmey (84) -> bgpab, xflsc, kidvt, rlurjca, uiryg, eaoxb, dulrvcq
        qpchzkg (57)
        mhaucon (2652) -> smvskd, wkgyfpp, fkgkcwd, jsaujq
        brpgc (12)
        vimnt (91) -> rwhgw, asbmk, lvarp, monhu
        excdqtl (378) -> hcsjun, trskdr, tfotbwz
        uxlisdh (89)
        isbdm (1276) -> aqgumo, svjnd
        alsfpfg (21)
        zzpwgcd (68)
        glaosn (86) -> ptmsqz, ivvyjos
        fvukbun (43)
        rokri (55)
        qrorhfv (93) -> redkli, thxqmj, esavxwq, knyvyft
        eziosif (33)
        jqgfu (40)
        nbqaqp (298) -> owxski, nyckm
        hyxtojc (5)
        awrbrc (14)
        ogbov (15)
        rbascuz (42)
        kdzhiq (63) -> kenxmax, tlwxovy, zezbp
        geapi (75)
        oiozpzq (127) -> uldgij, kiawm
        fzafkj (88)
        zejcp (76)
        wrkoi (124) -> capuc, wifrwut
        dhqrk (173) -> zdgyktc, etcsuuv, uhwlk
        jtyljl (348) -> bveilrx, nbqaqp, fuujiy
        otetl (1340) -> patlvg, gjhfseh
        mnufo (7)
        cfrbz (117) -> krtglj, jcoyk
        lsszcka (85)
        rgdvqu (166) -> dbkxe, clwtgt
        yjvfni (1179) -> ofnqgn, rsyxcdq, cpuxdo
        gfsnz (14) -> ydtykm, tjwynpa
        knciro (7)
        sfnix (89)
        cqfsg (61)
        izqxal (142) -> kxdbyp, sqljeh
        ivgpv (1040) -> ubllty, qtcck, bmxolxn
        bazwto (49)
        twimkx (24)
        evrtfzj (19)
        ofnqgn (19) -> qrinzet, fkitgnx
        nrloleu (93)
        ylazd (201) -> jqlbenn, oeixw
        hrfbh (49)
        dkexo (146) -> nffvt, llydxxz, bgtzw
        hibjkps (106) -> cmfwwv, jqgfu
        oacnj (93)
        inelfwo (91)
        vgkhk (76)
        cckrzh (6841) -> hdinwud, dqzkic, vrxkr
    "};
}
