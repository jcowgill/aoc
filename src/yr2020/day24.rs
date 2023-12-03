use std::collections::HashSet;

use itertools::Itertools;
use nalgebra::Vector2;
use num::Zero;

const NEIGHBORS: [Vector2<i32>; 6] = [
    Vector2::new(-1, 0),
    Vector2::new(1, 0),
    Vector2::new(0, 1),
    Vector2::new(1, 1),
    Vector2::new(-1, -1),
    Vector2::new(0, -1),
];

fn parse_tile(line: &str) -> Vector2<i32> {
    line.chars()
        .fold((Vector2::zero(), false), |(acc, comp), c| match (comp, c) {
            (_, 'e') => (acc + Vector2::new(1, 0), false),
            (true, 'w') => (acc, false),
            (false, 'w') => (acc - Vector2::new(1, 0), false),
            (false, 's') => (acc + Vector2::new(0, 1), true),
            (false, 'n') => (acc - Vector2::new(1, 1), true),
            _ => panic!("parse error"),
        })
        .0
}

fn parse_input(input: &str) -> impl Iterator<Item = Vector2<i32>> {
    input
        .lines()
        .map(parse_tile)
        .counts()
        .into_iter()
        .filter(|&(_, c)| c % 2 == 1)
        .map(|(pos, _)| pos)
}

pub fn star1(input: &str) -> String {
    parse_input(input).count().to_string()
}

