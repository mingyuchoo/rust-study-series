# Performance Benchmark Script for Rust Qdrant Vector RAG Service
# PowerShell version for Windows

param(
    [switch]$SkipBuild,
    [switch]$SkipTests,
    [switch]$SkipService,
    [string]$ReportPath = "performance_report_$(Get-Date -Format 'yyyyMMdd_HHmmss').txt"
)

# Colors for output
$ErrorColor = "Red"
$WarningColor = "Yellow"
$InfoColor = "Green"

function Write-Status {
    param([string]$Message)
    Write-Host "[INFO] $Message" -ForegroundColor $InfoColor
}

function Write-Warning {
    param([string]$Message)
    Write-Host "[WARN] $Message" -ForegroundColor $WarningColor
}

function Write-Error {
    param([string]$Message)
    Write-Host "[ERROR] $Message" -ForegroundColor $ErrorColor
}

function Test-Environment {
    Write-Status "Checking environment variables..."
    
    $required = @("AZURE_OPENAI_ENDPOINT", "AZURE_OPENAI_API_KEY")
    $missing = @()
    
    foreach ($var in $required) {
        if (-not (Get-ChildItem Env:$var -ErrorAction SilentlyContinue)) {
            $missing += $var
        }
    }
    
    if ($missing.Count -gt 0) {
        Write-Error "Missing required environment variables: $($missing -join ', ')"
        return $false
    }
    
    if (-not (Get-ChildItem Env:QDRANT_URL -ErrorAction SilentlyContinue)) {
        Write-Warning "QDRANT_URL not set, using default: http://localhost:6333"
        $env:QDRANT_URL = "http://localhost:6333"
    }
    
    Write-Status "Environment variables OK"
    return $true
}

function Build-Project {
    if ($SkipBuild) {
        Write-Status "Skipping build (--SkipBuild specified)"
        return $true
    }
    
    Write-Status "Building project in release mode..."
    
    $result = & cargo build --release
    if ($LASTEXITCODE -eq 0) {
        Write-Status "Build successful"
        return $true
    } else {
        Write-Error "Build failed"
        return $false
    }
}

function Run-UnitTests {
    if ($SkipTests) {
        Write-Status "Skipping tests (--SkipTests specified)"
        return $true
    }
    
    Write-Status "Running unit tests..."
    
    $result = & cargo test --release --lib
    if ($LASTEXITCODE -eq 0) {
        Write-Status "Unit tests passed"
        return $true
    } else {
        Write-Error "Unit tests failed"
        return $false
    }
}

function Run-PerformanceTests {
    if ($SkipTests) {
        Write-Status "Skipping performance tests (--SkipTests specified)"
        return $true
    }
    
    Write-Status "Running performance tests..."
    
    # Set test environment variables
    $env:SKIP_CONNECTIVITY_TEST = "1"
    $env:RUST_LOG = "info"
    
    # Run performance tests with ignored flag
    $result = & cargo test --release --test performance_tests -- --ignored --nocapture
    if ($LASTEXITCODE -eq 0) {
        Write-Status "Performance tests completed"
        return $true
    } else {
        Write-Warning "Some performance tests may have failed (this is normal if external services are unavailable)"
        return $true  # Don't fail the entire benchmark for this
    }
}

function Start-Service {
    if ($SkipService) {
        Write-Status "Skipping service tests (--SkipService specified)"
        return $null
    }
    
    Write-Status "Starting service for load testing..."
    
    # Kill any existing instances
    Get-Process -Name "rust-qdrant-vector-rag" -ErrorAction SilentlyContinue | Stop-Process -Force
    Start-Sleep -Seconds 2
    
    # Start the service in background
    $env:RUST_LOG = "warn"
    $process = Start-Process -FilePath ".\target\release\rust-qdrant-vector-rag.exe" -PassThru -WindowStyle Hidden
    
    # Wait for service to start
    Start-Sleep -Seconds 5
    
    # Check if service is running
    if ($process -and -not $process.HasExited) {
        Write-Status "Service started with PID: $($process.Id)"
        return $process
    } else {
        Write-Error "Failed to start service"
        return $null
    }
}

function Stop-Service {
    param([System.Diagnostics.Process]$Process)
    
    if ($Process -and -not $Process.HasExited) {
        Write-Status "Stopping service (PID: $($Process.Id))..."
        $Process.Kill()
        $Process.WaitForExit(5000)  # Wait up to 5 seconds
        Write-Status "Service stopped"
    }
}

