use crate::domain::entities::Member;
use async_trait::async_trait;

/// Member 리포지토리 트레이트
/// 도메인 계층에서 정의하며, 인프라 계층에서 구현합니다.
/// 이를 통해 도메인이 인프라에 의존하지 않도록 합니다 (의존성 역전 원칙).
#[async_trait]
pub trait MemberRepository: Send + Sync {
    /// 새로운 Member를 생성합니다.
    async fn create(&self, name: String) -> Result<Member, Box<dyn std::error::Error>>;

    /// ID로 Member를 조회합니다.
    async fn find_by_id(&self, id: &str) -> Result<Option<Member>, Box<dyn std::error::Error>>;

    /// 모든 Member를 조회합니다.
    async fn find_all(&self) -> Result<Vec<Member>, Box<dyn std::error::Error>>;

    /// Member의 총 개수를 조회합니다.
    async fn count(&self) -> Result<i64, Box<dyn std::error::Error>>;

    /// Member를 업데이트합니다.
    async fn update(&self, member: &Member) -> Result<(), Box<dyn std::error::Error>>;

    /// ID로 Member를 삭제합니다.
    async fn delete(&self, id: &str) -> Result<(), Box<dyn std::error::Error>>;
}
