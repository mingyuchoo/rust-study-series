# Docker 배포 가이드

## 개요

이 프로젝트는 멀티 스테이지 빌드를 사용하여 최적화된 Docker 이미지를 생성합니다.

## 멀티 스테이지 빌드 구조

### 1단계: Planner (의존성 분석)

- `cargo-chef`를 사용하여 프로젝트 의존성 레시피 생성
- 의존성 캐싱을 위한 준비 단계

### 2단계: Cacher (의존성 빌드)

- 레시피를 기반으로 의존성만 빌드
- 소스 코드 변경 시에도 의존성은 캐시에서 재사용

### 3단계: Builder (애플리케이션 빌드)

- 캐시된 의존성을 사용하여 애플리케이션 빌드
- Release 모드로 최적화된 바이너리 생성

### 4단계: Runtime (최종 이미지)

- 경량화된 Debian Slim 이미지 사용
- 빌드된 바이너리만 포함하여 이미지 크기 최소화

## 빌드 및 실행

### 로컬에서 Docker 이미지 빌드

```bash
# 프로젝트 루트 디렉토리에서 실행
cd ..

# 이미지 빌드
docker build -t actix-sqlx-mysql:latest -f docker/Dockerfile .

# 이미지 실행 (MySQL이 별도로 실행 중이어야 함)
docker run -p 8000:8000 \
  -e DATABASE_URL=mysql://test:test@host.docker.internal:3306/test \
  -e HOST=0.0.0.0 \
  -e PORT=8000 \
  -e RUST_LOG=info \
  actix-sqlx-mysql:latest
```

### Docker Compose로 전체 스택 실행

```bash
# docker 디렉토리로 이동
cd docker

# 전체 스택 시작 (MySQL + 애플리케이션)
docker compose up -d

# 로그 확인
docker compose logs -f app

# 전체 스택 중지
docker compose down

# 볼륨 포함 완전 삭제
docker compose down -v
```

## 환경 변수

| 변수명 | 기본값 | 설명 |
|--------|--------|------|
| `DATABASE_URL` | - | MySQL 연결 URL (필수) |
| `HOST` | `0.0.0.0` | 서버 바인딩 호스트 |
| `PORT` | `8000` | 서버 포트 |
| `RUST_LOG` | `info` | 로그 레벨 (trace, debug, info, warn, error) |

## 빌드 최적화 팁

### 캐시 활용

- 의존성이 변경되지 않으면 2단계(Cacher)가 캐시에서 재사용됨
- 소스 코드만 변경 시 3단계(Builder)부터 빠르게 빌드

### 빌드 시간 비교

- **첫 빌드**: 5-10분 (의존성 다운로드 및 컴파일)
- **소스 변경 후 재빌드**: 1-2분 (의존성 캐시 재사용)

### 이미지 크기

- **Builder 이미지**: ~2GB (Rust 툴체인 포함)
- **최종 Runtime 이미지**: ~100MB (바이너리만 포함)

## 프로덕션 배포

### 이미지 태깅 및 푸시

```bash
# 이미지 태깅
docker tag actix-sqlx-mysql:latest your-registry/actix-sqlx-mysql:v1.0.0

# 레지스트리에 푸시
docker push your-registry/actix-sqlx-mysql:v1.0.0
```

### Kubernetes 배포 예시

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: actix-sqlx-mysql
spec:
  replicas: 3
  selector:
    matchLabels:
      app: actix-sqlx-mysql
  template:
    metadata:
      labels:
        app: actix-sqlx-mysql
    spec:
      containers:
      - name: app
        image: your-registry/actix-sqlx-mysql:v1.0.0
        ports:
        - containerPort: 8000
        env:
        - name: DATABASE_URL
          valueFrom:
            secretKeyRef:
              name: db-secret
              key: url
        - name: HOST
          value: "0.0.0.0"
        - name: PORT
          value: "8000"
        - name: RUST_LOG
          value: "info"
        livenessProbe:
          httpGet:
            path: /health
            port: 8000
          initialDelaySeconds: 5
          periodSeconds: 30
        readinessProbe:
          httpGet:
            path: /health
            port: 8000
          initialDelaySeconds: 5
          periodSeconds: 10
```

## 헬스체크

컨테이너는 `/health` 엔드포인트를 통해 헬스체크를 수행합니다.

```bash
# 헬스체크 테스트
curl http://localhost:8000/health
```

## 트러블슈팅

### 빌드 실패 시

1. Docker 빌드 캐시 삭제: `docker builder prune -a`
2. 충분한 디스크 공간 확인
3. 메모리 부족 시 Docker 메모리 제한 증가

### 연결 실패 시

1. MySQL 컨테이너가 정상 실행 중인지 확인
2. `DATABASE_URL`이 올바른지 확인
3. 네트워크 설정 확인: `docker network ls`

## 참고 자료

- [cargo-chef](https://github.com/LukeMathWalker/cargo-chef) - Rust 의존성 캐싱 도구
- [Docker 멀티 스테이지 빌드](https://docs.docker.com/build/building/multi-stage/)
- [Actix-web 배포 가이드](https://actix.rs/docs/deployment/)
