//! 그래프 검색 엔드포인트 (임베딩 기반 시드 + 관계 BFS)

use actix_web::{post, web, Result};
use log::debug;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::time::Instant;

use crate::error::Error;
use crate::models::{GraphPathItem, GraphSearchRequest, GraphSearchResponse};
use crate::types::AppState;
use lib_db::DB;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct EntityRow {
    name: String,
    r#type: String,
    score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RelationRow {
    subject: String,
    predicate: String,
    object: String,
    weight: f64,
}

#[utoipa::path(
    tag = "graph",
    post,
    path = "/api/search/graph",
    request_body = GraphSearchRequest,
    responses(
        (status = 200, description = "그래프 검색 결과", body = GraphSearchResponse),
        (status = 400, description = "잘못된 요청"),
        (status = 500, description = "서버 오류"),
    )
)]
#[post("/api/search/graph")]
pub async fn graph_search(
    state: web::Data<AppState>,
    payload: web::Json<GraphSearchRequest>,
) -> Result<web::Json<GraphSearchResponse>, Error> {
    let t0 = Instant::now();
    let query = payload.query.trim();
    if query.is_empty() {
        return Err(Error::BadRequest("query가 비어 있습니다".into()));
    }
    let top_k = payload.top_k.max(1).min(50) as usize;
    let max_hops = payload.max_hops.max(1).min(5) as usize;

    // 1) 쿼리 임베딩 생성 → 엔티티 테이블에서 코사인 유사도 상위 시드 선택
    let embeddings = state
        .azure
        .embed(&[query])
        .await
        .map_err(|e| Error::External(e.to_string()))?;
    let query_vec = embeddings.get(0).cloned().unwrap_or_default();

    let k_entities = (top_k * 5).min(100) as i64;
    let mut q = DB
        .query(
            r#"
            SELECT name, type,
                   vector::similarity::cosine(embedding_semantic, $q) AS score
            FROM entity
            WHERE embedding_type = 'azure'
              AND embedding_deployment = $dep
              AND array::len(embedding_semantic) = array::len($q)
            ORDER BY score DESC
            LIMIT $k;
            "#,
        )
        .bind(("q", query_vec))
        .bind(("dep", state.azure.embed_deployment().to_string()))
        .bind(("k", k_entities)
        )
        .await
        .map_err(|e| Error::External(e.to_string()))?;
    let all_entities: Vec<EntityRow> = q.take(0)?;
    debug!("[graph] candidate entities: {}", all_entities.len());
    let seeds: Vec<EntityRow> = all_entities.into_iter().take(top_k).collect();
    if seeds.is_empty() {
        let elapsed = t0.elapsed().as_secs_f32();
        return Ok(web::Json(GraphSearchResponse { paths: vec![], total: 0, query_time: elapsed }));
    }

    // 시드 이름과 스코어 맵
    let mut seed_score: HashMap<String, f64> = HashMap::new();
    let mut frontier: HashSet<String> = HashSet::new();
    for e in &seeds {
        seed_score.insert(e.name.clone(), e.score);
        frontier.insert(e.name.clone());
    }

    // BFS로 경로 확장
    #[derive(Clone, Debug)]
    struct Path {
        nodes: Vec<String>,
        edges: Vec<RelationRow>,
        score: f64,
    }

    let mut paths: Vec<Path> = seeds
        .iter()
        .map(|s| Path { nodes: vec![s.name.clone()], edges: vec![], score: s.score })
        .collect();

    // 경로 점수 계산 함수
    let score_path = |seed_score: f64, edges: &[RelationRow]| -> f64 {
        if edges.is_empty() {
            return seed_score as f64;
        }
        let avg_w = edges.iter().map(|e| e.weight).sum::<f64>() / (edges.len() as f64);
        // 0..1 범위로 클램프
        let avg_w = avg_w.clamp(0.0, 1.0);
        // 임의 가중 결합
        0.6 * seed_score + 0.4 * avg_w
    };

    let mut visited_edges: HashSet<(String, String, String)> = HashSet::new();
    let mut current_frontier: HashSet<String> = frontier.clone();

    for hop in 1..=max_hops {
        if current_frontier.is_empty() { break; }
        let names_vec: Vec<String> = current_frontier.iter().cloned().collect();
        debug!("[graph] hop {} frontier size {}", hop, names_vec.len());

        // 현재 프론티어에 인접한 관계 조회
        let mut res = DB
            .query(
                r#"
                SELECT subject, predicate, object, weight
                FROM relation
                WHERE subject IN $names OR object IN $names
                LIMIT 500;
                "#,
            )
            .bind(("names", names_vec.clone()))
            .await
            .map_err(|e| Error::External(e.to_string()))?;
        let rels: Vec<RelationRow> = res.take(0)?;
        debug!("[graph] hop {} relations fetched {}", hop, rels.len());

        if rels.is_empty() { break; }

        // 새 프론티어 구성 및 경로 확장
        let mut next_frontier: HashSet<String> = HashSet::new();
        let mut new_paths: Vec<Path> = Vec::new();

        // 빠른 조회를 위해 관계를 양방향 adjacency로 구성
        let mut adj: HashMap<String, Vec<RelationRow>> = HashMap::new();
        for r in rels.into_iter() {
            // 방문 체크로 중복/사이클 일부 방지
            let key = (r.subject.clone(), r.predicate.clone(), r.object.clone());
            if !visited_edges.insert(key) {
                continue;
            }
            adj.entry(r.subject.clone()).or_default().push(r.clone());
            // 역방향도 탐색 허용(무방향 그래프처럼)
            let rev = RelationRow { subject: r.object.clone(), predicate: r.predicate.clone(), object: r.subject.clone(), weight: r.weight };
            adj.entry(rev.subject.clone()).or_default().push(rev);
        }

        for p in &paths {
            let last = p.nodes.last().unwrap();
            let edges_out = adj.get(last).cloned().unwrap_or_default();
            if edges_out.is_empty() {
                // 확장 불가 시 기존 경로 유지
                new_paths.push(p.clone());
                continue;
            }
            for e in edges_out {
                // 사이클 방지: 이미 포함된 노드로 되돌아가지 않음
                if p.nodes.contains(&e.object) { continue; }
                let mut nodes = p.nodes.clone();
                nodes.push(e.object.clone());
                let mut edges = p.edges.clone();
                edges.push(e.clone());
                let seed_name = nodes.first().cloned().unwrap_or_default();
                let sscore = *seed_score.get(&seed_name).unwrap_or(&0.0);
                let new_score = score_path(sscore, &edges);
                new_paths.push(Path { nodes, edges, score: new_score });
                next_frontier.insert(e.object);
            }
        }

        // 상위 top_k로 가지치기
        new_paths.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        new_paths.truncate(top_k);
        paths = new_paths;
        current_frontier = next_frontier;
    }

    // 결과 구성
    let mut items: Vec<GraphPathItem> = Vec::new();
    for p in &paths {
        // 간단 path 문자열: A -[pred]-> B -[pred]-> C
        let mut parts: Vec<String> = Vec::new();
        for (i, node) in p.nodes.iter().enumerate() {
            parts.push(node.clone());
            if i < p.edges.len() {
                parts.push(format!(" -[{}]-> ", p.edges[i].predicate));
            }
        }
        let path_str = parts.join("");

        let nodes_json = serde_json::json!(p
            .nodes
            .iter()
            .map(|n| serde_json::json!({ "name": n }))
            .collect::<Vec<_>>());
        let rels_json = serde_json::json!(p
            .edges
            .iter()
            .map(|e| serde_json::json!({ "subject": e.subject, "predicate": e.predicate, "object": e.object, "weight": e.weight }))
            .collect::<Vec<_>>());

        items.push(GraphPathItem { path: path_str, nodes: nodes_json, relationships: rels_json });
    }

    let elapsed = t0.elapsed().as_secs_f32();
    let total = items.len() as u32;
    Ok(web::Json(GraphSearchResponse { paths: items, total, query_time: elapsed }))
}