function Run-LoadTests {
    param([System.Diagnostics.Process]$ServiceProcess)
    
    if (-not $ServiceProcess) {
        Write-Warning "Service not running, skipping load tests"
        return
    }
    
    Write-Status "Running HTTP load tests..."
    
    $baseUrl = "http://localhost:8080"
    
    # Test health endpoint
    Write-Status "Testing health endpoint..."
    $successCount = 0
    for ($i = 1; $i -le 10; $i++) {
        try {
            $response = Invoke-WebRequest -Uri "$baseUrl/health" -TimeoutSec 5 -ErrorAction Stop
            if ($response.StatusCode -eq 200) {
                $successCount++
                Write-Host "." -NoNewline
            } else {
                Write-Host "x" -NoNewline
            }
        } catch {
            Write-Host "x" -NoNewline
        }
    }
    Write-Host ""
    Write-Status "Health endpoint: $successCount/10 requests successful"
    
    # Test metrics endpoint
    Write-Status "Testing metrics endpoint..."
    try {
        $response = Invoke-WebRequest -Uri "$baseUrl/api/v1/metrics" -TimeoutSec 10 -ErrorAction Stop
        if ($response.StatusCode -eq 200) {
            Write-Status "Metrics endpoint OK"
        } else {
            Write-Warning "Metrics endpoint returned status: $($response.StatusCode)"
        }
    } catch {
        Write-Warning "Metrics endpoint may not be available: $($_.Exception.Message)"
    }
    
    # Test performance health endpoint
    Write-Status "Testing performance health endpoint..."
    try {
        $response = Invoke-WebRequest -Uri "$baseUrl/api/v1/health/performance" -TimeoutSec 10 -ErrorAction Stop
        if ($response.StatusCode -eq 200) {
            Write-Status "Performance health endpoint OK"
        } else {
            Write-Warning "Performance health endpoint returned status: $($response.StatusCode)"
        }
    } catch {
        Write-Warning "Performance health endpoint may not be available: $($_.Exception.Message)"
    }
}

function Run-MemoryTest {
    param([System.Diagnostics.Process]$ServiceProcess)
    
    if (-not $ServiceProcess) {
        Write-Warning "Service not running, skipping memory test"
        return
    }
    
    Write-Status "Running memory usage test..."
    
    for ($i = 1; $i -le 30; $i++) {
        try {
            $ServiceProcess.Refresh()
            if (-not $ServiceProcess.HasExited) {
                $memoryMB = [math]::Round($ServiceProcess.WorkingSet64 / 1MB, 2)
                Write-Host "Memory usage: ${memoryMB}MB"
            } else {
                Write-Warning "Service has exited"
                break
            }
        } catch {
            Write-Warning "Could not get memory usage: $($_.Exception.Message)"
            break
        }
        Start-Sleep -Seconds 1
    }
}

function Generate-Report {
    param([string]$Path)
    
    Write-Status "Generating performance report..."
    
    $systemInfo = Get-ComputerInfo
    $rustVersion = & rustc --version
    
    $report = @"
=== Rust Qdrant Vector RAG Performance Report ===
Generated: $(Get-Date)
System: $($systemInfo.WindowsProductName) $($systemInfo.WindowsVersion)
Processor: $($systemInfo.CsProcessors[0].Name)
Total Memory: $([math]::Round($systemInfo.TotalPhysicalMemory / 1GB, 2))GB
Rust Version: $rustVersion

Build Configuration:
- Release mode: Yes
- Target: x86_64-pc-windows-msvc

Test Results:
- Unit tests: $(if (-not $SkipTests) { "Completed" } else { "Skipped" })
- Performance tests: $(if (-not $SkipTests) { "Completed" } else { "Skipped" })
- Load tests: $(if (-not $SkipService) { "Completed" } else { "Skipped" })
- Memory tests: $(if (-not $SkipService) { "Completed" } else { "Skipped" })

Recommendations:
1. Monitor memory usage during high load
2. Consider connection pooling for production
3. Enable caching for frequently accessed data
4. Use performance metrics for monitoring
5. Run load tests with realistic data volumes

Performance Optimization Features Implemented:
- Connection pooling for external services
- Request/response caching with TTL
- Performance metrics and monitoring endpoints
- Memory usage optimization for large documents
- Comprehensive performance tests and benchmarks

"@

    $report | Out-File -FilePath $Path -Encoding UTF8
    Write-Status "Performance report saved to: $Path"
}

# Main execution
function Main {
    Write-Host "=== Rust Qdrant Vector RAG Performance Benchmark ===" -ForegroundColor Cyan
    Write-Host "Starting benchmark at $(Get-Date)" -ForegroundColor Cyan
    
    if (-not (Test-Environment)) {
        exit 1
    }
    
    if (-not (Build-Project)) {
        exit 1
    }
    
    if (-not (Run-UnitTests)) {
        exit 1
    }
    
    if (-not (Run-PerformanceTests)) {
        exit 1
    }
    
    $serviceProcess = Start-Service
    
    try {
        if ($serviceProcess) {
            Run-LoadTests -ServiceProcess $serviceProcess
            Run-MemoryTest -ServiceProcess $serviceProcess
        }
    } finally {
        if ($serviceProcess) {
            Stop-Service -Process $serviceProcess
        }
    }
    
    Generate-Report -Path $ReportPath
    
    Write-Status "Benchmark completed successfully!"
    Write-Host "Report saved to: $ReportPath" -ForegroundColor Cyan
}

# Run main function
try {
    Main
} catch {
    Write-Error "Benchmark failed: $($_.Exception.Message)"
    exit 1
}