-- ============================================================
-- Blog SurrealDB Schema Definition
-- ============================================================
-- 이 파일은 blog-server가 시작될 때 자동으로 적용됩니다.
-- 환경변수 SCHEMA_PATH 로 경로를 지정할 수 있습니다.
-- ============================================================

-- ── User 테이블 ──────────────────────────────────────────────

DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD username ON TABLE user TYPE string;
DEFINE FIELD email ON TABLE user TYPE string;
DEFINE FIELD password_hash ON TABLE user TYPE string;
DEFINE FIELD role ON TABLE user TYPE string;
DEFINE FIELD created_at ON TABLE user TYPE string;
DEFINE FIELD bio ON TABLE user TYPE string DEFAULT '';
DEFINE FIELD website ON TABLE user TYPE string DEFAULT '';
DEFINE FIELD theme ON TABLE user TYPE string DEFAULT 'dark';
DEFINE INDEX idx_user_username ON TABLE user COLUMNS username UNIQUE;
DEFINE INDEX idx_user_email ON TABLE user COLUMNS email UNIQUE;

-- ── Post 테이블 ──────────────────────────────────────────────

DEFINE TABLE post SCHEMAFULL;
DEFINE FIELD title ON TABLE post TYPE string;
DEFINE FIELD content ON TABLE post TYPE string;
DEFINE FIELD author_id ON TABLE post TYPE string;
DEFINE FIELD author_username ON TABLE post TYPE string;
DEFINE FIELD visibility ON TABLE post TYPE string;
DEFINE FIELD created_at ON TABLE post TYPE string;
DEFINE FIELD updated_at ON TABLE post TYPE string;

-- ── Comment 테이블 ───────────────────────────────────────────

DEFINE TABLE comment SCHEMAFULL;
DEFINE FIELD content ON TABLE comment TYPE string;
DEFINE FIELD post_id ON TABLE comment TYPE string;
DEFINE FIELD author_id ON TABLE comment TYPE string;
DEFINE FIELD author_username ON TABLE comment TYPE string;
DEFINE FIELD visibility ON TABLE comment TYPE string;
DEFINE FIELD created_at ON TABLE comment TYPE string;
