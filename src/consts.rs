use cargo_snippet::snippet;

#[snippet("@adj_4")]
const ADJ_4: [(usize, usize); 4] = [(1, 0), (0, 1), (!0, 0), (0, !0)];

#[snippet("@adj_8")]
const ADJ_8: [(usize, usize); 8] = [
    (1, 0),
    (0, 1),
    (!0, 0),
    (0, !0),
    (1, 1),
    (!0, !0),
    (1, !0),
    (!0, 1),
];
