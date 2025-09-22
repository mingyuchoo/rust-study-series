# PowerShell 스크립트: Docker 컨테이너 관리
param(
    [Parameter(Position=0)]
    [string]$Command = "help"
)

# 색상 함수
function Write-ColorOutput {
    param(
        [string]$Message,
        [string]$Color = "White"
    )
    
    $colorMap = @{
        "Red" = "Red"
        "Green" = "Green" 
        "Yellow" = "Yellow"
        "Blue" = "Cyan"
        "White" = "White"
    }
    
    Write-Host $Message -ForegroundColor $colorMap[$Color]
}

# Docker Compose 파일 경로
$DockerComposeFile = "docker/docker-compose.yml"

# 도움말 함수
function Show-Help {
    Write-ColorOutput "사용법: .\container.ps1 [COMMAND]" "Blue"
    Write-Host ""
    Write-ColorOutput "Commands:" "Yellow"
    Write-Host "  up, start     - 컨테이너 시작"
    Write-Host "  down, stop    - 컨테이너 중지 및 제거"
    Write-Host "  restart       - 컨테이너 재시작"
    Write-Host "  logs          - 컨테이너 로그 보기"
    Write-Host "  status        - 컨테이너 상태 확인"
    Write-Host "  build         - 이미지 빌드"
    Write-Host "  rebuild       - 이미지 재빌드 후 시작"
    Write-Host "  clean         - 중지된 컨테이너, 사용하지 않는 이미지 정리"
    Write-Host "  help          - 도움말 표시"
    Write-Host ""
    Write-ColorOutput "Examples:" "Yellow"
    Write-Host "  .\container.ps1 up         # 컨테이너 시작"
    Write-Host "  .\container.ps1 down       # 컨테이너 중지"
    Write-Host "  .\container.ps1 logs       # 로그 보기"
}

# Docker와 Docker Compose 설치 확인
function Test-Docker {
    try {
        $null = Get-Command docker -ErrorAction Stop
    }
    catch {
        Write-ColorOutput "❌ Docker가 설치되어 있지 않습니다." "Red"
        exit 1
    }

    $composeV2 = $false
    try {
        $result = docker compose version 2>$null
        if ($LASTEXITCODE -eq 0) {
            $composeV2 = $true
        }
    }
    catch {
        # Docker Compose V2가 없으면 V1 확인
    }

    if (-not $composeV2) {
        try {
            $null = Get-Command docker-compose -ErrorAction Stop
        }
        catch {
            Write-ColorOutput "❌ Docker Compose가 설치되어 있지 않습니다." "Red"
            exit 1
        }
    }

    return $composeV2
}

# Docker Compose 명령어 결정
function Get-ComposeCommand {
    $useV2 = Test-Docker
    if ($useV2) {
        return "docker compose"
    }
    else {
        return "docker-compose"
    }
}

# 컨테이너 시작
function Start-Containers {
    Write-ColorOutput "🚀 컨테이너를 시작합니다..." "Blue"
    
    # 프론트엔드 빌드
    Write-ColorOutput "📦 프론트엔드를 빌드합니다..." "Yellow"
    
    if (Test-Path "frontend/package.json") {
        Push-Location frontend
        try {
            npm run build:backend
            if ($LASTEXITCODE -ne 0) {
                Write-ColorOutput "❌ 프론트엔드 빌드에 실패했습니다." "Red"
                exit 1
            }
        }
        finally {
            Pop-Location
        }
    }
    else {
        Write-ColorOutput "❌ frontend/package.json 파일을 찾을 수 없습니다." "Red"
        exit 1
    }
    
    # Docker 네트워크 생성 (존재하지 않는 경우)
    docker network create docker-link 2>$null
    
    # 컨테이너 시작
    $composeCmd = Get-ComposeCommand
    Invoke-Expression "$composeCmd -f $DockerComposeFile up -d"
    
    if ($LASTEXITCODE -eq 0) {
        Write-ColorOutput "✅ 컨테이너가 성공적으로 시작되었습니다." "Green"
        Write-ColorOutput "🌐 애플리케이션: http://localhost:8080" "Blue"
    }
    else {
        Write-ColorOutput "❌ 컨테이너 시작에 실패했습니다." "Red"
        exit 1
    }
}

