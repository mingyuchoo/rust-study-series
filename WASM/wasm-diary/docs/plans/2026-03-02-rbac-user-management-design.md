# RBAC 사용자 관리 설계

## 개요
wasm-diary 애플리케이션에 RBAC(Role-Based Access Control) 기반 사용자 관리 기능을 추가한다.

## 접근 방식
방안 A: UserManager를 DiaryManager와 동일 레벨의 별도 구조체로 추가.

## 데이터 모델

### Role enum
- Admin: 관리자
- User: 일반 사용자

### UserAccount
- id, username, password_hash, salt, role, created_at

### DiaryEntry 변경
- owner_id 필드 추가

## UserManager API
- register(username, password, role)
- login(username, password)
- get_all_users()
- delete_user(id)
- change_role(id, role)
- validate_registration(username, password)
- load_from_json / save_to_json

## 권한 매트릭스
- 일반 사용자: 자신의 일기 CRUD, 자신의 통계
- 관리자: 모든 일기 조회/삭제, 사용자 관리, 전체 통계

## 비밀번호 해싱
SHA-256 (sha2 크레이트) + 랜덤 salt

## 초기 관리자
첫 실행 시 admin/admin123 자동 생성