pub fn star2(input: &str) -> String {
    let mut state: HashSet<_> = parse_input(input).collect();
    for _ in 0..100 {
        let mut white_check = HashSet::new();
        let mut new_state: HashSet<_> = state
            .iter()
            .copied()
            .filter(|&tile| {
                let mut neighbor_count = 0;
                for off in NEIGHBORS.iter() {
                    if state.contains(&(tile + off)) {
                        neighbor_count += 1;
                    } else {
                        white_check.insert(tile + off);
                    }
                }

                neighbor_count == 1 || neighbor_count == 2
            })
            .collect();

        new_state.extend(white_check.into_iter().filter(|tile| {
            NEIGHBORS
                .iter()
                .filter(|&n| state.contains(&(tile + n)))
                .count()
                == 2
        }));
        state = new_state;
    }
    state.len().to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse_tile() {
        assert_eq!(parse_tile("w"), Vector2::new(-1, 0));
        assert_eq!(parse_tile("e"), Vector2::new(1, 0));
        assert_eq!(parse_tile("sw"), Vector2::new(0, 1));
        assert_eq!(parse_tile("se"), Vector2::new(1, 1));
        assert_eq!(parse_tile("nw"), Vector2::new(-1, -1));
        assert_eq!(parse_tile("ne"), Vector2::new(0, -1));
        assert_eq!(parse_tile("seswneswswsenwwnwse"), Vector2::new(0, 3));
    }

    star_test!(example1a, star1, IN1, "10");
    star_test!(me1, star1, ME, "473");

    star_test!(example1b, star2, IN1, "2208");
    star_test!(me2, star2, ME, "4070");

    const IN1: &str = indoc! {"
        sesenwnenenewseeswwswswwnenewsewsw
        neeenesenwnwwswnenewnwwsewnenwseswesw
        seswneswswsenwwnwse
        nwnwneseeswswnenewneswwnewseswneseene
        swweswneswnenwsewnwneneseenw
        eesenwseswswnenwswnwnwsewwnwsene
        sewnenenenesenwsewnenwwwse
        wenwwweseeeweswwwnwwe
        wsweesenenewnwwnwsenewsenwwsesesenwne
        neeswseenwwswnwswswnw
        nenwswwsewswnenenewsenwsenwnesesenew
        enewnwewneswsewnwswenweswnenwsenwsw
        sweneswneswneneenwnewenewwneswswnese
        swwesenesewenwneswnwwneseswwne
        enesenwswwswneneswsenwnewswseenwsese
        wnwnesenesenenwwnenwsewesewsesesew
        nenewswnwewswnenesenwnesewesw
        eneswnwswnwsenenwnwnwwseeswneewsenese
        neswnwewnwnwseenwseesewsenwsweewe
        wseweeenwnesenwwwswnew
    "};

    const ME: &str = indoc! {"
        neeneneneswneneee
        eeeswneseseeseeeseeeeesesee
        swswsweswwswswswnw
        neseewwswwneswnewnewwswswwswwse
        wswsweenweesenweeeeeenweeee
        neswwwswwwnewwwwwwwwsewwswsww
        neseeeswesweenweeeeeeeeee
        neeeneneeneeswneeneneneene
        ewwsenwwenwnwswnwswwnewwwsewne
        sesesesesesewseseweeeeneweseeww
        neeeswnwneswneneneweeneeneswnwnesw
        neseswnwnenenewneneesenenenenenenenenenene
        eneeeeseseeseesewseseseeeseee
        enenewseeenewnweseewnwneseweesw
        sesesesesenwseseseseseseseswseseee
        nenenenenwneeesweseneeeeeenenenwe
        neeenenwseswsenewseswenwewsww
        neeenesweswwneeneeeeeeeeenwe
        swseneswewwsewsweeeseeswwsesww
        nenwwnwnwswwnwwnwnwnwnwnweswswnenww
        swswseseswnwswswswseswseeswswswswnwswsesenw
        nwnewwswnwesewnwwwsenwwnewewe
        newwsewwsenwswwswswswswseswwnwwwsw
        swswsweswseswswwswswnwseswswswswswswswsw
        nenwnwnwnenwnwnwnenenwnenwneswnesenwenw
        nwswnwwnwnwwnwnenwwse
        neesweeeesewewneese
        nwneweswwwswswwswwswswsewsw
        wwwwsenwwnwneswsewsewnenewnwwseswne
        eeeeeneneneneweneeeeeweswnene
        enenwneneeneswnewnweewseswneswnew
        swsesesesesenweswseseseswswseswswsesese
        nweeeneweewsw
        nwswswneswswswneswsewswswswswsesw
        swenwwnenwnwnwswnwsesenenw
        swswswnwneswswswseswswswswswswswseswse
        sweseseseeseseweenese
        wnwnwnwnwnenwnwswnwnwenwsenwenwnwnene
        eeenweeeeeeeeeeesweeee
        nesenwswnwseswneewnenwenwsenwenwewnw
        eeeeeewesenweeseeeeswe
        swwwswneswsewwswsw
        nwnenwnenwnenwnenesenenwnwnwnwnwnwwnw
        newnesewnwnenwneneenwnenenwnenenenenesw
        eeeeeeeeeeenweeeseeeesw
        swnwnwwnwwwnwwewnwwsewwwwnwnew
        nwnwnwenenwswnwnwnwnwwnwnenwswewnwnw
        swseenwnwswnwnenesenenwnwneeswneenwwnw
        enwwseeswwenwswswesw
        enwseseeeeeeenweeeseeseswseee
        eseswseseswseneenwwneeswwwse
        swswswwesenwswswswswswswswswswsenesesw
        neswenwswswswnwnwsweswnweswswsweswsw
        newsenenwswesewenesewwseeesenwesew
        seswswswswswswneseseswweswseswsewseswnw
        wnwwwwnwnwnwwnenwwewnwnwswnwnww
        newwwwwwsenwwwewwwnwwwww
        seseseeeswseseswseswswwsewseseswswse
        eenenwswnenwneswnesenwnenenenwnwnenenewsw
        swnwewswwewnwnwswwwnwnwewwwse
        nwenewneneneewnene
        wwwwwswwswenwwsewwwwwwswsww
        enenewswnwneneneneseneneneeneneeneene
        wnwswswnewwwswnewwwsewwseswnew
        nenenwwsenweswnwswswnwswnwnenwnwne
        seeenwesesesewseseeseseeesesesese
        eeneeeeeeeneenwenweswsweeene
        wneneneswseneneneneneneeneenenwsenwnene
        seseseswsenewesesesenwneseseeseseesese
        wsesenesenwsesweseswnwsesweseseesenwsese
        wwwwnwwnewewwswsenwseseewnwnenwnw
        wnenenenenenenenenwnenenwnenenesenenene
        neeewnwwneneesweneeweseswnwneswsw
        neesewwneenwnwwwwseswwnwwwwswww
        esweeeeeeneneenwe
        eseeseeeeenwswseeseneeseseseswee
        neneenewnweneseswnenenwneneenwwseswne
        senwwnesenwswnwneeewswnenwnwwnwnwsene
        eeeeseeeeeneewwneeweneeenene
        nwnenesenwwwseswseswnwwesesenwwnene
        swseneswswswswswsewswnwneseswseneeswsesw
        nwnwwnwswnwnwwnwnwenenwswwnwnwnwnwenw
        seswswseeesesenwnweeseeeneeenewese
        nwsenenesenwwnenenwnwneswnenenenenenene
        swsewnwwwseswwwsewneneswneenene
        nwenenwnwnenwsenwnenwnwswnwnenenenwnenwnw
        swwneneneneneenweeeneenesweenesenesw
        swnwnwswnwwwswsenewsewswsewswswseswsww
        enenenenesenenenenenenenenenenwswnenewne
        newewesesewseseseseeeseseeesesese
        nenwwwnwsewswwwwewsewwwwwsew
        enwewnwwwwnwwnwnwnwnwnwnwwwnwnwswnw
        neweeswesweeswewneseewnw
        swswswswsewesewswswswswneseswswswseswse
        swneeewseneeeeeswneewwneseenwe
        wwwwwwwwwneewwsewwwwweww
        eeeesweneenweeeneneeweeenee
        wnenwneseeenwseeneneewneseswwnene
        wwwnwewnwnwnwnwnwewneenwnwnwswswswnw
        enenenenwesesewnwneeeswneneeneenenene
        nwnewseneseenwwwseswneswseneeeswnew
        swsewwsweseswneseseeseswneseneswswse
        swswswwswswnewsesesweswswswswseneswnesww
        enwwnwwnwwnewnwswswseeswnenwwnwwnwse
        sesenwnenwnwnenwnwnwswnwnenwnwnenenwnwnwsw
        seeneswswwswwswswwwwswwwswnenesw
        swwwwswneseswwwwswwnwswswswswwswsew
        swseeenenwnwswenwneneenwnwnenwnwsewsw
        wswswswwwswwwswswswneswswswseswswsw
        neswnewnenenwnenewenenwneneeenenenw
        eswneneneneneenenwneneeeneenenewe
        enenenenenenwneneeneeeeeeenese
        seeseewswwseseneesenwneseseswsewswsw
        ewwewswwswwswswwewwnwswwsww
        swwswswswswswswswswswne
        nwnenwnenenenwnwnwswnwne
        nwnenwnwswnwwewnwnwswnwsenwwnwnwnwnw
        nwnwnenwnwnwnwwnwnwnwswnwswnwnwewnwnw
        eseneseeswenwneeneneneneewnenwnenene
        swwnwwswwswswwswwswswswsewwwwswne
        swsewsenenweseesweee
        nwnwnwnwsenwnwnwnenenwnewnwnwnenwnwnwnw
        nwsweeeseseeeeseeeeseneeswene
        neenwneneswnwnesewewswneswnwnwneneswe
        esenenwnwnewseneswnwsewswwnwsenwsewswse
        nwwwweneeewwwesesenwneneswswswnw
        nwnwsenwnewseswnwesenewnenenwnenenenwe
        wnwwnwnenwnwsenwnewwnwwsewwnwwwwe
        wwwewwswwswwwswwwwswsw
        swswwswswseswswswswseswswswswnesw
        wwwwwwwwwwswwewwnwwwww
        nwnenwnwnwnwnwsenenwnwnwnwnwnw
        swnwswswswswwseeeswswswswswswsewnwnwsw
        nenwnenwnenewnwnenwnwnenwnwseenenwnenwne
        seewseeseeeeseseeeweneseseseese
        seenesesesesesesesewseseseseesesesese
        nenenenenwneneneenweneneneesenenenesene
        swswneneeseswwswnwseswswwswswseswsesw
        nwswsenwneenwnwnwnwsenwnwwsenwenewnwsw
        nweseseseswswswswswswswswswsw
        seseswseseseswsenwseseseseesesesesesese
        wwwwwwsenwwnewewnwwsewswnesww
        nwseswswnwseswswsewseseeseswswesesesesw
        seswseseenwswseseseenwswseswseesewswse
        nwwnwnwswnwwnwnwnwnwesenwnwnwwwwnww
        neneswseneneneswwnenweneeneneenesenww
        swswswswneseswswseswswswwswswsesweswswse
        swneswseswswswswswswswswswseswswseseswsene
        swswneneswswwneswwwswswswwswnweswsene
        nenwnwnwwnwnwnwnwnwnwnwnwnwenwnwnwswnwnw
        wwneswwwwwwwwewwwnwwwsewww
        neneeswnwneneeneeee
        swesweswswnesweswsesewwnenwnwnewnwnee
        nwwwwewwwwwwwnwwseswwwswwe
        sweenwseeseneweseseeewnwseeseenenw
        eeeeeenweseeneeeeeeeenesw
        nwnwnwswnwnwnwswnwnwnwnweeenwnwswnenw
        eneeneeeeweeeeeneeeeneewsee
        swnenwnweneneenwnwnwnwswnwnenenenwnwnwnw
        nenwnenwnwnenwnenesenenwnwnwnenw
        neeswnweswswseewwnwwweswwwwwne
        nwseeeeeeweseeeeseeeeeeee
        sweseneneseeseeseeneeeeeewsweew
        wnwwnewswwwswwwwewwwwnwww
        wswnwsweswwneseswnwswswwseswsweswnwwsw
        nweswneeweeeeswsenwew
        neeswneneenenweeneneneeneseenesenwe
        nwnwnwnwnwnwnwnwnwnwnenenwnwnwnwsenwnwse
        seswseseswseseswseseseseswsesenw
        nwnenwewnesenwnwnwnwnewnenwnenwne
        swswswswesweswswswwswswswswswswwswsw
        nwnweeeeeeeeeseeeeeeeese
        wswnwnwswswswwwwswswswswswwwseswswse
        seneswwseswsesewsenwsenenesenwseseese
        neneneswneswenenweeneenenenenwnwswnene
        newwwnwwwwnwsewwwwseswwwwse
        swsenwenwnwnwnwnewnwswenwwesw
        newwnwnwnewsewwswwwwnwnwnwswww
        eseswsenenesesenwwseswseseseeseesese
        nwwenwenwsenwnewnenenwnwwnweneswnwne
        neswneneseswseeseseswswsenesenwswswsww
        neneeeswseneswnwnwwenenenwseswwwe
        nwswseseseeeseseseseesesesesese
        wswwnwswswnenwneswnweswseenwewwwse
        swseseswseneswswseswswseseswswnwswseswswsw
        sesesenwseswseseseseseeesesesesesesese
        nenenenenenenewwneeeneneseneeneneene
        wnwswwewswnwnwsenenwesesweeswsene
        enenenwneneneeneeeneeseswneenenenene
        seenwseseeneeswwneseeeseswwesenww
        seseswnwwsewnewswnwseswneswswswesweenw
        nenenenenwnwnenenwwnwenwnenw
        nwwwwnwneeeeneneneseswsenewesenw
        swswswneneseswwwwswswswswswnwswswswsw
        nesesenwswswseswnenwsewswseseseseswnese
        neeeenenweswenweneneeneeneeeeese
        seeweneseeneseseseswseewsenenwe
        seseseseseseseseseseseseseseswsenwsw
        wswswswswsweswwwwseweswwwswswne
        nwseeswneswneswwnenwseswenewseswwsw
        neeweenweeeeneeeeeesweeeee
        swwswwswwneweswwswswwwswsewwsw
        sesweswwswenwweswswesenwnwnwnweswse
        nwneenenwnewenesw
        wnwseewwenwwwwesewwwnwewwnw
        eeeneesenenenweswene
        seenwseseseeseseewsweesesesewsese
        wswsweswswseseseseseseseseswswsesesenwse
        wweeeeeneeseeeeeeeeesese
        swseswseseseeswsenenwseswseseseseseswsw
        wwwewwwwwnwswwwnwnwnwwwewnw
        wwnwnenwseneeswswswewewswswswwenw
        neswswswswswseseswseseseseswseesenwswsw
        neeswnewnwwswsenwnwnenwnwsenesewene
        nenwseeswwswseswsesesweseswswswswseswsw
        nwwwwseneneswseseneeseswswnwnwseene
        seesesesesenwseweseeseseeseseswseese
        ewwwswswwsewnwwwwswnewneeswsw
        eeneneneneneneeneenenewnenenenesewne
        enweeswnwsweseneeneenwsenewnwswswne
        sesenwseewsesenweseeeseseeeseeenee
        nwnwnwnenwnwnwswnwnwnwnenwnwenwnenwswnwnwnw
        seswnwsenwnenwnwnenwnenwnwnenw
        nenwnwnwsenwnwnwnwnwnwnwnesenwswnwnwnwnwne
        swnwnenwnenenwnwnwnwnenwweenwwenwnwnwnw
        senwnwnwwnwwwwnwwnwnwnwswnenwnwnwnewnw
        neswnenwswsesewnwnewsewwnewnesenewnwnw
        wwwsenwnewwwwewwnwwwwwsww
        enwneeneneswswnesw
        enweesesewsenesesw
        eeswewneesenenweneneneneneeseee
        nwnwseneswnwnwwewwwwsenwwwnwww
        swwneswneswwwswsweeswswswwswswseswnw
        ewwwwwsewswwwwwwenwwwneww
        swneswseeswswswwswnwsewwswswswswwswwsw
        sweswwswseneseeewswwnenwnwneewwsw
        swwswswseswswswwswswswswswswswwswswne
        nwwwswwwwwwswwweweswneswwew
        neneneswsenenenwnenenwneswnweweenwsww
        senwswnewwseseeseenewseseneeeswene
        nenwsewwsenewswseenesewsenenewnwe
        swswswswwwwwewswwnewwswwwwww
        swsweswsenwnweenenwe
        wnwsesesesewseswnesenesenwswsesesesesesese
        eeewseeeseeweeeseeewwenee
        eeeeeneeeeeeeweseeewwe
        neneneneenenwneneneneneneneneneneneswne
        seeseeseesesesenweseeeswneenwnwee
        seeeseenwswneeeewswewneseeew
        nwseenwnwnwnwnwswnwnewwseenwnwnwsenw
        esweeeweweneeseeneseee
        ewswnwswswwnewswwsweseswweswswww
        sweseeneswwwwnwnenwsenwwsweseswe
        eweeseseeswnwneneeeneew
        swnweseeeeeneweeeeeeweene
        wwnwwweswwnenwwnwnwswwwewwsw
        neneeeweenewneneeeeneeeneswe
        swwwesewnwsenenwnewnwneseenwwswnew
        senwnwnenwnwnewnwneneswwneeneneenewse
        nwwwnwwwwwnwwwwnwsewnwwwsew
        wwwswwswwwswwwwseneneeswwwww
        neswswseswswnwseswseseswswwseswsesesesese
        nwsewnwnwwnewnwnwnenwneenwnwseneenw
        seseseseseseseesenewsesesesenesewsese
        newwnwnwnwwnwwwnwsenwnwnwwwsenww
        swswenwnewswswswneweweswswswswnwsesese
        seeseswseseseseesesesesesewseseswnwse
        nenewneneneswnenwsenenenenewneenenenee
        sewseneseswneseneseeewswnwswsewswwsw
        eeswwswswswswswswswswswsenwswswnwwswsw
        seneenwswseseeeneeeeeswswsenewe
        nenenwnwnenwenenenwnewnwwsenenenwnwne
        nenwenwswenwnenwnwsenwwnwenwnwswnwnw
        wwwsenwwswnwwwewenewwwwwnwnw
        nwnwnwnesenwnwnwsenwnwnwswnwnwnwnwnwnwnw
        swswswswseswsweswswnwswwswseswneswswswwsw
        eswnenwneenwewnwnwnww
        seseseseeseneseseseseseseweeseseseswnw
        enwwnwnwswnwnwnwnwwwnwnwnwwnw
        swsewseneswneewwwwnwnwnwnwwseswsesww
        nenesenwnenenenenenenenenenewneneswnee
        nwweswwsweswswswneswswnwswswswswww
        neeneeneneneeneenenenenenenewsenene
        nwswswswsweswwswswswwswewwwswswsw
        swwswwwwwswwseswswswnwweneswswsw
        wswnwneeneswnwwswwnwwsenenewwnwww
        nenenenenwwenwnwnenwenenww
        seseneseseswwneseeseewseseesesenwse
        nwenwsenwnewneswnwnenwnwnwneenwnwnwnenw
        nwnwnenwnwnwnwnweswnenwsenenwnwnwnenwnwwne
        nwnenwenwswnwenwwswnenwnenwnwnwnwewnee
        wwnwenwnwwnwwwnenwnwwnwnwnwesww
        neseeneeeneeeeewneneneneeenee
        swseseswnwneeseseneswswwswswnwswse
        swnwswseswswwswswswneswsw
        swwweeeeswnwneneeewnweeeeee
        enenwnenwsenwswnwnwnenenwnewnesenwnwsenwnw
        swewswwneneseeeenwneenenwnenwseee
        seseseseseenwseneseseeseswseeseswnwse
        seneeeseseeeseswnwswsesesenwwwseenw
        wwnewwwwwwwsenewwwwwwswwnww
        nwswwwswswwnwsweswswsweswswsesweswsw
        swswnwnwnwnwwenwnwnwenenwnenw
        seeseseseseweseseeeseseesee
        wnwnwnwsewnwnwnwnw
        nwnenenwnwneswnwnwnwnwnwnwnwneenwnwwnw
        sweeseneweseneswenwsesenweswsenwnee
        wswnwnweseseewenenenwenenesweene
        nwnesenwnwseesewneswwnesweswnwnwsenww
        sewneswwwnwewnenwnwnwwwswnwwnwswe
        eeesweseneeweenweeswenweneewe
        neneneeneeeeneneeesweeenwenene
        eeneneeeneeeenwneeswweneenenee
        sesweeneeseeeweeeenweeeese
        swnwswneewwwnwwsewwnwnwwnwnwwenwnw
        eeeeesweseeeseswenwnwenwnwee
        sewwnewwswwwwswnesewswsw
        nesewseewseswsesewwswweseseneswswe
        newneenwsewnenenenenenwne
        sewsenwseeseesewneswwwnwsenweswnene
        swswswwwwwswwwswneww
        wnewwwwwwwwwwwwwwwswsw
        swwwnewwwwswsenwnwwwnenwseenww
        neeenenwweneeneeneneneneneeneneswse
        nwsenwnwnwnwnwswwnenenwsewnwswnwenwse
        nwesenwnewwnwwwnwwwwnwwswnwnww
        neswwsenwswnwwwswswseswswwsw
        neneeneswneneneenewnewseenenenenene
        seswseswsesesenesesesesesese
        wwwnwwwwweswewswnw
        seneseswsesewseseneesewwseswwswsenese
        enenwnwnwwnwswnwnwnwswnwenwnwswenwnw
        neswwsenwswsweneswswnwswseswnesenwnesw
        senwwnweswseswneswneewneseesesesesene
        nwenwwnesenwnwwswenwnenwswnenwnenwse
        weneeseeneeswweswnweswenwewswnw
        eswnwewwnwnwswwswwswene
        swwnwneseseseneneseswewswwsenewseee
        seswwnwwwswseeswwswnwwsewwenene
        neeneneneneneenenenenewsenenenwswnene
        eesesweeeeeeeneenweeweseww
        seswswswnwswswsweswswnwswswswseswswswsesw
        nenwnwnwnenenwnwnwnwwnwsenesenwnenwnew
        neneswneneneenesenwnenenwnenwwnewnesene
        wnwnwewnwwswsenewenenwnwswwwwwwnw
        weneswnenwnenwnenenenenwnenesewnwnene
        eneneewswneenenwswnenweeeeneesee
        neswseseswseswwseseseseswnwnwnwswnwsesw
        nenenenenenwenenewnwnwwsenwnenenenene
        weseswnewnenwwenwnwnwewnenenwsenw
        swenwnewewwwnwneswewwsewnwsewnw
        seseseseneswseseswseswseseseseswnesewsesese
        nwnwnwnenwnwnwnwnenwnwenwnwnwnwnwnwnwsww
        nwesweeeneeeeseeeeeneeeeee
        esweseswnenwsenwnw
        swneswwswswswewswwnwnewswsenenenwsw
        enenwnwswnwnwnwsenenwnwnenenwnwnenwnwsenw
        neswneswswswsenwseseneswseseneswseswswse
        ewneneenwnenwneneswnwnw
        swsenwsenesweswwnwsenesewswswwseswswsesw
        swwswwswwwswwswswwswnwswswswe
        nwwnewwewwwwswneswwswswseswswsw
        seeseswwnesesenwsenwswweseswenwnese
        nwnenwneneswswwnenesenenwnenenwnwnenenene
        nenwnenwnwswnwnenwnewneneneneneneneenwne
        eeeenenenenwneneeeeeeneewsese
        neseswsesesesewsenewsesenwseneswnesesw
        eswnwswsweeswnwnwnwwswwwswwesesesw
        wnwwsenwwwenwswswswsewwenewnwwnw
        seweeeeeeeneeeeseenwswnenwee
        nwsenwwwwwwwwwwwwnwwwneww
        swneswswswswwswsweswwwswnenwwswseswsw
        swneeeeeeesweenwneneeeeeee
        neewsewswseswswsenwswswneswwswenesw
        wnwnwsewwswnwwwwnwnwnwnenw
        neseseeseswsesesewswsenwswseseswseswswnw
        nwenenwnwnwnwnwnwnwwswnwnwnwnwnwsenwe
        neneeewnewnweseenenwswwnesewsee
        sesewseswneswswseseswnwswswneswneswswe
        seseseswseswsesesesesesesesenesesewsesenwse
        enwwwswswwwnwsesewwewwwnwew
        nwsenwnweseswnwnenwnwnwnwnwnwwwswnwnw
        nwseseeneseesweseseseseseseseeswnwsese
        seswswseswneseswswneswswswseswswswswswswnw
        swswsenwseswswseswswswseseneseseseswneswsw
        seseeseswswswswswseswnwswswnwsweswsesw
        wneseswseseneseesenenwwnesewseswswese
        neneswnenwnenenenenwneesenwnenenenwnwnene
        sweneseweseeeeeseeeeeenweeee
        enwnwnwnwnwnewenwsenenenwnenenwnwswne
        weeeeneeeseseeeeseseesewewee
        wsenwnwsewwwwwwwnwwnwnewnwww
        nwnwewnwnewnwsewnwwnwnwnwnwwwwwse
        swswswwwwswwswswsewnewwwswwneww
        nenwswewwswneswwswswwweswswwww
        neneneswenenenwneneweewnwnenenenene
        swswseneseseseseswseswsese
        nwnesenwnenenewneseswwwnwenenwseene
        wsesewseseseseseseesesesesesenesesese
        seseweswsesesesesesesese
        nwnwnwnenwsenwnwnwneswnenwnenwenwnenwnw
        seseswewseneseseseseseseswswsesewnese
        seswseswsesenesesesesesenwswseseseswsese
        sweenwseeeeeeeeenweeeeeeswe
        eenwwnewswsesewewswswseneswnewww
        wwwwwwwwwnwwsewww
        wnwnenwswenwnwnwnwse
        swneseneneswneneesewwnenene
        wwswewwwwwwwwnwwnwwwswwswwse
        sesesewseeseswsewseswnesesesesesesene
        swwwwwwwwwswwwwnewwwsenesw
        swswseswswnwswswswseseeswsw
        newwswsenwnewwsewseseswenwneeswwnw
        nwsweswswswswswswnwswswswseswswswswsesw
        nwswneseswseswsenewenewwneseewsenw
        sewseneseneseseseseesewseseneswsesesese
        wnwnwenwnwnwnwnwnwnwnwsenwnwneswnwnwne
        sewneewwsesenwnwnewwswwwwneswwsw
        wwwnwsewwwwswwwwwnwwwwwe
        ewneenwseneneewswneswneenenwenee
        swwswswswswswswneswswseswswweswswswswswsw
        nwswnwswnenenwswneneneneeneene
        esenenenwneneneswenenewneneneenenenenene
        swswsweseewweswwsesw
        enwewwnwnwsenwneseswwesewnewenew
        eeeseseseeneesweeeeeee
        ewenwwwswwseswnwswnwwswsweswwww
        wnwweeneenewnenenwnewswneeesew
        nenwnenwnenenwswnenwnewnwnenwenwnwnene
        neneneseneeeneneesewneneeew
        neneneneneneneenenwneneneneswneenesenene
        nenwnenenwnwnwenenwnwneneneenewswnenenw
        swwwswwwwewswswswwwswnwseneswswww
        neenenewneneseneneesenewenenenenwnwnese
        wnwwweswswswwswweswswwswwswwwwnw
        wswwwewwswwswwwwwwnwwwsw
        senwesewenwseseswesesesenwswnwsesee
        nwnwnwnwnwneneswnwsenenwnenwsenenwnwnw
        seewwwwwnwnwwwwnwneswwwnwswnww
        seeweseeeseeeeseneeswseeseene
        enesweeweeeneesweneseeeeneee
        senwseweswneeesweeneseweeesenesew
        swswseswseswswsweneswswswwenwwneswsw
        swswswwewnwswswswswewwswswswswswswsw
        swswswseswsesewseswseenwseswseswnwnesese
        ewswswswswswneswswnwwenwsesenwswwsee
        swnenwweesenwswnewneneneswnwnwnwswnenw
        nwnwnwnwnenwswnenwnenwenwnwwnwnenenwnwne
        neneseenewnenenenewnenenenesenenenenenee
        nwnwnwswnenwnwenwsenenwnwnenwnwnenenwnwnene
        nenenenenewnenewsenenenenesenenenenene
        swswswswseseesesesenwswswsesenwseseseese
        neneneweeeenesesewneweneeeneene
        wwnwewswsewwnewwnewwwww
        seseeeeseeeeeesewseee
        senesesesweseswenwnesesweseneeseswe
        wweeswswnwswwsesesewesenwseewee
        nwwnwwnwnwnwnwwwnwnwswenwwnwnw
        wwswnewwwwwwwwwwsewwwww
        neneeneneneneneneeneenenesenenenewne
        neweseeeeseeeeeeeswseeseesee
        nweneneswnwnwnwnenesenesewnwnwneenwnwnw
        neneseswnenenwnwswwnwnenwnewseneseneneenw
        nenenewesewseweeeswswneswneseee
        swswseseeseseeeseesesesenwsesesesenwsese
        seneeneenweeneneenesweeewneee
        wwwewsenewwswneeneeswseswswenw
        nwnwwnewsenewswnesenwwwnwenwwsenwnw
        seeenwneewnweswnweewseewseese
        sewnwswwwwnewwnewnwwwsewwww
        swsenwnwewnwnwnwnwnwnwwnwnw
        enwesweeseeneswneeseewsewwnee
        wnwweneswswwsenwwwnwwwwnwnwww
        wwwswwwwwwnewswwwweswwww
        swswnewswswsweswswwwswseswswwwsw
        wwwnwwewnwwwwnwwwwnwwwnwsew
        nwnwnwnwnwsenwwnwsewnwnwnwnwnwnenwnwnw
        nesenweswnenenesenenenwneneeeneenwne
        sweeeseeeeesenweseeseseeese
        eseswwswseswseswneseseseseseswswseswwswsw
        nwenwnwsenwnwnenwnwnwnwnwnwsenwnwnwwne
        wewwwwwnewwwwewwnwwswww
        wnwwwswswneeswswswswswseseseesenwesw
        seweeeenweeeeesweeeeee
        nenenenenwsewnenenwneneswnenenenenenenene
        neesewneswneeeneneneneeenwwwneene
        swseswneswseswwesesweswswswnwseswnwswswe
        newnwwwseswswwswwswnwswwwsewswwne
        swswswwswswswswswsenwseswnenewswwswsw
        neeneenenewneeweeeswneseeeeee
        newsewnenwnewsenesewnwee
        nwnwswswseseneswswnweseswsenwneseweswsw
        seswswwwwwswwneswswwenewswne
        eneeenwwnwwwwwswnwnwswnwnwwww
        nwnwnwnwnwnwnwwewnwsww
        seeswneseseseeseesesesewenwseenwwse
        nwnwwswnwwwwewnwnwwwnwnwwewnwew
        eeseseseeeseneseeeseseeneseseswwsese
        swwswswweswwwnwswneswwswswswwww
        wnwswwwwwnwnwwwsenenwewswwwnenw
        sewswseseswnwnwsweenweswwseswswswnenwne
        wwnwwwnwnwwwwwnenwwnewswnwwnwwse
        seweswwneeswsenwswnwwswswswnwseswwsw
        nwnenenwnwnwneswnenenenenwenwnenwnenwneswne
        sewsenwnenwnwwwnenwnewwsenwwnwwnwse
        seseeseseenweneseweseswswswnesesenw
        swwneswwwwswwnewwswwseswswwsww
        seesweswnwseseseswnwnwswswnwse
        eesesweseeeseeeeeeenwsweene
        swseswswswseswswswswswswswwswswenwswsesw
        swwswswsweswwwnwswswswnwswwsewswsw
        nwnwwwnwwnwnwwnwwneenwwwwnwsesee
        weeenwseseseseeseenesesee
        nwnwwseenwwwwnwwnwwnwnenwsenwnww
        eeeeseseeeeswseeseseswneseeseeenw
        eeeeeneesweeeesweeenenweeee
        eweeseneeseesesenesweeseewsewne
        neneneswswenenwwnenewsweneswenwswsenw
        wwswnwsewneseswswneesenewswwwsenwsw
        seswswsweswnweswesweswswwnwnwsesese
        weneeeneeeeewseeeeeeeneee
        nwwnewwswwswswswwwswswwewswwwe
        seseesewseneeseesesweswsenwwsenwsese
        wenwseeswwnwewwneswnwenwwnwnwse
        wwwewswwwwwwwewnwwwwwnww
        seseswsesweweeesenweeseeeeseesenw
        swseneswneseswwswswswswseneseswswswswsesww
        esenwseseeeseseseesesesesenwwnwseene
        neeneneneeneeenenenwwswe
        wnewwswswswwwswswswswsewswwswwsw
        eswnwesenesweeeneesweeneseeesw
        nenwneneneeswneneneswnwnenene
        swswswswswswswneseswswenwwnweswswswsw
        eeeeneeeneeewneneeeeeeenwesw
        neneeesweseneneenwseenwnee
        nwnenenenwswewnwnwnwnwnwnweswseswnenw
        wneseseseseseseswseneswseseswsesesesese
        wswsweseseswswneswwswneswneswswnwswsw
        swnwnweneneswsewnenwnw
        nwnwnwnwnewnwnenwnenwnwnwnwnwnesenenesene
        nenenenenenwnenenwneenwswnenwsenenenenw
        swnwswnweesenwswswswsenwswseseeswswsesw
        ewswnwseswsenweenwsenesesewsee
        nwnewseewsewnwnwnesewwnwsesenwwswnw
        nwnwnenwnenwneneneenwwnenenenwswnwenenw
        nwwwswwwwwwnwwwswwenwnwwwe
        wnwsenesenwswsewswneswswnwsewswwwwsw
    "};
}
