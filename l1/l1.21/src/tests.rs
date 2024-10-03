use crate::bigint::BigInt;

#[cfg(test)]
mod tests {
    use crate::bigint::BigInt;

    #[test]
    fn add_test() {
        let b1 = BigInt::from_str("108");
        let b2 = BigInt::from_str("99");
        let b3 = b1 + b2;
        assert_eq!(b3.into_str(), "207");

        let b1 = BigInt::from_str("999");
        let b2 = BigInt::from_str("1");
        let b3 = b1 + b2;
        assert_eq!(b3.into_str(), "1000");

        let b1 = BigInt::from_str("89898989");
        let b2 = BigInt::from_str("11111111");
        let b3 = b1 + b2;
        assert_eq!(b3.into_str(), "101010100");
    }

    #[test]
    fn sub_test() {
        let b1 = BigInt::from_str("101");
        let b2 = BigInt::from_str("9");
        let b3 = b1 - b2;
        assert_eq!(b3.into_str(), "92");

        let b1 = BigInt::from_str("999");
        let b2 = BigInt::from_str("1999");
        let b3 = b1 - b2;
        assert_eq!(b3.into_str(), "-1000");

        let b1 = BigInt::from_str("-100");
        let b2 = BigInt::from_str("-100");
        let b3 = b1 - b2;
        assert_eq!(b3.into_str(), "0");
    }

    #[test]
    fn mul_test() {
        let b1 = BigInt::from_str("2048");
        let b2 = BigInt::from_str("2048");
        let b3 = b1 * b2;
        assert_eq!(b3.into_str(), "4194304");

        let b1 = BigInt::from_str("10111");
        let b2 = BigInt::from_str("11101");
        let b3 = b1 * b2;
        assert_eq!(b3.into_str(), "112242211");

        let b1 = BigInt::from_str("340282366920938463463374607431768211455");
        let b2 = BigInt::from_str("340282366920938463463374607431768211455");
        let b3 = b1 * b2;
        assert_eq!(b3.into_str(), "115792089237316195423570985008687907852589419931798687112530834793049593217025");
    }

    #[test]
    fn div_test() {
        let b1 = BigInt::from_str("2048");
        let b2 = BigInt::from_str("1024");
        let b3 = b1 / b2;
        assert_eq!(b3.into_str(), "2");

        let b1 = BigInt::from_str("1172054");
        let b2 = BigInt::from_str("2");
        let b3 = b1 / b2;
        assert_eq!(b3.into_str(), "586027");

        let b1 = BigInt::from_str("641912");
        let b2 = BigInt::from_str("2");
        let b3 = b1 / b2;
        assert_eq!(b3.into_str(), "320956");

        let b1 = BigInt::from_str("28934290932");
        let b2 = BigInt::from_str("223423");
        let b3 = b1 / b2;
        assert_eq!(b3.into_str(), "129504");
    }
}