use std::collections::HashMap;
use std::hash::Hash;

/// Returns map containing the frequencies of each item in the given
/// iterator
fn frequency_map<I, T>(iter: I) -> HashMap<T, usize>
where
    I: Iterator<Item = T>,
    T: Eq + Hash,
{
    let mut map = HashMap::new();
    for item in iter {
        *map.entry(item).or_insert(0) += 1;
    }
    map
}

/// Returns checksum of input box ids
pub fn star1(input: &str) -> String {
    let mut exactly_twice = 0;
    let mut exactly_thrice = 0;

    for freqs in input.lines().map(|line| frequency_map(line.trim().chars())) {
        if freqs.values().any(|&f| f == 2) {
            exactly_twice += 1;
        }
        if freqs.values().any(|&f| f == 3) {
            exactly_thrice += 1;
        }
    }

    (exactly_twice * exactly_thrice).to_string()
}

/// Returns the Hamming distance of two strings
fn hamming_distance(a: &str, b: &str) -> usize {
    assert_eq!(a.len(), b.len());
    a.chars().zip(b.chars()).filter(|(c1, c2)| c1 != c2).count()
}

/// Returns a new string containing the characters common to both
fn hamming_common(a: &str, b: &str) -> String {
    assert_eq!(a.len(), b.len());
    a.chars()
        .zip(b.chars())
        .filter(|(c1, c2)| c1 == c2)
        .map(|(c, _)| c)
        .collect()
}

