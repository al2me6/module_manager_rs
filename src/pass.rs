use std::borrow::Cow;
use std::cmp::Ordering;

use ksp_cfg_formatter::parser;

// TODO: Custom ordering.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PassIdentifier<'a>(pub Cow<'a, str>);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Pass<'a> {
    Default,
    First,
    Before(PassIdentifier<'a>),
    For(PassIdentifier<'a>),
    After(PassIdentifier<'a>),
    Last(PassIdentifier<'a>),
    Final,
}

impl<'a> From<&'a str> for PassIdentifier<'a> {
    fn from(value: &'a str) -> Self {
        Self(value.into())
    }
}

impl<'a> From<String> for PassIdentifier<'a> {
    fn from(value: String) -> Self {
        Self(value.into())
    }
}

impl<'a> From<parser::Pass<'a>> for Pass<'a> {
    fn from(value: parser::Pass<'a>) -> Self {
        match value {
            parser::Pass::Default => Self::Default,
            parser::Pass::First => Self::First,
            parser::Pass::Before(ident) => Self::Before(ident.into()),
            parser::Pass::For(ident) => Self::For(ident.into()),
            parser::Pass::After(ident) => Self::After(ident.into()),
            parser::Pass::Last(ident) => Self::Last(ident.into()),
            parser::Pass::Final => Self::Final,
        }
    }
}

impl<'a> std::fmt::Display for PassIdentifier<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[macro_export]
macro_rules! pass {
    () => {
        $crate::pass::Pass::Default
    };
    (FIRST) => {
        $crate::pass::Pass::First
    };
    (BEFORE[$ident:literal]) => {
        $crate::pass::Pass::Before($ident.into())
    };
    (FOR[$ident:literal]) => {
        $crate::pass::Pass::For($ident.into())
    };
    (AFTER[$ident:literal]) => {
        $crate::pass::Pass::After($ident.into())
    };
    (LAST[$ident:literal]) => {
        $crate::pass::Pass::Last($ident.into())
    };
    (FINAL) => {
        $crate::pass::Pass::Final
    };
}

impl<'a> Pass<'a> {
    const fn numerical_ordering(&self) -> u8 {
        match self {
            Self::Default => 0,
            Self::First => 1,
            Self::Before(_) => 2,
            Self::For(_) => 3,
            Self::After(_) => 4,
            Self::Last(_) => 5,
            Self::Final => 6,
        }
    }
}

impl<'a> PartialOrd for Pass<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for Pass<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (
                a @ (Self::Before(ident_a) | Self::For(ident_a) | Self::After(ident_a)),
                b @ (Self::Before(ident_b) | Self::For(ident_b) | Self::After(ident_b)),
            ) => ident_a
                .cmp(ident_b)
                .then_with(|| a.numerical_ordering().cmp(&b.numerical_ordering())),
            (Self::Last(a), Self::Last(b)) => a.cmp(b),
            (a, b) => a.numerical_ordering().cmp(&b.numerical_ordering()),
        }
    }
}

impl<'a> std::fmt::Display for Pass<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pass::Default => f.write_str(":<DEFAULT>"),
            Pass::First => f.write_str(":FIRST"),
            Pass::Before(pass) => write!(f, ":BEFORE[{pass}]"),
            Pass::For(pass) => write!(f, ":FOR[{pass}]"),
            Pass::After(pass) => write!(f, ":AFTER[{pass}]"),
            Pass::Last(ident) => write!(f, ":LAST[{ident}]"),
            Pass::Final => f.write_str(":FINAL"),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn pass_ordering() {
        assert_eq!(pass![FOR["foo"]], pass![FOR["foo"]]);
        assert!(pass![] < pass![FIRST]);
        assert!(pass![FIRST] < pass![BEFORE["foo"]]);
        assert!(pass![BEFORE["foo"]] < pass![FOR["foo"]]);
        assert!(pass![FOR["foo"]] < pass![AFTER["foo"]]);
        assert!(pass![AFTER["foo"]] < pass![BEFORE["qux"]]);
        assert!(pass![LAST["foo"]] < pass![FINAL]);
    }
}
