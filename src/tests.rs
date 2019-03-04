use crate::*;

macro_rules! test_suite {
    ($n:ident, $u:ident, $r:ty) => {
        mod $n {
            use super::*;

            #[test]
            fn default_works() {
                let bb = BitBoard::<$u, $r>::default();
                for i in 0..$u::USIZE {
                    for j in 0..$u::USIZE {
                        assert_eq!(bb.is_set(i, j), false);
                    }
                }
            }

            #[test]
            fn new_works() {
                let initial = (0..$u::USIZE).map(|n| (n, n)).collect();

                let bb = BitBoard::<$u, $r>::new(initial);
                for i in 0..$u::USIZE {
                    for j in 0..$u::USIZE {
                        assert_eq!(bb.is_set(i, j), i == j);
                    }
                }
            }

            #[test]
            fn set_works() {
                let mut bb = BitBoard::<$u, $r>::default();
                for i in 0..$u::USIZE {
                    for j in 0..$u::USIZE {
                        bb.set(i, j);
                    }
                }

                assert_eq!(bb.count_ones(), $u::USIZE * $u::USIZE);
            }

            #[test]
            fn shl_works() {
                let mut bb = BitBoard::<$u, $r>::new(vec![(0, 0)]);
                dbg!(&bb);
                bb = &bb << ($u::USIZE * $u::USIZE) - 1;

                assert_eq!(bb.is_set($u::USIZE - 1, $u::USIZE - 1), true);
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

test_suite!(u2_u16, U2, u16);
test_suite!(u3_u16, U3, u16);
test_suite!(u4_u16, U4, u16);
test_suite!(u5_u16, U5, u16);
test_suite!(u6_u16, U6, u16);
test_suite!(u7_u16, U7, u16);
test_suite!(u8_u16, U8, u16);
test_suite!(u20_u16, U20, u16);
test_suite!(u50_u16, U50, u16);

test_suite!(u2_u32, U2, u32);
test_suite!(u3_u32, U3, u32);
test_suite!(u4_u32, U4, u32);
test_suite!(u5_u32, U5, u32);
test_suite!(u6_u32, U6, u32);
test_suite!(u7_u32, U7, u32);
test_suite!(u8_u32, U8, u32);
test_suite!(u20_u32, U20, u32);
test_suite!(u50_u32, U50, u32);

test_suite!(u2_u64, U2, u64);
test_suite!(u3_u64, U3, u64);
test_suite!(u4_u64, U4, u64);
test_suite!(u5_u64, U5, u64);
test_suite!(u6_u64, U6, u64);
test_suite!(u7_u64, U7, u64);
test_suite!(u8_u64, U8, u64);
test_suite!(u20_u64, U20, u64);
test_suite!(u50_u64, U50, u64);
