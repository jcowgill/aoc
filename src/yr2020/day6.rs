pub fn star1(input: &str) -> String {
    input
        .split("\n\n")
        .map(|group| {
            let mut ans: Vec<char> = group.chars().filter(char::is_ascii_alphabetic).collect();
            ans.sort_unstable();
            ans.dedup();
            ans.len()
        })
        .sum::<usize>()
        .to_string()
}

pub fn star2(input: &str) -> String {
    input
        .split("\n\n")
        .map(|group| {
            let mut lines = group.lines();
            let mut all: Vec<char> = lines.next().unwrap().chars().collect();
            for l in lines {
                all.retain(|&c| l.chars().any(|c2| c == c2))
            }
            all.len()
        })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    star_test!(example1a, star1, IN1, "6");
    star_test!(example2a, star1, IN2, "11");
    star_test!(me1, star1, ME, "7120");

    star_test!(example1b, star2, IN1, "3");
    star_test!(example2b, star2, IN2, "6");
    star_test!(me2, star2, ME, "3570");

    const IN1: &str = indoc! {"
        abcx
        abcy
        abcz
    "};

    const IN2: &str = indoc! {"
        abc

        a
        b
        c

        ab
        ac

        a
        a
        a
        a

        b
    "};

    const ME: &str = indoc! {"
        fbqjswm
        qmbjwfs
        fmsbjwq
        smjbqwf
        hwsqmbfj

        sxpoqbueg
        fbhxqzjrtdkgwaepos
        qbepgsxo
        pbxvosegq
        giqepcobsxl

        gd
        pfdy

        yijwkdzvoub
        cifujkyvbwz

        htumjfskxapveoblncy
        tkvfxpejomnbhcuslya
        xukpmyenothacjfbsvl

        dyt
        hclpfxvykumo
        bwjyi

        ezfiytx
        pitkrf
        fpeki
        gbuovijdfqaw

        wyxabpekmlovhu
        lueyowbmvkapxh
        emhakvlyouwbxp
        obyxlewkmaphvu
        vmhaobxeklypwu

        qerwhnkioygzcsulbjxa
        wlegxsyupobaqzcnri

        awroqhfs
        abyfqops
        ikqcgzatjuen

        lnxdjmpvirbeus
        gazhcwfy

        jqo
        qonj
        oqjcs

        qgahfybi
        dbhjfwcgy
        zgfhyqb

        urtsfljxgc
        teyorfwlxgu
        uvrlzpkixgnmtq
        tardhlxjuog
        uxetgbrlw

        apehszymrlwjqgu
        mhqjgdzlswryuxa

        vnamzidfsojth
        ouvqmlhpbrdjfzawit

        zus
        muczs
        xuzsr
        puzsg

        actgknebysj
        cjbgesnkty
        uemgdkcynst
        stfqcgneyk

        rgxsfmiplkobquvzedntwya
        wyietkrnhzamxouvsbdqlfp
        sevfloziqnmtjrubypkxwda

        lsj
        sjl

        ihyvfqesulxorctjgmzpdank
        hycmevxlsfdugproiqztknja
        ujmdnqyltxvswzfhpkgoiecrab

        mynlpojwqtk
        hisrag

        gaw
        agw
        agw
        gaw
        wag

        pkoq
        dtqbypcoj
        loqpf
        eprvmoq
        rqepo

        bfuheqvzsjignmctwl
        jfszbqegvhmlcniuwt
        txjsgivlzcmwauehbdnqf
        tejwzcvlbnuqmgfish

        qutrxvl
        zytdmqrx
        rxecqta

        fayzme
        vazey
        yazj
        aybhzsdc

        csnjhrfyqbxiav
        rfvaycxqihsbn
        fixpvahcqsnrjy
        nclzsixrfvoyeatkmwhq

        quey
        enxuq
        qeup
        utwqe

        rbosi
        tirdojubs
        boris
        borsi
        iobsr

        mdskbhvyiqertul
        uqbltrykimehsv
        ktvmyjuchreqblis
        hequalkrsimybtv

        rsitupnoy
        fzwksemhcbq

        vkrsfmljcbzdthi
        tfdlkqjpmgh
        mkqtfhlpxajd
        thdkljmf

        mrfpbwyx
        rwfxmyb
        yrxwbmf
        fmwryxb

        rmztba
        uabrzmtpy

        kajxyerviqthbzcomgnsfwp
        wkhjetspdmblnyiqgzafcvxr
        egyirpvajwxmnbzqkhosctf
        whpcbruzkxeivftysgnjmqao

        o
        o
        o
        o

        vmjwhzngqtaxl
        ivhnfdcgtzqjwma
        zagqmhnwvfcjtky
        gjatqhwzyrnvm
        wckanzhvmgqtj

        azixnsjvh
        hxvsaj
        xvzhnajws
        lgxvhasj
        chqksxrevjuoaf

        wlbocnkiyjdfmvgue
        mychvijfbekulrgw
        fvymucjewsghlbki

        jczgonpt
        jfzibgon

        k
        k
        k
        k

        r
        v

        oeafxtjqhrdlgupwisc
        lvhyjegkwrfpasxcto
        cszalfyrtxbvwgjpeoh
        axhzcepjgolstwfvr

        hqfdymlisknuaejc
        rptnamdkobqciujxh
        nlgayihvumdkqcj
        ickfhdauznmjq
        niqzkcjaudmh

        zavtjyul
        uyaihcbjtzmnfvq
        yjwdaerztp
        tqoazjsxlymk
        ybjaztql

        gz
        qxgal

        sdhcagp
        scahdpg
        ahgdpsc
        adsgcph
        gdsfcaph

        ysud
        ydoxpqhesu
        tbsjyudn
        bsdvyul
        dsyvcut

        onizqprkeumy
        teckol

        fxlohzwgkebcn
        rznhkfocgjvde
        ktgefihnomzc
        bsmypfchqxneoazgk

        robwtniusy
        quisghwtdv

        mhtxubnl
        yhmdtwcupezvxabl
        kthbulmgrxs
        bqhutximl
        uxqhibtlm

        nyifbxuq
        ohmpfrntzdc

        aityekusrc
        yuecirak
        yricukea

        fbhntzkpalduvrjeqxg
        najtimzflgcdqpkyrxb

        byi
        puln
        ytdc
        vafzwhsjkqxge

        zngipme
        inzpmjsx

        sxrwd
        yla
        nf
        uhjzpvbme

        xpmhcy
        yxhpcm
        ychxpm
        ychxpm
        cpxhmy

        wvmspxgrf
        wxgsfivm

        uwcfnb
        caiyqxovdp
        jstmchku
        lctezs

        hy
        hy
        yh
        hy
        hy

        urepkmtblcfw
        ejxuiacmvonpbr
        pmzgbqruyech
        wcpkuzbrmhes

        jo
        oj
        jo
        oj

        gepdsrkwazxfyql
        equrwsaxyl
        ylwsxaqer
        lrwyqesmax

        ceist
        iwhudaxgreyvfbz
        kqeospit
        ienjqm

        roqikgmtubnapl
        rtepiyvfsouj
        iqrdlgtubpoc

        rgtedxo
        xegotdr

        tkz
        kzt
        ktz
        tzk

        aimcqg
        qagrebmic

        ra
        sa
        a
        a
        a

        djzixkqyt
        lhreum
        cgwel
        lamr

        dom
        omd
        mdo

        virpctxulowjq
        xrwlpjucqsiot

        ljxzenhfvqburo
        jqoenfbvhxyl
        vcmfbyhwnoqjel
        sikqfvlnebgtjho
        lnejfqoxhyvpwba

        ebqkrwfijg
        jkrgbwf
        ybwrfjgvk
        wjkqegrbfv
        hwjukgbflxr

        mnqukrsvfctlpbewidz
        coiuealptqmhwvdrskf
        slyjekvhmrptwcudfqi

        gdhqrenioakvfj
        gedaivwzxqyfoskr
        dckqormvbgjelhfai

        cegixsmlduz
        iegdslzcumx
        zedksiugcmxla

        aznelohvw
        jovendlawh
        uehwyvorminfa
        avwenoh
        hnwpaove

        xvbtpinmaud
        qxruionfedm
        nxigmhud
        uitaxznmd

        dirpsgulvjwxkqzof
        rmdpjuyivlwkozqfsx
        wzfiskheluoqxjrpvd
        qasjkzxilcdofvurpbw
        wrqhudxkfvspizjlo

        dhefy
        ehfyqnsdlb
        yhezdf
        heyfd

        hyla
        hyal
        lyha
        avlyuh

        q
        p
        ip
        y
        p

        nasejwkfdgz
        awegjndsfkz
        sdwjfgezakn
        neawsdzkfjg
        gdnzwakjesf

        vdsa
        asdv
        svad
        savd
        savd

        dqyxjwvu
        afxhvbgz

        yvhxzrckdmebwoistlfpaqguj
        qhsyupcixrwfobkjzlmvt
        uyftrkhwlspnzixqbcomvj

        zukomqjdtwaphyrcfb
        qdyhjwuzcrmfk
        yuhkjdcrgefmzqw
        jwfqmgcxrkyheudz

        m
        gjkar
        hni
        hs

        emc
        ecm
        cem

        bfuxtzqyrwsldgjhep
        ymlrtqpbosdewgaxju

        uvphrftqsaekbzxj
        vzhxajqfpuekrsbt
        tqzpvjuskrabehfx
        ypqrbafxksjzuevoth

        tm
        ap
        a
        a
        o

        zknhwxbclamjgvsefoy
        dlrfvhkyjnsexzbmagcw
        gxwklbjehofacnmyzsv
        hvajsfnzkcxemlygwb

        fmsjqhkxyw
        jsfmhqywxk
        wsqmjfyxhlk
        bqhjcmfyskwx
        mksxwqyhfj

        buxi
        i
        wh
        s
        ab

        ywjbtszuhniaopd
        wyibtcejfznmdohqua

        cjgstbahpzfrnqkeo
        ecpboaqzgnkrjftsh
        ztajpchkgnbvqsrofue

        vtibnfojdaxekhgmy
        myajhtdnbfoxvgik
        jnavmlytwdfshbkxrigo
        khiamfxngvdojtyb
        kidbhfmagyjvtxon

        lnupbwhodmei
        imelnobphdwu
        neipbomlduwh
        lumneowihbxdp
        hiublekowpdnm

        lypirvqzcw
        kziwqypxvr

        yeadtgixzn
        nydxeztga
        qdfkoagyncxtze

        otvm
        ilvham
        qrvbkxj
        wveoz
        fulicv

        leazscfdotkuhxmr
        eocdxshlaufrmtzk

        mdtlwyin
        ydnitwlm
        ymtwlind
        idmtnlyw

        zuonrg
        gwzxneor
        urznbgol
        qbrkzgnou
        roqkngz

        ywbpxicgstaufmozedqkrjv
        xbvgnfltorzwceiqampkyshuj

        chkmy
        qsbamjn

        emvlydfruaits
        kerixnwlbu
        zxwjbliepur
        ouzeilr

        ybiu
        cyru
        zpkv

        pqcdevnlraiwtjmfxk
        njtlacpbixdekv
        dxlykvipagctnj
        xkjltagcndivp
        suvhkcaizpdoltnxj

        cgqnapzvfwyimk
        qpcvzgaywmfnik
        zykcamvpniwqgf
        kyfiavmgzpcqwn

        t
        t

        xqhpuljcgbzyi
        thyndcqw
        ncqyhmvr
        dhfaqmcy

        s
        sgz
        opvswqtx
        s

        rikeqfvdla
        jqfacvosgu
        vfqae
        vbdfqrwa

        mfewqjdkcb
        xnogrlmah
        nvimla

        zcfxogw
        xgcpotfljw
        geyfaomxc

        uync
        dny
        yn
        ydn

        zvpuajx
        pquz

        q
        y
        y
        w

        phk
        hmk
        kdh

        ljcumvrwdsfpzixhqgantoybk
        lcgwubvzspqoramyetkxjdnhf
        uqmsdhlptyfobanvgkrczxjw
        uxqvyhbzkjangmtsldfprocw
        avgmoswrxhdjypftczknuqbl

        vyxclotkrzefghanjmipsu
        tvzmrcywlsoqkebdai

        gdrnybwsuxfc
        yrbpmnfus
        bvfcshmntuyr
        onaisrfqkbyzu

        slpetjhcvmaufd
        tpfvdaumjechsl
        ejcasvltdfpmuh
        shtuaepcfjlmdv

        bqwhndkeotmjcv
        skcwtjeymbvqdohi

        rebfqy
        gqeybf
        beqyf
        yfbeq

        kzoenrdtc
        zdwtnx
        nzdxgt

        cdta
        kjroqnldu
        tuvc
        hyegxbwfmzis

        qpdfokyxsubanv
        sfxkupnabvy
        ukbypvxafsrn
        srfpvxnbyauk

        wxdkupoba
        cdfxkpwobau
        obkdwnxupa
        qzulkwoxdhpba

        vqzxurtjcbeild
        qzbgklntdijvhcsxwye
        jtdvxqzebclmio
        plutvbjiqedzxc
        utbzcjxovliqde

        shvwemxcjlodknriy
        xowueychnfmsrpkvj
        ynewmskzjvhpqcxor
        wcsqkhxzjmyvoern

        tghcpi
        igcthp
        pctigh
        higatcp

        khnlfdogbtmqsc
        rstoyencgilkb
        ktzgljpboauncs
        govcnjlwbqthsk
        bnlkotqgcus

        clfqxtajgw
        wxaclufj
        wfjxdaucvl
        xwljfcia
        jcyplrhsowxmbfaz

        jwvuczdk
        ujzwvdck
        cdjzuvwk

        wqrpskjbzxd
        sqevgnkropcaxiu
        xhsbfrmkqzp

        g
        g
        l

        lbtvkxeuiycrwsjpad
        watikedxycljrupbv
        cypkrxtlbvomawueidj
        jbewavliyrdxctkup

        r
        jwur
        r
        r
        r

        fmdxyhr
        yrdfhxkm
        mhyrfxad
        lrdhixmfty

        rpanbukoyqmcew
        wdku
        ktluvgwi
        wkuf
        guhkjtlw

        kbfcnapuvxdme
        demhltfxpuinvc
        fcnpehixdvm
        nqzoxcdmpfyvejg

        qscuwkvpl
        uafwkqylspi
        zchkwfvusa
        shxnwyquk
        romjtbsudkwe

        bzorewil
        oebzwli

        ihzmnublpsga
        gpaziqbvsxnmth
        mnzispgaohb
        zshingbamp
        yimlhznrepgbas

        slbzwpftqcvj
        vfupdwrezscojyx

        ukofntijmbryecd
        cyejmbfnudorikt
        dryfiukceojnatbm
        enfdmjtyboickru

        ukzfgsanmldiqej
        zldmsqgnifjbke
        oylgasqfkjnmedzpi
        glfmqeijdzsknh
        ijmesznldkgfq

        fyxhmu
        btyvfih

        fcasuqmiborhtyw
        cyusmhpwrfoqae

        pvmj
        qmpuvd
        mbpvr

        fu
        fu

        cmsharebt
        tycsehablrm
        ebhcrsmzao
        ebasmxchr
        seacrmhbn

        phf
        fph
        fhp
        pfh

        x
        x

        cklhmiqbfrav
        hfivmclbqakr
        acvilfbqhmrk
        vlfahbrickqm
        xamcrbqifvkhld

        ldykra
        arkdly
        lrdkay
        aldryk
        dkyrla

        bf
        f
        f
        f
        f

        sja
        j
        jn
        hj

        syr
        r
        w

        ajitzbsk
        ktiasbzj
        bijhskazt
        jbiamtskz
        btkiazjs

        ci
        b
        x
        b

        jgbzatyevmdwoirh
        ayidemzjhrvb
        muzbjqfishxenyrdlav
        vmyjbredaihz

        nsta
        pvxsn

        pwe
        pwe
        pew

        skydwacovhbrq
        cdbhorkvywaqs
        avwybqkcdshro
        rbakdocqywsphv
        qowhgfcavbsrdyk

        soewtgjpcdvla
        gijtxwdlevpaysc
        todevpzwglcjas
        gacdfrwmpsuvebtlj
        wgvlcdkejtpaons

        kboigesmfr
        ufjo
        pohfjqn
        qozf

        jet
        zrejt

        ygb
        yug
        myg
        gnysplqjod

        kjqsrbofmeinawhlzgxtd
        ehdtcmjrksfziqvlabwxo

        nqdvlzregyfp
        wvhlpnmdzegkyf
        mldhexvnzpgsfy

        m
        m
        r

        yxkrzaov
        urvzmxaigdk
        sazvketrjncb

        jhmgqebw
        zalupkdf

        myhwf
        yhwfm
        wmyhf

        ujqmvybntwlfgrskz
        vufrdgszeblymkntjq
        vuonrfqbslxgjyzptk
        yzkunbfrjtlqgsv
        fjqyrgnluvbtzsk

        yvxrmbogtfziqeclpjwdknauhs
        ldwjkqzotvynuxahcpfmersgbi

        bapjknqrem
        qgmpnrh
        hnrpmocq
        doqurpmn

        ramwtungpzjv
        wzrvngmtjpua
        zurgmatnjwvp
        pgtzrmvujnaw
        janzpvurmtgw

        mencbuo
        folkueabn
        zqayub
        pusrwjd
        tkylhu

        x
        qsfmnyuow
        eg
        xbde
        xr

        jqiwyx
        wix
        iwx
        xiw

        d
        jilwe
        xgvf
        b

        qauwef
        yvbcijs

        pxykceoad
        k
        kznifl
        k

        rh
        hrm
        hdr

        iertpcvhdyojnsfa
        edchyptlnsjiaow
        onbpaysiuchedjt

        ulrjawfezdq
        dytianoelkqxwmgfju
        ufvawqjdhespcl

        pagzxn
        zangpx
        xnpagz

        xlksupgydimtb
        dgpbsxtymluik
        lgfubtvxcsdmypiwjz
        yimltngspdbuox

        phc
        ihpc
        phc
        cph
        cph

        ybumkwiegvpsfrladzjh
        sfajurxbncqevwkgdyptl

        gr
        rg
        rg
        gr
        rg

        hrkvgpintusxzcfbjdo
        habgkptcrzdfxylvui
        ngdctqkpuxrzvhifbo
        kfuztmcrhdpbxoivg

        zslmwoeh
        lehzswp
        zewlsh
        zswelh

        tqgwiukvmxbopradencjsylfzh
        tnojksulryzdehqvawbcpgxmfi
        iqotslzpjeyrgbxfhudwvackmn
        rbvxpctodlegihunyfjwzqkams

        cuzwomlixbfganhk
        yglfvnwzbkimorux
        xiqgnmkfobzwul
        ifkbxolngwmduz
        jztklouwfgibnmx

        heobpftukwcrxyngj
        gfhemvqxajotpz

        maektfpjqbyhz
        kiabmewjf
        abkvocefmj
        arkmjenfb

        xyufbcmjtlvwgp
        ouwcfryxvzsje
        hxwcaqyfuojvn
        fcqujheyvrxw

        lqtbuxhfwnk
        wynbhe
        zhwnvb
        rswdhcnib
        bwhnr

        wxcnqskthobyzumapgejl
        npcjubmkhwyolgszaeq
        opzhqjlyuegmwnsabkc
        okehynrujmapqigswzcbl

        lyjnuwtpgobavfh
        oawgyuhvpntfjl
        vgjydfopntlwahux

        vfioljdqtsz
        dqlvojfztwp
        elofzvdjtuq
        gqhzdfbotlvj

        ebhnvgqjct
        gkqulcjwze

        ugncyzxqvjmlirf
        xlomqekjyrdanwhfv

        tqhupz
        lwtjuai

        uhvk
        l
        lg

        qjy
        pcyjq
        jbqy

        duoyfsnhlc
        jtlpvohxsauzb

        hkoqtgyvmjdwlnpicx
        noxkdlhygpmvqjtcwi
        ojqdthcywlgxenvimkp
        hcxwygvdjpklmoiqnt
        wxtjcpdylgnmkhviqo

        jzuei
        eib
        ei
        eiy
        iez

        pujfagxbze
        faejixpg
        mgtwqevaxsfjrhp
        aexzycgfdkjpl

        iunmadwyvlsxtpjck
        ruihstgbkmqoxzfcdenw

        qkvxdctszruljmpganbho
        pumocnxkdlvrhtjbszaq
        hurqpmotnlcsfdjvxzakb
        bsrjuhxntkpmoqvaldzc

        suktgmbnrqfahj
        jqhsrkbfmntgu

        ogaxbpiwvufktecdynlz
        bqshpjmgr

        dbikqj
        kqbjdi
        kibjdq
        bdqijfkl
        qjdbik

        pa
        otd
        akn
        jsrqmvwf

        jr
        mr

        xdgrfvbncsjaltiqkyhm
        qtswydkimlnzagvxch
        equasoltvmyidngkpxch

        maqpjnebszkftoc
        rskduxejyvqg

        k
        k
        k
        k
        xark

        tiokl
        hnzadijvbq
        xi
        lieo

        xfjlskdabohcvzmtpur
        zgvlqrjoapmbtufhc

        urceayoxjsh
        lnzvwqmpdbt

        g
        g
        x
        g
        g

        zuwgj
        guzj
        guzqhrj
        zjgu

        xsjotcbfzqlkewiv
        ypxfhkjliuc
        ljrmcxkif

        ycgevt
        wkxhe

        a
        a
        a
        a
        ja

        imwespvrudzaqojc
        wvjiodspemqrzuca
        mwvdzqosuciapjre
        cpwejouzqadsirvm
        eirmvjqzapcwsdou

        yt
        i
        i

        ovxmc
        xmsczv
        smvxc
        xvmc

        ynogzuevhipsbrwt
        uezhvrpbontwiyg
        swivytbuzegrhno
        tbvyhoezurngiw
        nuvygohteirbxzw

        hvktyilwjpxdngouam
        tayjndpghiorzvluxkm
        aonsixtfpgmhlejkudyv

        hdatbcv
        zmyxugq

        lmajoitgubwvpf
        ibloytfwumjapvk
        alfrwiutmhvjbpeon
        mojdtpbaliuykwfv

        wblkycgps
        lkgpcyswb
        wsybgplcvk
        cublspdwgjyk
        ygplbkcsw

        pwyqvokde
        vdreypwk
        ypevdkwr

        lykv
        riz
        sgjwtmoub

        jobiqznpsugm
        czjamsyuipnklqobg
        bpnotjugsfzmqi
        bgoqnsjipuzdm
        gubnqozpijsm

        qukzafenwyhblosmrgvx
        zvlfurkwceqasohi
        suzeolakhvfqrwpi

        iycm
        ciy
        icy

        hmxz
        zxd
        xzl
        xmdz
        xz

        yftbrxvucjkniem
        dqtl
        tqog
        tphazw

        imkegrwdxybuav
        kneiomxygdavrwb
        rgoiamenwypdkxvb
        wrqdahgfiemkyblvx
        emxrkywigvbad

        zk
        mw
        d
        e

        ihcwvxeotpzfrkqsl
        syjiahxvnoebfcpgqkzmrud

        foktjcszgwhr
        fokqcndbyizrhaeswgl
        vkrfsocpgwhuzm
        rwhkgscxfzo

        y
        skt
        fg

        ejirwohkzfub
        hujfbrkozn
        oknucvzgfjhbr
        jzlormuvhfkb
        xfsouqjzrpbhk

        scbejldiaz
        jsblace
        hbejsal
        slabuje

        osfej
        hjsoek
        oejszk

        la
        wl
        al

        icquzebwdkrhs
        cbiukwresh
        ruwhkseicb

        ikcqygtvfsnaurelwd
        ybukjdmlecvxpawzho

        wfhyiscdpjvgktoz
        dvhfkowiyzts
        dtvysikzwoufh
        vesyhiqtkdzfow
        vtskzynhfiowd

        xszyugwmdoflejaknqibt
        fzoavqsumdwpgynkjltx

        lctpvbshq
        pslc
        scwlp
        cglpnws

        xpyteraozhluwfcj
        jueoaltzypcfhxrw
        rwycztuahoxjpelf
        fuhoprexjytazlcw
        wyapzfeohxtrculj

        xd
        rsxv
        xc
        x
        uxmle

        uh
        p
        pw

        u
        u
        u
        u

        qvhmxwtfrzcjolynesbkdip
        qvnsibxmzrdowltpkhfeyjc

        ezvopirtq
        vitqzope

        iewnmbouxctfadgh
        tgsmjucxadby
        daubxgctm

        b
        b
        b

        fhswi
        limr
        rhlp
        fihp
        uykdoc

        wbfhklqaug
        jlxvnqadzhksfw

        tunfikcwpexs
        gkwpmcsurtnieo
        akpwxstecinuo
        upskvwtnyjiqzech
        wudbengklptsric

        loedjctvbszkw
        pgxrlihqa

        gjwok
        jgowk
        owjgk
        kwogj
        gwojk

        f
        fyd

        gajmfcikbnrylhpsx
        mrcpingkjlbhfxysa
        hacxpgrifkynljmbs
        flajskgnbrhpixcmy
        fincblhsgxrakypmj

        xazukhfjyqtm
        makzthqyf
        qakymfzhlct
        kaihzqydfgmtb
        tzfjkqhyvma

        dgzlpe
        elqpzg
        ezlpg
        eplgz

        s
        r
        r
        a

        cohzy
        zlyv
        dfwuptbg

        ypjodgshuetcrlxfzv
        tsrzlbgdpvqojueycfai
        reopcljuysvtdgfmz

        n
        fp

        tjdbzgfs
        z
        zreuvpqy
        zpci

        noekg
        gonek
        eaogkn
        oknge

        tkmxfod
        domkft
        kdtfmo
        ktmdofy

        anode
        aoke
        voea

        ryhe
        rhey
        yerh
        hyer

        ntabgs
        bg
        bg

        huktgwiosylrpdxjbcf
        dfrixhluowksjbytp
        sjhrolybpuwzxdikft
        ysjhwokurbfptdlix

        bifxwln
        xh
        fwmsou
        hmyuqwc
        apkrjvegtd

        ezhytmakiwou
        whmyptnuasodj
        toruwaqmyxh
        vgnyualodmthfw
        whmuygato

        qk
        mq
        huqz
        lq

        bejyhfndolxqt
        axebjclthoqdniy
        tnjbqyhwxdsel
        lztqyfbvjkedhncx

        eqyfglxicuzdphjkvnmsoat
        equwiagnvpjcbyfxorhdzsm

        msjyh
        ymjh
        yhmrj
        uydjmhx

        xqbldatriuhm
        rnbczjkgyow
        rteib

        jkqnoawyvbreizsdp
        zbsxirwpynkveoaqmjf
        darzwpqijnbouekvsyh
        kjipoehwbyqszvarn
        ervysjwbpzanokiq

        npbehmoiswz
        opinhmwszdb
        howmbpsnzi

        rvpgdjaoc
        opjdracvgh
        cdvjgqaprno

        liegxqpjzmvrfdykbsncahwo
        gnfhykipdxzwcbvlqeajmors
        dqcjlfonayphbvkzsgrweimx
        oglnszmehxfriyjkdvawbpqc

        pokiwqthmvfzadbjrucxglye
        orzuptjqblnaedghvwciyx

        tw
        ocdgpm
        aj
        jhuti

        pclrgmiefodu
        bxvg
        jzhngt
        thgvsxnq

        duosnjqakwleh
        kzjendahmlworqsu
        suwlbjqkedonah
        qwsjuaheknidol
        lpjfaonekwiudhsq

        antqsypzmujbkihwclvg
        rtqwhlzkasiyjubgmncv
        qytmphkvbunijgzswacl
        qumiyzsehadnckjtvwlbg

        sbec
        sb

        puflvmxnwzcdqotirgejab
        clwoqjmsgpurezfdivnxbta
        qnfwizdgelvaoujspbmcxrt
        lozmarwqjevbpgxtcifnud
        adcqjwporbnvltfhgezuixm

        zfayukrmjv
        xkveunzar
        kevozrcanu
        zuaowvrk
        itvazgkrnue

        skocmigb
        kgsimnbco
        getuchifkqpbo
        ogbcki
        gvokbic

        frmzlsjgenowiuvdqka
        svliznoqgdufjerwkam
        kiyoafjmvdsnwlurzqge
        gwrdfniuavqejxmplzsok
        mwzqvaiedolksnrfhjgu

        kfluwecmzvnx
        mefvkqcnuzxlwp
        yvkeumnxlcfzw
        kclfnzuvwemx

        kmgerotqfv
        kgmevqrotf
        vmqtofkreg
        terkfvgomq
        rqvofketmg

        akenjtzfbycvxgu
        xhwnozegkubj

        tveuglsjwpfmq
        lphsowtduefg
        saeuptowfdcgl

        axqvchtd
        xaqbkvct
        taxiscvgnyq
        zluqpxwavctjo
        cvtafqx

        phowmcdy
        ocdmhpw
        pdmhowc

        ifakxsn
        sixfkna
        insxkfa
        iknfxas
        sfixkna

        jtsoef
        osfwt

        flm
        mf

        xh
        xh
        xjh
        xh

        yvkxuiphbl
        ivuxkhlpy
        ukpxlhivy

        hgbz
        z
        z
        z

        gvwfz
        zvw

        dvs
        vnesudy

        nmajxvozyhelpbt
        avthblepmzjyonx
        jzxevylaotbpnmh
        ehxajptmlyndovbz

        knayxrlzgoiwjvsdh
        lyrhgwenxsvjkiaozd
        vrwdstgoqkjzahxnlyi
        yanojxkrsvwzldihg

        ekcoubjlryfvdaqmztxnw
        ethkxgdwvmjsconbifzpqra

        egcibkxu
        bigxuekc
        uxeikcgb
        gkbcuxei
        begukxic

        mvhcaezgbs
        bmcegavh
        ghevzuabcm
        hsagcmbejvl
        eabgrvmchyx

        meglpzdkqrsohf
        yzhedarogbmql
        rzmioqdyhengl

        tdosc
        cstod
        otdcs

        szxwv
        xhns

        fsmvhdn
        ldvsfh
        hfskvd
        sfdvh
        chledfvs

        gdmwvnyqihtspbfcexrou
        bumficazjrpgxynwldoqvt

        dyplaojbvhmtseiczkrn
        zmihtdnpsbryoklcejav
        zidcnhvmajteplkbsroy
        hoyvemcnjrdbtalpzsik
        lvbrejikdynxuozamhpsct

        ouqbsjwxadlftyhi
        ofvuzqtxyihldjeswab
        aotxfjwduqimsyhlb
        bdoalxwhfjtysqiu

        ie
        ec
        se

        mnflwaopgbcvhxt
        boqymvanthlxwcfg
        faowzgvxcthbml

        jl
        jl
        jli
        jl

        kxdpqbcgvuimw
        wmpgidkxujc
        wmgyxrupickd
        kpdgwiumfxc
        mxwicgrdukp

        jfsvrhqwpteckmuz
        ltcqsdhjbauxikepwvr
        jqtkwfhepcvrnus
        hnqupekscvtrjwz

        tfzxdcs
        zcdkx

        wiocksjvegyptdbnaml
        inohseycvjpwdgtmak
        jodxsiwmazkqvgcypnte
        rigwopdtceymvasknj
        nytbjmkcwpsgeofriaudv

        vghoqamjewufp
        qevtoxkmagfdsw
        xaeqgmwvocf

        cfkheaivnjqp
        qdpnezv
        loubwgsryxtm

        pgsdcueklha
        wthpgvjf
        qzyxbonr

        z
        v

        mzlonai
        djcfgnrkatuxl
        qnlaie
        bnpal
        vanle

        qcszfvwlagrbniuo
        bfrqzluigncswao
        oacwngblqiusrzf
        qrzbclnfswguoeai
        noalwcsgfzirqbu

        bk
        gbks
        ukb

        opgkvynaqexmiublrsfwjthc
        mbasfctuyerhqwvxgldnozk

        cdlnpxkqsbvhgemr
        gdheqkmlnvbpsrwx
        fmdnsrvghkebqxltp
        rgszqbkemldxpnhv

        phociwjgq
        inospahluxet

        qup
        aln

        lk
        l

        s
        s
        s
        s

        pyucgnt
        ynltgc
        ycnugtd
        ctqsngimyo
        ycngt

        vchjzu
        ujczh
        chzju
        czhju
        jzuchi

        rdl
        oljzkd
        bdle
        ldvb
        drl

        ypaxs
        ycxps
        ysxp
        pxsy
        slypx

        dmeniwsrapqchtv
        rbsizl
        bxgrfusiyok

        qdbpuxghfelwjtzsomyiva
        oiqhdzluajvtswekxp
        xjnlaihstvpqudzweo

        tdzjxu
        zjutdx
        ztudxj
        xdtjzu

        xd
        dx
        dx
        dx

        notxgaqvpbyfje
        raexdhikm
        zalxdscewm

        tlhk
        clg
        yo
        gcnzot
        frveaxij

        riohxqt
        hvdqts
        vcqsdht

        mi
        fw
        ch

        mcfgtbsp
        ekpgstc

        hugfk
        adn
        st
        itno
        cod

        btkzcna
        czntbk
        zkctbn
        tbczkn

        yaisrtpjuhngmox
        psjoatrgynzduh

        jxersuyobnqkathwpid
        jsbeaixorpyqwhduktn
        eidwbvyjpurnxkhotalsq
        yorqhesaxiwjnudbktp
        kunxwsytoijdqbaephr

        gpozmvuctyaljws
        yfxzogtjmua
        efbyznatgjorimu
        ojgruzmqytah

        s
        ezj
        p
        gdnmykaxo
        irlp

        fakoitvqlwxumjcypnrbh
        xoikwhmjevcrfblqptnuy
        jhpixuvltyeqbokwcfrn
        nxorwfctihpyldbjsvkuq
        twrmbujavfcnxhiqolkpy

        gfcwqieompdtyluhz
        gwhcoqtfelmzpyiu
        tuyhinpflqcwomegz
        owlckiyfpgzuqhtje

        knpagzftyjxcs
        ptxakjnygf
        nsyxafjtgukop
        pkgjnrztevfya
        qytdmbpwkfgajn

        yjuzgsvermfhotainbckd
        ckbyoagrnmvfestihj
        bgincyhfosvtkmejra
        togvjersmynbfcihak

        wicho
        ihokc
        icohw

        buacxrg
        xurabcg
        uagbxrc
        bxugcfar
        uckxbgar

        fdjnxbgzky
        xngyajkzd
        yjozgnxkd
        njxgzkaody
        zodyjngxk

        pfuebk
        ku
        ku
        uk

        ysgf
        j
        sgy
        qb
        mhe

        oygrdsel
        lrysdke
        dpyesiflrw
        ydselr
        pieyrsfjwld

        zpewx
        ewxpcz
        wepzx

        xyredbkszaicqolv
        idqolxrbsyczea
        iyrqdxcasezlbo

        ilqd
        qlid
        idulyq

        buwdolahxfrcnjsz
        rfujabnwhx
        ufrjpxnwahb

        abulqncx
        ocxauqn
        cznuqxak

        mhnyswie
        omcbwyhefv

        pbrxtumijaqgdkwzvceonls
        mkhgzbdvxwcirnpeujoaqtls
        npcxyftmjzgsraoilbqekvwu

        dhiysqlbmjvoxf
        ovxshjmfliqdy
        zolsifhxvjyqmd
        rnxdfvahmoyptjkieqgslu

        ibvnlah
        lavuibnhzp
        bahilvn
        bilnhva
        hilbvna

        voqfmayzbektjwui
        iezmjtbovkwfu
        vkwbeozfmtiju

        jaxetsymlnr
        eakjrmynslx
        mnajeysgxrl
        marlnxejygs
        gmnjaiselxry

        pyufthdvaqxngbk
        nvhlbfjtwmdzsax

        icwsjyfge
        wyjfgeci

        dut
        tu
        ugt
        bhtu

        g
        g
        fgtohm
        g

        ozagcvu
        xqsfjgzouc
        zoucgm
        mzcugto

        qujrhkixfslameon
        ukdmpgeshrxnzj

        excbvslqhad
        zjotgsyenurp

        jrqiwlcz
        cerwlizh
        wlpndrbci

        gjstrhckqnw
        rntqjskhcwg
        thckgwjnrsq
        qcrstwkhnjg
        wcsknqgrtjh

        wmcfgezspqdha
        kgswzfdqbmcrnehp
        hwdztacemgfspq
        ywehgqcfsdmzp
        yhcdgpzqsfmwex

        okyxslua

        gpioxhjkdbqlzsey
        gkebjhyzxlqso
        lbejqzxhyokgs
        seoyjgqzlxhbrk
        qblxvhejygskoz

        zgl
        gvlzn
        gzl
        ldgz

        us
        us
        musql
        us
        us

        uhv
        hvu
        vhu
        hruv
        huv

        oaxt
        ptdghjq

        xgvmr
        vzgxemrj
        rvxglm
        xcrmgvl

        glb
        lgb
        wgble

        kngvreujcafyblwxpqi
        pqufgkwayxejlvnbci
        xcekdwlpyjqbfgvahuin
        ujxanebwlmrfpivqykgc
        slaputxbcwfgyjoevnqizk

        lyvfakznbuw
        olfkrsbp
        xojfmqlhbpk

        hax
        jp
        nmf
        eh
        awi

        q
        nxmch
        k
        q
        k

        dch
        dh
        djgh

        cmtpfzibwqkg
        cgkwtiqzbpm

        zksd
        dzekascj
        zhikpgru
        kyoz

        py
        tpy
        yp

        hkbuqicdzretsxvmgoa
        mdgfwnyvtlqpecrak

        jc
        cjv
        jc
        cj
        jc

        dtalrc
        dlct
        mdchlt
        ctld
        cadtlr

        ostayzjxpilrcuebngvwdmkfqh
        udgxyozfepsncwbhqvlikarjmt
        bcaiurltfjqedgpsmzvkwyxhon
        isawcgvmdyjhtbpnfoluxezrqk
        ndrtewqcbmuovigpsyzxkfhjal

        w
        mz
        z
        p
        m

        ilmdn
        dsilum

        swaorjlgbevdmcftqy
        whgxuakfpitvonqmz

        odmuanyzbjekhrl
        gwlzhamukeoby
        huoyavkmblez

        ctsauv
        tubvcg
        huczavt

        gtljuexfiopmdhnczsaw
        gjypnkrhsdxlumziea

        zdrsfectygwjviob
        qobcjzmrthpygdvewi
        iatxzgldynjbvkwo

        emknygaiptjlwozxc
        cewyziltojnphkxga
        xfkizjnaepytcsgwol

        cxvdkfn
        uxwtf

        hd
        d
        d
        d

        uqzhkesytb
        zsktbuhyieq
        ybhqzsekjaut
        hztybeuksq
        kubyqehtsz

        r
        e
        w

        sgqnzlp
        nqlfsg
        uengqdcl
        gyflsqtn
        fmglnq

        rokebxmfy
        btemfy
        byfkeagvt
        bynfdezsh
        ebfyir

        aqpmvyefrdwongujs
        jvmnwyuadefqrgsp
        uvgaqrfydsjwnemp
        lvcdbwuaesqzgmyjnpfr

        rgcbejutw
        gjebwr
        gbjrwe

        wnucbkymsxirfovhaezpltdq
        fqmwrtynzohbxkspdviealcu
        rdhseiqycpfztvlnmxboauwk
        drualzqxiykobhecswptnmfv
        sevolnphmryaxwdfqkuctbiz

        kxesgvrpfatzomncu
        poynusfktagrzmvxe
        taszrmefonwukpvgx
        nzvkyepsfxramuhtog

        x
        z

        mqf
        fqm
        fmq

        plqknez
        znrucvosqe
        ewzqnak
        jaqzednl

        mvfsz
        v
        su
        getnakyh
        x

        jdvrwyizulgthfacpb
        zkhmsyaxwoebcdpngtrfj

        zhd
        dhz

        vqidfwokxbhzatslcn
        wcbzgfutloskarvxnhe

        rhds
        rtp
        nlrfsh

        stqk
        owtksmq
        kstzq
        sqtk

        kjunerphs
        hnjskeu
        sehdjkoun
        snheujkt
        uknjesh

        syelrcmhftj
        mbvtofei
        bpeftmvd
        zfxnpteom
        efoztm

        ih
        hi
        hi

        ypiaz
        piay
        yapiov
        ypeia
        apiy

        lvriun
        vinul
        vlin
        vzldni

        rwkvyshp
        uwasr

        bxpgfq
        pyqgrbf
        bpfhqg
        qpfcbxg
        pbgfq

        zvtcugahijyenw
        lvngtcduirwjakzm

        fpqbrzjimtdlwnvchy
        bjpzmwytcvhildqnfr
        dzqmcflinbprjtvwyh
        whytcnjilqfvzdbprm

        iwfpka
        ipfwa
        imfwap

        hjuyc
        mlwjv

        ulpjhvsnbztfiqe
        nlhbqteupzio
        inzepqhbtlu
        hmbztrcliquenpd

        fjxgrhpeozl
        rholfeu
        rlqochfe
        eflroqh
        ofrhle

        rowfxmt
        f
        fp
        ijf

        zqbokvdhw
        zvohebqgw
        howvprmjztlnqbu
        vgqbzoewha

        dtwf
        wvtfd
        fwtd
        wtfd
        ftdw

        tdmzifqp
        tzpiqdmf
        zftpdqim

        lgv
        qhl
        ljyaxzwocmbrk
        nls
        npuld

        zuol
        vo
        oxnykdec
        oulmz

        uoevmcpgyixrflwtasjhqzn
        elpcxtgmvjiynfowzuashqr

        v
        l
        c
        l

        fuz
        ufz
        zuf
        fzu

        zgfhiodeybckqxp
        tiquwkmeaxcp
        gzieqkpjhxnc

        dlpcyjzfhekuiqgom
        khpgieujcfmoylqdrz
        uqeckojzfimyldhrgp
        ciyqmlapgfjduehzko
        fijhqmkeucgzwdploy

        odqthnlcgsu
        csprkbo
        cebzso
        cswbero

        fsph
        psh
        psh
        phsiju

        tgrwxaqibelzhm
        jynqdvfo
        cjuqopnk

        iheu
        uih

        rjyqhauivsoz
        srvcnahokuq

        i
        kon

        tgvukpcxnory
        fxuhsmeativp

        kvyha
        vykah
        hkvya
        hyvak

        lpahrbqe
        hlrpqbwe

        ndtoxjfsp
        tvjdeoxlp
        phxasrotjdn

        vcpzksneyxfo
        uevyhslzcpxgt

        wpxvmnghz
        vmxphwz
        hvwxzkmp

        usvdwyomecfip
        suweomifpvcdy
        csiuwmvdpfeyo
        mosdfwuyeivcp
        ycupmidwofsve

        hlqbanmtjy
        tdrvxcajgnfpoke
        jtiunkpsroa
    "};
}
