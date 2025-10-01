# Actix-Web + SQLx + MySQL - Clean Architecture

Clean Architecture 원칙을 적용한 Rust 웹 애플리케이션입니다.

## 프로젝트 구조

```text
src/
├── domain/              # 도메인 계층 - 비즈니스 로직의 핵심
│   ├── entities/        # 도메인 엔티티
│   └── repositories/    # 리포지토리 트레이트 (인터페이스)
├── application/         # 애플리케이션 계층 - 유스케이스
│   └── usecases/        # 비즈니스 로직 조율
├── adapters/            # 어댑터 계층 - 외부 인터페이스
│   └── http/            # HTTP 핸들러 및 라우트
├── infra/               # 인프라 계층 - 외부 시스템 연동
│   ├── database/        # 데이터베이스 연결
│   └── repositories/    # 리포지토리 구현체
├── lib.rs               # 라이브러리 진입점
└── main.rs              # 애플리케이션 진입점
```

## Clean Architecture 원칙

이 프로젝트는 다음 원칙을 따릅니다:

1. **의존성 역전 (Dependency Inversion)**: 도메인 계층이 인프라에 의존하지 않습니다.
2. **관심사의 분리 (Separation of Concerns)**: 각 계층은 명확한 책임을 가집니다.
3. **테스트 용이성**: 비즈니스 로직을 외부 의존성 없이 테스트할 수 있습니다.
4. **유지보수성**: 각 계층을 독립적으로 수정할 수 있습니다.

## 데이터베이스 설정

### 자동 초기화

서버가 시작될 때 자동으로 다음 작업이 수행됩니다:

1. **테이블 생성**: `members` 테이블이 존재하지 않으면 자동으로 생성됩니다.
2. **샘플 데이터 삽입**: 테이블이 비어있으면 5개의 샘플 회원 데이터가 자동으로 삽입됩니다.

생성되는 테이블 구조:

```sql
CREATE TABLE IF NOT EXISTS members (
    id VARCHAR(8) PRIMARY KEY COMMENT '회원 ID (8자리)',
    name VARCHAR(64) NOT NULL COMMENT '회원 이름',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '생성 시각',
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '수정 시각'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci
COMMENT='회원 정보 테이블';
```

샘플 데이터:

- MEMB0001: 홍길동
- MEMB0002: 김철수
- MEMB0003: 이영희
- MEMB0004: 박민수
- MEMB0005: 정수진

### 수동 테이블 생성 (선택사항)

필요시 수동으로 테이블을 생성할 수도 있습니다:

```sql
-- 시퀀스 테이블
CREATE TABLE `tb_sequence` (
  `seq_name` varchar(4) NOT NULL,
  `seq_no` int NOT NULL,
  PRIMARY KEY (`seq_name`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COMMENT='tb_sequence';

-- Members 테이블
CREATE TABLE `members` (
  `id` varchar(9) CHARACTER SET utf8 COLLATE utf8_general_ci NOT NULL,
  `name` varchar(64) CHARACTER SET utf8 COLLATE utf8_general_ci DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_bin;

-- 시퀀스 생성 함수
CREATE DEFINER=`postgres`@`%` FUNCTION `postgres`.`fn_get_seq_8`(p_seq_name VARCHAR(4)) 
RETURNS varchar(8) CHARSET utf8
begin
    DECLARE RTN_VAL VARCHAR(8);

    INSERT INTO tb_sequence (seq_name, seq_no)
         values (p_seq_name, LAST_INSERT_ID(1))
    ON DUPLICATE KEY UPDATE seq_no=LAST_INSERT_ID(seq_no+1);

    set @ret = row_count();

    if @ret = 0 then
        set RTN_VAL = '0';
    else
        SET RTN_VAL = (SELECT CONCAT(p_seq_name,  LPAD(LAST_INSERT_ID(),4,'0')));
    end if;

    RETURN RTN_VAL;
END
```

## 환경 설정

`.env` 파일을 생성하고 다음 내용을 추가하세요:

```bash
cp .env.example .env
```

`.env` 파일 내용:

```env
DATABASE_URL=mysql://postgres:postgres@localhost:3306/postgres?prefer_socket=false
HOST=127.0.0.1
PORT=8000
RUST_LOG=info
```

## 빌드 및 실행

### 의존성 확인

```bash
cargo check
```

### 빌드

```bash
cargo build
```

### 실행

```bash
cargo run
```

서버는 `http://localhost:8000`에서 실행됩니다.

## API 엔드포인트

### 헬스 체크

```bash
GET /api/health
```

### Member 생성

```bash
POST /api/members
Content-Type: application/json

{
  "name": "홍길동"
}
```

### 모든 Member 조회

```bash
GET /api/members
```

### Member 개수 조회

```bash
GET /api/members/count
```

### ID로 Member 조회

```bash
GET /api/members/{id}
```

### Member 업데이트

```bash
PUT /api/members/{id}
Content-Type: application/json

{
  "name": "김철수"
}
```

### Member 삭제

```bash
DELETE /api/members/{id}
```

## 테스트

```bash
cargo test
```

## 의존성

주요 의존성:

- `actix-web`: 웹 프레임워크
- `sqlx`: 비동기 SQL 라이브러리
- `async-trait`: 비동기 트레이트 지원
- `serde`: 직렬화/역직렬화
- `tokio`: 비동기 런타임
- `env_logger`: 로깅
- `dotenv`: 환경 변수 관리

## 참고 자료

- [Clean Architecture](https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html)
- [Actix Web](https://actix.rs/)
- [SQLx](https://github.com/launchbadge/sqlx)
- [MySQL Sequence 만들기](https://velog.io/@inyong_pang/MySQL-MySQL-Sequence-%EB%A7%8C%EB%93%A4%EA%B8%B0)
