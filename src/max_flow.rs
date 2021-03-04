use cargo_snippet::snippet;

#[snippet("@max_flow")]
struct Edge {
    to: usize,
    rev: usize,
    cap: i128,
}

#[snippet("@max_flow")]
struct Dinitz {
    g: Vec<Vec<Edge>>,
}

#[snippet("@max_flow")]
impl Dinitz {
    fn new(v: usize) -> Dinitz {
        let mut g: Vec<Vec<Edge>> = Vec::new();
        for _ in 0..v {
            g.push(Vec::new());
        }
        Dinitz { g }
    }

    fn add_edge(&mut self, from: usize, to: usize, cap: i128) {
        let to_len = self.g[to].len();
        let from_len = self.g[from].len();
        self.g[from].push(Edge {
            to,
            rev: to_len,
            cap,
        });
        self.g[to].push(Edge {
            to: from,
            rev: from_len,
            cap: 0,
        });
    }

    fn dfs(
        &mut self,
        v: usize,
        sink: usize,
        flow: i128,
        level: &[i32],
        iter: &mut [usize],
    ) -> i128 {
        if v == sink {
            return flow;
        }
        while iter[v] < self.g[v].len() {
            let flow = std::cmp::min(flow, self.g[v][iter[v]].cap);
            let to = self.g[v][iter[v]].to;
            if flow > 0 && level[v] < level[to] {
                let flowed = self.dfs(to, sink, flow, level, iter);
                if flowed > 0 {
                    let rev = self.g[v][iter[v]].rev;
                    self.g[v][iter[v]].cap -= flowed;
                    self.g[to][rev].cap += flowed;
                    return flowed;
                }
            }
            iter[v] += 1;
        }
        0
    }

    fn bfs(&self, s: usize) -> Vec<i32> {
        let v = self.g.len();
        let mut level = vec![-1; v];
        level[s] = 0;
        let mut deque = std::collections::VecDeque::new();
        deque.push_back(s);
        while let Some(v) = deque.pop_front() {
            for e in self.g[v].iter() {
                if e.cap > 0 && level[e.to] < 0 {
                    level[e.to] = level[v] + 1;
                    deque.push_back(e.to);
                }
            }
        }
        level
    }

    fn max_flow(&mut self, s: usize, t: usize) -> i128 {
        let v = self.g.len();
        let mut flow: i128 = 0;
        loop {
            let level = self.bfs(s);
            if level[t] < 0 {
                return flow;
            }
            let mut iter = vec![0; v];
            loop {
                let f = self.dfs(s, t, std::i128::MAX, &level, &mut iter);
                if f == 0 {
                    break;
                }
                flow += f;
            }
        }
    }
}

#[test]
fn test_max_flow() {
    let mut dinitz = Dinitz::new(6);
    for &(from, to, cap) in [
        (0, 1, 5),
        (0, 3, 5),
        (1, 2, 4),
        (1, 3, 37),
        (2, 4, 56),
        (2, 5, 9),
        (3, 2, 3),
        (3, 4, 9),
        (4, 5, 2),
    ]
    .iter()
    {
        dinitz.add_edge(from, to, cap);
    }
    assert_eq!(dinitz.max_flow(0, 5), 9);
}
