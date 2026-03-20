-- GraphRAG 스키마 정의
-- SurrealDB 2.x

-- 문서 테이블
DEFINE TABLE document SCHEMAFULL;
DEFINE FIELD title       ON document TYPE string;
DEFINE FIELD created_at  ON document TYPE datetime DEFAULT time::now();

-- 청크 테이블
DEFINE TABLE chunk SCHEMAFULL;
DEFINE FIELD doc_id                ON chunk TYPE string;
DEFINE FIELD index                 ON chunk TYPE int;
DEFINE FIELD level                 ON chunk TYPE int;
DEFINE FIELD kind                  ON chunk TYPE string;
DEFINE FIELD content               ON chunk TYPE string;
DEFINE FIELD embedding_type        ON chunk TYPE string;
DEFINE FIELD embedding_deployment  ON chunk TYPE string;
DEFINE FIELD embedding_semantic    ON chunk TYPE array<float>;
DEFINE FIELD embedding_structural  ON chunk TYPE array<float>;
DEFINE FIELD embedding_functional  ON chunk TYPE array<float>;
DEFINE FIELD metadata              ON chunk TYPE object FLEXIBLE;

DEFINE INDEX idx_chunk_doc_id ON chunk FIELDS doc_id;
DEFINE INDEX idx_chunk_embedding_type ON chunk FIELDS embedding_type, embedding_deployment;

-- 엔티티 테이블
DEFINE TABLE entity SCHEMAFULL;
DEFINE FIELD doc_id                ON entity TYPE string;
DEFINE FIELD name                  ON entity TYPE string;
DEFINE FIELD type                  ON entity TYPE string;
DEFINE FIELD embedding_type        ON entity TYPE string;
DEFINE FIELD embedding_deployment  ON entity TYPE string;
DEFINE FIELD embedding_semantic    ON entity TYPE array<float>;
DEFINE FIELD embedding_structural  ON entity TYPE array<float>;
DEFINE FIELD embedding_functional  ON entity TYPE array<float>;

DEFINE INDEX idx_entity_doc_id ON entity FIELDS doc_id;
DEFINE INDEX idx_entity_name ON entity FIELDS name;
DEFINE INDEX idx_entity_embedding_type ON entity FIELDS embedding_type, embedding_deployment;

-- 관계 테이블
DEFINE TABLE relation SCHEMAFULL;
DEFINE FIELD doc_id                ON relation TYPE string;
DEFINE FIELD subject               ON relation TYPE string;
DEFINE FIELD predicate             ON relation TYPE string;
DEFINE FIELD object                ON relation TYPE string;
DEFINE FIELD weight                ON relation TYPE float;
DEFINE FIELD embedding_type        ON relation TYPE string;
DEFINE FIELD embedding_deployment  ON relation TYPE string;
DEFINE FIELD embedding_semantic    ON relation TYPE array<float>;
DEFINE FIELD embedding_structural  ON relation TYPE array<float>;
DEFINE FIELD embedding_functional  ON relation TYPE array<float>;

DEFINE INDEX idx_relation_doc_id ON relation FIELDS doc_id;
DEFINE INDEX idx_relation_subject ON relation FIELDS subject;
DEFINE INDEX idx_relation_object ON relation FIELDS object;
DEFINE INDEX idx_relation_embedding_type ON relation FIELDS embedding_type, embedding_deployment;
