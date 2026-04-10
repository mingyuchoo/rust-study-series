# 추적성 매트릭스

최종 갱신: 2026-04-10 17:23

## 정방향 추적 (요구사항 -> 구현)

| PRD | FR ID | FR 제목 | SPEC | TC | 테스트 파일 | 구현 파일 | 구현 심볼 | 상태 |
|-----|-------|--------|------|-----|-----------|----------|----------|------|

## 역방향 추적 (구현 -> 요구사항)

| 구현 파일 | 심볼 | SPEC | TC | FR | PRD | 상태 |
|----------|------|------|-----|-----|-----|------|
| crates/eval-harness/src/main.rs | build_registry | - | - | - | - | UNTRACED |
| crates/eval-harness/src/main.rs | main | - | - | - | - | UNTRACED |
| crates/eval-harness/src/web/mod.rs | build_router | SPEC-002 | SPEC-002/TC-1 | PRD-002/FR-1 | PRD-002 | OK |
| crates/eval-harness/src/web/mod.rs | run_server | - | - | - | - | UNTRACED |
| crates/eval-harness/src/web/api_exec.rs | agent_execute | - | - | - | - | UNTRACED |
| crates/eval-harness/src/web/api_exec.rs | agent_execute_impl | SPEC-004 | SPEC-004/TC-4, SPEC-004/TC-5 | PRD-004/FR-3 | PRD-004 | OK |
| crates/eval-harness/src/web/api_exec.rs | build_full_tool_registry | - | - | - | - | UNTRACED |
| crates/eval-harness/src/web/api_exec.rs | fault_sim | - | - | - | - | UNTRACED |
| crates/eval-harness/src/web/api_exec.rs | fault_sim_impl | SPEC-004 | SPEC-004/TC-10 | PRD-004/FR-7 | PRD-004 | OK |
| crates/eval-harness/src/web/api_exec.rs | get_trajectory | - | - | FR-1, FR-2, FR-3, FR-4, FR-5, FR-6, FR-7 | - | OK |
| crates/eval-harness/src/web/api_exec.rs | get_trajectory_impl | - | - | - | - | UNTRACED |
| crates/eval-harness/src/web/api_exec.rs | golden_entry | - | - | - | - | UNTRACED |
| crates/eval-harness/src/web/api_exec.rs | golden_entry_impl | SPEC-004 | SPEC-004/TC-8 | PRD-004/FR-5 | PRD-004 | OK |
| crates/eval-harness/src/web/api_exec.rs | list_trajectories | - | - | - | - | UNTRACED |
| crates/eval-harness/src/web/api_exec.rs | list_trajectories_impl | SPEC-004 | SPEC-004/TC-11 | PRD-004/FR-7 | PRD-004 | OK |
| crates/eval-harness/src/web/api_exec.rs | load_domain_scenarios | SPEC-004 | SPEC-004/TC-1, SPEC-004/TC-2 | PRD-004/FR-1 | PRD-004 | OK |
| crates/eval-harness/src/web/api_exec.rs | run_scenario | - | - | - | - | UNTRACED |
| crates/eval-harness/src/web/api_exec.rs | run_scenario_impl | SPEC-004 | SPEC-004/TC-3 | PRD-004/FR-2 | PRD-004 | OK |
| crates/eval-harness/src/web/api_exec.rs | score | - | - | - | - | UNTRACED |
| crates/eval-harness/src/web/api_exec.rs | score_impl | SPEC-004 | SPEC-004/TC-9 | PRD-004/FR-6 | PRD-004 | OK |
| crates/eval-harness/src/web/api_exec.rs | test_tc_10_list_trajectories | - | SPEC-004/TC-11 | PRD-004/FR-7 | PRD-004 | OK |
| crates/eval-harness/src/web/api_exec.rs | test_tc_11_get_trajectory_traversal_rejected | - | - | - | - | UNTRACED |
| crates/eval-harness/src/web/api_exec.rs | test_tc_1_run_scenario_ok | - | SPEC-004/TC-2 | PRD-004/FR-1 | PRD-004 | OK |
| crates/eval-harness/src/web/api_exec.rs | test_tc_2_run_scenario_missing | - | SPEC-004/TC-3 | PRD-004/FR-2 | PRD-004 | OK |
| crates/eval-harness/src/web/api_exec.rs | test_tc_3_agent_execute_passthrough | - | SPEC-004/TC-4 | PRD-004/FR-3 | PRD-004 | OK |
| crates/eval-harness/src/web/api_exec.rs | test_tc_4_tool_invoke_ok | - | SPEC-004/TC-5 | PRD-004/FR-3 | PRD-004 | OK |
| crates/eval-harness/src/web/api_exec.rs | test_tc_5_unknown_tool | - | SPEC-004/TC-6 | PRD-004/FR-4 | PRD-004 | OK |
| crates/eval-harness/src/web/api_exec.rs | test_tc_6_golden_entry_found | - | SPEC-004/TC-7 | PRD-004/FR-4 | PRD-004 | OK |
| crates/eval-harness/src/web/api_exec.rs | test_tc_7_golden_entry_missing | - | SPEC-004/TC-8 | PRD-004/FR-5 | PRD-004 | OK |
| crates/eval-harness/src/web/api_exec.rs | test_tc_8_score_empty_trajectory | - | SPEC-004/TC-9 | PRD-004/FR-6 | PRD-004 | OK |
| crates/eval-harness/src/web/api_exec.rs | test_tc_9_fault_sim_returns | - | SPEC-004/TC-10 | PRD-004/FR-7 | PRD-004 | OK |
| crates/eval-harness/src/web/api_exec.rs | tool_invoke | - | - | - | - | UNTRACED |
| crates/eval-harness/src/web/api_exec.rs | tool_invoke_impl | SPEC-004 | SPEC-004/TC-6, SPEC-004/TC-7 | PRD-004/FR-4 | PRD-004 | OK |
| crates/eval-harness/src/web/api_exec.rs | ws_scenarios | - | SPEC-004/TC-1 | PRD-004/FR-1 | PRD-004 | OK |
| crates/eval-harness/src/web/api.rs | build_agent_registry | SPEC-003 | SPEC-003/TC-1 | PRD-003/FR-1 | PRD-003 | OK |
| crates/eval-harness/src/web/api.rs | compare_impl | SPEC-005 | SPEC-005/TC-4, SPEC-005/TC-5, SPEC-005/TC-6 | PRD-005/FR-2 | PRD-005 | OK |
| crates/eval-harness/src/web/api.rs | compare_reports | SPEC-005 | SPEC-005/TC-7 | PRD-005/FR-3 | PRD-005 | OK |
| crates/eval-harness/src/web/api.rs | compare_with_save_impl | SPEC-005 | SPEC-005/TC-7 | PRD-005/FR-3 | PRD-005 | OK |
| crates/eval-harness/src/web/api.rs | default_threshold | - | - | - | - | UNTRACED |
| crates/eval-harness/src/web/api.rs | list_agents | - | - | - | - | UNTRACED |
| crates/eval-harness/src/web/api.rs | list_agents_impl | SPEC-003 | SPEC-003/TC-2 | PRD-003/FR-2 | PRD-003 | OK |
| crates/eval-harness/src/web/api.rs | list_all | - | - | FR-1, FR-2, FR-3, FR-4, FR-5, FR-6 | - | OK |
| crates/eval-harness/src/web/api.rs | list_all_impl | - | - | - | - | UNTRACED |
| crates/eval-harness/src/web/api.rs | list_golden_sets | - | - | - | - | UNTRACED |
| crates/eval-harness/src/web/api.rs | list_golden_sets_impl | SPEC-003 | SPEC-003/TC-4, SPEC-003/TC-5 | PRD-003/FR-4 | PRD-003 | OK |
| crates/eval-harness/src/web/api.rs | list_tools | - | - | - | - | UNTRACED |
| crates/eval-harness/src/web/api.rs | list_tools_impl | SPEC-003 | SPEC-003/TC-3 | PRD-003/FR-3 | PRD-003 | OK |
| crates/eval-harness/src/web/api.rs | run_suite | - | - | - | - | UNTRACED |
| crates/eval-harness/src/web/api.rs | run_suite_impl | SPEC-005 | SPEC-005/TC-1, SPEC-005/TC-2, SPEC-005/TC-3 | PRD-005/FR-1 | PRD-005 | OK |
| crates/eval-harness/src/web/api.rs | run_suite_with_save_impl | SPEC-003 | SPEC-003/TC-8, SPEC-003/TC-9 | PRD-003/FR-6 | PRD-003 | OK |
| crates/eval-harness/src/web/api.rs | scenario_detail | - | - | - | - | UNTRACED |
| crates/eval-harness/src/web/api.rs | scenario_detail_impl | SPEC-003 | SPEC-003/TC-6, SPEC-003/TC-7 | PRD-003/FR-5 | PRD-003 | OK |
| crates/eval-harness/src/web/api.rs | test_spec005_tc_1_run_with_default_save | - | SPEC-005/TC-2 | PRD-005/FR-1 | PRD-005 | OK |
| crates/eval-harness/src/web/api.rs | test_spec005_tc_2_run_with_custom_output | - | SPEC-005/TC-3 | PRD-005/FR-1 | PRD-005 | OK |
| crates/eval-harness/src/web/api.rs | test_spec005_tc_3_run_rejects_traversal_output | - | SPEC-005/TC-4 | PRD-005/FR-2 | PRD-005 | OK |
| crates/eval-harness/src/web/api.rs | test_spec005_tc_4_compare_with_save | - | SPEC-005/TC-5 | PRD-005/FR-2 | PRD-005 | OK |
| crates/eval-harness/src/web/api.rs | test_spec005_tc_5_compare_without_save | - | SPEC-005/TC-6 | PRD-005/FR-2 | PRD-005 | OK |
| crates/eval-harness/src/web/api.rs | test_spec005_tc_6_compare_rejects_traversal_output | - | SPEC-005/TC-7 | PRD-005/FR-3 | PRD-005 | OK |
| crates/eval-harness/src/web/api.rs | test_spec005_tc_7_list_all | - | - | - | - | UNTRACED |
| crates/eval-harness/src/web/api.rs | test_tc_1_list_agents_includes_passthrough | - | SPEC-003/TC-2 | PRD-003/FR-2 | PRD-003 | OK |
| crates/eval-harness/src/web/api.rs | test_tc_2_list_tools_not_empty | - | SPEC-003/TC-3 | PRD-003/FR-3 | PRD-003 | OK |
| crates/eval-harness/src/web/api.rs | test_tc_3_load_golden_sets | - | SPEC-003/TC-4 | PRD-003/FR-4 | PRD-003 | OK |
| crates/eval-harness/src/web/api.rs | test_tc_4_scenario_detail_found | - | SPEC-003/TC-5 | PRD-003/FR-4 | PRD-003 | OK |
| crates/eval-harness/src/web/api.rs | test_tc_5_scenario_detail_missing | - | SPEC-003/TC-6 | PRD-003/FR-5 | PRD-003 | OK |
| crates/eval-harness/src/web/api.rs | test_tc_6_run_suite_passthrough | - | SPEC-003/TC-7 | PRD-003/FR-5 | PRD-003 | OK |
| crates/eval-harness/src/web/api.rs | test_tc_7_unknown_agent_rejected | - | SPEC-003/TC-8 | PRD-003/FR-6 | PRD-003 | OK |
| crates/eval-harness/src/web/api.rs | test_tc_8_compare_identical_passes | - | SPEC-003/TC-9 | PRD-003/FR-6 | PRD-003 | OK |
| crates/eval-harness/src/web/api.rs | test_tc_9_compare_rejects_traversal | - | SPEC-005/TC-1 | PRD-005/FR-1 | PRD-005 | OK |
| crates/eval-harness/src/web/api.rs | workspace_scenarios | - | SPEC-003/TC-1 | PRD-003/FR-1 | PRD-003 | OK |
| crates/eval-harness/src/web/handlers.rs | get_report | - | - | - | - | UNTRACED |
| crates/eval-harness/src/web/handlers.rs | get_report_impl | - | - | - | - | UNTRACED |
| crates/eval-harness/src/web/handlers.rs | help | - | - | - | - | UNTRACED |
| crates/eval-harness/src/web/handlers.rs | help_html_body | - | SPEC-002/TC-2 | FR-2, FR-3, FR-4, FR-5, PRD-002/FR-2 | PRD-002 | OK |
| crates/eval-harness/src/web/handlers.rs | index | SPEC-007 | SPEC-007/TC-1 | PRD-007/FR-1 | PRD-007 | OK |
| crates/eval-harness/src/web/handlers.rs | index_html_body | - | - | - | - | UNTRACED |
| crates/eval-harness/src/web/handlers.rs | is_safe_name | SPEC-002 | SPEC-002/TC-2 | PRD-002/FR-2 | PRD-002 | OK |
| crates/eval-harness/src/web/handlers.rs | list_reports | - | - | - | - | UNTRACED |
| crates/eval-harness/src/web/handlers.rs | list_reports_impl | SPEC-002 | SPEC-002/TC-4, SPEC-002/TC-5 | PRD-002/FR-4 | PRD-002 | OK |
| crates/eval-harness/src/web/handlers.rs | list_scenarios | - | - | - | - | UNTRACED |
| crates/eval-harness/src/web/handlers.rs | list_scenarios_impl | - | - | - | - | UNTRACED |
| crates/eval-harness/src/web/handlers.rs | test_spec006_tc_1_tabs_present | - | SPEC-006/TC-2 | PRD-006/FR-2 | PRD-006 | OK |
| crates/eval-harness/src/web/handlers.rs | test_spec006_tc_2_run_form | - | SPEC-006/TC-3 | PRD-006/FR-3 | PRD-006 | OK |
| crates/eval-harness/src/web/handlers.rs | test_spec006_tc_3_scenario_run | - | SPEC-006/TC-4 | PRD-006/FR-4 | PRD-006 | OK |
| crates/eval-harness/src/web/handlers.rs | test_spec006_tc_4_tool_invoke | - | SPEC-006/TC-5 | PRD-006/FR-5 | PRD-006 | OK |
| crates/eval-harness/src/web/handlers.rs | test_spec006_tc_5_agent_execute | - | SPEC-006/TC-6 | PRD-006/FR-6 | PRD-006 | OK |
| crates/eval-harness/src/web/handlers.rs | test_spec006_tc_6_compare | - | SPEC-006/TC-7 | PRD-006/FR-7 | PRD-006 | OK |
| crates/eval-harness/src/web/handlers.rs | test_spec006_tc_7_trajectories_score | - | SPEC-006/TC-8 | PRD-006/FR-8 | PRD-006 | OK |
| crates/eval-harness/src/web/handlers.rs | test_spec006_tc_8_goldens | - | SPEC-007/TC-1 | PRD-007/FR-1 | PRD-007 | OK |
| crates/eval-harness/src/web/handlers.rs | test_spec007_tc_1_help_embedded | - | SPEC-007/TC-2 | PRD-007/FR-1 | PRD-007 | OK |
| crates/eval-harness/src/web/handlers.rs | test_spec007_tc_2_help_has_guides | - | SPEC-007/TC-3 | PRD-007/FR-2 | PRD-007 | OK |
| crates/eval-harness/src/web/handlers.rs | test_spec007_tc_3_index_has_help_link | - | - | - | - | UNTRACED |
| crates/eval-harness/src/web/handlers.rs | test_tc_2_list_scenarios_loads_yaml | - | SPEC-002/TC-3 | PRD-002/FR-3 | PRD-002 | OK |
| crates/eval-harness/src/web/handlers.rs | test_tc_3_list_reports_filters_json | - | SPEC-002/TC-4 | PRD-002/FR-4 | PRD-002 | OK |
| crates/eval-harness/src/web/handlers.rs | test_tc_4_get_report_returns_content | - | SPEC-002/TC-5 | PRD-002/FR-4 | PRD-002 | OK |
| crates/eval-harness/src/web/handlers.rs | test_tc_5_path_traversal_rejected | - | SPEC-002/TC-6 | PRD-002/FR-5 | PRD-002 | OK |
| crates/eval-harness/src/web/handlers.rs | test_tc_6_index_html_embedded | - | SPEC-006/TC-1 | PRD-006/FR-1 | PRD-006 | OK |
| crates/eval-harness/src/web/handlers.rs | to_summary | SPEC-002 | SPEC-002/TC-3 | PRD-002/FR-3 | PRD-002 | OK |
| crates/eval-harness/src/tui/mod.rs | event_loop | - | - | - | - | UNTRACED |
| crates/eval-harness/src/tui/mod.rs | run_tui | - | - | - | - | UNTRACED |
| crates/eval-harness/src/tui/view.rs | draw | - | - | - | - | UNTRACED |
| crates/eval-harness/src/tui/view.rs | render_list | - | - | - | - | UNTRACED |
| crates/eval-harness/src/tui/state.rs | focused_list | - | - | - | - | UNTRACED |
| crates/eval-harness/src/tui/state.rs | handle_key | - | - | - | - | UNTRACED |
| crates/eval-harness/src/tui/state.rs | load_files | - | SPEC-001/TC-2 | FR-2, FR-3, FR-4, PRD-001/FR-2 | PRD-001 | OK |
| crates/eval-harness/src/tui/state.rs | new | SPEC-001 | TC-3 | PRD-001/FR-2 | PRD-001 | OK |
| crates/eval-harness/src/tui/state.rs | next | SPEC-001 | TC-3 | PRD-001/FR-2 | PRD-001 | OK |
| crates/eval-harness/src/tui/state.rs | prev | SPEC-001 | TC-5, TC-6 | PRD-001/FR-3, PRD-001/FR-4 | PRD-001 | OK |
| crates/eval-harness/src/tui/state.rs | set_focused_idx | - | - | - | - | UNTRACED |
| crates/eval-harness/src/tui/state.rs | test_tc_2_load_scenarios | - | SPEC-001/TC-3 | PRD-001/FR-2 | PRD-001 | OK |
| crates/eval-harness/src/tui/state.rs | test_tc_3_next_prev_moves_index | - | SPEC-001/TC-4 | PRD-001/FR-3 | PRD-001 | OK |
| crates/eval-harness/src/tui/state.rs | test_tc_4_load_reports | - | SPEC-001/TC-5 | PRD-001/FR-3 | PRD-001 | OK |
| crates/eval-harness/src/tui/state.rs | test_tc_5_tab_toggles_focus | - | SPEC-001/TC-6 | PRD-001/FR-4 | PRD-001 | OK |
| crates/eval-harness/src/tui/state.rs | test_tc_6_quit_keys_set_should_quit | - | - | - | - | UNTRACED |

