#![allow(dead_code)]

#[derive(Clone, Debug, PartialEq)]
pub enum OneOf2<A, B> {
    A(A),
    B(B),
}

#[derive(Clone, Debug, PartialEq)]
pub enum OneOf3<A, B, C> {
    A(A),
    B(B),
    C(C),
}

#[derive(Clone, Debug, PartialEq)]
pub enum OneOf4<A, B, C, D> {
    A(A),
    B(B),
    C(C),
    D(D),
}
