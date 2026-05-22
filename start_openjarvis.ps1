# OpenJarvis / TEO — Script de inicio PowerShell
# Uso: doble-clic en el acceso directo del escritorio "Iniciar TEO"
# Puerto 8222 = backend TEO  |  Puerto 5173 = frontend React

param(
    [int]$BackendPort  = 8222,
    [int]$FrontendPort = 5173
)

$ProjectDir  = "C:\proyectos_5090\OpenJarvis"
$FrontendDir = "$ProjectDir\frontend"
$BackendHost = "127.0.0.1"

Set-Location $ProjectDir

# Cargar variables sensibles (.secrets.ps1 contiene TAVILY_API_KEY, etc.)
$SecretsPath = Join-Path $ProjectDir ".secrets.ps1"
if (Test-Path $SecretsPath) { . $SecretsPath }

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  TEO - OpenJarvis  |  RTX 5090"         -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

function Test-Port([int]$Port) {
    $conn = netstat -ano 2>$null | Select-String ":$Port " | Select-String "LISTEN"
    return [bool]$conn
}

# ── [1/2] Backend ─────────────────────────────────────────────
if (Test-Port $BackendPort) {
    Write-Host "[1/2] Backend TEO ya activo en puerto $BackendPort." -ForegroundColor Green
} else {
    Write-Host "[1/2] Iniciando backend TEO (puerto $BackendPort)..." -ForegroundColor Yellow
    $backendCmd = "cd '$ProjectDir'; uv run jarvis serve --host $BackendHost --port $BackendPort"
    Start-Process powershell -ArgumentList "-NoProfile", "-NoExit", "-ExecutionPolicy", "Bypass", "-Command", $backendCmd

    Write-Host "      Esperando que el backend levante (hasta 90 s)..." -ForegroundColor Gray
    $elapsed = 0
    while (-not (Test-Port $BackendPort) -and $elapsed -lt 90) {
        Start-Sleep -Seconds 2
        $elapsed += 2
        Write-Host "      $elapsed s..." -ForegroundColor DarkGray
    }

    if (Test-Port $BackendPort) {
        Write-Host "      Backend listo en ${elapsed}s." -ForegroundColor Green
    } else {
        Write-Host "      ERROR: backend no levanto en 90 s. Revisa la ventana 'TEO Backend'." -ForegroundColor Red
        Write-Host "      Presiona ENTER para continuar de todas formas..."
        Read-Host
    }
}

# ── [2/2] Frontend ────────────────────────────────────────────
if (Test-Port $FrontendPort) {
    Write-Host "[2/2] Frontend React ya activo en puerto $FrontendPort." -ForegroundColor Green
} else {
    Write-Host "[2/2] Iniciando frontend React (puerto $FrontendPort)..." -ForegroundColor Yellow
    $frontendCmd = "cd '$FrontendDir'; `$env:VITE_API_URL='http://${BackendHost}:$BackendPort'; npm run dev -- --host 127.0.0.1 --port $FrontendPort"
    Start-Process powershell -ArgumentList "-NoProfile", "-NoExit", "-ExecutionPolicy", "Bypass", "-Command", $frontendCmd

    Write-Host "      Esperando frontend (hasta 30 s)..." -ForegroundColor Gray
    $elapsed = 0
    while (-not (Test-Port $FrontendPort) -and $elapsed -lt 30) {
        Start-Sleep -Seconds 2
        $elapsed += 2
    }
    Write-Host "      Frontend listo en ${elapsed}s." -ForegroundColor Green
}

# ── Abrir navegador ───────────────────────────────────────────
Write-Host ""
Write-Host "  TEO listo." -ForegroundColor Green
Write-Host "  Interfaz : http://localhost:$FrontendPort" -ForegroundColor Cyan
Write-Host "  API      : http://${BackendHost}:$BackendPort" -ForegroundColor Cyan
Write-Host "  Cierra las ventanas de PowerShell para detener." -ForegroundColor Gray
Start-Process "http://localhost:$FrontendPort"
