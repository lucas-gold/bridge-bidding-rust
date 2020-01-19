#[cfg(test)]
use crate::with_permutation;
use crate::format_game;
#[test]
fn userTest1() {
    assert_eq!(
"            North
            S A
            H A Q 7 5 3
            D A J 10 8 2
            C K 6
West                    East
S K                     S A K
H Q J 8 4 3             H A K J 10 8 4 2
D K 9                   D A J 7
C A Q 10 6 2            C Q
            South
            S A K 10 3
            H K 7 3
            D Q 8 4
            C K 9 5
   South West North East
   1C    1H   3H    5H
   Pass  Pass 6H    7H
   Pass  Pass Pass
   Declarer: West
",
        format_game(with_permutation(from_north_east_south_west(
            "SA HA HQ H7 H5 H3 DA DJ D10 D8 D2 CK C6",
            "SA SK HA HK HJ H10 H8 H4 H2 DA DJ D7 CQ",
            "SA SK S10 S3 HK H7 H3 DQ D8 D4 CK C9 C5",
            "SK HQ HJ H8 H4 H3 DK D9 CA CQ C10 C6 C2" ))))
}

#[test]
fn userTest2() {
    assert_eq!(
"            North
            S J 8 4
            H Q 7 5
            D A J 10 8 2
            C A 6
West                    East
S J                     S K J 9
H Q J 8 4               H Q J 4 2
D K 9 5                 D J 7 3 2
C A Q 10 6 2            C Q 8
            South
            S A K 10 3
            H J 7 3
            D Q 8 4
            C K 9 5
   South West North East
   1C    2C   3D    3H
   Pass  Pass Pass
   
   Declarer: East
",
        format_game(with_permutation(from_north_east_south_west(
            "SJ S8 S4 HQ H7 H5 DA DJ D10 D8 D2 CA C6",
            "SK SJ S9 HQ HJ H4 H2 DJ D7 D3 D2 CQ C8",
            "SA SK S10 S3 HJ H7 H3 DQ D8 D4 CK C9 C5",
            "SJ HQ HJ H8 H4 DK D9 D5 CA CQ C10 C6 C2" ))))
}

#[test]
fn userTest3() {
    assert_eq!(
"          North
          S A 8 4
          H A 7 5
          D A J 10 2
          C A 6 3
West                East
S J 7 3             S K 9 5
H Q 8 4             H A 10 6 2
D K 9 5             D J 7 3
C A 10 6 2          C Q 8 4
          South
          S A 10 6 2
          H J 7 3
          D Q 8 4
          C K 9 5
   South West North East
   Pass  Pass 1NT   Pass
   2S    Pass 3NT   Pass
   Pass  Pass
   Declarer: North
",
        format_game(with_permutation(from_north_east_south_west(
        "SA S8 S4 HA H7 H5 DA DJ D10 D2 CA C6 C3",
	"SK S9 S5 HA H10 H6 H2 DJ D7 D3 CQ C8 C4",
	"SA S10 S6 S2 HJ H7 H3 DQ D8 D4 CK C9 C5",
	"SJ S7 S3 HQ H8 H4 DK D9 D5 CA C10 C6 C2" ))))
}

#[test]
fn all_pass() {
    assert_eq!(
        "          North
          S Q 8 4
          H K 9 5
          D A 10 6 2
          C J 7 3
West                East
S J 7 3             S K 9 5
H Q 8 4             H A 10 6 2
D K 9 5             D J 7 3
C A 10 6 2          C Q 8 4
          South
          S A 10 6 2
          H J 7 3
          D Q 8 4
          C K 9 5
   South West North East
   Pass  Pass Pass  Pass
   Declarer: None
",
        format_game(with_permutation(from_north_east_south_west(
            "SQ S8 S4 HK H9 H5 DA D10 D6 D2 CJ C7 C3",
            "SK S9 S5 HA H10 H6 H2 DJ D7 D3 CQ C8 C4",
            "SA S10 S6 S2 HJ H7 H3 DQ D8 D4 CK C9 C5",
            "SJ S7 S3 HQ H8 H4 DK D9 D5 CA C10 C6 C2"))))
}

#[test]
fn init() {
    assert_eq!(
        "              North
              S K 10 7 5
              H 9 7 5 3
              D 10 5 4 3
              C J
West                     East
S Q 4 3                  S 9 8 2
H A K Q J 10 2           H 8
D Q 8                    D A K J 7
C 3 2                    C A Q 10 8 5
              South
              S A J 6
              H 6 4
              D 9 6 2
              C K 9 7 6 4
   South West North East
   Pass  1H   Pass  3C
   Pass  4H   Pass  Pass
   Pass
   Declarer: West
",
        format_game(with_permutation(from_north_east_south_west(
            "SK S10 S7 S5 H9 H7 H5 H3 D10 D5 D4 D3 CJ",
            "S9 S8 S2 H8 DA DK DJ D7 CA CQ C10 C8 C5",
            "SA SJ S6 H6 H4 D9 D6 D2 CK C9 C7 C6 C4",
            "SQ S4 S3 HA HK HQ HJ H10 H2 DQ D8 C3 C2"
        ))))
}

