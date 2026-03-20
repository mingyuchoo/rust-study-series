//! 그래프 알고리즘 모듈
//! - PageRank (가중치 유향 그래프)
//! - Betweenness 중심성 (Brandes 알고리즘)

use std::collections::{HashMap, HashSet, VecDeque};

/// PageRank 계산 설정
pub struct PageRankConfig {
    pub damping: f32,
    pub iterations: usize,
}

impl Default for PageRankConfig {
    fn default() -> Self {
        Self {
            damping: 0.85,
            iterations: 20,
        }
    }
}

/// 그래프의 엣지를 표현
pub struct GraphEdge {
    pub source: String,
    pub target: String,
    pub weight: f32,
}

/// 중심성 계산 결과
pub struct CentralityResult {
    pub pagerank: HashMap<String, f32>,
    pub betweenness: HashMap<String, f32>,
}

/// 가중치 유향 그래프에서 PageRank를 계산한다.
pub fn compute_pagerank(
    nodes: &HashSet<String>,
    edges: &[GraphEdge],
    config: &PageRankConfig,
) -> HashMap<String, f32> {
    if nodes.is_empty() {
        return HashMap::new();
    }

    let n = nodes.len().max(1) as f32;
    let mut pr: HashMap<String, f32> = nodes.iter().map(|k| (k.clone(), 1.0 / n)).collect();

    // 인접 리스트 및 출력 가중치 합
    let mut out_edges: HashMap<String, HashMap<String, f32>> = HashMap::new();
    for e in edges {
        let entry = out_edges
            .entry(e.source.clone())
            .or_default();
        *entry.entry(e.target.clone()).or_insert(0.0) += e.weight;
    }

    let mut out_sum: HashMap<String, f32> = HashMap::new();
    for (u, nbrs) in &out_edges {
        let s: f32 = nbrs.values().copied().sum();
        out_sum.insert(u.clone(), if s > 0.0 { s } else { 1.0 });
    }

    for _ in 0..config.iterations {
        let mut new_pr: HashMap<String, f32> = nodes
            .iter()
            .map(|k| (k.clone(), (1.0 - config.damping) / n))
            .collect();
        for (u, nbrs) in &out_edges {
            let contrib_base = pr.get(u).copied().unwrap_or(0.0);
            let denom = out_sum.get(u).copied().unwrap_or(1.0);
            for (v, w) in nbrs {
                let inc = config.damping * contrib_base * (*w / denom);
                *new_pr.entry(v.clone()).or_insert(0.0) += inc;
            }
        }
        pr = new_pr;
    }

    // 정규화(0~1)
    let pr_max = pr.values().copied().fold(0.0_f32, f32::max).max(1e-6);
    for v in pr.values_mut() {
        *v /= pr_max;
    }

    pr
}

/// Brandes 알고리즘으로 Betweenness 중심성을 계산한다.
pub fn compute_betweenness(
    nodes: &HashSet<String>,
    edges: &[GraphEdge],
) -> HashMap<String, f32> {
    if nodes.is_empty() {
        return HashMap::new();
    }

    let node_list: Vec<String> = nodes.iter().cloned().collect();
    let index_of: HashMap<String, usize> = node_list
        .iter()
        .enumerate()
        .map(|(i, k)| (k.clone(), i))
        .collect();
    let n = node_list.len();

    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
    for e in edges {
        if let (Some(&ui), Some(&vi)) = (index_of.get(&e.source), index_of.get(&e.target)) {
            adj[ui].push(vi);
        }
    }

    let mut bc: Vec<f32> = vec![0.0; n];

    for s in 0..n {
        let mut stack: Vec<usize> = Vec::new();
        let mut pred: Vec<Vec<usize>> = vec![Vec::new(); n];
        let mut sigma: Vec<f32> = vec![0.0; n];
        sigma[s] = 1.0;
        let mut dist: Vec<i32> = vec![-1; n];
        dist[s] = 0;

        let mut queue: VecDeque<usize> = VecDeque::new();
        queue.push_back(s);

        while let Some(v) = queue.pop_front() {
            stack.push(v);
            let dv = dist[v];
            for &w in &adj[v] {
                if dist[w] < 0 {
                    dist[w] = dv + 1;
                    queue.push_back(w);
                }
                if dist[w] == dv + 1 {
                    sigma[w] += sigma[v];
                    pred[w].push(v);
                }
            }
        }

        let mut delta: Vec<f32> = vec![0.0; n];
        while let Some(w) = stack.pop() {
            for &v in &pred[w] {
                if sigma[w] > 0.0 {
                    delta[v] += (sigma[v] / sigma[w]) * (1.0 + delta[w]);
                }
            }
            if w != s {
                bc[w] += delta[w];
            }
        }
    }

    // 정규화(0~1)
    let bc_max = bc.iter().copied().fold(0.0_f32, f32::max).max(1e-6);
    node_list
        .into_iter()
        .enumerate()
        .map(|(i, name)| (name, bc[i] / bc_max))
        .collect()
}

/// PageRank + Betweenness 중심성을 한 번에 계산한다.
pub fn compute_centrality(
    nodes: &HashSet<String>,
    edges: &[GraphEdge],
    pr_config: &PageRankConfig,
) -> CentralityResult {
    let pagerank = compute_pagerank(nodes, edges, pr_config);
    let betweenness = compute_betweenness(nodes, edges);
    CentralityResult {
        pagerank,
        betweenness,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_edges(data: &[(&str, &str, f32)]) -> Vec<GraphEdge> {
        data.iter()
            .map(|(s, t, w)| GraphEdge {
                source: s.to_string(),
                target: t.to_string(),
                weight: *w,
            })
            .collect()
    }

    #[test]
    fn test_pagerank_simple_chain() {
        let nodes: HashSet<String> = ["A", "B", "C"].iter().map(|s| s.to_string()).collect();
        let edges = make_edges(&[("A", "B", 1.0), ("B", "C", 1.0)]);
        let pr = compute_pagerank(&nodes, &edges, &PageRankConfig::default());
        // C는 체인의 끝이므로 가장 높은 PR을 가져야 한다
        assert!(pr["C"] >= pr["B"]);
        assert!(pr["B"] >= pr["A"]);
    }

    #[test]
    fn test_betweenness_middle_node() {
        let nodes: HashSet<String> = ["A", "B", "C"].iter().map(|s| s.to_string()).collect();
        let edges = make_edges(&[("A", "B", 1.0), ("B", "C", 1.0)]);
        let bc = compute_betweenness(&nodes, &edges);
        // B는 A→C 경로의 중간 노드이므로 가장 높은 BC를 가져야 한다
        assert!(bc["B"] >= bc["A"]);
        assert!(bc["B"] >= bc["C"]);
    }

    #[test]
    fn test_empty_graph() {
        let nodes: HashSet<String> = HashSet::new();
        let edges: Vec<GraphEdge> = Vec::new();
        let result = compute_centrality(&nodes, &edges, &PageRankConfig::default());
        assert!(result.pagerank.is_empty());
        assert!(result.betweenness.is_empty());
    }
}
