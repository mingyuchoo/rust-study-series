# Clean Architecture 의존성 다이어그램

## 전체 아키텍처 개요

```
┌─────────────────────────────────────────────────────────────────┐
│                     Presentation Layer                          │
│  ┌──────────────────────┐         ┌──────────────────────┐     │
│  │  Frontend (Dioxus)   │         │  Backend (Tauri)     │     │
│  │  - UI Components     │◄────────┤  - Commands          │     │
│  │  - Services          │  HTTP   │  - DTOs              │     │
│  └──────────────────────┘         └──────────┬───────────┘     │
└─────────────────────────────────────────────┼─────────────────┘
                                               │
                    ┌──────────────────────────┼──────────────────────────┐
                    │                          │                          │
                    ▼                          ▼                          ▼
         ┌──────────────────┐      ┌──────────────────┐      ┌──────────────────┐
         │  Infrastructure  │      │   Application    │      │                  │
         │  - SQLite Repo   │      │  - Use Cases     │      │                  │
         │  - External APIs │      │  - Business      │      │                  │
         └────────┬─────────┘      │    Logic         │      │                  │
                  │                └────────┬─────────┘      │                  │
                  │                         │                │                  │
                  └─────────────────────────┼────────────────┘                  │
                                            ▼                                   │
                                 ┌──────────────────┐                          │
                                 │     Domain       │                          │
                                 │  - Entities      │                          │
                                 │  - Repositories  │◄─────────────────────────┘
                                 │  - Errors        │
                                 └──────────────────┘
```

## 크레이트 의존성 그래프

```
presentation_frontend (독립)
    │
    └─ (Tauri IPC를 통해 통신)
    
presentation_backend
    ├─ domain
    ├─ application
    │   └─ domain
    └─ infrastructure
        └─ domain
```

## 계층별 의존성 규칙

### 1. Domain (가장 안쪽 원)
```
domain/
  ├─ entities/
  ├─ repositories/ (인터페이스만)
  └─ errors/
  
의존성: 없음 ✅
```

### 2. Application (유스케이스)
```
application/
  └─ usecases/
      ├─ create_contact.rs
      ├─ get_contact.rs
      ├─ list_contacts.rs
      ├─ update_contact.rs
      ├─ delete_contact.rs
      └─ search_contacts.rs

의존성: domain ✅
```

### 3. Infrastructure (구현체)
```
infrastructure/
  └─ database/
      └─ sqlite_contact_repository.rs
      
의존성: domain ✅
```

### 4. Presentation Backend (조합)
```
presentation_backend/
  ├─ routes/
  │   └─ contact_commands.rs
  └─ models/
      └─ contact_dto.rs
      
의존성: domain, application, infrastructure ✅
```

### 5. Presentation Frontend (UI)
```
presentation_frontend/
  ├─ components/
  ├─ services/
  └─ models/
  
의존성: 없음 (Tauri IPC 통신) ✅
```

## 데이터 흐름

### 연락처 생성 예시

```
1. User Input (Frontend)
   │
   ▼
2. ContactService.create_contact() (Frontend Service)
   │
   ▼
3. Tauri IPC (invoke "create_contact")
   │
   ▼
4. contact_commands::create_contact() (Backend Command)
   │
   ▼
5. CreateContactUseCase.execute() (Application)
   │
   ▼
6. Contact::new() (Domain Entity)
   │
   ▼
7. repository.create() (Domain Interface)
   │
   ▼
8. SqliteContactRepository.create() (Infrastructure)
   │
   ▼
9. SQLite Database
   │
   ▼
10. Return Contact (역순으로 반환)
```

## 의존성 역전 원칙 (DIP)

```
┌─────────────────────────────────────────────────────────┐
│                    Application Layer                    │
│                                                          │
│  CreateContactUseCase                                   │
│         │                                                │
│         │ depends on                                    │
│         ▼                                                │
│  ContactRepository (trait) ◄─────────────────┐         │
└──────────────────────────────────────────────┼─────────┘
                                                │
                                                │ implements
                                                │
┌───────────────────────────────────────────────┼─────────┐
│                Infrastructure Layer           │         │
│                                               │         │
│  SqliteContactRepository ─────────────────────┘         │
│         │                                                │
│         ▼                                                │
│  SQLite Database                                        │
└─────────────────────────────────────────────────────────┘
```