#[test]
fn n1() {
    assert_eq!(
        "          North
          S Q 8 4
          H K 9 5
          D A 10 6 2
          C A J 3
West                East
S A 7 3             S K 9 5
H Q 8 4             H A 10 6 2
D K 9 5             D J 7 3
C 10 7 6 2          C Q 8 4
          South
          S J 10 6 2
          H J 7 3
          D Q 8 4
          C K 9 5
   South West North East
   Pass  Pass 1D    Pass
   2D    Pass Pass  Pass
   Declarer: North
",
        format_game(with_permutation(from_north_east_south_west(
            "SQ S8 S4 HK H9 H5 DA D10 D6 D2 CA CJ C3",
            "SK S9 S5 HA H10 H6 H2 DJ D7 D3 CQ C8 C4",
            "SJ S10 S6 S2 HJ H7 H3 DQ D8 D4 CK C9 C5",
            "SA S7 S3 HQ H8 H4 DK D9 D5 C10 C7 C6 C2"
        ))))
}

#[test]
fn n2() {
    assert_eq!(
        "          North
          S Q 8 4
          H K 9 5
          D A 10 6 2
          C A J 3
West                East
S J 7 3             S A K 9
H Q 8 4             H A 10 6 2
D K 9 5             D J 7 3
C 10 7 6 2          C Q 8 4
          South
          S 10 6 5 2
          H J 7 3
          D Q 8 4
          C K 9 5
   South West North East
   Pass  Pass 1D    2C
   2D    3C   Pass  Pass
   Pass
   Declarer: East
",
        format_game(with_permutation(from_north_east_south_west(
            "SQ S8 S4 HK H9 H5 DA D10 D6 D2 CA CJ C3",
            "SA SK S9 HA H10 H6 H2 DJ D7 D3 CQ C8 C4",
            "S10 S6 S5 S2 HJ H7 H3 DQ D8 D4 CK C9 C5",
            "SJ S7 S3 HQ H8 H4 DK D9 D5 C10 C7 C6 C2"
        ))))
}

#[test]
fn n3() {
    assert_eq!(
        "          North
          S Q 8 4
          H K 9 5
          D A 10 6 2
          C A J 3
West                  East
S J 7                 S K 9 5
H Q 8 7 4             H A 10 6 2
D K 9 5               D J 7 3
C 10 7 6 2            C Q 8 4
          South
          S A 10 6 3 2
          H J 3
          D Q 8 4
          C K 9 5
   South West North East
   Pass  Pass 1D    Pass
   2D    Pass Pass  Pass
   Declarer: North
",
        format_game(with_permutation(from_north_east_south_west(
            "SQ S8 S4 HK H9 H5 DA D10 D6 D2 CA CJ C3",
            "SK S9 S5 HA H10 H6 H2 DJ D7 D3 CQ C8 C4",
            "SA S10 S6 S3 S2 HJ H3 DQ D8 D4 CK C9 C5",
            "SJ S7 HQ H8 H7 H4 DK D9 D5 C10 C7 C6 C2"
        ))))
}

#[test]
fn check_from_north_east_south_west() {
    assert_eq!(
        vec![33, 3, 43, 44, 37, 46, 47, 48, 49, 50, 51, 52],
        from_north_east_south_west(
            "SQ S8 C4",
            "SK S9 S5",
            "SA S10 S6",
            "SJ HQ H8"))
}
#[test]
fn check_to_indices() {
    assert_eq!(vec![3, 46, 50],to_indices("SQ S8 C4"))
}
fn to_indices(cards: &str) -> Vec<usize> {
    let deck : Vec<&str> = "C2 C3 C4 C5 C6 C7 C8 C9 C10 CJ CQ CK CA D2 D3 D4 D5 D6 D7 D8 D9 D10 DJ DQ DK DA H2 H3 H4 H5 H6 H7 H8 H9 H10 HJ HQ HK HA S2 S3 S4 S5 S6 S7 S8 S9 S10 SJ SQ SK SA".split(" ").collect();
    let cards = cards.split(" ");
    let mut indices : Vec<usize> = cards.map(|card| deck.iter().position(|&r| r == card).unwrap()+1).collect();
    indices.sort();
    indices
}
fn from_north_east_south_west(north:&str,east:&str,south:&str,west:&str) -> Vec<usize> {
    let mut result : Vec<usize> = Vec::new();
    let north = to_indices(north);
    let east = to_indices(east);
    let south = to_indices(south);
    let west = to_indices(west);
    for i in 0..west.len() {
        result.push(west[i]);
        result.push(north[i]);
        result.push(east[i]);
        result.push(south[i])
    }
    result
}
