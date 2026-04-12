    // ---------- i18n ----------
    const I18N = {
      ko: {
        "header.sub": "— PPA 루프 에이전트 시나리오 실행·궤적 기록·골든셋 채점 하네스",
        "header.help": "📖 사용안내",
        "nav.execute": "실행",
        "execute.single": "단일",
        "execute.batch": "전체",
        "nav.run": "전체 평가 시나리오 실행",
        "nav.scenarios": "단일 평가 시나리오 실행",
        "nav.tools": "도구",
        "nav.agents": "에이전트",
        "nav.reports": "리포트",
        "nav.trajectories": "궤적",
        "nav.manage": "시나리오/골든셋",
        "nav.domains": "도메인",
        "domains.title": "도메인",
        "domains.refresh": "새로고침",
        "domains.new": "+ 도메인 추가",
        "domains.list": "도메인 목록",
        "domains.editor": "도메인 편집기",
        "domains.name": "이름 (소문자, A-Z 0-9 _ -)",
        "domains.desc": "설명",
        "domains.tools": "도구 (컴파일된 카탈로그 — 다중 선택)",
        "domains.keywords": "라우터 키워드 (쉼표로 구분)",
        "domains.keywordsPh": "환자, 처방, patient, prescription, ...",
        "domains.save": "저장",
        "domains.delete": "삭제",
        "domains.clear": "초기화",
        "domains.newTitle": "새 도메인",
        "domains.newHint": "POST /api/domains",
        "domains.editTitle": "편집: ",
        "domains.editHint": "PUT /api/domains/",
        "domains.bootstrap": " (부트스트랩)",
        "domains.selectFirst": "먼저 기존 도메인을 선택하세요",
        "domains.cantDeleteBootstrap": "부트스트랩 도메인은 삭제할 수 없습니다",
        "domains.confirmDelete": "정말 삭제?",
        "domains.deleted": "삭제됨: ",
        "domains.error": "오류: ",
        "extools.title": "외부 HTTP 도구",
        "extools.hint": "선택된 도메인의 외부 HTTP 도구를 등록/관리합니다. 사용자가 띄운 외부 서비스를 LLM 도구로 노출.",
        "prompts.title": "프롬프트 세트 (PromptSet)",
        "prompts.hint": "선택된 도메인의 system/user 프롬프트 4종을 번들로 버전관리합니다. 편집 = 새 버전 저장.",
        "prompts.notes": "메모",
        "prompts.saveNew": "새 버전으로 저장",
        "prompts.activate": "이 버전 활성화",
        "prompts.delete": "삭제",
        "prompts.clear": "초기화",
        "prompts.selectFirst": "먼저 도메인을 선택하세요",
        "prompts.selectVersionFirst": "먼저 버전을 선택하세요",
        "prompts.compareWith": "비교 대상",
        "prompts.compare": "차이 보기",
        "prompts.compareNeedTwo": "비교하려면 현재 버전과 비교 대상 버전을 모두 선택하세요",
        "extools.name": "이름",
        "extools.desc": "설명",
        "extools.method": "메서드",
        "extools.url": "URL",
        "extools.headers": "헤더 (JSON 객체, 선택)",
        "extools.body": "본문 템플릿 ({{var}} placeholder)",
        "extools.schema": "파라미터 스키마 (JSON Schema)",
        "extools.timeout": "타임아웃 (ms)",
        "extools.save": "저장",
        "extools.delete": "삭제",
        "extools.clear": "초기화",
        "extools.selectDomainFirst": "먼저 Domains 목록에서 도메인을 선택하세요.",
        "extools.selectToolFirst": "먼저 기존 도구를 선택하세요",
        "extools.confirmDelete": "정말 삭제?",
        "extools.deleted": "삭제됨: ",
        "extools.loadError": "로드 오류: ",
        "extools.error": "오류: ",
        "manage.title": "시나리오/골든셋 관리 (CRUD)",
        "manage.domain": "도메인",
        "manage.refresh": "새로고침",
        "manage.newScen": "+ 시나리오 추가",
        "manage.newGold": "+ 골든셋 추가",
        "manage.scenarios": "시나리오",
        "manage.goldens": "골든셋",
        "manage.editor": "편집기",
        "manage.save": "저장",
        "manage.delete": "삭제",
        "manage.clear": "초기화",
        "manage.pickScenario": "시나리오 선택",
        "run.title": "평가 시나리오 실행",
        "run.evalScenario": "평가 시나리오",
        "run.agent": "에이전트",
        "run.output": "출력",
        "run.outputPh": "(선택) my_report.json",
        "run.button": "실행 → POST /api/run",
        "run.hint": "POST /api/run 을 호출합니다. reports_dir 에 항상 저장됩니다 (output 생략 시 기본 파일명 사용).",
        "common.idle": "준비됨",
        "common.running": "실행 중...",
        "common.requesting": "요청 중…",
        "common.optional": "(선택)",
        "scenarios.title": "시나리오 (탐색 + 단일 실행)",
        "scenarios.empty": "시나리오를 선택하세요",
        "scenarios.formTitle": "이 시나리오 실행",
        "scenarios.runBtn": "실행 → POST /api/scenarios/:d/:id/run",
        "tools.title": "도구 (호출 + 폴트 주입)",
        "tools.empty": "도구를 선택하세요",
        "tools.params": "매개변수 (JSON)",
        "tools.invoke": "호출 → POST /api/tools/:n/invoke",
        "tools.fault": "폴트 시뮬레이션 → /simulate-fault",
        "agents.title": "에이전트 (직접 실행)",
        "agents.task": "태스크 설명",
        "agents.env": "환경 (JSON, 선택)",
        "agents.domain": "도메인 (선택)",
        "agents.execute": "실행 → POST /api/agents/:n/execute",
        "agents.loadExample": "예시 불러오기",
        "reports.title": "리포트 (조회 + 비교)",
        "reports.compareTitle": "두 리포트 비교",
        "reports.baseline": "기준",
        "reports.current": "현재",
        "reports.threshold": "임계값",
        "reports.compareBtn": "비교 → POST /api/compare",
        "traj.title": "궤적 (조회 + 채점)",
        "traj.refresh": "새로고침 → GET /api/trajectories",
        "traj.score": "이 궤적 채점 → POST /api/score",
        "traj.selectHint": "궤적을 선택하세요",
      },
      en: {
        "header.sub": "— Harness for running PPA-loop agents: scenarios, trajectory logging, and golden-set scoring",
        "header.help": "📖 Help",
        "nav.execute": "Execute",
        "execute.single": "Single",
        "execute.batch": "Batch",
        "nav.run": "Run all eval scenarios",
        "nav.scenarios": "Run single eval scenario",
        "nav.tools": "Tools",
        "nav.agents": "Agents",
        "nav.reports": "Reports",
        "nav.trajectories": "Trajectories",
        "nav.manage": "Scenarios/Goldens",
        "nav.domains": "Domains",
        "domains.title": "Domains",
        "domains.refresh": "Refresh",
        "domains.new": "+ New domain",
        "domains.list": "Domains",
        "domains.editor": "Domain editor",
        "domains.name": "Name (lowercase, A-Z 0-9 _ -)",
        "domains.desc": "Description",
        "domains.tools": "Tools (compiled catalog — multi-select)",
        "domains.keywords": "Router keywords (comma-separated)",
        "domains.keywordsPh": "patient, prescription, 환자, 처방, ...",
        "domains.save": "Save",
        "domains.delete": "Delete",
        "domains.clear": "Clear",
        "domains.newTitle": "New domain",
        "domains.newHint": "POST /api/domains",
        "domains.editTitle": "Edit: ",
        "domains.editHint": "PUT /api/domains/",
        "domains.bootstrap": " (bootstrap)",
        "domains.selectFirst": "select an existing domain first",
        "domains.cantDeleteBootstrap": "bootstrap domain cannot be deleted",
        "domains.confirmDelete": "Really delete?",
        "domains.deleted": "deleted: ",
        "domains.error": "error: ",
        "extools.title": "External HTTP Tools",
        "extools.hint": "Register/manage external HTTP tools for the selected domain. Exposes user-hosted services as LLM tools.",
        "prompts.title": "Prompt sets",
        "prompts.hint": "Bundle of 4 system/user templates per domain, versioned. Editing always creates a new version.",
        "prompts.notes": "notes",
        "prompts.saveNew": "Save as new version",
        "prompts.activate": "Activate this version",
        "prompts.delete": "Delete",
        "prompts.clear": "Clear",
        "prompts.selectFirst": "Select a domain first",
        "prompts.selectVersionFirst": "Select a version first",
        "prompts.compareWith": "compare with",
        "prompts.compare": "Show diff",
        "prompts.compareNeedTwo": "Select both the current version and a target to compare",
        "extools.name": "name",
        "extools.desc": "description",
        "extools.method": "method",
        "extools.url": "url",
        "extools.headers": "headers (JSON object, optional)",
        "extools.body": "body_template ({{var}} placeholders)",
        "extools.schema": "params_schema (JSON Schema)",
        "extools.timeout": "timeout_ms",
        "extools.save": "Save",
        "extools.delete": "Delete",
        "extools.clear": "Clear",
        "extools.selectDomainFirst": "Select a domain from the Domains list first.",
        "extools.selectToolFirst": "select an existing tool first",
        "extools.confirmDelete": "Really delete?",
        "extools.deleted": "deleted: ",
        "extools.loadError": "load error: ",
        "extools.error": "error: ",
        "run.title": "Run eval scenario",
        "run.evalScenario": "Eval Scenario",
        "run.agent": "Agent",
        "run.output": "Output",
        "run.outputPh": "(optional) my_report.json",
        "run.button": "Run → POST /api/run",
        "run.hint": "Calls POST /api/run. Always saves to reports_dir (default timestamp name if output omitted).",
        "common.idle": "Ready",
        "common.running": "running...",
        "common.requesting": "Requesting…",
        "common.optional": "(optional)",
        "scenarios.title": "Scenarios (list + single run)",
        "scenarios.empty": "select a scenario",
        "scenarios.formTitle": "Run this scenario",
        "scenarios.runBtn": "Run → POST /api/scenarios/:d/:id/run",
        "tools.title": "Tools (invoke + fault injection)",
        "tools.empty": "select a tool",
        "tools.params": "Parameters (JSON)",
        "tools.invoke": "Invoke → POST /api/tools/:n/invoke",
        "tools.fault": "Simulate fault → /simulate-fault",
        "agents.title": "Agents (direct execute)",
        "agents.task": "Task description",
        "agents.env": "Environment (JSON, optional)",
        "agents.domain": "Domain (optional)",
        "agents.execute": "Execute → POST /api/agents/:n/execute",
        "agents.loadExample": "Load example",
        "reports.title": "Reports (view + compare)",
        "reports.compareTitle": "Compare two reports",
        "reports.baseline": "Baseline",
        "reports.current": "Current",
        "reports.threshold": "Threshold",
        "reports.compareBtn": "Compare → POST /api/compare",
        "traj.title": "Trajectories (view + score)",
        "traj.refresh": "Refresh → GET /api/trajectories",
        "traj.score": "Score this trajectory → POST /api/score",
        "traj.selectHint": "select a trajectory",
      }
    };
    function t(key) { const lang = currentLang(); return (I18N[lang] && I18N[lang][key]) || (I18N.ko && I18N.ko[key]) || key; }
    function currentLang() { return localStorage.getItem('lang') || 'ko'; }
    function setLang(lang) {
      localStorage.setItem('lang', lang);
      document.documentElement.lang = lang;
      document.querySelectorAll('.lang-btn').forEach(b => b.classList.toggle('active', b.dataset.lang === lang));
      document.querySelectorAll('[data-i18n]').forEach(el => { el.textContent = t(el.dataset.i18n); });
      document.querySelectorAll('[data-i18n-placeholder]').forEach(el => { el.placeholder = t(el.dataset.i18nPlaceholder); });
    }
    document.querySelectorAll('.lang-btn').forEach(btn => { btn.onclick = () => setLang(btn.dataset.lang); });
    setLang(currentLang());

    // ---------- theme toggle (SPEC-014) ----------
    function currentTheme() { return localStorage.getItem('theme') || 'light'; }
    function setTheme(theme) {
      localStorage.setItem('theme', theme);
      document.documentElement.setAttribute('data-theme', theme);
      document.querySelectorAll('.theme-btn').forEach(b => b.classList.toggle('active', b.dataset.theme === theme));
    }
    function initTheme() { setTheme(currentTheme()); }
    document.querySelectorAll('.theme-btn').forEach(btn => { btn.onclick = () => setTheme(btn.dataset.theme); });
    initTheme();

    // ---------- utilities ----------
    const $ = (id) => document.getElementById(id);
    const API = {
      get: async (p) => { const r = await fetch(p); if (!r.ok) throw new Error(r.status+' '+await r.text()); return r.json(); },
      post: async (p, body) => {
        const r = await fetch(p, {method:'POST', headers:{'content-type':'application/json'}, body: JSON.stringify(body)});
        if (!r.ok) throw new Error(r.status+' '+await r.text());
        return r.json();
      },
      put: async (p, body) => {
        const r = await fetch(p, {method:'PUT', headers:{'content-type':'application/json'}, body: JSON.stringify(body)});
        if (!r.ok) throw new Error(r.status+' '+await r.text());
        return r.json();
      },
      del: async (p) => {
        const r = await fetch(p, {method:'DELETE'});
        if (!r.ok) throw new Error(r.status+' '+await r.text());
        return null;
      }
    };
    const pretty = (o) => JSON.stringify(o, null, 2);
    const showOk  = (el, data) => { el.className = 'ok'; el.textContent = pretty(data); };
    const showErr = (el, err)  => { el.className = 'error'; el.textContent = 'ERROR: ' + (err.message || err); };
    const showPending = (el, label) => { el.className = ''; el.textContent = (label || t('common.running') || 'running...'); };

    // ---------- tabs ----------
    document.querySelectorAll('#tab-bar button').forEach(btn => {
      btn.onclick = () => {
        document.querySelectorAll('#tab-bar button').forEach(b => b.classList.remove('active'));
        document.querySelectorAll('.panel').forEach(p => p.classList.remove('active'));
        btn.classList.add('active');
        const tab = btn.dataset.tab;
        $('panel-' + tab).classList.add('active');
        if (!initialized[tab]) { (initFns[tab] || (()=>{}))(); initialized[tab] = true; }
      };
    });
    const initialized = {};

    // ---------- shared agent list fill ----------
    let AGENTS = [];
    async function loadAgents() {
      AGENTS = await API.get('/api/agents');
      ['run-agent','scen-agent','agent-name'].forEach(id => {
        const sel = $(id); if (!sel) return;
        sel.innerHTML = AGENTS.map(a=>`<option>${a}</option>`).join('');
      });
    }

    // ---------- RUN ----------
    async function runEvalScenario(e) {
      e.preventDefault();
      const body = { eval_scenario: $('run-eval-scenario').value, agent: $('run-agent').value };
      const o = $('run-output').value.trim();
      if (o) body.output = o;
      showPending($('run-out'));
      try { showOk($('run-out'), await API.post('/api/run', body)); } catch (err) { showErr($('run-out'), err); }
    }

    // ---------- SCENARIOS ----------
    let SELECTED_SCEN = null;
    async function initScenarios() {
      const data = await API.get('/api/list');
      const el = $('scenarios-list'); el.innerHTML = '';
      data.domains.forEach(d => {
        const g = document.createElement('div'); g.className = 'group'; g.textContent = d.name; el.appendChild(g);
        d.scenarios.forEach(s => {
          const it = document.createElement('div'); it.className = 'item';
          it.innerHTML = `${s.id} — ${s.name}<span class="badge">${s.difficulty}</span>`;
          it.onclick = async () => {
            document.querySelectorAll('#scenarios-list .item').forEach(x=>x.classList.remove('active'));
            it.classList.add('active');
            SELECTED_SCEN = { domain: d.name, id: s.id };
            try {
              const full = await API.get(`/api/scenarios/${d.name}/${s.id}`);
              $('scenario-detail').innerHTML = `<h3>${full.name}</h3>
                <div><strong>${t('manage.domain')}:</strong> ${d.name} · <strong>difficulty:</strong> ${full.difficulty}</div>
                <div><strong>ID:</strong> ${full.id}</div>
                <div style="margin-top:6px"><small>${full.task_description}</small></div>
                <div style="margin-top:6px"><strong>tools:</strong> ${(full.expected_tools||[]).join(', ') || '—'}</div>`;
            } catch (err) { $('scenario-detail').innerHTML = `<div class="error">${err.message}</div>`; }
          };
          el.appendChild(it);
        });
      });
    }
    async function runScenario() {
      if (!SELECTED_SCEN) return showErr($('scen-out'), 'select a scenario first');
      const body = { agent: $('scen-agent').value };
      showPending($('scen-out'));
      try {
        showOk($('scen-out'), await API.post(`/api/scenarios/${SELECTED_SCEN.domain}/${SELECTED_SCEN.id}/run`, body));
      } catch (err) { showErr($('scen-out'), err); }
    }

    // ---------- TOOLS ----------
    let SELECTED_TOOL = null;
    async function initTools() {
      const tools = await API.get('/api/tools');
      const el = $('tools-list'); el.innerHTML = '';
      tools.forEach(tool => {
        const it = document.createElement('div'); it.className = 'item';
        it.textContent = tool.name;
        it.onclick = () => {
          document.querySelectorAll('#tools-list .item').forEach(x=>x.classList.remove('active'));
          it.classList.add('active');
          SELECTED_TOOL = tool.name;
          $('tool-detail').innerHTML = `<h3>${tool.name}</h3><div>${tool.description||''}</div><pre style="max-height:200px">${pretty(tool.parameters_schema||{})}</pre>`;
        };
        el.appendChild(it);
      });
    }
    async function invokeTool() {
      if (!SELECTED_TOOL) return showErr($('tool-out'), 'select a tool first');
      let params = {};
      try { params = JSON.parse($('tool-params').value || '{}'); } catch (e) { return showErr($('tool-out'), 'params is not valid JSON'); }
      showPending($('tool-out'));
      try { showOk($('tool-out'), await API.post(`/api/tools/${SELECTED_TOOL}/invoke`, { params })); } catch (err) { showErr($('tool-out'), err); }
    }
    async function invokeToolFault() {
      if (!SELECTED_TOOL) return showErr($('tool-out'), 'select a tool first');
      let params = {};
      try { params = JSON.parse($('tool-params').value || '{}'); } catch (e) { return showErr($('tool-out'), 'params is not valid JSON'); }
      showPending($('tool-out'));
      const seedStr = $('tool-seed').value;
      const config = {
        enabled: true,
        global_failure_rate: parseFloat($('tool-rate').value) || 0.5,
        tool_specific_rates: {},
        failure_mode_distribution: { timeout:0.2, partial_result:0.25, incorrect_result:0.2, exception:0.2, network_error:0.1, permission_denied:0.05 },
        seed: seedStr === '' ? null : parseInt(seedStr)
      };
      try { showOk($('tool-out'), await API.post(`/api/tools/${SELECTED_TOOL}/simulate-fault`, { params, config })); } catch (err) { showErr($('tool-out'), err); }
    }

    // ---------- AGENTS ----------
    // key: domain name ("" = no domain). Each entry: array of {label, task, env}.
    const AGENT_EXAMPLES = {
      "": [
        { label: "hello world",
          task: "hello world",
          env: {} },
      ],
      customer_service: [
        { label: "cs_001 · 고객 문의 분류",
          task: "고객으로부터 '주문한 상품이 불량이라 환불받고 싶습니다'라는 문의가 접수되었습니다. 문의를 적절한 카테고리로 분류하고 우선순위를 결정해주세요.",
          env: { inquiry_text: "주문한 상품이 불량이라 환불받고 싶습니다", customer_id: "C001" } },
        { label: "cs_002 · 환불 요청 처리",
          task: "고객이 주문번호 ORD-101의 상품에 대해 불량을 사유로 50,000원 환불을 요청했습니다. 문의를 분류한 후 환불을 처리해주세요.",
          env: { inquiry_text: "상품이 불량이라 환불 요청합니다", customer_id: "C010", order_id: "ORD-101", amount: 50000, reason: "상품 불량" } },
        { label: "cs_003 · 불만 에스컬레이션",
          task: "고객이 '도대체 왜 이렇게 서비스가 엉망인가요!'라며 강하게 불만을 표현하고 있습니다. 문의를 분류하고 심각도가 높으므로 상위 담당자에게 에스컬레이션해주세요.",
          env: { inquiry_text: "도대체 왜 이렇게 서비스가 엉망인가요! 화가 납니다!", customer_id: "C020", issue_id: "ISS-201", severity: "high", current_agent: "agent_01" } },
      ],
      financial: [
        { label: "fin_001 · 단리 이자 계산",
          task: "고객이 1,000,000원을 연 5% 금리로 2년간 예금했습니다. 단리 방식으로 이자를 계산하고 총 금액을 알려주세요.",
          env: { customer_id: "C001", deposit_amount: 1000000, interest_rate: 0.05, period_years: 2, calculation_method: "simple" } },
        { label: "fin_002 · 복리 이자 계산",
          task: "고객이 1,000,000원을 연 5% 금리로 2년간 예금했습니다. 월복리 방식으로 이자를 계산하고, 단리와 비교하여 어느 것이 더 유리한지 분석해주세요.",
          env: { customer_id: "C002", deposit_amount: 1000000, interest_rate: 0.05, period_years: 2, compounding_frequency: 12, calculation_method: "compound" } },
      ],
    };

    function rebuildAgentExampleSelect() {
      const domain = $('agent-domain').value || "";
      const list = AGENT_EXAMPLES[domain] || AGENT_EXAMPLES[""];
      const sel = $('agent-example-select');
      sel.innerHTML = list.map((ex, i) => `<option value="${i}">${ex.label}</option>`).join('');
    }

    function loadAgentExample() {
      const domain = $('agent-domain').value || "";
      const list = AGENT_EXAMPLES[domain] || AGENT_EXAMPLES[""];
      const idx = parseInt($('agent-example-select').value) || 0;
      const ex = list[idx] || list[0];
      if (!ex) return;
      $('agent-task').value = ex.task;
      $('agent-env').value = JSON.stringify(ex.env, null, 2);
    }

    async function initAgentsPanel() {
      if (AGENTS.length === 0) await loadAgents();
      try {
        const data = await API.get('/api/list');
        const sel = $('agent-domain');
        const none = sel.querySelector('option[value=""]');
        sel.innerHTML = '';
        if (none) sel.appendChild(none);
        (data.domains || []).forEach(d => {
          const opt = document.createElement('option');
          opt.value = d.name; opt.textContent = d.name;
          sel.appendChild(opt);
        });
        sel.onchange = () => { rebuildAgentExampleSelect(); loadAgentExample(); };
        // preselect first real domain if available
        const firstDomain = (data.domains || [])[0];
        if (firstDomain) sel.value = firstDomain.name;
      } catch (e) { /* domain list optional */ }
      rebuildAgentExampleSelect();
      loadAgentExample();
    }
    async function executeAgent() {
      let env = null;
      const envText = $('agent-env').value.trim();
      if (envText) { try { env = JSON.parse(envText); } catch (e) { return showErr($('agent-out'), 'environment is not valid JSON'); } }
      const body = { task: $('agent-task').value };
      if (env) body.environment = env;
      const dom = $('agent-domain').value;
      if (dom) body.domain = dom;
      showPending($('agent-out'));
      try { showOk($('agent-out'), await API.post(`/api/agents/${$('agent-name').value}/execute`, body)); } catch (err) { showErr($('agent-out'), err); }
    }

    // ---------- REPORTS ----------
    async function initReports() {
      const files = await API.get('/api/reports');
      const el = $('reports-list'); el.innerHTML = '';
      files.forEach(name => {
        const it = document.createElement('div'); it.className = 'item'; it.textContent = name;
        it.onclick = async () => {
          document.querySelectorAll('#reports-list .item').forEach(x=>x.classList.remove('active'));
          it.classList.add('active');
          showPending($('report-out'));
          try { showOk($('report-out'), await API.get(`/api/reports/${encodeURIComponent(name)}`)); } catch (err) { showErr($('report-out'), err); }
        };
        el.appendChild(it);
      });
      ['cmp-base','cmp-cur'].forEach(id => { $(id).innerHTML = files.map(f=>`<option>${f}</option>`).join(''); });
    }
    async function compareReports() {
      const body = {
        baseline: $('cmp-base').value,
        current: $('cmp-cur').value,
        threshold: parseFloat($('cmp-threshold').value) || 5.0,
      };
      const o = $('cmp-output').value.trim(); if (o) body.output = o;
      showPending($('report-out'));
      try { showOk($('report-out'), await API.post('/api/compare', body)); } catch (err) { showErr($('report-out'), err); }
    }

    // ---------- TRAJECTORIES ----------
    let SELECTED_TRAJ_DATA = null;
    async function initTrajectories() { await refreshTrajectories(); }
    async function refreshTrajectories() {
      const files = await API.get('/api/trajectories');
      const el = $('trajectories-list'); el.innerHTML = '';
      if (files.length === 0) el.innerHTML = '<div class="item" style="color:#666;font-style:italic">empty</div>';
      files.forEach(name => {
        const it = document.createElement('div'); it.className = 'item'; it.textContent = name;
        it.onclick = async () => {
          document.querySelectorAll('#trajectories-list .item').forEach(x=>x.classList.remove('active'));
          it.classList.add('active');
          showPending($('traj-out'));
          $('traj-badge').textContent = '';
          try {
            SELECTED_TRAJ_DATA = await API.get(`/api/trajectories/${encodeURIComponent(name)}`);
            showOk($('traj-out'), SELECTED_TRAJ_DATA);
            renderTrajBadge(SELECTED_TRAJ_DATA);
          } catch (err) { showErr($('traj-out'), err); }
        };
        el.appendChild(it);
      });
    }
    async function scoreTrajectory() {
      if (!SELECTED_TRAJ_DATA) return showErr($('traj-out'), 'select a trajectory first');
      showPending($('traj-out'));
      try {
        const res = await API.post('/api/score', { trajectory: SELECTED_TRAJ_DATA });
        showOk($('traj-out'), res);
        renderTrajBadge(SELECTED_TRAJ_DATA);
      } catch (err) { showErr($('traj-out'), err); }
    }

    // 선택된 궤적의 prompt_set_id 가 있으면 Domains 탭 배지 형식으로 표시.
    // 도메인이 같이 있으면 해당 도메인의 prompts 목록에서 version 번호까지
    // 해석해 "prompt_set v{N} (id=#)" 로 보여준다. 실패해도 id 만 표시.
    async function renderTrajBadge(data) {
      const el = $('traj-badge');
      if (!el) return;
      const traj = (data && data.trajectory) || data || {};
      const psid = traj.prompt_set_id;
      if (psid == null) { el.textContent = ''; return; }
      const domain = traj.domain || '';
      // 기본 표시 (즉시)
      el.textContent = `prompt_set #${psid}` + (domain ? ` · domain=${domain}` : '');
      if (!domain) return;
      try {
        const rows = await API.get('/api/domains/' + encodeURIComponent(domain) + '/prompts');
        const match = (rows || []).find(r => r.id === psid);
        if (match) {
          const active = match.is_active ? ' [active]' : '';
          const boot = match.is_bootstrap ? ' [bootstrap]' : '';
          el.textContent = `prompt_set v${match.version}${active}${boot} · domain=${domain} · id=${psid}`;
        }
      } catch (e) { /* 조회 실패 시 초기 표시 유지 */ }
    }

    // ---------- MANAGE (CRUD — SPEC-019) ----------
    // 편집 모드 상태: {kind: 'scenario'|'golden', mode: 'new'|'edit', domain, id}
    let MANAGE_STATE = null;
    // 현재 도메인의 시나리오 id 목록 (골든셋 생성 시 드롭다운 소스)
    let MANAGE_SCEN_IDS = [];

    async function initManage() {
      // 기존 /api/scenarios 로 도메인 목록만 뽑아 드롭다운 채우기
      const data = await API.get('/api/scenarios');
      const sel = $('manage-domain');
      sel.innerHTML = '';
      data.forEach(d => {
        const opt = document.createElement('option');
        opt.value = d.name; opt.textContent = d.name;
        sel.appendChild(opt);
      });
      sel.onchange = () => refreshManage();
      await refreshManage();
    }

    async function refreshManage() {
      const domain = $('manage-domain').value;
      if (!domain) return;
      // 시나리오 목록 (리스트 API 는 id/name/difficulty 만 반환하므로
      // 클릭 시 상세 API 로 전체 필드를 다시 로드한다)
      try {
        const data = await API.get('/api/scenarios');
        const d = data.find(x => x.name === domain);
        MANAGE_SCEN_IDS = (d?.scenarios || []).map(s => ({id: s.id, name: s.name}));
        const listEl = $('manage-scen-list'); listEl.innerHTML = '';
        (d?.scenarios || []).forEach(s => {
          const it = document.createElement('div');
          it.className = 'item';
          it.textContent = `${s.id} — ${s.name}`;
          it.onclick = async () => {
            try {
              const full = await API.get(`/api/scenarios/${domain}/${encodeURIComponent(s.id)}`);
              editScenario(domain, full);
            } catch (e) { showErr($('manage-out'), e); }
          };
          listEl.appendChild(it);
        });
      } catch (e) { showErr($('manage-out'), e); }
      // 골든셋 목록 (GET /api/golden-sets 는 전체 파일 반환 → 클라이언트에서 필터)
      try {
        const all = await API.get('/api/golden-sets');
        const gs = all.find(x => x.domain === domain) || {domain, version: '1.0', golden_sets: []};
        const listEl = $('manage-gold-list'); listEl.innerHTML = '';
        (gs.golden_sets || []).forEach(e => {
          const it = document.createElement('div');
          it.className = 'item';
          it.textContent = `${e.scenario_id} — ${e.input?.task?.slice(0, 40) || ''}`;
          it.onclick = () => editGolden(domain, e, gs.version || '1.0');
          listEl.appendChild(it);
        });
      } catch (e) { /* ignore empty */ }
    }

    function hideGoldenPicker() { $('manage-golden-picker').hidden = true; }

    function editScenario(domain, scen) {
      hideGoldenPicker();
      MANAGE_STATE = {kind: 'scenario', mode: 'edit', domain, id: scen.id};
      $('manage-editor-title').textContent = `Scenario: ${domain}/${scen.id}`;
      $('manage-editor-hint').textContent = 'PUT /api/eval-scenarios/:domain/:id 본문으로 저장';
      $('manage-editor').value = JSON.stringify({
        id: scen.id,
        name: scen.name,
        description: scen.description || '',
        task_description: scen.task_description,
        initial_environment: scen.initial_environment || {},
        expected_tools: scen.expected_tools || [],
        success_criteria: scen.success_criteria || {},
        difficulty: scen.difficulty || 'medium'
      }, null, 2);
    }

    function editGolden(domain, entry, version) {
      hideGoldenPicker();
      MANAGE_STATE = {kind: 'golden', mode: 'edit', domain, id: entry.scenario_id};
      $('manage-editor-title').textContent = `Golden: ${domain}/${entry.scenario_id}`;
      $('manage-editor-hint').textContent = 'PUT /api/golden-sets/:domain/:scenario_id';
      $('manage-editor').value = JSON.stringify({
        scenario_id: entry.scenario_id,
        version: version,
        task: entry.input?.task || '',
        environment: entry.input?.environment || {},
        tool_sequence: entry.expected_output?.tool_sequence || [],
        tool_results: entry.expected_output?.tool_results || {},
        tolerance: entry.expected_output?.tolerance ?? 0.01
      }, null, 2);
    }

    function newScenario() {
      hideGoldenPicker();
      const domain = $('manage-domain').value;
      MANAGE_STATE = {kind: 'scenario', mode: 'new', domain, id: null};
      $('manage-editor-title').textContent = `New scenario in ${domain}`;
      $('manage-editor-hint').textContent = 'POST /api/eval-scenarios/:domain';
      $('manage-editor').value = JSON.stringify({
        id: 'new_id',
        name: '이름',
        description: '',
        task_description: '작업 설명',
        initial_environment: {},
        expected_tools: [],
        success_criteria: {},
        difficulty: 'medium'
      }, null, 2);
    }

    function newGolden() {
      const domain = $('manage-domain').value;
      if (MANAGE_SCEN_IDS.length === 0) {
        return showErr($('manage-out'), '이 도메인에 시나리오가 없습니다. 먼저 평가 시나리오를 생성하세요.');
      }
      MANAGE_STATE = {kind: 'golden', mode: 'new', domain, id: null};
      $('manage-editor-title').textContent = `New golden in ${domain}`;
      $('manage-editor-hint').textContent = 'POST /api/golden-sets/:domain — scenario_id 는 드롭다운에서만 선택 가능';

      // 드롭다운 구성
      const sel = $('manage-golden-scenario-sel');
      sel.innerHTML = '';
      MANAGE_SCEN_IDS.forEach(s => {
        const o = document.createElement('option');
        o.value = s.id;
        o.textContent = `${s.id} — ${s.name}`;
        sel.appendChild(o);
      });
      $('manage-golden-picker').hidden = false;

      const writeJson = (sid) => {
        $('manage-editor').value = JSON.stringify({
          scenario_id: sid,
          version: '1.0',
          task: '작업',
          environment: {},
          tool_sequence: [],
          tool_results: {},
          tolerance: 0.01
        }, null, 2);
      };
      writeJson(sel.value);
      sel.onchange = () => writeJson(sel.value);
    }

    function clearEditor() {
      hideGoldenPicker();
      MANAGE_STATE = null;
      $('manage-editor-title').textContent = 'Editor';
      $('manage-editor-hint').textContent = '';
      $('manage-editor').value = '';
      showOk($('manage-out'), 'cleared');
    }

    async function saveEditor() {
      if (!MANAGE_STATE) return showErr($('manage-out'), '편집 대상을 선택하거나 새로 생성하세요');
      let body;
      try { body = JSON.parse($('manage-editor').value); }
      catch (e) { return showErr($('manage-out'), 'invalid JSON: ' + e.message); }
      const {kind, mode, domain, id} = MANAGE_STATE;
      // 신규 골든셋: scenario_id 는 반드시 드롭다운에서 온 값으로 강제.
      if (kind === 'golden' && mode === 'new') {
        const sid = $('manage-golden-scenario-sel').value;
        if (!sid) return showErr($('manage-out'), 'scenario_id 드롭다운에서 선택하세요');
        body.scenario_id = sid;
      }
      const base = kind === 'scenario' ? '/api/eval-scenarios' : '/api/golden-sets';
      showPending($('manage-out'));
      try {
        let res;
        if (mode === 'new') {
          res = await API.post(`${base}/${domain}`, body);
        } else {
          const key = kind === 'scenario' ? id : id;
          res = await fetch(`${base}/${domain}/${encodeURIComponent(key)}`, {
            method: 'PUT',
            headers: {'Content-Type': 'application/json'},
            body: JSON.stringify(body),
          }).then(r => r.ok ? r.json() : r.json().then(j => Promise.reject(j)));
        }
        showOk($('manage-out'), res);
        await refreshManage();
      } catch (err) { showErr($('manage-out'), err); }
    }

    async function deleteEditor() {
      if (!MANAGE_STATE || MANAGE_STATE.mode !== 'edit') return showErr($('manage-out'), '삭제할 기존 엔트리를 선택하세요');
      if (!confirm('정말 삭제하시겠습니까?')) return;
      const {kind, domain, id} = MANAGE_STATE;
      const base = kind === 'scenario' ? '/api/eval-scenarios' : '/api/golden-sets';
      showPending($('manage-out'));
      try {
        const r = await fetch(`${base}/${domain}/${encodeURIComponent(id)}`, {method: 'DELETE'});
        if (!r.ok) { const j = await r.json().catch(() => ({error: r.statusText})); throw j; }
        showOk($('manage-out'), {deleted: `${kind} ${domain}/${id}`});
        clearEditor();
        await refreshManage();
      } catch (err) { showErr($('manage-out'), err); }
    }

    // ---------- EXECUTE (단일/전체 통합 탭) ----------
    async function initExecute() {
      // 하위 탭 둘 다 초기화 — 토글 시 재초기화 불필요
      await loadAgents();
      await initScenarios();
    }
    function initExecuteSubnav() {
      document.querySelectorAll('#execute-subnav button').forEach(btn => {
        btn.onclick = () => {
          document.querySelectorAll('#execute-subnav button').forEach(b => b.classList.remove('active'));
          btn.classList.add('active');
          const target = btn.dataset.subtab;
          $('subpanel-single').hidden = (target !== 'single');
          $('subpanel-batch').hidden = (target !== 'batch');
        };
      });
    }

    // ---------- SPEC-022 Domains 탭 ----------
    let DOMAIN_EDITOR = {mode: 'new', name: null};
    let TOOL_CATALOG = [];

    async function initDomains() {
      try {
        TOOL_CATALOG = await API.get('/api/tools/catalog');
      } catch (e) { TOOL_CATALOG = []; }
      const sel = $('domain-tools-input');
      if (sel) {
        sel.innerHTML = '';
        TOOL_CATALOG.forEach(t => {
          const opt = document.createElement('option');
          opt.value = t.name;
          opt.textContent = `${t.name}  [${t.domain || 'general'}]`;
          sel.appendChild(opt);
        });
      }
      await refreshDomains();
    }

    async function refreshDomains() {
      const list = $('domains-list');
      if (!list) return;
      list.innerHTML = '';
      let domains = [];
      try { domains = await API.get('/api/domains'); }
      catch (e) { list.textContent = 'load error: ' + e; return; }
      domains.forEach(d => {
        const it = document.createElement('div');
        it.className = 'item';
        const lock = d.is_bootstrap ? ' 🔒' : '';
        const name = document.createElement('span');
        name.className = 'name';
        name.textContent = d.name + lock;
        const meta = document.createElement('span');
        meta.className = 'meta';
        meta.textContent = `scen ${d.scenario_count} · tools ${d.tool_class_names.length} · kw ${d.keywords.length}`;
        it.appendChild(name);
        it.appendChild(meta);
        it.onclick = () => {
          document.querySelectorAll('#domains-list .item').forEach(x => x.classList.remove('active'));
          it.classList.add('active');
          editDomain(d);
        };
        list.appendChild(it);
      });
    }

    function newDomain() {
      DOMAIN_EDITOR = {mode: 'new', name: null};
      $('domain-editor-title').textContent = t('domains.newTitle');
      $('domain-editor-hint').textContent = t('domains.newHint');
      $('domain-name-input').value = '';
      $('domain-name-input').disabled = false;
      $('domain-desc-input').value = '';
      Array.from($('domain-tools-input').options).forEach(o => o.selected = false);
      $('domain-keywords-input').value = '';
      $('domain-out').textContent = t('common.idle');
    }

    function editDomain(d) {
      DOMAIN_EDITOR = {mode: 'edit', name: d.name, isBootstrap: d.is_bootstrap};
      $('domain-editor-title').textContent = t('domains.editTitle') + d.name + (d.is_bootstrap ? t('domains.bootstrap') : '');
      $('domain-editor-hint').textContent = t('domains.editHint') + d.name;
      $('domain-name-input').value = d.name;
      $('domain-name-input').disabled = true;
      $('domain-desc-input').value = d.description || '';
      const sel = $('domain-tools-input');
      Array.from(sel.options).forEach(o => o.selected = d.tool_class_names.includes(o.value));
      $('domain-keywords-input').value = (d.keywords || []).join(', ');
      refreshExternalTools(d.name);
      refreshPromptSets(d.name);
    }

    // ---------- SPEC-023 external tools ----------
    let EXTOOL_EDITOR = {mode: 'new', domain: null, name: null};
    async function refreshExternalTools(domain) {
      EXTOOL_EDITOR.domain = domain;
      const list = $('extools-list');
      if (!list) return;
      list.innerHTML = '';
      try {
        const rows = await API.get('/api/external-tools/' + domain);
        rows.forEach(r => {
          const it = document.createElement('div');
          it.className = 'item';
          const name = document.createElement('span');
          name.className = 'name';
          name.textContent = r.name;
          const meta = document.createElement('span');
          meta.className = 'meta';
          meta.textContent = `${r.method} ${r.url}`;
          it.appendChild(name);
          it.appendChild(meta);
          it.onclick = () => {
            document.querySelectorAll('#extools-list .item').forEach(x => x.classList.remove('active'));
            it.classList.add('active');
            loadExternalTool(r);
          };
          list.appendChild(it);
        });
      } catch (e) { list.textContent = t('extools.loadError') + e; }
    }
    function loadExternalTool(r) {
      EXTOOL_EDITOR = {mode: 'edit', domain: r.domain, name: r.name};
      $('extool-name').value = r.name;
      $('extool-name').disabled = true;
      $('extool-desc').value = r.description || '';
      $('extool-method').value = r.method || 'POST';
      $('extool-url').value = r.url || '';
      $('extool-headers').value = r.headers_json || '';
      $('extool-body').value = r.body_template || '';
      $('extool-schema').value = r.params_schema || '';
      $('extool-timeout').value = r.timeout_ms || 10000;
    }
    function clearExternalToolForm() {
      EXTOOL_EDITOR = {mode: 'new', domain: EXTOOL_EDITOR.domain, name: null};
      $('extool-name').value = '';
      $('extool-name').disabled = false;
      $('extool-desc').value = '';
      $('extool-method').value = 'POST';
      $('extool-url').value = '';
      $('extool-headers').value = '';
      $('extool-body').value = '';
      $('extool-schema').value = '';
      $('extool-timeout').value = '10000';
      $('extool-out').textContent = t('common.idle');
    }
    async function saveExternalTool() {
      const out = $('extool-out');
      if (!EXTOOL_EDITOR.domain) { out.textContent = t('extools.selectDomainFirst'); return; }
      const body = {
        name: $('extool-name').value.trim(),
        description: $('extool-desc').value,
        method: $('extool-method').value,
        url: $('extool-url').value.trim(),
        headers_json: $('extool-headers').value.trim() || null,
        body_template: $('extool-body').value,
        params_schema: $('extool-schema').value || '{}',
        timeout_ms: parseInt($('extool-timeout').value || '10000', 10),
      };
      showPending(out);
      try {
        let res;
        if (EXTOOL_EDITOR.mode === 'new') {
          res = await API.post('/api/external-tools/' + EXTOOL_EDITOR.domain, body);
        } else {
          res = await API.put('/api/external-tools/' + EXTOOL_EDITOR.domain + '/' + EXTOOL_EDITOR.name, body);
        }
        out.textContent = JSON.stringify(res, null, 2);
        await refreshExternalTools(EXTOOL_EDITOR.domain);
      } catch (e) { out.textContent = t('extools.error') + e; }
    }
    async function deleteExternalTool() {
      if (EXTOOL_EDITOR.mode !== 'edit') { $('extool-out').textContent = t('extools.selectToolFirst'); return; }
      if (!confirm(t('extools.confirmDelete') + ' ' + EXTOOL_EDITOR.name)) return;
      showPending($('extool-out'));
      try {
        await API.del('/api/external-tools/' + EXTOOL_EDITOR.domain + '/' + EXTOOL_EDITOR.name);
        $('extool-out').textContent = t('extools.deleted') + EXTOOL_EDITOR.name;
        clearExternalToolForm();
        await refreshExternalTools(EXTOOL_EDITOR.domain);
      } catch (e) { $('extool-out').textContent = t('extools.error') + e; }
    }

    function clearDomainEditor() { newDomain(); }

    async function saveDomain() {
      const out = $('domain-out');
      const name = $('domain-name-input').value.trim();
      const description = $('domain-desc-input').value;
      const tool_class_names = Array.from($('domain-tools-input').selectedOptions).map(o => o.value);
      const keywords = $('domain-keywords-input').value.split(',').map(s => s.trim()).filter(s => s.length > 0);
      showPending(out);
      try {
        let res;
        if (DOMAIN_EDITOR.mode === 'new') {
          res = await API.post('/api/domains', {name, description, tool_class_names, keywords});
        } else {
          res = await API.put('/api/domains/' + DOMAIN_EDITOR.name, {description, tool_class_names, keywords});
        }
        out.textContent = JSON.stringify(res, null, 2);
        await refreshDomains();
      } catch (e) { out.textContent = t('domains.error') + e; }
    }

    async function deleteDomain() {
      if (DOMAIN_EDITOR.mode !== 'edit' || !DOMAIN_EDITOR.name) {
        $('domain-out').textContent = t('domains.selectFirst');
        return;
      }
      if (DOMAIN_EDITOR.isBootstrap) {
        $('domain-out').textContent = t('domains.cantDeleteBootstrap');
        return;
      }
      if (!confirm(t('domains.confirmDelete') + ' ' + DOMAIN_EDITOR.name)) return;
      showPending($('domain-out'));
      try {
        await API.del('/api/domains/' + DOMAIN_EDITOR.name);
        $('domain-out').textContent = t('domains.deleted') + DOMAIN_EDITOR.name;
        clearDomainEditor();
        await refreshDomains();
      } catch (e) { $('domain-out').textContent = t('domains.error') + e; }
    }

    // ---------- SPEC-025: PromptSet CRUD ----------
    let PROMPT_EDITOR = { domain: null, selectedVersion: null, rows: [] };

    async function refreshPromptSets(domain) {
      PROMPT_EDITOR.domain = domain;
      PROMPT_EDITOR.selectedVersion = null;
      PROMPT_EDITOR.rows = [];
      const list = $('prompts-list');
      if (!list) return;
      list.innerHTML = '';
      let rows = [];
      try { rows = await API.get('/api/domains/' + encodeURIComponent(domain) + '/prompts'); }
      catch (e) { list.textContent = 'load error: ' + e; return; }
      PROMPT_EDITOR.rows = rows;
      rows.forEach(r => {
        const it = document.createElement('div');
        it.className = 'item';
        const badges = (r.is_active ? ' [active]' : '') + (r.is_bootstrap ? ' [bootstrap]' : '');
        it.innerHTML = `<span class="name">v${r.version}${badges}</span><span class="meta">${r.created_at || ''}</span>`;
        it.onclick = () => {
          document.querySelectorAll('#prompts-list .item').forEach(x => x.classList.remove('active'));
          it.classList.add('active');
          loadPromptSetIntoEditor(r);
        };
        list.appendChild(it);
      });
      rebuildCompareSelect();
    }

    function rebuildCompareSelect() {
      const sel = $('ps-compare-select');
      if (!sel) return;
      const cur = PROMPT_EDITOR.selectedVersion;
      sel.innerHTML = '';
      const placeholder = document.createElement('option');
      placeholder.value = '';
      placeholder.textContent = '--';
      sel.appendChild(placeholder);
      for (const r of (PROMPT_EDITOR.rows || [])) {
        if (r.version === cur) continue;
        const opt = document.createElement('option');
        opt.value = String(r.version);
        opt.textContent = `v${r.version}${r.is_active ? ' [active]' : ''}${r.is_bootstrap ? ' [bootstrap]' : ''}`;
        sel.appendChild(opt);
      }
    }

    // 라인 단위 naive diff. 동일 인덱스 비교 → 같으면 "  line", 다르면 "-/+" 표시.
    // 최소 구현이라 이동·삽입은 정확치 않지만 템플릿 편집 비교에는 충분하다.
    function diffLines(a, b) {
      const la = (a || '').split('\n');
      const lb = (b || '').split('\n');
      const out = [];
      const max = Math.max(la.length, lb.length);
      for (let i = 0; i < max; i++) {
        const xa = la[i];
        const xb = lb[i];
        if (xa === xb && xa !== undefined) {
          out.push('  ' + xa);
        } else {
          if (xa !== undefined) out.push('- ' + xa);
          if (xb !== undefined) out.push('+ ' + xb);
        }
      }
      return out.join('\n');
    }

    function comparePromptSetVersions() {
      const out = $('ps-diff');
      const targetVer = parseInt($('ps-compare-select').value);
      const curVer = PROMPT_EDITOR.selectedVersion;
      if (!curVer || !targetVer || isNaN(targetVer)) {
        out.textContent = t('prompts.compareNeedTwo');
        return;
      }
      const cur = (PROMPT_EDITOR.rows || []).find(r => r.version === curVer);
      const tgt = (PROMPT_EDITOR.rows || []).find(r => r.version === targetVer);
      if (!cur || !tgt) { out.textContent = 'not found'; return; }
      const fields = ['perceive_system','perceive_user','policy_system','policy_user'];
      const parts = [`# diff: v${curVer}  →  v${targetVer}   (- = base, + = target)`, ''];
      for (const f of fields) {
        if (cur[f] === tgt[f]) {
          parts.push(`## ${f}`, '  (unchanged)', '');
        } else {
          parts.push(`## ${f}`, diffLines(cur[f], tgt[f]), '');
        }
      }
      out.textContent = parts.join('\n');
    }

    function loadPromptSetIntoEditor(row) {
      PROMPT_EDITOR.selectedVersion = row.version;
      $('ps-perc-sys').value  = row.perceive_system || '';
      $('ps-perc-user').value = row.perceive_user  || '';
      $('ps-pol-sys').value   = row.policy_system  || '';
      $('ps-pol-user').value  = row.policy_user    || '';
      $('ps-notes').value     = row.notes          || '';
      $('ps-out').textContent = `v${row.version}${row.is_active ? ' [active]' : ''}${row.is_bootstrap ? ' [bootstrap]' : ''}`;
      $('ps-diff').textContent = '';
      rebuildCompareSelect();
    }

    function clearPromptSetEditor() {
      PROMPT_EDITOR.selectedVersion = null;
      $('ps-perc-sys').value = '';
      $('ps-perc-user').value = '';
      $('ps-pol-sys').value = '';
      $('ps-pol-user').value = '';
      $('ps-notes').value = '';
      $('ps-out').textContent = t('common.idle');
      document.querySelectorAll('#prompts-list .item').forEach(x => x.classList.remove('active'));
    }

    async function savePromptSetNewVersion() {
      if (!PROMPT_EDITOR.domain) { $('ps-out').textContent = t('prompts.selectFirst'); return; }
      const body = {
        perceive_system: $('ps-perc-sys').value,
        perceive_user:   $('ps-perc-user').value,
        policy_system:   $('ps-pol-sys').value,
        policy_user:     $('ps-pol-user').value,
        notes:           $('ps-notes').value || null,
      };
      showPending($('ps-out'));
      try {
        const res = await API.post('/api/domains/' + encodeURIComponent(PROMPT_EDITOR.domain) + '/prompts', body);
        showOk($('ps-out'), res);
        await refreshPromptSets(PROMPT_EDITOR.domain);
      } catch (e) { showErr($('ps-out'), e); }
    }

    async function activateSelectedPromptSet() {
      if (!PROMPT_EDITOR.domain || PROMPT_EDITOR.selectedVersion == null) {
        $('ps-out').textContent = t('prompts.selectVersionFirst'); return;
      }
      showPending($('ps-out'));
      try {
        const res = await API.put(`/api/domains/${encodeURIComponent(PROMPT_EDITOR.domain)}/prompts/${PROMPT_EDITOR.selectedVersion}/activate`, {});
        showOk($('ps-out'), res);
        await refreshPromptSets(PROMPT_EDITOR.domain);
      } catch (e) { showErr($('ps-out'), e); }
    }

    async function deleteSelectedPromptSet() {
      if (!PROMPT_EDITOR.domain || PROMPT_EDITOR.selectedVersion == null) {
        $('ps-out').textContent = t('prompts.selectVersionFirst'); return;
      }
      if (!confirm(`정말 삭제? v${PROMPT_EDITOR.selectedVersion}`)) return;
      showPending($('ps-out'));
      try {
        await API.del(`/api/domains/${encodeURIComponent(PROMPT_EDITOR.domain)}/prompts/${PROMPT_EDITOR.selectedVersion}`);
        $('ps-out').textContent = 'deleted';
        clearPromptSetEditor();
        await refreshPromptSets(PROMPT_EDITOR.domain);
      } catch (e) { showErr($('ps-out'), e); }
    }

    // 웹 브라우저 환경에서만 target="_blank" 를 추가하여 새 탭에서 열리게 한다.
    // Tauri WebView2 는 target="_blank" 새 창 요청을 차단하므로, HTML 기본값은
    // target 없음(같은 창 이동)으로 두고 브라우저에서만 새 탭 동작을 부여한다.
    if (!window.__IS_TAURI__) {
      const hb = document.querySelector('a.help-btn');
      if (hb) hb.setAttribute('target', '_blank');
    }

    // ---------- init dispatcher ----------
    const initFns = {
      execute: initExecute,
      tools: initTools,
      agents: initAgentsPanel,
      reports: initReports,
      trajectories: initTrajectories,
      manage: initManage,
      domains: initDomains,
    };
    initExecuteSubnav();
    (async () => { try { await initFns.manage(); initialized.manage = true; } catch (e) { console.error(e); } })();
