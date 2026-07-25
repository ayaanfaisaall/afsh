enum Token {
    Num(i64),
    Word(String),
    Str(String),
    StrIntr(Vec<StrType>)
    Rslvr(String),
    Pipe,
    RdrctI,
    RdrctO,
    Appnd,
    LBrc,
    RBrc,
    And,
    AndAnd,
    Plus,
    Mins,
    Star,
    Divd,
    Qstn,
    If,
    While,
    Print,
    EOF,
}

enum StrType {
    Word(String),
    Var(String),
}
