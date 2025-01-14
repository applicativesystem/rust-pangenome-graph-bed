/*
 * holding all the structs in the separate files so that they
 * can be easily called as a reference call in the result.
 *
 *
 * */
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Segment {
    pub segment: String,
    pub id: String,
    pub seq: String,
    pub connection: String,
    pub tag: String,
}

#[derive(Debug, Default, Clone, PartialOrd, PartialEq)]
pub struct Analyze {
    pub tag: String,
    pub start: usize,
    pub end: usize,
    pub oritag: String,
    pub rank: usize,
}
