# OpenJarvis / TEO — Script de inicio PowerShell
# Equivalente a Iniciar_TEO.bat para uso desde terminal PS o acceso directo .ps1
#
# Uso:  .\start_openjarvis.ps1
#       O doble-clic > "Ejecutar con PowerShell"
#
# TAVILY_API_KEY: cargada desde $env:TAVILY_API_KEY si ya está en el sistema,
# o desde Iniciar_TEO.bat si lo ejecutas desde allí.

param(
    [int]$BackendPort  = 8222,
    [int]$FrontendPort = 5173
)

$ProjectDir  = "C:\proyectos_5090\OpenJarvis"
$FrontendDir = "$ProjectDir\frontend"
$BackendHost = "127.0.0.1"

# Leer variables sensibles locales si existen.
$SecretsPath = Join-Path $ProjectDir ".secrets.ps1"
if (Test-Path $SecretsPath) {
    . $SecretsPath
}

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  TEO - OpenJarvis  |  RTX 5090"         -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# [1/2] Backend
Write-Host "[1/2] Iniciando backend TEO (puerto $BackendPort)..." -ForegroundColor Yellow
$backendCmd = "cd '$ProjectDir'; if (Test-Path '.\.secrets.ps1') { . '.\.secrets.ps1' }; uv run jarvis serve --host $BackendHost --port $BackendPort"
Start-Process powershell -ArgumentList "-NoProfile", "-NoExit", "-ExecutionPolicy", "Bypass", "-Command", $backendCmd -WindowStyle Normal

Write-Host "Esperando backend (8 s)..." -ForegroundColor Gray
Start-Sleep -Seconds 8

# [2/2] Frontend
Write-Host "[2/2] Iniciando frontend React (puerto $FrontendPort)..." -ForegroundColor Yellow
$frontendCmd = "cd '$FrontendDir'; `$env:VITE_API_URL='http://${BackendHost}:$BackendPort'; npm run dev -- --host 127.0.0.1 --port $FrontendPort"
Start-Process powershell -ArgumentList "-NoProfile", "-NoExit", "-ExecutionPolicy", "Bypass", "-Command", $frontendCmd -WindowStyle Normal

Write-Host "Esperando frontend (5 s)..." -ForegroundColor Gray
Start-Sleep -Seconds 5

# Abrir navegador
Write-Host "Abriendo http://localhost:$FrontendPort ..." -ForegroundColor Green
Start-Process "http://localhost:$FrontendPort"

Write-Host ""
Write-Host " TEO listo."                                            -ForegroundColor Green
Write-Host " Interfaz : http://localhost:$FrontendPort"            -ForegroundColor Green
Write-Host " API      : http://${BackendHost}:$BackendPort"        -ForegroundColor Green
Write-Host " Cierra las ventanas de PowerShell para detener."      -ForegroundColor Gray
