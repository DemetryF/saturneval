use strum_macros::EnumIter;

macro_rules! operators {
    [
        $($Case:ident = $CaseValue:literal,)*
    ] => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, EnumIter)]
        pub enum Operator {
            $($Case,)*
        }

        impl From<Operator> for char {
            fn from(op: Operator) -> Self {
                match op {
                    $(
                        Operator::$Case => $CaseValue,
                    )*
                }
            }
        }
    };
}

#[rustfmt::skip]
operators![
    Addition = '+',
    Subtraction = '-',

    Multiplication = '*',
    Division = '/',
    WholeDivision = '\\',
    ModuloDivision = '%',

    Exponentiation = '^',
];