**핵심**: Application은 인터페이스(trait)에 의존하고, Infrastructure가 이를 구현합니다.

## Cargo 워크스페이스 구조

```toml
# 루트 Cargo.toml
[workspace]
members = [
    "domain",           # 1. 가장 안쪽
    "application",      # 2. domain에 의존
    "infrastructure",   # 3. domain에 의존
    "presentation_backend",   # 4. 모든 계층에 의존
    "presentation_frontend",  # 5. 독립
]
```

## 의존성 검증 명령어

```bash
# Domain이 외부에 의존하지 않는지 확인
cargo tree -p domain --depth 1

# Application이 domain에만 의존하는지 확인
cargo tree -p application --depth 1

# Infrastructure가 domain에만 의존하는지 확인
cargo tree -p infrastructure --depth 1

# Presentation Backend의 의존성 확인
cargo tree -p presentation_backend --depth 1
```

## 테스트 전략

### 1. Domain Layer
```rust
// domain/src/entities/contact.rs
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_contact_creation() {
        let contact = Contact::new(
            "John Doe".to_string(),
            Some("john@example.com".to_string()),
            None,
            None,
        );
        assert_eq!(contact.name, "John Doe");
    }
}
```

### 2. Application Layer (Mock 사용)
```rust
// application/tests/create_contact_test.rs
struct MockRepository;

#[async_trait]
impl ContactRepository for MockRepository {
    async fn create(&self, contact: Contact) -> Result<Contact, DomainError> {
        Ok(contact)
    }
    // ... 기타 메서드
}

#[tokio::test]
async fn test_create_contact_use_case() {
    let repo = Arc::new(MockRepository);
    let use_case = CreateContactUseCase::new(repo);
    
    let result = use_case.execute(
        "John Doe".to_string(),
        None,
        None,
        None,
    ).await;
    
    assert!(result.is_ok());
}
```

### 3. Infrastructure Layer (통합 테스트)
```rust
// infrastructure/tests/sqlite_repository_test.rs
#[tokio::test]
async fn test_sqlite_repository() {
    let pool = SqlitePool::connect(":memory:").await.unwrap();
    let repo = SqliteContactRepository::new(pool);
    repo.init().await.unwrap();
    
    let contact = Contact::new(
        "John Doe".to_string(),
        None,
        None,
        None,
    );
    
    let result = repo.create(contact).await;
    assert!(result.is_ok());
}
```

## 확장 시나리오

### 시나리오 1: PostgreSQL 추가

```
infrastructure/
  └─ database/
      ├─ sqlite_contact_repository.rs
      └─ postgres_contact_repository.rs  ← 새로 추가
```

**변경 필요**: Infrastructure 계층만 수정
**변경 불필요**: Domain, Application 계층

### 시나리오 2: REST API 추가

```
presentation_rest_api/  ← 새 크레이트
  ├─ Cargo.toml
  └─ src/
      ├─ routes/
      └─ models/
```

**변경 필요**: 새 Presentation 계층만 추가
**변경 불필요**: 기존 모든 계층

### 시나리오 3: CLI 추가

```
presentation_cli/  ← 새 크레이트
  ├─ Cargo.toml
  └─ src/
      └─ main.rs
```

**변경 필요**: 새 Presentation 계층만 추가
**변경 불필요**: 기존 모든 계층

## 결론

이 아키텍처의 핵심 장점:

1. **의존성 규칙 강제**: Cargo가 컴파일 타임에 검증
2. **테스트 용이성**: 각 계층을 독립적으로 테스트
3. **확장성**: 새로운 구현체나 프레젠테이션 계층 추가 용이
4. **유지보수성**: 변경 사항의 영향 범위 최소화
5. **비즈니스 로직 보호**: Domain이 외부 기술로부터 독립

---

**Clean Architecture = 의존성 방향 제어 + 계층 분리**
