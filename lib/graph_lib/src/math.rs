#[allow(non_snake_case)]
pub mod math {
    pub fn positiveMin(number1: i32, number2: i32) -> i32 {
        if number1 < 0 {
            return number2;
        } 

        if number2 < 0 {
            return number1;
        }

        return if number1 < number2 { number1 } else { number2 };
    }

    pub fn positiveMax(number1: i32, number2: i32) -> i32 {
        if number1 >= i32::MAX {
            return number2;
        } 

        if number2 >= i32::MAX {
            return number1;
        }

        return if number1 > number2 { number1 } else { number2 };
    }
}