#[cfg(test)]
#[allow(non_snake_case)]
pub mod math {
    use graph_lib::math::math;

    #[test]
    pub fn calculatesPositiveMinCorrectly() {
        let realMin1 = math::positiveMin(2, 5);
        let expectedMin1 = 2;
        assert_eq!(
            realMin1,
            expectedMin1
        );

        let realMin2 = math::positiveMin(10, 4);
        let expectedMin2 = 4;
        assert_eq!(
            realMin2,
            expectedMin2
        );

        let realMin3 = math::positiveMin(12, -1);
        let expectedMin3 = 12;
        assert_eq!(
            realMin3,
            expectedMin3
        );

        let realMin4 = math::positiveMin(-3, 20);
        let expectedMin4 = 20;
        assert_eq!(
            realMin4,
            expectedMin4
        );
    }

    #[test]
    pub fn calculatesPositiveMaxCorrectly() {
        let realMax1 = math::positiveMax(20, 10);
        let expectedMax1 = 20;
        assert_eq!(
            realMax1,
            expectedMax1
        );

        let realMax2 = math::positiveMax(11, 32);
        let expectedMax2 = 32;
        assert_eq!(
            realMax2,
            expectedMax2
        );

        let realMax3 = math::positiveMax(23, i32::MAX);
        let expectedMax3 = 23;
        assert_eq!(
            realMax3,
            expectedMax3
        );

        let realMax4 = math::positiveMax(i32::MAX, 28);
        let expectedMax4 = 28;
        assert_eq!(
            realMax4,
            expectedMax4
        );
    }
}