## 커버리지 요약

| PRD | 전체 FR | 커버된 FR | SPEC 수 | TC 수 | 통과 | 실패 | 커버리지 |
|-----|--------|----------|--------|-------|------|------|---------|

## 미추적 항목 (경고)

- WARN: 추적태그 없는 구현 함수: crates/eval-harness/src/main.rs::build_registry
- WARN: 추적태그 없는 구현 함수: crates/eval-harness/src/main.rs::main
- WARN: 추적태그 없는 구현 함수: crates/eval-harness/src/web/mod.rs::run_server
- WARN: 추적태그 없는 구현 함수: crates/eval-harness/src/web/api_exec.rs::build_full_tool_registry
- WARN: 추적태그 없는 구현 함수: crates/eval-harness/src/web/api_exec.rs::get_trajectory_impl
- WARN: 추적태그 없는 구현 함수: crates/eval-harness/src/web/api_exec.rs::run_scenario
- WARN: 추적태그 없는 구현 함수: crates/eval-harness/src/web/api_exec.rs::agent_execute
- WARN: 추적태그 없는 구현 함수: crates/eval-harness/src/web/api_exec.rs::tool_invoke
- WARN: 추적태그 없는 구현 함수: crates/eval-harness/src/web/api_exec.rs::golden_entry
- WARN: 추적태그 없는 구현 함수: crates/eval-harness/src/web/api_exec.rs::score
- WARN: 추적태그 없는 구현 함수: crates/eval-harness/src/web/api_exec.rs::fault_sim
- WARN: 추적태그 없는 구현 함수: crates/eval-harness/src/web/api_exec.rs::list_trajectories
- WARN: 추적태그 없는 구현 함수: crates/eval-harness/src/web/api_exec.rs::test_tc_11_get_trajectory_traversal_rejected
- WARN: 추적태그 없는 구현 함수: crates/eval-harness/src/web/api.rs::list_all_impl
- WARN: 추적태그 없는 구현 함수: crates/eval-harness/src/web/api.rs::list_agents
- WARN: 추적태그 없는 구현 함수: crates/eval-harness/src/web/api.rs::list_tools
- WARN: 추적태그 없는 구현 함수: crates/eval-harness/src/web/api.rs::list_golden_sets
- WARN: 추적태그 없는 구현 함수: crates/eval-harness/src/web/api.rs::scenario_detail
- WARN: 추적태그 없는 구현 함수: crates/eval-harness/src/web/api.rs::run_suite
- WARN: 추적태그 없는 구현 함수: crates/eval-harness/src/web/api.rs::default_threshold
- WARN: 추적태그 없는 구현 함수: crates/eval-harness/src/web/api.rs::test_spec005_tc_7_list_all
- WARN: 추적태그 없는 구현 함수: crates/eval-harness/src/web/handlers.rs::list_scenarios_impl
- WARN: 추적태그 없는 구현 함수: crates/eval-harness/src/web/handlers.rs::get_report_impl
- WARN: 추적태그 없는 구현 함수: crates/eval-harness/src/web/handlers.rs::help
- WARN: 추적태그 없는 구현 함수: crates/eval-harness/src/web/handlers.rs::list_scenarios
- WARN: 추적태그 없는 구현 함수: crates/eval-harness/src/web/handlers.rs::list_reports
- WARN: 추적태그 없는 구현 함수: crates/eval-harness/src/web/handlers.rs::get_report
- WARN: 추적태그 없는 구현 함수: crates/eval-harness/src/web/handlers.rs::index_html_body
- WARN: 추적태그 없는 구현 함수: crates/eval-harness/src/web/handlers.rs::test_spec007_tc_3_index_has_help_link
- WARN: 추적태그 없는 구현 함수: crates/eval-harness/src/tui/mod.rs::run_tui
- WARN: 추적태그 없는 구현 함수: crates/eval-harness/src/tui/mod.rs::event_loop
- WARN: 추적태그 없는 구현 함수: crates/eval-harness/src/tui/view.rs::draw
- WARN: 추적태그 없는 구현 함수: crates/eval-harness/src/tui/view.rs::render_list
- WARN: 추적태그 없는 구현 함수: crates/eval-harness/src/tui/state.rs::handle_key
- WARN: 추적태그 없는 구현 함수: crates/eval-harness/src/tui/state.rs::focused_list
- WARN: 추적태그 없는 구현 함수: crates/eval-harness/src/tui/state.rs::set_focused_idx
- WARN: 추적태그 없는 구현 함수: crates/eval-harness/src/tui/state.rs::test_tc_6_quit_keys_set_should_quit
