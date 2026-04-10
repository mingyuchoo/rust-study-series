use crate::{config::EvaluationConfig,
            llm_client::LlmClient};
use agent_models::{base_agent::{AgentMetadata,
                                BaseAgent},
                   domain_config::DomainConfig,
                   models::{AgentState,
                            PpaStage,
                            PpaStep,
                            ToolCall,
                            Trajectory}};
use execution_tools::registry::ToolRegistry;
use std::{collections::HashMap,
          sync::{Arc,
                 Mutex}};

pub struct PpaAgent {
    llm: Arc<LlmClient>,
    tools: Mutex<ToolRegistry>,
    config: EvaluationConfig,
}

fn block_on_future<F: std::future::Future>(fut: F) -> F::Output {
    // 이미 tokio 런타임 안(예: axum 핸들러의 spawn_blocking 스레드)이면
    // 해당 Handle을 재사용해야 한다. 중첩 런타임을 만들면 드롭 시
    // "Cannot drop a runtime in a context where blocking is not allowed"로
    // 패닉한다.
    match tokio::runtime::Handle::try_current() {
        | Ok(handle) => handle.block_on(fut),
        | Err(_) => tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("failed to build tokio runtime")
            .block_on(fut),
    }
}

impl PpaAgent {
    pub fn new(llm: LlmClient, config: EvaluationConfig) -> Self {
        Self {
            llm: Arc::new(llm),
            tools: Mutex::new(ToolRegistry::new()),
            config,
        }
    }

    fn perceive_step(&self, state: &mut AgentState, trajectory: &mut Trajectory) {
        let start = std::time::Instant::now();

        let mut ctx_map = HashMap::new();
        ctx_map.insert("iteration".into(), serde_json::json!(state.iteration));

        let messages = LlmClient::create_perceive_prompt(&state.task_description, &state.perceived_info, Some(&ctx_map));

        match block_on_future(self.llm.invoke(messages)) {
            | Ok(response) => {
                let parsed = LlmClient::parse_json_response(&response);
                let duration = start.elapsed().as_millis() as f64;

                let mut input_data = HashMap::new();
                input_data.insert("environment".into(), serde_json::json!(state.perceived_info));

                let output_data: HashMap<String, serde_json::Value> = parsed.clone();

                trajectory.steps.push(PpaStep {
                    stage: PpaStage::Perceive,
                    iteration: state.iteration,
                    timestamp: chrono::Utc::now(),
                    input_data,
                    output_data,
                    tool_calls: Vec::new(),
                    duration_ms: Some(duration),
                });

                state.current_stage = PpaStage::Policy;
                for (k, v) in parsed {
                    state.perceived_info.insert(k, v);
                }
            },
            | Err(e) => {
                state.error_message = Some(format!("Perceive 단계 오류: {}", e));
            },
        }
    }

    fn policy_step(&self, state: &mut AgentState, trajectory: &mut Trajectory) {
        let start = std::time::Instant::now();

        let tools_meta = self.tools.lock().unwrap().get_tools_metadata();
        let mut ctx_map = HashMap::new();
        ctx_map.insert("iteration".into(), serde_json::json!(state.iteration));

        let messages = LlmClient::create_policy_prompt(&state.task_description, &state.perceived_info, &tools_meta, Some(&ctx_map));

        match block_on_future(self.llm.invoke(messages)) {
            | Ok(response) => {
                let policy_data = LlmClient::parse_json_response(&response);
                let duration = start.elapsed().as_millis() as f64;

                let mut input_data = HashMap::new();
                input_data.insert("perceived_info".into(), serde_json::json!(state.perceived_info));

                let output_data: HashMap<String, serde_json::Value> = policy_data.clone();

                trajectory.steps.push(PpaStep {
                    stage: PpaStage::Policy,
                    iteration: state.iteration,
                    timestamp: chrono::Utc::now(),
                    input_data,
                    output_data,
                    tool_calls: Vec::new(),
                    duration_ms: Some(duration),
                });

                state.current_stage = PpaStage::Action;

                let planned = serde_json::json!({
                    "selected_tool": policy_data.get("selected_tool"),
                    "tool_parameters": policy_data.get("tool_parameters").cloned().unwrap_or(serde_json::json!({})),
                    "next_step": policy_data.get("next_step"),
                });
                state.planned_actions = vec![planned];

                if policy_data.get("task_completed").and_then(|v| v.as_bool()).unwrap_or(false) {
                    state.is_complete = true;
                }
            },
            | Err(e) => {
                state.error_message = Some(format!("Policy 단계 오류: {}", e));
            },
        }
    }

