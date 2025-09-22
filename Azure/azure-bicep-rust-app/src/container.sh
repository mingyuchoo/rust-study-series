#!/bin/bash

# 색상 정의
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Docker Compose 파일 경로
DOCKER_COMPOSE_FILE="docker/docker-compose.yml"

# 도움말 함수
show_help() {
    echo -e "${BLUE}사용법: $0 [COMMAND]${NC}"
    echo ""
    echo -e "${YELLOW}Commands:${NC}"
    echo "  up, start     - 컨테이너 시작"
    echo "  down, stop    - 컨테이너 중지 및 제거"
    echo "  restart       - 컨테이너 재시작"
    echo "  logs          - 컨테이너 로그 보기"
    echo "  status        - 컨테이너 상태 확인"
    echo "  build         - 이미지 빌드"
    echo "  rebuild       - 이미지 재빌드 후 시작"
    echo "  clean         - 중지된 컨테이너, 사용하지 않는 이미지 정리"
    echo "  help          - 도움말 표시"
    echo ""
    echo -e "${YELLOW}Examples:${NC}"
    echo "  $0 up         # 컨테이너 시작"
    echo "  $0 down       # 컨테이너 중지"
    echo "  $0 logs       # 로그 보기"
}

# Docker와 Docker Compose 설치 확인
check_docker() {
    if ! command -v docker &> /dev/null; then
        echo -e "${RED}❌ Docker가 설치되어 있지 않습니다.${NC}"
        exit 1
    fi

    if ! command -v docker-compose &> /dev/null && ! docker compose version &> /dev/null; then
        echo -e "${RED}❌ Docker Compose가 설치되어 있지 않습니다.${NC}"
        exit 1
    fi
}

# Docker Compose 명령어 결정
get_compose_cmd() {
    if docker compose version &> /dev/null; then
        echo "docker compose"
    else
        echo "docker-compose"
    fi
}

# 컨테이너 시작
start_containers() {
    echo -e "${BLUE}🚀 컨테이너를 시작합니다...${NC}"
    
    # 프론트엔드 빌드
    echo -e "${YELLOW}📦 프론트엔드를 빌드합니다...${NC}"
    cd frontend
    if [ -f "package.json" ]; then
        npm run build:backend
        if [ $? -ne 0 ]; then
            echo -e "${RED}❌ 프론트엔드 빌드에 실패했습니다.${NC}"
            exit 1
        fi
    else
        echo -e "${RED}❌ frontend/package.json 파일을 찾을 수 없습니다.${NC}"
        exit 1
    fi
    cd ..
    
    # Docker 네트워크 생성 (존재하지 않는 경우)
    docker network create docker-link 2>/dev/null || true
    
    # 컨테이너 시작
    $COMPOSE_CMD -f $DOCKER_COMPOSE_FILE up -d
    
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✅ 컨테이너가 성공적으로 시작되었습니다.${NC}"
        echo -e "${BLUE}🌐 애플리케이션: http://localhost:8080${NC}"
    else
        echo -e "${RED}❌ 컨테이너 시작에 실패했습니다.${NC}"
        exit 1
    fi
}

# 컨테이너 중지
stop_containers() {
    echo -e "${YELLOW}🛑 컨테이너를 중지합니다...${NC}"
    $COMPOSE_CMD -f $DOCKER_COMPOSE_FILE down
    
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✅ 컨테이너가 성공적으로 중지되었습니다.${NC}"
    else
        echo -e "${RED}❌ 컨테이너 중지에 실패했습니다.${NC}"
        exit 1
    fi
}

# 컨테이너 재시작
restart_containers() {
    echo -e "${BLUE}🔄 컨테이너를 재시작합니다...${NC}"
    stop_containers
    start_containers
}

# 로그 보기
show_logs() {
    echo -e "${BLUE}📋 컨테이너 로그를 표시합니다...${NC}"
    $COMPOSE_CMD -f $DOCKER_COMPOSE_FILE logs -f
}

# 상태 확인
show_status() {
    echo -e "${BLUE}📊 컨테이너 상태:${NC}"
    $COMPOSE_CMD -f $DOCKER_COMPOSE_FILE ps
}

# 이미지 빌드
build_images() {
    echo -e "${BLUE}🔨 이미지를 빌드합니다...${NC}"
    
    # 프론트엔드 빌드
    echo -e "${YELLOW}📦 프론트엔드를 빌드합니다...${NC}"
    cd frontend
    npm run build:backend
    cd ..
    
    # Docker 이미지 빌드
    $COMPOSE_CMD -f $DOCKER_COMPOSE_FILE build
    
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✅ 이미지 빌드가 완료되었습니다.${NC}"
    else
        echo -e "${RED}❌ 이미지 빌드에 실패했습니다.${NC}"
        exit 1
    fi
}

# 재빌드 후 시작
rebuild_and_start() {
    echo -e "${BLUE}🔄 이미지를 재빌드하고 컨테이너를 시작합니다...${NC}"
    stop_containers
    build_images
    start_containers
}

# 정리
clean_docker() {
    echo -e "${YELLOW}🧹 Docker 정리를 시작합니다...${NC}"
    
    # 중지된 컨테이너 제거
    echo -e "${BLUE}🗑️  중지된 컨테이너를 제거합니다...${NC}"
    docker container prune -f
    
    # 사용하지 않는 이미지 제거
    echo -e "${BLUE}🗑️  사용하지 않는 이미지를 제거합니다...${NC}"
    docker image prune -f
    
    # 사용하지 않는 볼륨 제거
    echo -e "${BLUE}🗑️  사용하지 않는 볼륨을 제거합니다...${NC}"
    docker volume prune -f
    
    echo -e "${GREEN}✅ Docker 정리가 완료되었습니다.${NC}"
}

# 메인 로직
main() {
    check_docker
    COMPOSE_CMD=$(get_compose_cmd)
    
    case "${1:-help}" in
        up|start)
            start_containers
            ;;
        down|stop)
            stop_containers
            ;;
        restart)
            restart_containers
            ;;
        logs)
            show_logs
            ;;
        status)
            show_status
            ;;
        build)
            build_images
            ;;
        rebuild)
            rebuild_and_start
            ;;
        clean)
            clean_docker
            ;;
        help|--help|-h)
            show_help
            ;;
        *)
            echo -e "${RED}❌ 알 수 없는 명령어: $1${NC}"
            echo ""
            show_help
            exit 1
            ;;
    esac
}

# 스크립트 실행
main "$@"