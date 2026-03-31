// =============================================================================
// @trace SPEC-002, SPEC-003
// @trace PRD: PRD-002, PRD-003
// @trace FR: PRD-002/FR-1, PRD-002/FR-2, PRD-002/FR-3, PRD-002/FR-4, PRD-002/FR-5
// @trace FR: PRD-003/FR-1, PRD-003/FR-2, PRD-003/FR-3, PRD-003/FR-4
// @trace file-type: impl
// =============================================================================

use std::path::PathBuf;
use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use axum::routing::{delete, get};
use axum::{Json, Router};
use chrono::Utc;
use serde::Serialize;
use tokio::sync::RwLock;

use crate::clock::get_clock_display;
use crate::config::{CityEntry, Config};
use crate::error::AppError;

/// 웹 서비스 공유 상태.
///
/// @trace SPEC: SPEC-002, SPEC-004
/// @trace FR: PRD-002/FR-1, PRD-002/FR-2, PRD-002/FR-3, PRD-002/FR-4, PRD-004/FR-1
pub struct AppState {
    pub config: RwLock<Config>,
    pub config_path: PathBuf,
    pub registry_path: PathBuf,
}

impl AppState {
    pub fn new(config: Config, config_path: PathBuf, registry_path: PathBuf) -> Self {
        Self {
            config: RwLock::new(config),
            config_path,
            registry_path,
        }
    }
}

/// 에러 응답 JSON 구조.
#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

/// axum 라우터를 생성한다.
///
/// @trace SPEC: SPEC-002, SPEC-003, SPEC-004
/// @trace FR: PRD-002/FR-1~4, PRD-003/FR-1, PRD-004/FR-1, PRD-004/FR-2
pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(index_html))
        .route("/trace", get(trace_html))
        .route("/api/clocks", get(get_clocks))
        .route("/api/cities", get(list_cities).post(add_city))
        .route("/api/cities/{name}", delete(remove_city))
        .route("/api/trace", get(get_trace))
        .with_state(state)
}

/// GET / — 세계 시계 웹 프론트엔드 HTML 페이지를 반환한다.
///
/// @trace SPEC: SPEC-003
/// @trace TC: SPEC-003/TC-1, SPEC-003/TC-2, SPEC-003/TC-3, SPEC-003/TC-4, SPEC-003/TC-5
/// @trace FR: PRD-003/FR-1, PRD-003/FR-2, PRD-003/FR-3, PRD-003/FR-4
async fn index_html() -> Html<&'static str> {
    Html(INDEX_HTML)
}

/// GET /api/clocks — 모든 도시의 현재 시간을 JSON으로 반환한다.
///
/// @trace SPEC: SPEC-002
/// @trace TC: SPEC-002/TC-1, SPEC-002/TC-2
/// @trace FR: PRD-002/FR-1
async fn get_clocks(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let config = state.config.read().await;
    let now = Utc::now();

    let displays: Vec<_> = config
        .cities
        .iter()
        .filter_map(|c| get_clock_display(&c.name, &c.timezone, now).ok())
        .collect();

    Json(displays)
}

/// GET /api/cities — 저장된 도시 목록을 JSON으로 반환한다.
///
/// @trace SPEC: SPEC-002
/// @trace TC: SPEC-002/TC-8
/// @trace FR: PRD-002/FR-4
async fn list_cities(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let config = state.config.read().await;
    Json(config.cities.clone())
}

