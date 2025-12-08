use std::collections::HashMap;

use itertools::{Itertools, sorted};
use multimap::MultiMap;

type P3 = (i64, i64, i64);
fn parse_input(input: &str) -> Vec<P3> {
    input
        .lines()
        .map(|l| {
            let mut parts = l.split(',').map(|n| n.trim().parse().unwrap());
            (
                parts.next().unwrap(),
                parts.next().unwrap(),
                parts.next().unwrap(),
            )
        })
        .collect()
}

fn dist(a: &P3, b: &P3) -> f64 {
    let dx = a.0 - b.0;
    let dy = a.1 - b.1;
    let dz = a.2 - b.2;
    ((dx * dx + dy * dy + dz * dz) as f64).sqrt()
}

fn read_input() -> (HashMap<P3, usize>, MultiMap<usize, P3>, Vec<(P3, P3, f64)>) {
    let input = include_str!("input.txt");
    let points = parse_input(input);

    let (cluster_assignment, clusters) = {
        let mut c = MultiMap::new();
        let mut assignments = HashMap::new();
        for (i, p) in points.iter().enumerate() {
            c.insert(i, *p);
            assignments.insert(*p, i);
        }
        (assignments, c)
    };
    let ordered_distances = {
        let mut dists = Vec::new();
        for i in 0..points.len() {
            for j in (i + 1)..points.len() {
                let (p1, p2) = (points[i], points[j]);
                dists.push((p1, p2, dist(&p1, &p2)));
            }
        }
        dists.sort_by(|(_, _, d1), (_, _, d2)| d1.partial_cmp(&d2).unwrap());
        dists
        // dists
        //     .iter()
        //     .map(|((p1, p2), _)| (*p1, *p2))
        //     .collect::<Vec<_>>()
    };
    (cluster_assignment, clusters, ordered_distances)
}

#[test]
fn part1() {
    let (
        // P3->cluster-id, tells you which cluster a point is in
        mut cluster_assignment,
        // the effective clusters, cluster-id -> Vec<P3>, initially every point is its own cluster
        mut clusters,
        // pair of points ordered by distance, shortest first
        ordered_distances,
    ) = read_input();

    for (p1, p2, _) in ordered_distances.iter().take(1000) {
        // merge cluster of p1 with cluster of p2
        let (ca1, ca2) = (
            *cluster_assignment.get(p1).unwrap(),
            *cluster_assignment.get(p2).unwrap(),
        );

        // move all of p2  into cluster of p1
        if let Some(cluster2) = clusters.remove(&ca2) {
            cluster2.into_iter().for_each(|p| {
                clusters.insert(ca1, p);
                cluster_assignment.insert(p, ca1);
            });
        } else {
            panic!("Cluster not found");
        }
    }

    let result: usize = clusters
        .iter_all()
        .map(|(_, v)| v.len())
        .sorted()
        .rev()
        .take(3)
        .product();

    println!("Part 1: {}", result);
    assert_eq!(50568, result);
}

#[test]
fn part2() {
    let (
        // P3->cluster-id, tells you which cluster a point is in
        mut cluster_assignment,
        // the effective clusters, cluster-id -> Vec<P3>, initially every point is its own cluster
        mut clusters,
        // pair of points ordered by distance, shortest first
        ordered_distances,
    ) = read_input();

    for (p1, p2, _) in ordered_distances.iter() {
        // merge cluster of p1 with cluster of p2
        let (ca1, ca2) = (
            *cluster_assignment.get(p1).unwrap(),
            *cluster_assignment.get(p2).unwrap(),
        );

        // move all of p2  into cluster of p1
        if let Some(cluster2) = clusters.remove(&ca2) {
            cluster2.into_iter().for_each(|p| {
                clusters.insert(ca1, p);
                cluster_assignment.insert(p, ca1);
            });
        } else {
            panic!("Cluster not found");
        }

        if clusters.len() == 1 {
            assert_eq!(36045012, p1.0 * p2.0);
            println!("part2: {}", p1.0 * p2.0);
            break;
        }
    }
}
