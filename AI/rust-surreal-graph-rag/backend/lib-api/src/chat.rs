//! 통합 질의응답 엔드포인트 (MVP)
//! - 벡터 검색 기반 RAG + 그래프 엔티티/관계 조회를 결합한 GraphRAG 확장


use actix_web::{post, web, HttpRequest, Result};
use std::time::Instant;
use std::collections::{HashSet, HashMap};

use crate::auth::require_auth;
use crate::error::Error;
use crate::models::{ChatAskRequest, ChatAskResponse, SourceItem, GraphPathItem};
use crate::search::AppState;
use lib_db::DB;

#[utoipa::path(
    tag = "chat",
    post,
    path = "/api/chat/ask",
    request_body = ChatAskRequest,
    responses(
        (status = 200, description = "질의응답 결과", body = ChatAskResponse),
        (status = 400, description = "잘못된 요청"),
        (status = 401, description = "인증 실패"),
        (status = 500, description = "서버 오류"),
    )
)]
#[post("/api/chat/ask")]
pub async fn chat_ask(state: web::Data<AppState>, req: HttpRequest, payload: web::Json<ChatAskRequest>) -> Result<web::Json<ChatAskResponse>, Error> {
    // 인증 토큰 검증
    let _user = require_auth(&req, &state.cfg)?;

    let t0 = Instant::now();

    // 1) 벡터 검색: 질의 임베딩 생성 후 chunk 테이블에서 유사도 상위 문맥 수집
    let embeddings = state
        .azure
        .embed(&[&payload.query])
        .await
        .map_err(|e| Error::External(e.to_string()))?;
    let query_vec = embeddings.get(0).cloned().unwrap_or_default();

    let mut res = DB
        .query(
            r#"
            SELECT id, doc_id, content, metadata,
                   vector::similarity::cosine(embedding_semantic, $q) AS score
            FROM chunk
            ORDER BY score DESC
            LIMIT 8;
            "#,
        )
        .bind(("q", query_vec.clone()))
        .await
        .map_err(|e| Error::External(e.to_string()))?;

    let rows: Vec<serde_json::Value> = res.take(0).unwrap_or_default();
    let mut sources: Vec<SourceItem> = Vec::new();
    let mut context_text = String::new();
    let mut doc_ids: HashSet<String> = HashSet::new();
    for v in rows {
        let score = v.get("score").and_then(|s| s.as_f64()).unwrap_or(0.0) as f32;
        let content = v
            .get("content")
            .and_then(|x| x.as_str())
            .unwrap_or_default()
            .to_string();
        let metadata = v.get("metadata").cloned().unwrap_or(serde_json::json!({}));
        if let Some(doc_id_val) = v.get("doc_id") {
            // 임시 문자열 참조를 피하고 소유하는 String으로 안전하게 변환
            let did = doc_id_val
                .as_str()
                .map(String::from)
                .unwrap_or_else(|| doc_id_val.to_string());
            if !did.is_empty() { doc_ids.insert(did); }
        }
        if !content.is_empty() {
            context_text.push_str("- ");
            context_text.push_str(&content);
            context_text.push_str("\n");
        }
        sources.push(SourceItem {
            r#type: "chunk".into(),
            content,
            score,
            metadata,
        });
    }

    // 2) 그래프 경로 확장: 상위 청크들이 속한 문서들의 엔티티/관계를 조회하여 간단 경로 구성
    //    - 추가: 엔티티/관계 임베딩과 질의 임베딩 간 코사인 유사도 기반 랭킹을 적용하여 상위 결과만 사용
    //    - 추가: 엣지 가중치(weight), 중심성(간단 도수 중심성), 엔티티 타입별 가중치 반영
    //    - 문서 집합이 비어 있으면 그래프 경로는 빈 배열로 유지
    let mut graph_paths: Vec<GraphPathItem> = Vec::new();
    if !doc_ids.is_empty() {
        // 옵션에서 그래프 임계값/상한값을 읽어옴(없으면 기본값 사용)
        let graph_threshold: f32 = payload
            .options
            .as_ref()
            .and_then(|o| o.get("graph_threshold").and_then(|v| v.as_f64()))
            .map(|v| v as f32)
            .unwrap_or(0.30);
        // 랭킹 가중치 계수(alpha: 관계 유사도, beta: 엔티티 유사도 평균, gamma: 엣지 가중치, delta: 중심성)
        let (alpha, beta, gamma, delta) = {
            let opts = payload.options.as_ref();
            let coeffs = opts.and_then(|o| o.get("graph_coeffs"));
            let a = coeffs.and_then(|c| c.get("alpha").and_then(|v| v.as_f64())).unwrap_or(0.50) as f32;
            let b = coeffs.and_then(|c| c.get("beta").and_then(|v| v.as_f64())).unwrap_or(0.30) as f32;
            let g = coeffs.and_then(|c| c.get("gamma").and_then(|v| v.as_f64())).unwrap_or(0.10) as f32;
            let d = coeffs.and_then(|c| c.get("delta").and_then(|v| v.as_f64())).unwrap_or(0.10) as f32;
            (a, b, g, d)
        };
        // 엔티티 타입별 가중치 (예: {"PERSON":1.2,"ORG":1.1,"LOC":1.0,"DATE":0.8})
        let type_weights: HashMap<String, f32> = payload
            .options
            .as_ref()
            .and_then(|o| o.get("entity_type_weights").and_then(|m| m.as_object().cloned()))
            .map(|m| m.into_iter().filter_map(|(k, v)| v.as_f64().map(|f| (k, f as f32))).collect())
            .unwrap_or_else(HashMap::new);
        let top_entities: i64 = payload
            .options
            .as_ref()
            .and_then(|o| o.get("top_entities").and_then(|v| v.as_i64()))
            .unwrap_or(50);
        let top_relations: i64 = payload
            .options
            .as_ref()
            .and_then(|o| o.get("top_relations").and_then(|v| v.as_i64()))
            .unwrap_or(100);

        // SurrealDB에서 해당 문서들의 엔티티/관계 조회
        let doc_id_list: Vec<String> = doc_ids.into_iter().collect();
        let mut gq = DB
            .query(
                r#"
                LET $doc_ids = $doc_ids;
                SELECT name, type, vector::similarity::cosine(embedding_semantic, $q) AS score
                FROM entity
                WHERE doc_id IN $doc_ids
                ORDER BY score DESC
                LIMIT $top_entities;
                SELECT subject, predicate, object, weight, vector::similarity::cosine(embedding_semantic, $q) AS score
                FROM relation
                WHERE doc_id IN $doc_ids
                ORDER BY score DESC
                LIMIT $top_relations;
                "#,
            )
            .bind(("doc_ids", doc_id_list))
            .bind(("q", query_vec.clone()))
            .bind(("top_entities", top_entities))
            .bind(("top_relations", top_relations))
            .await
            .map_err(|e| Error::External(e.to_string()))?;

        let entities_rows: Vec<serde_json::Value> = gq.take(0).unwrap_or_default();
        let relations_rows: Vec<serde_json::Value> = gq.take(1).unwrap_or_default();

        // 엔티티 정보 맵(name -> (score, type))을 구성(동일 이름은 최고 점수로 유지)
        let mut entity_info: HashMap<String, (f32, String)> = HashMap::new();
        for e in &entities_rows {
            let name = e.get("name").and_then(|x| x.as_str()).unwrap_or_default().to_string();
            let etype = e.get("type").and_then(|x| x.as_str()).unwrap_or("").to_string();
            let score = e.get("score").and_then(|x| x.as_f64()).unwrap_or(0.0) as f32;
            if name.is_empty() { continue; }
            let entry = entity_info.entry(name).or_insert((score, etype.clone()));
            if score > entry.0 { *entry = (score, etype); }
        }

        // 중심성(PageRank + Betweenness) 계산 및 엣지 가중치 최대치 추정
        // 옵션: PageRank 감쇠/반복 수, 결합 가중치
        let pr_damping: f32 = payload
            .options
            .as_ref()
            .and_then(|o| o.get("pr_damping").and_then(|v| v.as_f64()))
            .map(|v| v as f32)
            .unwrap_or(0.85);
        let pr_iters: usize = payload
            .options
            .as_ref()
            .and_then(|o| o.get("pr_iters").and_then(|v| v.as_u64()))
            .map(|v| v as usize)
            .unwrap_or(20);
        let pr_weight: f32 = payload
            .options
            .as_ref()
            .and_then(|o| o.get("pr_weight").and_then(|v| v.as_f64()))
            .map(|v| v as f32)
            .unwrap_or(0.6);
        let bc_weight: f32 = payload
            .options
            .as_ref()
            .and_then(|o| o.get("bc_weight").and_then(|v| v.as_f64()))
            .map(|v| v as f32)
            .unwrap_or(0.4);

        // 그래프 구축(유향 그래프, 중복 엣지는 합산)
        let mut out_edges: HashMap<String, HashMap<String, f32>> = HashMap::new();
        let mut nodes_all: HashSet<String> = HashSet::new();
        let mut max_weight: f32 = 1.0;
        for r in &relations_rows {
            let s = r.get("subject").and_then(|x| x.as_str()).unwrap_or_default().to_string();
            let o = r.get("object").and_then(|x| x.as_str()).unwrap_or_default().to_string();
            let w = r.get("weight").and_then(|x| x.as_f64()).unwrap_or(1.0) as f32;
            if s.is_empty() || o.is_empty() { continue; }
            nodes_all.insert(s.clone());
            nodes_all.insert(o.clone());
            let entry = out_edges.entry(s).or_insert_with(HashMap::new);
            let e = entry.entry(o).or_insert(0.0);
            *e += w;
            if w > max_weight { max_weight = w; }
        }

        // PageRank 계산
        let n = nodes_all.len().max(1) as f32;
        let mut pr: HashMap<String, f32> = nodes_all.iter().map(|k| (k.clone(), 1.0 / n)).collect();
        // out-degree 합(가중치 합) 계산
        let mut out_sum: HashMap<String, f32> = HashMap::new();
        for (u, nbrs) in &out_edges {
            let s: f32 = nbrs.values().copied().sum();
            out_sum.insert(u.clone(), if s > 0.0 { s } else { 1.0 });
        }
        for _ in 0..pr_iters {
            let mut new_pr: HashMap<String, f32> = nodes_all.iter().map(|k| (k.clone(), (1.0 - pr_damping) / n)).collect();
            for (u, nbrs) in &out_edges {
                let contrib_base = pr.get(u).cloned().unwrap_or(0.0);
                let denom = out_sum.get(u).cloned().unwrap_or(1.0);
                for (v, w) in nbrs {
                    let inc = pr_damping * contrib_base * (*w / denom);
                    *new_pr.entry(v.clone()).or_insert(0.0) += inc;
                }
            }
            pr = new_pr;
        }
        // PageRank 정규화(0~1)
        let pr_max = pr.values().cloned().fold(0.0_f32, f32::max).max(1e-6);
        for v in pr.values_mut() { *v /= pr_max; }

        // Betweenness 중심성(Brandes 알고리즘, 무가중치 근사)
        let node_list: Vec<String> = nodes_all.iter().cloned().collect();
        let index_of: HashMap<String, usize> = node_list.iter().enumerate().map(|(i, k)| (k.clone(), i)).collect();
        let mut adj: Vec<Vec<usize>> = vec![Vec::new(); node_list.len()];
        for (u, nbrs) in &out_edges {
            if let Some(&ui) = index_of.get(u) {
                for v in nbrs.keys() {
                    if let Some(&vi) = index_of.get(v) { adj[ui].push(vi); }
                }
            }
        }
        let mut bc: Vec<f32> = vec![0.0; node_list.len()];
        // Brandes: 모든 s에 대해 단일-원천 최단경로 집계(무가중치)
        for s in 0..node_list.len() {
            let mut stack: Vec<usize> = Vec::new();
            let mut pred: Vec<Vec<usize>> = vec![Vec::new(); node_list.len()];
            let mut sigma: Vec<f32> = vec![0.0; node_list.len()];
            sigma[s] = 1.0;
            let mut dist: Vec<i32> = vec![-1; node_list.len()];
            dist[s] = 0;
            // BFS
            let mut queue: std::collections::VecDeque<usize> = std::collections::VecDeque::new();
            queue.push_back(s);
            while let Some(v) = queue.pop_front() {
                stack.push(v);
                let dv = dist[v];
                for &w in &adj[v] {
                    if dist[w] < 0 { dist[w] = dv + 1; queue.push_back(w); }
                    if dist[w] == dv + 1 { sigma[w] += sigma[v]; pred[w].push(v); }
                }
            }
            // 누적 의존도 계산
            let mut delta: Vec<f32> = vec![0.0; node_list.len()];
            while let Some(w) = stack.pop() {
                for &v in &pred[w] {
                    if sigma[w] > 0.0 { delta[v] += (sigma[v] / sigma[w]) * (1.0 + delta[w]); }
                }
                if w != s { bc[w] += delta[w]; }
            }
        }
        // Betweenness 정규화(0~1)
        let bc_max = bc.iter().cloned().fold(0.0_f32, f32::max).max(1e-6);
        if bc_max > 0.0 { for v in bc.iter_mut() { *v /= bc_max; } }

        // 노드 집합(중복 제거)과 간단 경로 문자열 생성(재랭킹 결과 기반)
        let mut node_set: HashSet<String> = HashSet::new();
        let mut nodes_json: Vec<serde_json::Value> = Vec::new();
        for (name, (escore, etype)) in &entity_info {
            if *escore < graph_threshold { continue; }
            if node_set.insert(format!("{}|{}", name, etype)) {
                let tw = type_weights.get(etype).cloned().unwrap_or(1.0);
                let cen_pr = pr.get(name).cloned().unwrap_or(0.0);
                let cen_bc = index_of.get(name).map(|&i| bc[i]).unwrap_or(0.0);
                let cen = pr_weight * cen_pr + bc_weight * cen_bc;
                let ranked = beta * *escore + delta * cen; // 노드 랭킹 지표(간단)
                nodes_json.push(serde_json::json!({"name": name, "type": etype, "score": escore, "type_weight": tw, "centrality_pr": cen_pr, "centrality_bc": cen_bc, "centrality": cen, "rank": ranked}));
            }
        }

        // 관계 재랭킹: 관계 점수/엔티티 점수 평균/엣지 가중치/중심성(PR+BC)/엔티티 타입 가중치(평균)를 결합
        let mut rel_items: Vec<(f32, serde_json::Value, String)> = Vec::new();
        for r in &relations_rows {
            let s = r.get("subject").and_then(|x| x.as_str()).unwrap_or_default();
            let p = r.get("predicate").and_then(|x| x.as_str()).unwrap_or("REL");
            let o = r.get("object").and_then(|x| x.as_str()).unwrap_or_default();
            let w = r.get("weight").and_then(|x| x.as_f64()).unwrap_or(1.0) as f32;
            let r_score = r.get("score").and_then(|x| x.as_f64()).unwrap_or(0.0) as f32;
            if s.is_empty() || o.is_empty() { continue; }
            let (s_es, s_ty) = entity_info.get(s).cloned().unwrap_or((0.0, String::new()));
            let (o_es, o_ty) = entity_info.get(o).cloned().unwrap_or((0.0, String::new()));
            let ent_avg = if s_es > 0.0 || o_es > 0.0 { (s_es + o_es) / 2.0 } else { 0.0 };
            let cen_s = pr.get(s).cloned().unwrap_or(0.0) * pr_weight + index_of.get(s).map(|&i| bc[i]).unwrap_or(0.0) * bc_weight;
            let cen_o = pr.get(o).cloned().unwrap_or(0.0) * pr_weight + index_of.get(o).map(|&i| bc[i]).unwrap_or(0.0) * bc_weight;
            let cen = (cen_s + cen_o) / 2.0;
            let w_norm = if max_weight > 0.0 { w / max_weight } else { 0.0 };
            let tw_s = type_weights.get(&s_ty).cloned().unwrap_or(1.0);
            let tw_o = type_weights.get(&o_ty).cloned().unwrap_or(1.0);
            let type_mul = (tw_s + tw_o) / 2.0;
            let combined = (alpha * r_score + beta * ent_avg + gamma * w_norm + delta * cen) * type_mul;
            if combined < graph_threshold { continue; }
            let line = format!("{} -[{}]-> {} (rel={:.3}, ent={:.3}, w={:.2}, cen={:.2}, tw={:.2}, score={:.3})", s, p, o, r_score, ent_avg, w, cen, type_mul, combined);
            let json = serde_json::json!({
                "subject": s, "predicate": p, "object": o, "weight": w,
                "score_relation": r_score, "score_entity_avg": ent_avg,
                "centrality": cen, "centrality_subject": cen_s, "centrality_object": cen_o,
                "weight_norm": w_norm, "type_weight": type_mul,
                "score": combined
            });
            rel_items.push((combined, json, line));
        }
        // 점수 기준 내림차순 정렬 및 상위 제한 유지
        rel_items.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        let mut rels_json: Vec<serde_json::Value> = Vec::new();
        let mut path_lines: Vec<String> = Vec::new();
        for (i, (_sc, j, l)) in rel_items.into_iter().enumerate() {
            if (i as i64) >= top_relations { break; }
            rels_json.push(j);
            path_lines.push(l);
        }

        // 간단히 모든 관계를 하나의 경로 묶음으로 구성
        let path_str = if path_lines.is_empty() {
            String::from("")
        } else {
            path_lines.join("\n")
        };
        graph_paths.push(GraphPathItem {
            path: path_str,
            nodes: serde_json::Value::Array(nodes_json),
            relationships: serde_json::Value::Array(rels_json),
        });
    }

    // 3) LLM 호출 — 컨텍스트를 system_prompt에 포함하여 RAG + 그래프 힌트 제공
    let system_prompt = format!(
        "{}\n\n[컨텍스트]\n{}\n\n[그래프 요약]\n{}",
        "당신은 제공된 문서 청크 컨텍스트와 엔티티/관계 그래프를 활용하여 질문에 대해 간결하고 정확하게 한국어로 답변합니다. 모르는 내용은 추측하지 말고 모른다고 답하세요.",
        context_text,
        graph_paths.get(0).map(|g| g.path.as_str()).unwrap_or("")
    );
    let temperature = payload
        .options
        .as_ref()
        .and_then(|o| o.get("temperature").and_then(|v| v.as_f64()))
        .map(|v| v as f32);

    let (answer, tokens_used) = state
        .azure
        .chat_complete(&system_prompt, &payload.query, temperature)
        .await
        .map_err(|e| Error::External(e.to_string()))?;

    let elapsed = t0.elapsed().as_secs_f32();

    Ok(web::Json(ChatAskResponse {
        response: answer,
        conversation_id: payload.conversation_id.clone(),
        sources,
        graph_paths,
        query_time: elapsed,
        tokens_used,
    }))
}
