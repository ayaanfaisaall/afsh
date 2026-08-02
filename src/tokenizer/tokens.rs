
#[derive(Debug)]
pub enum Token<'a> {
    Num(&'a str),
    Word(&'a str),
    Str(Vec<StrType<'a>>),
    Rslvr(&'a str),
    Pipe,
    RdrctI,
    RdrctO,
    Appnd,
    LBrc,
    RBrc,
    And,
    AndAnd,
    OrOr,
    Bang,
    Plus,
    Mins,
    Star,
    Divd,
    Qstn,
    Let,
    VarNam(&'a str),
    If,
    While,
    Print,
    EOF,
}

#[derive(Debug)]
pub enum StrType<'a> {
    Str(&'a str),
    Var(&'a str),
}
