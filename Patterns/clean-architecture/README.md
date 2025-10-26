# Clean Architecture Example

## Project Structure

```bash
src/
 ├── domain/         - 가장 내부 계층 (핵심 비즈니스 로직)
 │   ├── models.rs   - 도메인 엔티티
 │   └── services.rs - 도메인 서비스
 ├── application/    - 애플리케이션 서비스 계층
 │   └── services.rs - 유스케이스 구현
 ├── infrastructure/ - 가장 외부 계층 (외부 의존성)
 │   ├── repositories.rs - 저장소 구현
 │   └── api.rs      - API 구현
 └── main.rs         - 애플리케이션 진입점
```