# 컨테이너 중지
function Stop-Containers {
    Write-ColorOutput "🛑 컨테이너를 중지합니다..." "Yellow"
    
    $composeCmd = Get-ComposeCommand
    Invoke-Expression "$composeCmd -f $DockerComposeFile down"
    
    if ($LASTEXITCODE -eq 0) {
        Write-ColorOutput "✅ 컨테이너가 성공적으로 중지되었습니다." "Green"
    }
    else {
        Write-ColorOutput "❌ 컨테이너 중지에 실패했습니다." "Red"
        exit 1
    }
}

# 컨테이너 재시작
function Restart-Containers {
    Write-ColorOutput "🔄 컨테이너를 재시작합니다..." "Blue"
    Stop-Containers
    Start-Containers
}

# 로그 보기
function Show-Logs {
    Write-ColorOutput "📋 컨테이너 로그를 표시합니다..." "Blue"
    
    $composeCmd = Get-ComposeCommand
    Invoke-Expression "$composeCmd -f $DockerComposeFile logs -f"
}

# 상태 확인
function Show-Status {
    Write-ColorOutput "📊 컨테이너 상태:" "Blue"
    
    $composeCmd = Get-ComposeCommand
    Invoke-Expression "$composeCmd -f $DockerComposeFile ps"
}

# 이미지 빌드
function Build-Images {
    Write-ColorOutput "🔨 이미지를 빌드합니다..." "Blue"
    
    # 프론트엔드 빌드
    Write-ColorOutput "📦 프론트엔드를 빌드합니다..." "Yellow"
    Push-Location frontend
    try {
        npm run build:backend
    }
    finally {
        Pop-Location
    }
    
    # Docker 이미지 빌드
    $composeCmd = Get-ComposeCommand
    Invoke-Expression "$composeCmd -f $DockerComposeFile build"
    
    if ($LASTEXITCODE -eq 0) {
        Write-ColorOutput "✅ 이미지 빌드가 완료되었습니다." "Green"
    }
    else {
        Write-ColorOutput "❌ 이미지 빌드에 실패했습니다." "Red"
        exit 1
    }
}

# 재빌드 후 시작
function Rebuild-AndStart {
    Write-ColorOutput "🔄 이미지를 재빌드하고 컨테이너를 시작합니다..." "Blue"
    Stop-Containers
    Build-Images
    Start-Containers
}

# 정리
function Clean-Docker {
    Write-ColorOutput "🧹 Docker 정리를 시작합니다..." "Yellow"
    
    # 중지된 컨테이너 제거
    Write-ColorOutput "🗑️  중지된 컨테이너를 제거합니다..." "Blue"
    docker container prune -f
    
    # 사용하지 않는 이미지 제거
    Write-ColorOutput "🗑️  사용하지 않는 이미지를 제거합니다..." "Blue"
    docker image prune -f
    
    # 사용하지 않는 볼륨 제거
    Write-ColorOutput "🗑️  사용하지 않는 볼륨을 제거합니다..." "Blue"
    docker volume prune -f
    
    Write-ColorOutput "✅ Docker 정리가 완료되었습니다." "Green"
}

# 메인 로직
function Main {
    switch ($Command.ToLower()) {
        { $_ -in @("up", "start") } {
            Start-Containers
        }
        { $_ -in @("down", "stop") } {
            Stop-Containers
        }
        "restart" {
            Restart-Containers
        }
        "logs" {
            Show-Logs
        }
        "status" {
            Show-Status
        }
        "build" {
            Build-Images
        }
        "rebuild" {
            Rebuild-AndStart
        }
        "clean" {
            Clean-Docker
        }
        { $_ -in @("help", "--help", "-h") } {
            Show-Help
        }
        default {
            Write-ColorOutput "❌ 알 수 없는 명령어: $Command" "Red"
            Write-Host ""
            Show-Help
            exit 1
        }
    }
}

# 스크립트 실행
Main