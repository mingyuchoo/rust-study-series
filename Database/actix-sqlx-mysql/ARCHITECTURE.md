# Clean Architecture 설계 문서

## 개요

이 프로젝트는 Robert C. Martin의 Clean Architecture 원칙을 Rust로 구현한 웹 애플리케이션입니다.

## 아키텍처 다이어그램

```text
┌─────────────────────────────────────────────────────────────┐
│                        외부 세계                              │
│                    (HTTP 클라이언트)                          │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                    Adapters Layer                            │
│                    (어댑터 계층)                              │
│  ┌──────────────────────────────────────────────────────┐   │
│  │  HTTP Handlers (handlers.rs)                         │   │
│  │  - health_check()                                    │   │
│  │  - create_member()                                   │   │
│  │  - get_member()                                      │   │
│  │  - get_all_members()                                 │   │
│  │  - update_member()                                   │   │
│  │  - delete_member()                                   │   │
│  └──────────────────────────────────────────────────────┘   │
│  ┌──────────────────────────────────────────────────────┐   │
│  │  DTOs (models.rs)                                    │   │
│  │  - CreateMemberRequest                               │   │
│  │  - MemberResponse                                    │   │
│  │  - ErrorResponse                                     │   │
│  └──────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                  Application Layer                           │
│                  (애플리케이션 계층)                          │
│  ┌──────────────────────────────────────────────────────┐   │
│  │  Use Cases (member_usecase.rs)                       │   │
│  │  - create_member()                                   │   │
│  │  - get_member_by_id()                                │   │
│  │  - get_all_members()                                 │   │
│  │  - get_member_count()                                │   │
│  │  - update_member()                                   │   │
│  │  - delete_member()                                   │   │
│  └──────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                    Domain Layer                              │
│                    (도메인 계층)                              │
│  ┌──────────────────────────────────────────────────────┐   │
│  │  Entities (member.rs)                                │   │
│  │  - Member                                            │   │
│  │    - validate()                                      │   │
│  │    - validate_id()                                   │   │
│  │    - validate_name()                                 │   │
│  └──────────────────────────────────────────────────────┘   │
│  ┌──────────────────────────────────────────────────────┐   │
│  │  Repository Traits (member_repository.rs)            │   │
│  │  - MemberRepository (trait)                          │   │
│  │    - create()                                        │   │
│  │    - find_by_id()                                    │   │
│  │    - find_all()                                      │   │
│  │    - count()                                         │   │
│  │    - update()                                        │   │
│  │    - delete()                                        │   │
│  └──────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
                              ▲
                              │ (의존성 역전)
                              │
┌─────────────────────────────────────────────────────────────┐
│                Infrastructure Layer                          │
│                (인프라 계층)                                  │
│  ┌──────────────────────────────────────────────────────┐   │
│  │  Repository Implementations                          │   │
│  │  (mysql_member_repository.rs)                        │   │
│  │  - MySqlMemberRepository                             │   │
│  │    implements MemberRepository                       │   │
│  └──────────────────────────────────────────────────────┘   │
│  ┌──────────────────────────────────────────────────────┐   │
│  │  Database Connection (mysql_pool.rs)                 │   │
│  │  - create_pool()                                     │   │
│  │  - get_database_url()                                │   │
│  └──────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                        외부 세계                              │
│                    (MySQL Database)                          │
└─────────────────────────────────────────────────────────────┘
```

## 계층별 설명

### 1. Domain Layer (도메인 계층)

**위치**: `src/domain/`

**책임**:
- 비즈니스 로직의 핵심 개념 정의
- 엔티티와 비즈니스 규칙 포함
- 외부 프레임워크, 데이터베이스, UI에 독립적

**구성요소**:
- **Entities** (`entities/member.rs`): 비즈니스 객체와 규칙
  - `Member`: 회원 엔티티
  - 유효성 검증 로직 포함
- **Repository Traits** (`repositories/member_repository.rs`): 데이터 접근 인터페이스
  - `MemberRepository`: 리포지토리 트레이트 정의

**특징**:
- 다른 계층에 의존하지 않음
- 순수한 비즈니스 로직만 포함
- 테스트가 가장 용이함

### 2. Application Layer (애플리케이션 계층)

**위치**: `src/application/`

**책임**:
- 유스케이스(Use Case) 구현
- 비즈니스 로직 조율
- 도메인 객체를 사용하여 작업 수행

**구성요소**:
- **Use Cases** (`usecases/member_usecase.rs`): 비즈니스 로직 흐름
  - `MemberUseCase`: 회원 관련 유스케이스
  - 리포지토리 트레이트에 의존 (구현체 아님)

