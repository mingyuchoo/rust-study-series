// =============================================================================
// @trace SPEC-017
// @trace SPEC-019
// @trace FR: PRD-017/FR-2, PRD-017/FR-7
// @trace file-type: impl
// =============================================================================
//
// 크레이트에 내장된 기본 시드 데이터. `include_str!` 로 컴파일 시점에 YAML /
// JSON 을 바이너리에 포함하여, 런타임에 외부 파일/디렉토리가 없어도 최초 기동
// 시 DB 를 채울 수 있게 한다. `SqliteStore::seed_from_embedded` 가 이 테이블을
// 읽어 `INSERT OR IGNORE` 로 시드한다.

/// `(domain_stem, yaml_body)` 쌍. domain_stem 은 파일명 stem 과 동일하며
/// 에러 메세지에만 쓰인다.
pub const EMBEDDED_SCENARIO_YAMLS: &[(&str, &str)] = &[
    ("customer_service", include_str!("../seed/scenarios/customer_service.yaml")),
    ("financial", include_str!("../seed/scenarios/financial.yaml")),
];

/// `(domain_stem, json_body)` 쌍.
pub const EMBEDDED_GOLDEN_JSONS: &[(&str, &str)] = &[
    ("customer_service", include_str!("../seed/goldens/customer_service.json")),
    ("financial", include_str!("../seed/goldens/financial.json")),
];
