use cargo_snippet::snippet;

#[snippet("@move_4")]
const MOVE_4: [(usize, usize); 4] = [(1, 0), (0, 1), (!0, 0), (0, !0)];

#[snippet("@move_8")]
const MOVE_8: [(usize, usize); 8] = [
    (1, 0),
    (0, 1),
    (!0, 0),
    (0, !0),
    (1, 1),
    (!0, !0),
    (1, !0),
    (!0, 1),
];
