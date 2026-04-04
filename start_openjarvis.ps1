# OpenJarvis / TEO — Script de inicio PowerShell
# Equivalente a Iniciar_TEO.bat para uso desde terminal PS o acceso directo .ps1
#
# Uso:  .\start_openjarvis.ps1
#       O doble-clic > "Ejecutar con PowerShell"
#
# TAVILY_API_KEY: cargada desde $env:TAVILY_API_KEY si ya está en el sistema,
# o desde Iniciar_TEO.bat si lo ejecutas desde allí.

param(
    [int]$BackendPort  = 8000,
    [int]$FrontendPort = 5173
)

$ProjectDir  = "C:\proyectos_5090\OpenJarvis"
$FrontendDir = "$ProjectDir\frontend"

# Leer clave Tavily: primero entorno, luego fallback hardcoded local
if (-not $env:TAVILY_API_KEY) {
    $env:TAVILY_API_KEY = "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"
}

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  TEO - OpenJarvis  |  RTX 5090"         -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# [1/2] Backend
Write-Host "[1/2] Iniciando backend TEO (puerto $BackendPort)..." -ForegroundColor Yellow
$backendCmd = "cd '$ProjectDir'; `$env:TAVILY_API_KEY='$($env:TAVILY_API_KEY)'; uv run jarvis serve --host 0.0.0.0 --port $BackendPort"
Start-Process powershell -ArgumentList "-NoExit", "-Command", $backendCmd -WindowStyle Normal

Write-Host "Esperando backend (8 s)..." -ForegroundColor Gray
Start-Sleep -Seconds 8

# [2/2] Frontend
Write-Host "[2/2] Iniciando frontend React (puerto $FrontendPort)..." -ForegroundColor Yellow
Start-Process powershell -ArgumentList "-NoExit", "-Command", "cd '$FrontendDir'; npm run dev" -WindowStyle Normal

Write-Host "Esperando frontend (5 s)..." -ForegroundColor Gray
Start-Sleep -Seconds 5

# Abrir navegador
Write-Host "Abriendo http://localhost:$FrontendPort ..." -ForegroundColor Green
Start-Process "http://localhost:$FrontendPort"

Write-Host ""
Write-Host " TEO listo."                                            -ForegroundColor Green
Write-Host " Interfaz : http://localhost:$FrontendPort"            -ForegroundColor Green
Write-Host " API      : http://localhost:$BackendPort"             -ForegroundColor Green
Write-Host " Cierra las ventanas de PowerShell para detener."      -ForegroundColor Gray
