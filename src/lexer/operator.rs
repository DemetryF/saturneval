macro_rules! operators {
    [ $($Case:ident = $CaseValue:literal,)* ] => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum Operator {
            $($Case,)*
        }

        impl From<Operator> for char {
            fn from(op: Operator) -> Self {
                match op {
                    $( Operator::$Case => $CaseValue, )*
                }
            }
        }

        impl Operator {
            pub fn all() -> Vec<Self> {
                vec![ $(Self::$Case,)* ]
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