**특징**:
- 도메인 계층에만 의존
- 인프라 세부사항을 알지 못함
- 의존성 주입을 통해 리포지토리 사용

### 3. Adapters Layer (어댑터 계층)

**위치**: `src/adapters/`

**책임**:
- 외부 인터페이스 제공
- HTTP 요청/응답 처리
- DTO 변환

**구성요소**:
- **HTTP Handlers** (`http/handlers.rs`): HTTP 요청 처리
  - RESTful API 엔드포인트 구현
- **DTOs** (`http/models.rs`): 데이터 전송 객체
  - Request/Response 모델
- **Routes** (`http/routes.rs`): 라우팅 설정

**특징**:
- 애플리케이션 계층과 외부 세계를 연결
- 프레임워크 종속적 (Actix-Web)
- 도메인 엔티티와 DTO 간 변환 담당

### 4. Infrastructure Layer (인프라 계층)

**위치**: `src/infra/`

**책임**:
- 외부 시스템과의 실제 연동
- 데이터베이스 접근
- 리포지토리 트레이트 구현

**구성요소**:
- **Repository Implementations** (`repositories/mysql_member_repository.rs`)
  - `MySqlMemberRepository`: MySQL 구현체
  - 도메인의 `MemberRepository` 트레이트 구현
- **Database Connection** (`database/mysql_pool.rs`)
  - 데이터베이스 연결 풀 관리

**특징**:
- 도메인 계층의 인터페이스를 구현
- 기술 세부사항 포함 (SQLx, MySQL)
- 교체 가능한 구현체

## 의존성 흐름

```text
main.rs
  │
  ├─> Infrastructure Layer (구현체 생성)
  │     └─> MySqlMemberRepository
  │
  ├─> Application Layer (유스케이스 생성)
  │     └─> MemberUseCase(repository: Arc<dyn MemberRepository>)
  │
  └─> Adapters Layer (HTTP 서버 시작)
        └─> Handlers (usecase 주입)
```

## 의존성 역전 원칙 (Dependency Inversion Principle)

```text
고수준 모듈 (Application Layer)
       │
       │ 의존
       ▼
   인터페이스 (Domain Layer - MemberRepository trait)
       ▲
       │ 구현
       │
저수준 모듈 (Infrastructure Layer - MySqlMemberRepository)
```

- **고수준 모듈**이 **저수준 모듈**에 직접 의존하지 않음
- 둘 다 **추상화**(트레이트)에 의존
- 인프라 변경 시 도메인/애플리케이션 계층은 영향받지 않음

## 테스트 전략

### 1. 도메인 계층 테스트
```rust
// src/domain/entities/member.rs
#[cfg(test)]
mod tests {
    // 순수한 비즈니스 로직 테스트
    // 외부 의존성 없음
}
```

### 2. 애플리케이션 계층 테스트
```rust
// src/application/usecases/member_usecase.rs
#[cfg(test)]
mod tests {
    // Mock 리포지토리 사용
    // 유스케이스 로직 검증
}
```

### 3. 인프라 계층 테스트
```rust
// 통합 테스트
// 실제 데이터베이스 또는 테스트 컨테이너 사용
```

## 확장 가능성

### 새로운 엔티티 추가
1. `domain/entities/`에 새 엔티티 추가
2. `domain/repositories/`에 리포지토리 트레이트 정의
3. `application/usecases/`에 유스케이스 추가
4. `infra/repositories/`에 구현체 추가
5. `adapters/http/`에 핸들러 추가

### 데이터베이스 변경 (예: PostgreSQL)
1. `infra/repositories/`에 새 구현체 추가
   - `PostgresMemberRepository`
2. `main.rs`에서 의존성 주입 변경
3. 도메인/애플리케이션 계층은 변경 불필요!

### 새로운 인터페이스 추가 (예: gRPC)
1. `adapters/grpc/`에 새 어댑터 추가
2. 동일한 유스케이스 재사용
3. 도메인/애플리케이션 계층은 변경 불필요!

## 장점

1. **테스트 용이성**: 각 계층을 독립적으로 테스트 가능
2. **유지보수성**: 관심사가 명확히 분리됨
3. **확장성**: 새로운 기능 추가가 용이함
4. **독립성**: 프레임워크, 데이터베이스 변경에 유연함
5. **재사용성**: 비즈니스 로직을 다양한 인터페이스에서 재사용 가능

## 참고 자료

- [The Clean Architecture - Robert C. Martin](https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html)
- [Hexagonal Architecture](https://alistair.cockburn.us/hexagonal-architecture/)
- [Domain-Driven Design](https://martinfowler.com/bliki/DomainDrivenDesign.html)
