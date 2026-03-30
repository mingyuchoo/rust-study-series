// =============================================================================
// @trace SPEC-001
// @trace PRD: PRD-001
// @trace FR: FR-1, FR-2, FR-3, FR-4, FR-5
// @trace file-type: impl
// =============================================================================

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use chrono::Utc;

use crate::model::{CreatePostRequest, Post, UpdatePostRequest};

/// 블로그 게시글 인메모리 저장소.
///
/// @trace SPEC: SPEC-001
/// @trace TC: SPEC-001/TC-1 ~ TC-10
/// @trace FR: PRD-001/FR-1, PRD-001/FR-2, PRD-001/FR-3, PRD-001/FR-4, PRD-001/FR-5
#[derive(Clone, Default)]
pub struct BlogStore {
    inner: Arc<RwLock<StoreInner>>,
}

#[derive(Default)]
struct StoreInner {
    posts: HashMap<u64, Post>,
    next_id: u64,
}

impl BlogStore {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(StoreInner {
                posts: HashMap::new(),
                next_id: 1,
            })),
        }
    }

    /// 게시글을 생성하고 반환한다.
    ///
    /// @trace SPEC: SPEC-001
    /// @trace TC: SPEC-001/TC-1, SPEC-001/TC-2
    /// @trace FR: PRD-001/FR-1
    pub fn create_post(&self, req: CreatePostRequest) -> Post {
        let mut inner = self.inner.write().unwrap();
        let now = Utc::now().to_rfc3339();
        let post = Post {
            id: inner.next_id,
            title: req.title,
            content: req.content,
            created_at: now.clone(),
            updated_at: now,
        };
        inner.next_id += 1;
        inner.posts.insert(post.id, post.clone());
        post
    }

    /// ID로 게시글을 조회한다.
    ///
    /// @trace SPEC: SPEC-001
    /// @trace TC: SPEC-001/TC-3, SPEC-001/TC-4
    /// @trace FR: PRD-001/FR-2
    pub fn get_post(&self, id: u64) -> Option<Post> {
        let inner = self.inner.read().unwrap();
        inner.posts.get(&id).cloned()
    }

    /// 모든 게시글을 목록으로 반환한다.
    ///
    /// @trace SPEC: SPEC-001
    /// @trace TC: SPEC-001/TC-5, SPEC-001/TC-6
    /// @trace FR: PRD-001/FR-3
    pub fn list_posts(&self) -> Vec<Post> {
        let inner = self.inner.read().unwrap();
        let mut posts: Vec<Post> = inner.posts.values().cloned().collect();
        posts.sort_by_key(|p| p.id);
        posts
    }

    /// 게시글을 수정하고 수정된 게시글을 반환한다.
    ///
    /// @trace SPEC: SPEC-001
    /// @trace TC: SPEC-001/TC-7, SPEC-001/TC-8
    /// @trace FR: PRD-001/FR-4
    pub fn update_post(&self, id: u64, req: UpdatePostRequest) -> Option<Post> {
        let mut inner = self.inner.write().unwrap();
        let post = inner.posts.get_mut(&id)?;
        post.title = req.title;
        post.content = req.content;
        post.updated_at = Utc::now().to_rfc3339();
        Some(post.clone())
    }

    /// 게시글을 삭제한다. 삭제 성공 시 true를 반환한다.
    ///
    /// @trace SPEC: SPEC-001
    /// @trace TC: SPEC-001/TC-9, SPEC-001/TC-10
    /// @trace FR: PRD-001/FR-5
    pub fn delete_post(&self, id: u64) -> bool {
        let mut inner = self.inner.write().unwrap();
        inner.posts.remove(&id).is_some()
    }
}