    fn action_step(&self, state: &mut AgentState, trajectory: &mut Trajectory) {
        let start = std::time::Instant::now();
        let mut tool_calls = Vec::new();

        for planned in &state.planned_actions.clone() {
            if let Some(tool_name) = planned.get("selected_tool").and_then(|v| v.as_str()) {
                let tool_params: HashMap<String, serde_json::Value> = planned
                    .get("tool_parameters")
                    .and_then(|v| v.as_object())
                    .map(|o| o.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
                    .unwrap_or_default();

                let mut tc = ToolCall::new(tool_name.to_string(), tool_params.clone());

                let tools = self.tools.lock().unwrap();
                if let Some(tool) = tools.get_tool(tool_name) {
                    if !tool.validate_parameters(&tool_params) {
                        tc.success = false;
                        tc.error = Some("파라미터 유효성 검사 실패".into());
                    } else {
                        let result = tool.execute(&tool_params);
                        tc.success = result.get("success").and_then(|v| v.as_bool()).unwrap_or(false);
                        if !tc.success {
                            tc.error = result.get("error").and_then(|v| v.as_str()).map(|s| s.to_string());
                        }
                        tc.result = Some(serde_json::json!(result));

                        if tc.success {
                            state.perceived_info.insert("last_tool_result".into(), tc.result.clone().unwrap());
                        } else {
                            state.perceived_info.insert("last_tool_error".into(), serde_json::json!(tc.error));
                        }
                    }
                } else {
                    tc.success = false;
                    tc.error = Some(format!("도구를 찾을 수 없음: {}", tool_name));
                }

                tool_calls.push(tc);
            }
        }

        let duration = start.elapsed().as_millis() as f64;

        let mut input_data = HashMap::new();
        input_data.insert("planned_actions".into(), serde_json::json!(state.planned_actions));

        let mut output_data = HashMap::new();
        output_data.insert("executed".into(), serde_json::json!(tool_calls.len()));

        trajectory.steps.push(PpaStep {
            stage: PpaStage::Action,
            iteration: state.iteration,
            timestamp: chrono::Utc::now(),
            input_data,
            output_data,
            tool_calls: tool_calls.clone(),
            duration_ms: Some(duration),
        });

        state.current_stage = PpaStage::Perceive;
        for tc in &tool_calls {
            state.executed_actions.push({
                let mut m = HashMap::new();
                m.insert("tool_name".into(), serde_json::json!(tc.tool_name));
                m.insert("success".into(), serde_json::json!(tc.success));
                m
            });
        }
    }

    fn get_action_signature(planned: &[serde_json::Value]) -> String {
        planned
            .iter()
            .filter_map(|a| a.get("selected_tool").and_then(|v| v.as_str()))
            .collect::<Vec<_>>()
            .join("|")
    }
}

impl BaseAgent for PpaAgent {
    fn metadata(&self) -> AgentMetadata {
        AgentMetadata {
            name: "ppa".into(),
            description: "Perceive-Policy-Action 루프 기반 AI Agent".into(),
            version: "0.1.0".into(),
        }
    }

    fn execute_task(&self, task_description: &str, initial_environment: Option<HashMap<String, serde_json::Value>>) -> Trajectory {
        let task_id = uuid::Uuid::new_v4().to_string();
        let started_at = std::time::Instant::now();
        println!("▶ PPA 실행 시작: task=\"{}\" (task_id={})", task_description, task_id);
        let mut trajectory = Trajectory {
            task_id: task_id.clone(),
            task_description: task_description.to_string(),
            start_time: chrono::Utc::now(),
            end_time: None,
            steps: Vec::new(),
            final_state: None,
            success: false,
            total_iterations: 0,
        };

        let mut state = AgentState::new(task_description.to_string()).with_environment(initial_environment.unwrap_or_default());

        let mut repeated_action_count = 0u32;
        let mut last_action = String::new();

        while !state.is_complete && state.iteration < self.config.max_iterations {
            state.iteration += 1;
            println!("  [Iteration {}/{}]", state.iteration, self.config.max_iterations);

            self.perceive_step(&mut state, &mut trajectory);
            if state.error_message.is_some() {
                break;
            }

            self.policy_step(&mut state, &mut trajectory);
            if state.error_message.is_some() {
                break;
            }

            if state.is_complete {
                break;
            }

            self.action_step(&mut state, &mut trajectory);
            if state.error_message.is_some() {
                break;
            }

            let current_action = Self::get_action_signature(&state.planned_actions);
            if current_action == last_action {
                repeated_action_count += 1;
                if repeated_action_count >= self.config.early_stop_threshold {
                    state.error_message = Some("동일한 행동 반복으로 조기 종료".into());
                    break;
                }
            } else {
                repeated_action_count = 0;
            }
            last_action = current_action;
        }

        trajectory.end_time = Some(chrono::Utc::now());
        trajectory.success = state.is_complete && state.error_message.is_none();
        trajectory.total_iterations = state.iteration;
        let elapsed = started_at.elapsed().as_secs_f64();
        let status = if trajectory.success { "성공" } else { "실패" };
        let err_suffix = state.error_message.as_deref().map(|m| format!(" (사유: {})", m)).unwrap_or_default();
        println!(
            "✔ PPA 실행 완료: task_id={} · {} · {}회 반복 · {:.2}초{}",
            task_id, status, trajectory.total_iterations, elapsed, err_suffix
        );
        trajectory.final_state = Some(state);
        trajectory
    }

    fn load_domain_tools(&self, domain_config: &DomainConfig) {
        // 도메인 교체 시 이전 도메인 도구가 남지 않도록 레지스트리를 재생성한다.
        // ToolRegistry::new()는 read_file/write_file/list_directory 기본 도구를 항상
        // 포함.
        let mut registry = self.tools.lock().unwrap();
        *registry = execution_tools::registry::ToolRegistry::new();
        match domain_config.name.as_str() {
            | "financial" => {
                registry.register(Arc::new(domains::financial::tools::SimpleInterestCalculatorTool::new()));
                registry.register(Arc::new(domains::financial::tools::CompoundInterestCalculatorTool::new()));
                registry.register(Arc::new(domains::financial::tools::TransactionValidatorTool::new()));
            },
            | "customer_service" => {
                registry.register(Arc::new(domains::customer_service::tools::ClassifyInquiryTool::new()));
                registry.register(Arc::new(domains::customer_service::tools::ProcessRefundTool::new()));
                registry.register(Arc::new(domains::customer_service::tools::EscalateIssueTool::new()));
            },
            | _ => {},
        }
    }
}
