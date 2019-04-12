use crate::*;
use typenum::*;

macro_rules! test_suite {
    ($n:ident, $u:ident, $r:ty) => {
        mod $n {
            use super::*;

            const SIZE: usize = $u::USIZE;
            type TestBoard = BitBoard<$u, $r>;

            #[test]
            fn default_works() {
                let bb = TestBoard::default();
                assert_eq!(false, bb.into_iter().any(|b| b));
            }

            #[test]
            fn new_works() {
                let initial = (0..SIZE).map(|n| (n, n)).collect();
                let bb = TestBoard::new(initial);
                for i in 0..SIZE {
                    assert_eq!(bb.is_set(i, i), true);
                }
                assert_eq!(bb.count_ones(), SIZE);
            }

            #[test]
            fn set_works() {
                let mut bb = TestBoard::default();
                for i in 0..SIZE {
                    for j in 0..SIZE {
                        bb.set(i, j);
                    }
                }

                assert_eq!(bb.count_ones(), SIZE * SIZE);
            }

            #[test]
            fn move_left_works() {
                let mut bb = TestBoard::new(vec![(SIZE - 1, 0)]);
                bb = &bb << Move::Left(SIZE - 1);
                assert_eq!(bb.is_set(0, 0), true);
            }

            #[test]
            fn move_right_works() {
                let mut bb = TestBoard::new(vec![(0, 0)]);
                bb = &bb << Move::Right(SIZE - 1);
                assert_eq!(bb.is_set(SIZE - 1, 0), true);
            }

            #[test]
            fn move_up_works() {
                let mut bb = TestBoard::new(vec![(0, 0)]);
                bb = &bb << Move::Up(SIZE - 1);
                assert_eq!(bb.is_set(0, SIZE - 1), true);
            }

            #[test]
            fn move_down_works() {
                let mut bb = TestBoard::new(vec![(0, SIZE - 1)]);
                bb = &bb << Move::Down(SIZE - 1);
                assert_eq!(bb.is_set(0, 0), true);
            }

            #[test]
            fn move_upright_works() {
                let mut bb = TestBoard::new(vec![(0, 0)]);
                bb = &bb << Move::UpRight(SIZE - 1, SIZE - 1);
                assert_eq!(bb.is_set(SIZE - 1, SIZE - 1), true);
            }

            #[test]
            fn move_upleft_works() {
                let mut bb = TestBoard::new(vec![(SIZE - 1, 0)]);
                bb = &bb << Move::UpLeft(SIZE - 1, SIZE - 1);
                assert_eq!(bb.is_set(0, SIZE - 1), true);
            }

            #[test]
            fn move_downright_works() {
                let mut bb = TestBoard::new(vec![(0, SIZE - 1)]);
                bb = &bb << Move::DownRight(SIZE - 1, SIZE - 1);
                assert_eq!(bb.is_set(SIZE - 1, 0), true);
            }

            #[test]
            fn move_downleft_works() {
                let mut bb = TestBoard::new(vec![(SIZE - 1, SIZE - 1)]);
                bb = &bb << Move::DownLeft(SIZE - 1, SIZE - 1);
                assert_eq!(bb.is_set(0, 0), true);
            }

            #[test]
            fn move_doesnt_add_bits() {
                let mut bb = TestBoard::new(vec![(SIZE - 1, SIZE - 1)]);
                bb = &bb << Move::Right(1);
                assert_eq!(bb.count_ones(), 0);
                bb = &bb << Move::Left(1);
                assert_eq!(bb.count_ones(), 0);
            }

            #[test]
            fn left_edge_mask_works() {
                for column in 0..SIZE {
                    let init = (0..SIZE).into_iter().map(|i| (column, i)).collect();
                    let mut bb = TestBoard::new(init);
                    assert_eq!(bb.count_ones(), SIZE);
                    bb = &bb << Move::Left(column + 1);
                    assert_eq!(bb.count_ones(), 0);
                }
            }

            #[test]
            fn right_edge_mask_works() {
                for column in 0..SIZE {
                    let init = (0..SIZE)
                        .into_iter()
                        .map(|i| (SIZE - column - 1, i))
                        .collect();
                    let mut bb = TestBoard::new(init);
                    assert_eq!(bb.count_ones(), SIZE);
                    bb = &bb << Move::Right(column + 1);
                    assert_eq!(bb.count_ones(), 0);
                }
            }

        }
    };
}

test_suite!(u2_u8, U2, u8);
test_suite!(u3_u8, U3, u8);
test_suite!(u4_u8, U4, u8);
test_suite!(u5_u8, U5, u8);
test_suite!(u6_u8, U6, u8);
test_suite!(u7_u8, U7, u8);
test_suite!(u8_u8, U8, u8);
test_suite!(u20_u8, U20, u8);
test_suite!(u50_u8, U50, u8);
test_suite!(u100_u8, U100, u8);

test_suite!(u2_u16, U2, u16);
test_suite!(u3_u16, U3, u16);
test_suite!(u4_u16, U4, u16);
test_suite!(u5_u16, U5, u16);
test_suite!(u6_u16, U6, u16);
test_suite!(u7_u16, U7, u16);
test_suite!(u8_u16, U8, u16);
test_suite!(u20_u16, U20, u16);
test_suite!(u100_u16, U100, u16);

test_suite!(u2_u32, U2, u32);
test_suite!(u3_u32, U3, u32);
test_suite!(u4_u32, U4, u32);
test_suite!(u5_u32, U5, u32);
test_suite!(u6_u32, U6, u32);
test_suite!(u7_u32, U7, u32);
test_suite!(u8_u32, U8, u32);
test_suite!(u20_u32, U20, u32);
test_suite!(u100_u32, U100, u32);

test_suite!(u2_u64, U2, u64);
test_suite!(u3_u64, U3, u64);
test_suite!(u4_u64, U4, u64);
test_suite!(u5_u64, U5, u64);
test_suite!(u6_u64, U6, u64);
test_suite!(u7_u64, U7, u64);
test_suite!(u8_u64, U8, u64);
test_suite!(u20_u64, U20, u64);
test_suite!(u100_u64, U100, u64);