/// Finds the first pair of lines with hamming distance 1 and returns
/// common chars
pub fn star2(input: &str) -> String {
    let lines: Vec<&str> = input.lines().map(|line| line.trim()).collect();

    for i in 0..lines.len() {
        for j in 0..i {
            if hamming_distance(lines[i], lines[j]) == 1 {
                return hamming_common(lines[i], lines[j]);
            }
        }
    }

    panic!("Prototype fabric not found!")
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    star_test!(example1a, star1, IN1A, "12");
    star_test!(me1, star1, ME, "5704");

    star_test!(example1b, star2, IN1B, "fgij");
    star_test!(me2, star2, ME, "umdryabviapkozistwcnihjqx");

    const IN1A: &str = indoc! {"
        abcdef
        bababc
        abbcde
        abcccd
        aabcdd
        abcdee
        ababab
    "};

    const IN1B: &str = indoc! {"
        abcde
        fghij
        klmno
        pqrst
        fguij
        axcye
        wvxyz
    "};

    const ME: &str = indoc! {"
        umdryebvlapkozostecnihjexg
        amdryebalapkozfstwcnrhjqxg
        umdcyebvlapaozfstwcnihjqgg
        ymdryrbvlapkozfstwcuihjqxg
        umdrsebvlapkozxstwcnihjqig
        umdryibvlapkohfstwcnfhjqxg
        umdryebvqapkozfatwcnihjqxs
        umzrpebvlapkozfshwcnihjqxg
        fmhryebvlapkozfstwckihjqxg
        umdryebvlahkozfstwcnizjrxg
        qmdryebvlapkozfslwcnihgqxg
        umdiyebjlapknzfstwcnihjqxg
        umdryebvlapkoqfstwcaihvqxg
        cmdryebvlapkpzfstwcnihjvxg
        umdryebvlakkozfstwcgihjixg
        umdryebvlasjozfstwcnihqqxg
        umdryebvladkozfsvwcnifjqxg
        umdrlebvlapaozfstwcniwjqxg
        umdryebvlhpkozrstwsnihjqxg
        umdryebvcapkozfqtwcnihjrxg
        ubdrykbvlapkowfstwcnihjqxg
        umdryebvldpkozfstwcnihtqsg
        umdryebvlapaozyutwcnihjqxg
        umdryibvlapkozfstdfnihjqxg
        umdryebvlapgozkstwznihjqxg
        umdrxebvlapkozfstwcngxjqxg
        umdryekvlapkozfstwclchjqxg
        nmdryebvlapkozjsewcnihjqxg
        umdryebvyapkozfstfcniheqxg
        umdfyebvlapkozfstwcnhhjpxg
        umdryelylupkozfstwcnihjqxg
        smdryebvlqpkozfstwcnihjdxg
        umdryebvlapaozfsuwcnihjqxc
        umdryebvlrzkozrstwcnihjqxg
        umdbycbvlapkojfstwcnihjqxg
        umdryebvlapkonfstwpnirjqxg
        uecryebvlapkozfstwcnihpqxg
        uqdryebvltpkozfstwcnihrqxg
        umdryebvlqsknzfstwcnihjqxg
        cmdryebvlapkocfstwcvihjqxg
        umdrkebvlapkozqsfwcnihjqxg
        umdryabveapkoifstwcnihjqxg
        ummrnehvlapkozfstwcnihjqxg
        umdryebvlxpkozfstwqnihjtxg
        umdryebvlagkozastwcnihjqxh
        umdryebvlatkozzhtwcnihjqxg
        umdryebvlcpkozfstwrnihjqvg
        umdryebvlapkozfsnwcnrhjcxg
        umdzyebvlypkozfstwcnibjqxg
        nmdryebvlvpkozbstwcnihjqxg
        uwdryebvlipkozfstwcnihvqxg
        umdraebvlavkozfstwcnihjqwg
        umdeyebvlspbozfstwcnihjqxg
        umdryxlvlapkozfstwcnihjqxu
        umdryegvlapkqzfstwcnirjqxg
        umdrupbvlapkozfstwcnihjqog
        imxryebvlapkxzfstwcnihjqxg
        umdrfebvlapkozowtwcnihjqxg
        umdreebvlapkozmstwczihjqxg
        undryebdlapkozbstwcnihjqxg
        umdryebvlapkpzfetwcnihjqxb
        ymdnyebvlapkozfstwinihjqxg
        umdryebvaapkozfstwcnihyqqg
        umdryebvlapkzzwsrwcnihjqxg
        umdrkebvlapkmzfskwcnihjqxg
        umdrmebvlapkozfsvwcnidjqxg
        umdlyehvlapkozfstwcnihjqkg
        umnryebvlrpkozfstwjnihjqxg
        uqdryebvlapxozfsawcnihjqxg
        vmdruebvlapkozfstwcnihjqqg
        umdryabviapkozistwcnihjqxg
        umdryebvlapkzzfstwfnihkqxg
        uvdryebvlapkozfsxwcuihjqxg
        umdlhebvlapkozfstwcnvhjqxg
        umdreebvlapkopfstjcnihjqxg
        umdryebvlazkomfstwynihjqxg
        kmdryebulapkoznstwcnihjqxg
        umdryebvxakkozfstwinihjqxg
        ukdryobvlapkozistwcnihjqxg
        umdryebveapkozfstwcnthjqgg
        mmdrtebvlapcozfstwcnihjqxg
        umdryebvlapkolistwnnihjqxg
        umdryebxlapkozfatwcnihjqxx
        uxdryebvlapkozfstwhniheqxg
        ufdryebvzapkozfstwcnbhjqxg
        amdryhbvlapkozfstwcnifjqxg
        umqryebvlaphozfstwcnihjqxn
        umdryebvlapkosfstfcnihjqxe
        gmkryebvlapkozfstwcnihjmxg
        umdrnebvlkpkozfstwcnihjnxg
        umdryebvrapkozfstmcndhjqxg
        umdryebvmapkozfstichihjqxg
        umdryesvnapkozestwcnihjqxg
        umeryhbvlapkozfstfcnihjqxg
        umdryedvbapkozfstwcnihqqxg
        umdryebllapzozfstwcnihjvxg
        umdcyebvlzdkozfstwcnihjqxg
        umdrybbvlapkbvfstwcnihjqxg
        umdrytbglapkozfsthcnihjqxg
        umdryebvlkpkozfsteclihjqxg
        umdntebvlapkmzfstwcnihjqxg
        lkdryebveapkozfstwcnihjqxg
        ymdryubvlapkozfstwbnihjqxg
        tmrryebvlapkozfstwcnqhjqxg
        umdryeovlaekonfstwcnihjqxg
        umiryeuvlapkozfstwcnihjwxg
        umdryebvlspvozwstwcnihjqxg
        umdrtebvlapkoznxtwcnihjqxg
        umvryebvlaphozfstwcnahjqxg
        umdryebvlapkozfstinniajqxg
        umdryebqlapkozfctwcnihjqxx
        umdryebvlapkbzfptwcnihjqvg
        umdryabviapkozistwcnihjqxd
        umdryrbvlapkezfstscnihjqxg
        umhryebvlapkozfstacnihxqxg
        umdxyelvlapkozfitwcnihjqxg
        umdryevvuapkozfstwcnihtqxg
        uydrypbvxapkozfstwcnihjqxg
        umdryebvlapkopfstwcnihzqxo
        uedryebvlapkozistwceihjqxg
        umdiyebvlapkozfgtwcnihjqxv
        ymdryebvlapkozfsticniqjqxg
        umbrkebvlapkozfslwcnihjqxg
        umdryebliapkozbstwcnihjqxg
        umvryebolapkozfstwcnihjqig
        umdryeavbackozfstwcnihjqxg
        umdryfbvlapsozfstwcnihaqxg
        umdqyebvlapkozfjtgcnihjqxg
        umdrjebvlaqkozfstwcyihjqxg
        umdryebklaqkozrstwcnihjqxg
        umdryebvpapkozfstwcpihjqjg
        uydryebhlawkozfstwcnihjqxg
        umdyyebvlapkozfstwcykhjqxg
        umdryebvlapkozfstwcnitjnxh
        umdzyebvlapkozfstwcnehyqxg
        mmcryebvlapkozfstwinihjqxg
        umdryebvlapuozfstwmvihjqxg
        umdryfbvlapkozqstwcnihjmxg
        umdryebslapsozfhtwcnihjqxg
        umdtyemvlapmozfstwcnihjqxg
        umdrxevvlapkozfytwcnihjqxg
        umdahebvlapjozfstwcnihjqxg
        umdryebvlapkozfstacnivjqxb
        umdryebvlzpkozfjtwcnihjyxg
        umdryebvlaqkozfstwcnisjqxu
        umdrydbvlapkozfsuwcnihjlxg
        umdryebvlapkomrstwcnihjqkg
        umdryebvlapcozfstmcnwhjqxg
        umdryebvlahkozfstwcibhjqxg
        gmdrzebvlapkozlstwcnihjqxg
        umdryebvlapkezfsswcnrhjqxg
        umdryebvlapkoqfitwcgihjqxg
        umdrnebvlapkozfsiwcninjqxg
        umdryebvlapkozfsrwckohjqxg
        umdryebtlapkomfstwcnihjexg
        umdryxbvlapjozfstwcnihoqxg
        umdpyebvlapkosustwcnihjqxg
        umdryebvlapkvzfawwcnihjqxg
        umhnyebvlaikozfstwcnihjqxg
        umdryebvlagkozfstvknihjqxg
        uodryebjlapkoxfstwcnihjqxg
        umdryefdlapkozfstwcnyhjqxg
        umprmebvtapkozfstwcnihjqxg
        umdhyebvlapoozfstwcnihjqgg
        uddryebvidpkozfstwcnihjqxg
        umdryebtlapkozfetwfnihjqxg
        umdbyebolapkozfstwcoihjqxg
        umdryebvlapkonfstwcnihjpxo
        umdryebvlapkohfstwcnihjqwk
        umdryebolalkkzfstwcnihjqxg
        updryebvxapkozfstwcnshjqxg
        umdryebvlapkovfktwcnuhjqxg
        umdrqrbvlppkozfstwcnihjqxg
        umdrylgvlapkozfstwrnihjqxg
        umdryebvlapkozfstxcnihbqig
        uvdryeevlappozfstwcnihjqxg
        zmdryebvlapkozfstwcnihqqxt
        umdryebvlapvozfstwenihiqxg
        umdryebvlbpkozfsgwcnihjlxg
        umdryhbvlapkozfstwcnihtqxw
        umdreecvlapkozwstwcnihjqxg
        umwryebvlapkoztsmwcnihjqxg
        ukdryebvfapkozrstwcnihjqxg
        umdrylbdlamkozfstwcnihjqxg
        umdryebvlapoozwsmwcnihjqxg
        umdryebvlapkozfqtwcnnzjqxg
        umdryekvlapktzfstwcnohjqxg
        umdryebvlapkozfstwcnihjwqo
        umdrrebflapkogfstwcnihjqxg
        umdryevvlapkozfztwctihjqxg
        umdrybbvlapkozfstwcnihxaxg
        umdryebvlapkozfsowcnphjqag
        smdryebvlapbozfitwcnihjqxg
        umdryebvtapiozfstwcnihjqxe
        umdryebjlakkozfstwccihjqxg
        umdryebvlapdozfshwckihjqxg
        umnryebvlapiozfstwcnihlqxg
        umdrycbvlapkjzfsnwcnihjqxg
        umdryebvyaprozjstwcnihjqxg
        ucdryebvlapkozfstwomihjqxg
        umdryebvlagklzfstwcnihjqyg
        umdryebvladkozfstwcnihjqjh
        umdrwebvlapkozfstwdnicjqxg
        umdryebvlapkmzfstwcniheqxr
        umdryebvlapkjzfstwcviheqxg
        umdrvebvlapkozfstwcbihjqmg
        umdrfebvlapkoffstwcnihsqxg
        umdryebvtarkazfstwcnihjqxg
        umdryebvlapkozfstwcfihjcng
        umdryebvlapkktostwcnihjqxg
        uedryeevlapkozfstwcniijqxg
        bmdryebylapkozfstwcnihjqog
        umdryebvlmpkoztstwcnihjqeg
        umdryepvlarkohfstwcnihjqxg
        uwdryebvlapklzfstzcnihjqxg
        umdryebklapkozfsswcbihjqxg
        umdtyeavlapkozfstwsnihjqxg
        umdryebvaapkozfhtfcnihjqxg
        umdrpebvlapuozfstwvnihjqxg
        umdryebvlapkozffmwcniijqxg
        uqdpyebvlapkozfstwfnihjqxg
        umdryebvlapuozdstwcnihjhxg
        tmdryhbvlapkozfptwcnihjqxg
        umdryevvmapkozfstwcnihjgxg
        umdryeuvlapmozfstwcnihjwxg
        umdryebqlzpkozfbtwcnihjqxg
        umdryebvsapkozystwcniqjqxg
        imdryebvlapkozfscwinihjqxg
        umdryebvlzpkozustwcnmhjqxg
        umdrypbvlapbozfsnwcnihjqxg
        bmdryebvlapqoznstwcnihjqxg
        umdrfebvlapaozfstwcnihxqxg
        umdiyebvxapkozfstwcnchjqxg
        umdrygbvlapkozfstwcnizjqxz
        amdryedvlapkozfstwcnihfqxg
        umdryebvvapzozfstwcnihjgxg
        undryebvlapkzzfstjcnihjqxg
        umdryvbvlapgozfrtwcnihjqxg
        umdrkebvlapkozfstwcnihihxg
        umdryebvrppkozfsowcnihjqxg
        umdryebvlapktzfsdwclihjqxg
        otdrdebvlapkozfstwcnihjqxg
        mmdryebvlazkozfxtwcnihjqxg
        umdryebvlapkozfsbwtnihjqxa
        imqryebvrapkozfstwcnihjqxg
        umdryebvlrpkozfscwcnihjqlg
        uedryebvlapkoznsvwcnihjqxg
        umdryebvlqpkozfstscnihjqxj
        umerycbvlapkozfstwcnihjqxh
        umdkykbvlapjozfstwcnihjqxg
    "};
}
