use std::collections::VecDeque;

fn parse_input(input: &str) -> (usize, Vec<u64>) {
    let (preamble_len, rem) = if let Some(s) = input.strip_prefix("# ") {
        let (a, b) = s.split_once('\n').unwrap();
        (a.parse().unwrap(), b)
    } else {
        (25, input)
    };

    (
        preamble_len,
        rem.lines().map(|l| l.parse::<u64>().unwrap()).collect(),
    )
}

fn invalid_number(preamble_len: usize, nums: &[u64]) -> u64 {
    let mut queue = VecDeque::with_capacity(preamble_len);
    for &num in nums {
        if queue.len() >= preamble_len {
            if queue
                .iter()
                .all(|&d| d > num || !queue.contains(&(num - d)))
            {
                return num;
            }
            queue.pop_front();
        }
        queue.push_back(num);
    }

    panic!("no invalid numbers found")
}

pub fn star1(input: &str) -> String {
    let (preamble_len, nums) = parse_input(input);
    invalid_number(preamble_len, &nums).to_string()
}

pub fn star2(input: &str) -> String {
    let (preamble_len, nums) = parse_input(input);
    let invalid = invalid_number(preamble_len, &nums);

    for wsize in 2..input.len() {
        for w in nums.windows(wsize) {
            if w.iter().sum::<u64>() == invalid {
                return (w.iter().min().unwrap() + w.iter().max().unwrap()).to_string();
            }
        }
    }

    panic!("no solution")
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    star_test!(example1a, star1, IN1, "127");
    star_test!(me1, star1, ME, "85848519");

    star_test!(example1b, star2, IN1, "62");
    star_test!(me2, star2, ME, "13414198");

    const IN1: &str = indoc! {"
        # 5
        35
        20
        15
        25
        47
        40
        62
        55
        65
        95
        102
        117
        150
        182
        127
        219
        299
        277
        309
        576
    "};

    const ME: &str = indoc! {"
        10
        17
        1
        42
        27
        34
        48
        38
        35
        12
        30
        16
        15
        44
        33
        47
        21
        6
        49
        31
        20
        43
        32
        19
        41
        7
        8
        9
        14
        17
        13
        59
        22
        27
        18
        23
        28
        16
        30
        24
        34
        15
        55
        25
        21
        72
        50
        58
        53
        79
        94
        26
        29
        32
        31
        33
        40
        36
        37
        38
        39
        41
        54
        44
        45
        51
        80
        73
        57
        63
        55
        61
        59
        60
        83
        70
        144
        62
        90
        152
        69
        74
        75
        76
        155
        84
        102
        89
        95
        133
        106
        114
        112
        130
        115
        116
        121
        119
        122
        131
        132
        191
        151
        143
        144
        145
        149
        159
        267
        173
        265
        207
        184
        201
        237
        220
        226
        241
        260
        274
        238
        262
        254
        263
        523
        275
        308
        470
        287
        289
        332
        322
        422
        357
        490
        439
        385
        425
        421
        446
        458
        464
        501
        527
        492
        993
        550
        517
        702
        583
        562
        905
        849
        576
        1325
        654
        747
        847
        742
        877
        806
        1269
        871
        867
        904
        922
        1591
        1103
        1252
        1828
        1067
        1521
        1487
        1444
        3315
        1230
        2428
        1318
        1323
        1396
        2333
        1489
        1548
        1613
        3697
        1710
        2222
        1738
        1826
        1989
        2174
        2170
        2482
        2297
        3960
        2385
        2807
        2548
        2714
        2553
        2626
        3866
        2641
        4704
        2885
        3037
        3102
        3161
        3323
        3536
        3448
        3564
        4291
        3815
        4163
        6638
        5367
        4682
        4850
        7865
        5599
        5101
        5189
        5194
        5179
        6198
        6666
        6484
        7265
        5922
        6139
        6263
        6609
        9047
        6984
        8454
        10793
        7978
        8497
        11798
        9532
        9861
        9783
        10449
        11387
        13247
        15055
        12906
        10373
        11101
        22767
        12061
        12402
        12185
        12531
        20822
        12872
        17510
        14962
        18771
        16432
        23245
        21330
        25045
        19315
        19393
        22510
        28493
        22980
        21474
        22434
        22558
        23632
        22775
        23162
        24246
        37917
        34346
        24716
        29304
        27834
        37906
        31394
        40789
        35203
        39412
        40645
        41873
        45333
        46190
        40867
        45414
        43908
        59449
        47150
        57138
        58835
        62574
        83320
        65113
        48962
        84697
        56110
        52550
        59228
        79779
        66597
        72261
        106009
        87057
        80057
        93340
        98005
        84775
        86281
        88017
        113747
        91058
        115559
        166836
        125825
        137325
        101512
        105072
        168109
        108190
        152040
        108660
        124811
        131489
        138858
        146654
        189847
        164832
        240149
        181357
        171056
        187793
        216850
        199248
        179075
        192570
        196130
        217071
        311486
        346189
        206584
        287735
        213262
        310564
        418754
        233471
        247518
        270347
        565408
        285512
        317710
        457220
        396146
        377487
        575221
        394377
        590749
        371645
        385659
        730240
        388700
        454102
        771992
        673394
        419846
        440055
        446733
        460780
        480989
        517865
        503818
        657157
        766501
        757304
        603222
        689355
        749132
        760345
        875463
        766022
        774359
        791491
        832425
        808546
        828755
        866579
        859901
        880626
        900835
        1574568
        921044
        978645
        964598
        984807
        1107040
        1448648
        1260379
        1292577
        1480846
        1363567
        1438487
        1534704
        1626924
        2455194
        3936040
        1565850
        1600037
        1640971
        1773144
        1695334
        1726480
        1740527
        2459491
        1821879
        1885642
        1899689
        3267895
        2625778
        2091847
        2367419
        2826229
        2656144
        2731064
        2802054
        3387729
        2973191
        3100554
        3451492
        3165887
        3292330
        3721568
        5543339
        3626169
        4267108
        3421814
        3467007
        3562406
        3707521
        4253061
        6662960
        3991536
        4459266
        5533118
        9992384
        5023563
        7013898
        5387208
        5704255
        5775245
        9155747
        6073745
        6392884
        6458217
        9337651
        6714144
        6888821
        6984220
        7699057
        9266661
        7029413
        10410771
        12517338
        10691741
        9378744
        10533011
        19148131
        10234511
        10556681
        10727818
        10798808
        11091463
        11162453
        11479500
        16308256
        12466629
        12531962
        14157274
        13172361
        17562424
        17405885
        13873041
        27938896
        14728470
        16296074
        16408157
        19911755
        19613255
        20106562
        28828036
        20767522
        20791192
        33323154
        25035494
        21526626
        21961261
        22253916
        24334814
        35909329
        28874786
        24998591
        25704323
        27045402
        27900831
        39659801
        28601511
        30169115
        31024544
        31136627
        32704231
        41558714
        42294148
        40380777
        41633188
        42317818
        56700521
        42752453
        43487887
        47230949
        47958239
        44215177
        47252507
        49333405
        50702914
        52749725
        55167706
        57214517
        73318692
        81977619
        58770626
        73085008
        82918840
        62161171
        130533209
        74262945
        98994669
        82013965
        99151403
        83951006
        85070271
        86240340
        91467684
        87703064
        112864085
        148484808
        155062627
        96585912
        173445303
        85848519
        107917431
        112382223
        148401511
        155356538
        120931797
        133033571
        135246179
        136424116
        225246308
        156276910
        158213951
        168254305
        165964971
        169021277
        197452494
        170918790
        173943404
        278836221
        173551583
        220299654
        182434431
        193765950
        198230742
        269457687
        315468002
        314697747
        271670295
        289953074
        288390109
        513636417
        268279750
        291523089
        292701026
        314490861
        322241881
        324178922
        334219276
        334986248
        532438742
        454104726
        520472623
        356377835
        554608577
        986543468
        376200381
        778283648
        522409664
        593636609
        563193384
        1507016091
        539950045
        581476163
        607191887
        680556757
        657228129
        615702011
        614942907
        842714504
        636732742
        646420803
        658398198
        937853998
        878787499
        1549736852
        810482561
        732578216
        2392451356
        939393765
        3049679485
        898610045
        1209338620
        1062359709
        1877247763
        1188668050
        1251675649
        1121426208
        1196419070
        1222134794
        1230644918
        1252434753
        1261363710
        1304819001
        2410802844
        1283153545
        1390976414
        1468880759
        1671971981
        1611365715
        2360219413
        1543060777
        1985012969
        1960969754
        1838003810
        2860640031
        2883623014
        2183785917
        2251027759
        2535463919
        3090438563
        4144755671
        2859857173
        4845653000
        2535588298
        3796827629
        2513798463
        4089031569
        2916184716
        3002342129
        3121157355
        2934037191
        5043643090
        5276404129
        4664218132
        3381064587
        4697584380
        3798973564
        6332415927
        5564850504
        4434813676
        4719249836
        5429983179
        4786491678
        7595801193
        5049386761
        5373655636
        5395445471
        5447835654
        6715158280
        7943781642
        10423042397
        5936379320
        5850221907
        7907649033
        7368850867
        6315101778
        10633963700
        10114695307
        7815878263
        7180038151
        10825428650
        8233787240
        9154063512
        17603080548
        9484200437
        13116417471
        15184729130
        9835878439
        10899608668
        11298057561
        10769101107
        13339227113
        12627873805
        23453302455
        11786601227
        13666100170
        12165323685
        14548889018
        13495139929
        13683952645
        38002191473
        20399110925
        27888116131
        24982010206
        17015916590
        17387850752
        17717987677
        18989941951
        19320078876
        20253301544
        20604979546
        31057214790
        22001202124
        22067158668
        26293973975
        22934424792
        61455493928
        33919401714
        23951924912
        25281741156
        25660463614
        49233666068
        41339775664
        27179092574
        30699869235
        38117098602
        34403767342
        39719189801
        46499171450
        80775572804
        35105838429
        36707929628
        38310020827
        39573380420
        40858281090
        42606181670
        44935626916
        52701071359
        76652996271
        46886349704
        73222937031
        50942204770
        49612388526
        74515407224
        52460833730
        65805707664
        57878961809
        61582859916
        62284931003
        65103636577
        115179340146
        69509605771
        74825028230
        109621245653
        71813768057
        124127795750
        104765311513
        77883401247
        80431661510
        150479526743
        89492531374
        91821976620
        121970439501
        96498738230
        97828554474
        100554593296
        103403038500
        137619475721
        177464271149
        114043693646
        119461821725
        120163892812
        123867790919
        127388567580
        134613242348
        144334634001
        166008344001
        146638796287
        158315062757
        149697169304
        259589915222
        235448030195
        167375932621
        169924192884
        242134332313
        224422384215
        188320714850
        194327292704
        197053331526
        221696345393
        203957631796
        281252038635
        315705513305
        290973430288
        362298225125
        488026761814
        244031683731
        282182853676
        262001809928
        278947876349
        730161094127
        296335965591
        304953859044
        308012232061
        430949207939
        337300125505
        355696647471
        358244907734
        366977524410
        382648007554
        385374046376
        391380624230
        398284924500
        401010963322
        425653977189
        465959441724
        620246717662
        552975240216
        506033493659
        522979560080
        692951017867
        526214537407
        617698457399
        540949686277
        575283841940
        601289824635
        604348197652
        612966091105
        674989756471
        1227964996687
        692996772976
        1100643733660
        740892915288
        941960649599
        768022053930
        1803248838627
        1081317335599
        939234610777
        907044456981
        1464940209679
        971992935383
        1029013053739
        1032248031066
        1127327757732
        1049194097487
        1067164223684
        1143912994806
        1482328298921
        1188249933045
        1176573666575
        1205638022287
        1279337954123
        2742483449404
        1367986529447
        1461018826906
        1680127526065
        1508914969218
        1911227546160
        2448149579995
        1956238554468
        1846279067758
        1879037392364
        1936057510720
        2978391769844
        2510212924393
        2001005989122
        2061261084805
        3509920958340
        2116358321171
        2193107092293
        2243737890259
        2320486661381
        2364823599620
        4072596875639
        2455911620698
        2484975976410
        3472445046416
        2829005356353
        3324225083915
        5388958350704
        3636366080533
        4199524053745
        3725316460122
        4294428647753
        3782336578478
        4062267073927
        4300881110340
        3937063499842
        4117364310293
        8261791127672
        4177619405976
        4608561489879
        4481181920791
        4360096211430
        5517332176208
        4564224551640
        7625106194255
        4820735220318
        9384959771958
        4940887597108
        7507653038600
        7028529410098
        6554321816475
        8858653199393
        8757798720160
        7361682540655
        7719400078320
        7662379959964
        9235316244861
        11970244030534
        7999330573769
        12870352617551
        13238980640951
        8294983716269
        8537715617406
        8658801326767
        9761622817426
        8841278132221
        8924320763070
        11925907092295
        9505112148748
        15381780038284
        11495209413583
        16202960672876
        11969417007206
        16059433965223
        17765598895291
        19157589373547
        25764929469060
        15024062500619
        15081082618975
        18996939062287
        20629045357301
        16294314290038
        16537046191175
        27894415118170
        17462036380476
        28172377680082
        43196440180701
        24005403382045
        17500079458988
        21687529909721
        20850227855365
        18429432911818
        21000321562331
        21474529155954
        28506463198381
        23464626420789
        26993479507825
        27050499626181
        31140516584198
        30105145119594
        31318376790657
        34723747201856
        44466691970120
        41505482841033
        40116962821539
        45634414060558
        42537757765086
        33999082571651
        34962115839464
        35891469292294
        77261504966942
        35929512370806
        58729150583901
        42324757011319
        53005906700378
        39279660767183
        52615045740152
        42474850718285
        44939155576743
        55556962824562
        117378467788481
        54043979134006
        87263912588062
        61245661703792
        67247889161463
        65317459362308
        101266908348987
        80868667947549
        75209173137989
        77286872850783
        68961198411115
        69890551863945
        106184817280535
        75171130059477
        103139358453757
    "};
}
