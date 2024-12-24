use crate::{AdventHashMap, AdventHashSet};

pub static INPUT: &str = include_str!("../input/23.txt");
pub static TEST_INPUT: &str = include_str!("../input/23_test.txt");

pub fn a(input: &str) -> i32 {
    let mut computers = AdventHashMap::<&str, AdventHashSet<&str>>::default();

    for (a, b) in input.lines().map(|l| l.split_once('-').unwrap()) {
        computers.entry(a).or_default().insert(b);
        computers.entry(b).or_default().insert(a);
    }

    let mut unique_paths = AdventHashSet::default();

    for (start, next) in computers.iter().filter(|c| c.0.starts_with('t')) {
        for second in next {
            let second_next = computers.get(second).unwrap();
            for third in second_next {
                if next.contains(third) {
                    let mut id = [*start, *second, *third];
                    id.sort();

                    if !unique_paths.contains(&id) {
                        unique_paths.insert(id);
                    }
                }
            }
        }
    }

    unique_paths.len() as _
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 7);
    assert_eq!(a(INPUT), 1154);
}

fn bron_kerbosch<'a, F>(
    r: AdventHashSet<&'a str>,
    mut p: AdventHashSet<&'a str>,
    mut x: AdventHashSet<&'a str>,
    max_clique: &mut AdventHashSet<&'a str>,
    n_fn: &F,
) where
    F: Fn(&str) -> &'a AdventHashSet<&'a str>,
{
    if p.is_empty() && x.is_empty() {
        if r.len() > max_clique.len() {
            *max_clique = r;
        }
        return;
    }

    for v in p.clone() {
        let n_v = n_fn(&v);
        bron_kerbosch(
            {
                let mut r_next = r.clone();
                r_next.insert(v);
                r_next
            },
            p.intersection(n_v).copied().collect(),
            x.intersection(n_v).copied().collect(),
            max_clique,
            n_fn,
        );

        p.remove(&v);
        x.insert(v);
    }
}

pub fn b(input: &str) -> String {
    let mut connections = AdventHashMap::<&str, AdventHashSet<&str>>::default();

    for (a, b) in input.lines().map(|l| l.split_once('-').unwrap()) {
        connections.entry(a).or_default().insert(b);
        connections.entry(b).or_default().insert(a);
    }

    let mut max_clique = AdventHashSet::default();

    bron_kerbosch(
        AdventHashSet::default(),
        connections.keys().copied().collect(),
        AdventHashSet::default(),
        &mut max_clique,
        &|n| connections.get(n).unwrap(),
    );

    let mut max_clique = max_clique.iter().copied().collect::<Vec<_>>();
    max_clique.sort();

    max_clique.join(",")
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), "co,de,ka,ta");
    assert_eq!(b(INPUT), "aj,ds,gg,id,im,jx,kq,nj,ql,qr,ua,yh,zn");
}