/// POST /api/cities — 도시를 추가한다.
///
/// @trace SPEC: SPEC-002
/// @trace TC: SPEC-002/TC-3, SPEC-002/TC-4, SPEC-002/TC-5
/// @trace FR: PRD-002/FR-2
async fn add_city(
    State(state): State<Arc<AppState>>,
    Json(entry): Json<CityEntry>,
) -> impl IntoResponse {
    let mut config = state.config.write().await;

    if let Err(e) = config.add(entry.clone()) {
        let status = match &e {
            AppError::DuplicateCity(_) => StatusCode::CONFLICT,
            AppError::UnknownTimezone(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        return (
            status,
            Json(ErrorResponse {
                error: e.to_string(),
            }),
        )
            .into_response();
    }

    let _ = config.save(&state.config_path);

    (StatusCode::CREATED, Json(entry)).into_response()
}

/// DELETE /api/cities/{name} — 도시를 삭제한다.
///
/// @trace SPEC: SPEC-002
/// @trace TC: SPEC-002/TC-6, SPEC-002/TC-7
/// @trace FR: PRD-002/FR-3
async fn remove_city(
    State(state): State<Arc<AppState>>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    let mut config = state.config.write().await;

    if let Err(e) = config.remove(&name) {
        let status = match &e {
            AppError::CityNotFound(_) => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        return (
            status,
            Json(ErrorResponse {
                error: e.to_string(),
            }),
        )
            .into_response();
    }

    let _ = config.save(&state.config_path);

    StatusCode::NO_CONTENT.into_response()
}

/// GET /api/trace — registry.json의 추적성 데이터를 JSON으로 반환한다.
///
/// @trace SPEC: SPEC-004
/// @trace TC: SPEC-004/TC-1, SPEC-004/TC-2
/// @trace FR: PRD-004/FR-1
async fn get_trace(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    match std::fs::read_to_string(&state.registry_path) {
        Ok(content) => match serde_json::from_str::<serde_json::Value>(&content) {
            Ok(json) => Json(json).into_response(),
            Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Invalid registry JSON").into_response(),
        },
        Err(_) => Json(serde_json::json!({
            "entries": [],
            "trace_map": {}
        }))
        .into_response(),
    }
}

/// GET /trace — 추적성 맵 그래프 시각화 HTML 페이지를 반환한다.
///
/// @trace SPEC: SPEC-004
/// @trace TC: SPEC-004/TC-3, SPEC-004/TC-4, SPEC-004/TC-5, SPEC-004/TC-6, SPEC-004/TC-7
/// @trace FR: PRD-004/FR-2, PRD-004/FR-3, PRD-004/FR-4
async fn trace_html() -> Html<&'static str> {
    Html(TRACE_HTML)
}

/// 세계 시계 웹 프론트엔드 HTML.
///
/// @trace SPEC: SPEC-003
/// @trace FR: PRD-003/FR-1, PRD-003/FR-2, PRD-003/FR-3, PRD-003/FR-4
const INDEX_HTML: &str = r#"<!DOCTYPE html>
<html lang="ko">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>World Clock</title>
<style>
  *, *::before, *::after { box-sizing: border-box; margin: 0; padding: 0; }
  body {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
    background: #0f172a; color: #e2e8f0; min-height: 100vh;
    display: flex; flex-direction: column; align-items: center;
    padding: 2rem 1rem;
  }
  h1 { font-size: 2rem; margin-bottom: 1.5rem; color: #f1f5f9; }
  #add-form {
    display: flex; gap: 0.5rem; flex-wrap: wrap; justify-content: center;
    margin-bottom: 1.5rem; max-width: 600px; width: 100%;
  }
  #add-form input {
    flex: 1; min-width: 140px; padding: 0.6rem 0.8rem;
    border: 1px solid #334155; border-radius: 0.5rem;
    background: #1e293b; color: #e2e8f0; font-size: 0.9rem;
  }
  #add-form input::placeholder { color: #64748b; }
  #add-form button {
    padding: 0.6rem 1.2rem; border: none; border-radius: 0.5rem;
    background: #3b82f6; color: #fff; font-size: 0.9rem; cursor: pointer;
    font-weight: 600; transition: background 0.2s;
  }
  #add-form button:hover { background: #2563eb; }
  #error-message {
    color: #f87171; margin-bottom: 1rem; min-height: 1.2em;
    font-size: 0.85rem; text-align: center;
  }
  #clocks {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(260px, 1fr));
    gap: 1rem; max-width: 900px; width: 100%;
  }
  .clock-card {
    background: #1e293b; border: 1px solid #334155; border-radius: 0.75rem;
    padding: 1.2rem; position: relative; transition: border-color 0.2s;
  }
  .clock-card:hover { border-color: #3b82f6; }
  .clock-card .city { font-size: 1.1rem; font-weight: 600; color: #f1f5f9; }
  .clock-card .timezone { font-size: 0.75rem; color: #64748b; margin-top: 0.2rem; }
  .clock-card .time {
    font-size: 1.6rem; font-weight: 700; color: #38bdf8;
    margin-top: 0.6rem; font-variant-numeric: tabular-nums;
  }
  .clock-card .offset { font-size: 0.8rem; color: #94a3b8; margin-top: 0.2rem; }
  .clock-card .delete-btn {
    position: absolute; top: 0.8rem; right: 0.8rem;
    background: none; border: none; color: #64748b; cursor: pointer;
    font-size: 1.1rem; line-height: 1; padding: 0.2rem;
    transition: color 0.2s;
  }
  .clock-card .delete-btn:hover { color: #f87171; }
  .empty-message { color: #64748b; text-align: center; grid-column: 1/-1; padding: 2rem; }
</style>
</head>
<body>
<h1>World Clock</h1>

<form id="add-form" onsubmit="return addCity(event)">
  <input type="text" id="city-name" placeholder="도시명 (예: Seoul)" required>
  <input type="text" id="city-timezone" placeholder="타임존 (예: Asia/Seoul)" required>
  <button type="submit">추가</button>
</form>

<div id="error-message"></div>
<div id="clocks"></div>

<script>
  const clocksEl = document.getElementById('clocks');
  const errorEl = document.getElementById('error-message');

  async function fetchClocks() {
    try {
      const res = await fetch('/api/clocks');
      const data = await res.json();
      if (data.length === 0) {
        clocksEl.innerHTML = '<div class="empty-message">도시를 추가해 주세요.</div>';
        return;
      }
      clocksEl.innerHTML = data.map(c => `
        <div class="clock-card">
          <button class="delete-btn" onclick="removeCity('${c.city.replace(/'/g, "\\'")}')" title="삭제">&times;</button>
          <div class="city">${esc(c.city)}</div>
          <div class="timezone">${esc(c.timezone)}</div>
          <div class="time">${esc(c.time)}</div>
          <div class="offset">UTC ${esc(c.utc_offset)}</div>
        </div>
      `).join('');
    } catch (e) {
      console.error('fetchClocks error:', e);
    }
  }

  async function addCity(event) {
    event.preventDefault();
    errorEl.textContent = '';
    const name = document.getElementById('city-name').value.trim();
    const timezone = document.getElementById('city-timezone').value.trim();
    if (!name || !timezone) return false;

    try {
      const res = await fetch('/api/cities', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ name, timezone }),
      });
      if (!res.ok) {
        const err = await res.json();
        errorEl.textContent = err.error || '추가 실패';
        return false;
      }
      document.getElementById('city-name').value = '';
      document.getElementById('city-timezone').value = '';
      await fetchClocks();
    } catch (e) {
      errorEl.textContent = '네트워크 오류';
    }
    return false;
  }

  async function removeCity(name) {
    errorEl.textContent = '';
    try {
      const res = await fetch(`/api/cities/${encodeURIComponent(name)}`, { method: 'DELETE' });
      if (!res.ok && res.status !== 204) {
        const err = await res.json();
        errorEl.textContent = err.error || '삭제 실패';
        return;
      }
      await fetchClocks();
    } catch (e) {
      errorEl.textContent = '네트워크 오류';
    }
  }

  function esc(s) {
    const d = document.createElement('div');
    d.textContent = s;
    return d.innerHTML;
  }

  fetchClocks();
  setInterval(fetchClocks, 1000);
</script>
</body>
</html>"#;

/// 추적성 맵 그래프 시각화 HTML.
///
/// @trace SPEC: SPEC-004
/// @trace FR: PRD-004/FR-2, PRD-004/FR-3, PRD-004/FR-4
const TRACE_HTML: &str = r##"<!DOCTYPE html>
<html lang="ko">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>Traceability Map</title>
<style>
  *, *::before, *::after { box-sizing: border-box; margin: 0; padding: 0; }
  body {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
    background: #0f172a; color: #e2e8f0; min-height: 100vh;
    padding: 1.5rem;
  }
  .header {
    display: flex; align-items: center; justify-content: space-between;
    flex-wrap: wrap; gap: 1rem; margin-bottom: 1.5rem;
  }
  h1 { font-size: 1.6rem; color: #f1f5f9; }
  .nav-link { color: #38bdf8; text-decoration: none; font-size: 0.85rem; }
  .nav-link:hover { text-decoration: underline; }
  .tabs {
    display: flex; gap: 0.5rem; margin-bottom: 1rem;
  }
  .tab {
    padding: 0.5rem 1.2rem; border: 1px solid #334155; border-radius: 0.5rem;
    background: #1e293b; color: #94a3b8; cursor: pointer; font-size: 0.85rem;
    transition: all 0.2s;
  }
  .tab.active { background: #3b82f6; color: #fff; border-color: #3b82f6; }
  .tab:hover:not(.active) { border-color: #64748b; }
  #trace-graph {
    width: 100%; overflow-x: auto; background: #1e293b;
    border: 1px solid #334155; border-radius: 0.75rem; padding: 1rem;
    min-height: 400px;
  }
  svg text { font-family: inherit; }
  .node { cursor: pointer; }
  .node rect { rx: 6; ry: 6; stroke-width: 1.5; }
  .node text { fill: #f1f5f9; font-size: 11px; text-anchor: middle; dominant-baseline: central; }
  .edge { stroke: #475569; stroke-width: 1.2; fill: none; marker-end: url(#arrow); }
  .tooltip-box {
    position: fixed; background: #1e293b; border: 1px solid #475569;
    border-radius: 0.5rem; padding: 0.6rem 0.8rem; font-size: 0.78rem;
    color: #e2e8f0; pointer-events: none; display: none; z-index: 100;
    max-width: 320px; line-height: 1.5;
  }
  .legend {
    display: flex; gap: 1rem; flex-wrap: wrap; margin-bottom: 1rem; font-size: 0.78rem;
  }
  .legend-item { display: flex; align-items: center; gap: 0.35rem; }
  .legend-color {
    width: 14px; height: 14px; border-radius: 3px; display: inline-block;
  }
</style>
</head>
<body>
<div class="header">
  <h1>Traceability Map</h1>
  <a class="nav-link" href="/">&larr; World Clock</a>
</div>

<div class="legend">
  <div class="legend-item"><span class="legend-color" style="background:#a78bfa"></span> PRD</div>
  <div class="legend-item"><span class="legend-color" style="background:#60a5fa"></span> FR</div>
  <div class="legend-item"><span class="legend-color" style="background:#34d399"></span> SPEC</div>
  <div class="legend-item"><span class="legend-color" style="background:#fbbf24"></span> TC</div>
  <div class="legend-item"><span class="legend-color" style="background:#f87171"></span> CODE</div>
</div>

<div class="tabs">
  <button class="tab active" id="tab-forward" onclick="switchTab('forward')">정방향 (PRD → CODE)</button>
  <button class="tab" id="tab-reverse" onclick="switchTab('reverse')">역방향 (CODE → PRD)</button>
</div>

<div id="trace-graph"></div>
<div class="tooltip-box" id="tooltip"></div>

<script>
const COLORS = { PRD:'#a78bfa', FR:'#60a5fa', SPEC:'#34d399', TC:'#fbbf24', CODE:'#f87171' };
const STROKES = { PRD:'#8b5cf6', FR:'#3b82f6', SPEC:'#10b981', TC:'#f59e0b', CODE:'#ef4444' };
let traceData = null;
let currentDir = 'forward';

async function fetchTrace() {
  const res = await fetch('/api/trace');
  traceData = await res.json();
  render();
}

function switchTab(dir) {
  currentDir = dir;
  document.getElementById('tab-forward').classList.toggle('active', dir==='forward');
  document.getElementById('tab-reverse').classList.toggle('active', dir==='reverse');
  render();
}

function render() {
  if (!traceData || !traceData.trace_map) return;
  if (currentDir === 'forward') renderForward(traceData);
  else renderReverse(traceData);
}

function buildNodes(data) {
  const nodes = []; const edges = [];
  const tm = data.trace_map;
  const layers = { PRD:[], FR:[], SPEC:[], TC:[], CODE:[] };

  for (const [prdId, prdData] of Object.entries(tm)) {
    const prdNode = { id: prdId, type:'PRD', label: prdId, detail: (data.entries||[]).find(e=>e.id===prdId)?.title || '' };
    layers.PRD.push(prdNode); nodes.push(prdNode);

    for (const [frId, fr] of Object.entries(prdData.fr||{})) {
      const frNodeId = prdId+'/'+frId;
      const frNode = { id: frNodeId, type:'FR', label: frId, detail: fr.title||'' };
      layers.FR.push(frNode); nodes.push(frNode);
      edges.push({ from: prdId, to: frNodeId });

      for (const specId of (fr.specs||[])) {
        let specNode = nodes.find(n=>n.id===specId);
        if (!specNode) {
          specNode = { id: specId, type:'SPEC', label: specId, detail: (data.entries||[]).find(e=>e.id===specId)?.title || '' };
          layers.SPEC.push(specNode); nodes.push(specNode);
        }
        edges.push({ from: frNodeId, to: specId });
      }

      for (const tc of (fr.test_cases||[])) {
        let tcNode = nodes.find(n=>n.id===tc);
        if (!tcNode) {
          tcNode = { id: tc, type:'TC', label: tc.split('/').pop(), detail: tc };
          layers.TC.push(tcNode); nodes.push(tcNode);
        }
        const specPart = tc.split('/')[0];
        edges.push({ from: specPart, to: tc });
      }

      for (let i = 0; i < (fr.impl_files||[]).length; i++) {
        const f = fr.impl_files[i];
        const sym = (fr.impl_symbols||[])[i] || f;
        const codeId = f + ':' + sym;
        let codeNode = nodes.find(n=>n.id===codeId);
        if (!codeNode) {
          codeNode = { id: codeId, type:'CODE', label: sym, detail: f };
          layers.CODE.push(codeNode); nodes.push(codeNode);
        }
        for (const tc of (fr.test_cases||[])) {
          if (nodes.find(n=>n.id===tc)) edges.push({ from: tc, to: codeId });
        }
      }
    }
  }
  return { nodes, edges, layers };
}

function layout(layers, dir) {
  const order = dir==='forward' ? ['PRD','FR','SPEC','TC','CODE'] : ['CODE','TC','SPEC','FR','PRD'];
  const positions = {};
  const colW = 180, nodeH = 32, padY = 14, startX = 40, startY = 60;

  for (let col = 0; col < order.length; col++) {
    const type = order[col];
    const items = layers[type] || [];
    const unique = [...new Map(items.map(n=>[n.id,n])).values()];
    for (let row = 0; row < unique.length; row++) {
      positions[unique[row].id] = {
        x: startX + col * colW,
        y: startY + row * (nodeH + padY),
        w: 140, h: nodeH,
        type: unique[row].type
      };
    }
  }
  return positions;
}

function renderGraph(nodes, edges, positions) {
  const maxX = Math.max(...Object.values(positions).map(p=>p.x+p.w)) + 60;
  const maxY = Math.max(...Object.values(positions).map(p=>p.y+p.h)) + 60;

  let svg = `<svg xmlns="http://www.w3.org/2000/svg" width="${maxX}" height="${maxY}">`;
  svg += `<defs><marker id="arrow" viewBox="0 0 10 10" refX="10" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse"><path d="M 0 0 L 10 5 L 0 10 z" fill="#475569"/></marker></defs>`;

  const drawnEdges = new Set();
  for (const e of edges) {
    const key = e.from+'->'+e.to;
    if (drawnEdges.has(key)) continue;
    drawnEdges.add(key);
    const f = positions[e.from], t = positions[e.to];
    if (!f || !t) continue;
    const x1 = f.x+f.w, y1 = f.y+f.h/2, x2 = t.x, y2 = t.y+t.h/2;
    svg += `<line class="edge" x1="${x1}" y1="${y1}" x2="${x2}" y2="${y2}"/>`;
  }

  const drawn = new Set();
  for (const n of nodes) {
    if (drawn.has(n.id)) continue;
    drawn.add(n.id);
    const p = positions[n.id];
    if (!p) continue;
    const fill = COLORS[n.type]; const stroke = STROKES[n.type];
    const label = n.label.length > 18 ? n.label.slice(0,16)+'..' : n.label;
    svg += `<g class="node" onmouseover="showTip(evt,'${esc(n.id)}','${esc(n.detail)}','${n.type}')" onmouseout="hideTip()">`;
    svg += `<rect x="${p.x}" y="${p.y}" width="${p.w}" height="${p.h}" fill="${fill}22" stroke="${stroke}"/>`;
    svg += `<text x="${p.x+p.w/2}" y="${p.y+p.h/2}">${esc(label)}</text>`;
    svg += `</g>`;
  }

  svg += `</svg>`;
  document.getElementById('trace-graph').innerHTML = svg;
}

function renderForward(data) {
  const { nodes, edges, layers } = buildNodes(data);
  const positions = layout(layers, 'forward');
  renderGraph(nodes, edges, positions);
}

function renderReverse(data) {
  const { nodes, edges, layers } = buildNodes(data);
  const revEdges = edges.map(e => ({ from: e.to, to: e.from }));
  const positions = layout(layers, 'reverse');
  renderGraph(nodes, revEdges, positions);
}

function esc(s) { return String(s).replace(/&/g,'&amp;').replace(/</g,'&lt;').replace(/>/g,'&gt;').replace(/"/g,'&quot;').replace(/'/g,'&#39;'); }

const tooltip = document.getElementById('tooltip');
function showTip(evt, id, detail, type) {
  tooltip.innerHTML = `<strong>[${type}]</strong> ${esc(id)}<br/>${esc(detail)}`;
  tooltip.style.display = 'block';
  tooltip.style.left = (evt.clientX + 12) + 'px';
  tooltip.style.top = (evt.clientY + 12) + 'px';
}
function hideTip() { tooltip.style.display = 'none'; }

fetchTrace();
</script>
</body>
</html>"##;
