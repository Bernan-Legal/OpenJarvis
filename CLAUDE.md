# CLAUDE.md — OpenJarvis

## Qué es este proyecto

OpenJarvis es un framework local-first para construir agentes de IA on-device, desarrollado por el Stanford Scaling Intelligence Lab y Hazy Research. **No es una aplicación — es una plataforma de infraestructura.** Sobre ella se construyen agentes personalizados.

Este repositorio contiene la instalación activa del proyecto con **TEO** — el agente operativo personalizado para Bernan Salazar / ADN Legal, corriendo sobre RTX 5090.

## Instalación oficial (README)

Seguir **estrictamente** este proceso. No instalar dependencias manualmente con `pip install` ni `uv pip install` fuera de este flujo:

```bash
# 1. Dependencias oficiales
uv sync

# 2. Para desarrollo completo
uv sync --extra dev

# 3. Extensión Rust — comando exacto, con el flag -m
uv run maturin develop -m rust/crates/openjarvis-python/Cargo.toml

# 4. Verificar
uv run jarvis doctor
```

**Nunca** correr `maturin develop` desde el directorio `rust/` ni desde `rust/crates/openjarvis-python/` directamente — instala en el venv equivocado. Usar siempre el comando con `-m` desde la raíz del proyecto.

## Backends opcionales

Se activan con `uv sync --extra <nombre>`, **nunca** con `pip install` manual:

```bash
uv sync --extra memory-faiss          # búsqueda vectorial
uv sync --extra memory-pdf            # ingesta de PDFs
uv sync --extra memory-colbert        # dense retrieval
uv sync --extra memory-bm25           # ranking sparse
uv sync --extra orchestrator-training # torch + LoRA fine-tuning
uv sync --extra gpu-metrics           # métricas RTX 5090
uv sync --extra channel-telegram      # bot Telegram
uv sync --extra server                # FastAPI REST API
```

Combinar en un solo comando: `uv sync --extra memory-faiss --extra memory-pdf`

## Interfaz principal de TEO

TEO se usa principalmente via la interfaz web, NO via `jarvis ask` en CLI:

```
Iniciar_TEO.bat           ← acceso directo en escritorio (GITIGNORED — contiene API key)
start_openjarvis.ps1      ← alternativa PowerShell (en git, sin key hardcoded)
    → set TAVILY_API_KEY=...
    → jarvis serve --host 0.0.0.0 --port 8000   (backend API, ventana separada)
    → npm run dev en frontend/                   (React en localhost:5173, ventana separada)
    → abre http://localhost:5173 en el navegador
```

El servidor usa `orchestrator` como agente por defecto.
Las herramientas se configuran en `[server]` y `[agent]` del config.

**Acceso directo en escritorio:** `C:\Users\USUARIO\Desktop\Iniciar TEO.lnk`
→ apunta a `C:\proyectos_5090\OpenJarvis\Iniciar_TEO.bat`

**`jarvis ask` en CLI** = modo directo sin agente (LLM puro). Para activar agente:
```bash
uv run jarvis ask -a orchestrator --tools "web_search,code_interpreter,file_read,shell_exec" "query"
```

## Desktop Tauri App (modo alternativo)

El proyecto tiene **dos** apps Tauri separadas:

| Directorio | Propósito | Puerto |
|-----------|-----------|--------|
| `frontend/src-tauri/` | Tauri wrapper del frontend React (dev mode) | 8000 |
| `desktop/src-tauri/` | App standalone con auto-boot de jarvis serve | 8222 |

**Modo web (recomendado para TEO):** `Iniciar_TEO.bat` → navegador en localhost:5173  
**Modo Tauri desktop completo:** `cd desktop && npm run tauri dev` (auto-inicia todo en puerto 8222)

## Comandos clave

```bash
uv run jarvis ask "pregunta"            # consulta directa
uv run jarvis ask "pregunta" --model teo:latest  # forzar modelo
uv run jarvis doctor                    # diagnóstico de instalación
uv run jarvis init                      # detectar hardware y generar config
uv run jarvis chat                      # chat interactivo
uv run jarvis operators                 # gestión de operadores autónomos
uv run jarvis memory                    # gestión de memoria
uv run pytest tests/ -v                 # suite de tests (~4000 tests)
```

## TEO — Agente personalizado

TEO es el agente operativo general de Bernan Salazar — no se limita a lo legal:

- **Modelo base:** `qwen2.5:14b` (8.5 GB VRAM) — cambiado 30 Mar 2026 desde qwen3.5:35b
- **Tag Ollama:** `teo:latest`
- **Modelfile base:** `Modelfile` (FROM qwen2.5:14b + num_gpu 999) — backup en `Modelfile.bak.35b`
- **Modelfile con system prompt:** `Modelfile_TEO.txt` (FROM teo + identidad ADN Legal + fecha de referencia + instrucciones de herramientas)
- **Modelfile optimizado alternativo:** `Modelfile_TEO_OPTIMIZADO.txt` (template simplificado, no activo)
- **Identidad:** Coordinador operativo de Bernan — análisis legal, educación IA, código, investigación
- **Capacidades:** Derecho penal CR, forense digital, instrucción IA, automatización Python, redacción, razonamiento general

**Recrear TEO** (necesario tras cambios en Modelfile):
```bash
ollama create teo -f Modelfile
ollama create teo -f Modelfile_TEO.txt
```

**Rollback al 35B:** `ollama create teo -f Modelfile.bak.35b && ollama create teo -f Modelfile_TEO.txt`

El warning `teo not found` en `jarvis doctor` es esperado — Ollama registra el modelo como `teo:latest` pero la config dice `teo`. En práctica ambos nombres funcionan.

## Entorno

- **OS:** Windows 11 Pro, shell Git Bash
- **Python:** 3.10.18 (venv en `.venv/`)
- **uv:** 0.10.10
- **GPU:** NVIDIA RTX 5090, CUDA 12.8, Blackwell (compute 12.0)
- **Ollama:** `http://127.0.0.1:11434` — **NO usar `localhost`**: Docker Desktop intercepta `localhost:11434` (IPv6 `::1`) con solo 2 modelos. El Ollama nativo escucha en `127.0.0.1:11434` y tiene todos los modelos.
- **Modelos disponibles:** `teo:latest`, `qwen3.5:35b`, `qwen2.5:14b`, `qwen2.5:7b-instruct`, `nomic-embed-text`
- **Node.js:** v24.14.0 (requerido para WhatsApp Baileys bridge)

## Configuración

Hay dos archivos de config:

| Archivo | Propósito |
|---------|-----------|
| `C:\proyectos_5090\OpenJarvis\config.toml` | Referencia hardware/scheduler (NO lo lee el framework) |
| `C:\Users\USUARIO\.openjarvis\config.toml` | Config activa del usuario (home) |

**Formato correcto** para herramientas (string separado por comas, no array):
```toml
[agent]
tools = "think,calculator,web_search,code_interpreter,file_read,shell_exec"
```

**Config activa actual** (`~/.openjarvis/config.toml`):
```toml
[engine]         default = "ollama"
[engine.ollama]  host = "http://127.0.0.1:11434" | keep_alive = "2h"
[intelligence]   default_model = "teo" | temperature = 0.65 | max_tokens = 4096
[agent]          tools = "think,calculator,web_search,code_interpreter,file_read,shell_exec"
                 max_turns = 5 | context_from_memory = true
[tools.storage]  context_top_k = 3 | context_max_tokens = 1024
[server]         agent = "orchestrator" | model = "teo" | port = 8000
```

## Estructura del proyecto

```
src/openjarvis/
├── system.py          # JarvisSystem — composición principal
├── agents/            # Simple, React, Orchestrator, RLM, Monitor, NativeReAct
├── tools/             # 18 herramientas registradas (web_search, code_interpreter, etc.)
├── tools/storage/     # Backends de memoria (SQLite, FAISS, ColBERT, BM25)
├── learning/          # Learning loop: traces → SFT → LoRA → eval
├── operators/         # Agentes autónomos con manifests TOML
├── engine/            # Backends de inferencia (Ollama, vLLM, cloud, etc.)
├── intelligence/      # Router aprendido query→modelo
├── channels/          # 28 integraciones de mensajería
├── security/          # RBAC, guardrails, audit log
├── evals/             # Framework de evaluación
├── traces/            # Store SQLite de trazas de ejecución
├── telemetry/         # Métricas de uso
└── cli/               # 36 comandos CLI

rust/crates/
├── openjarvis-python/ # Crate que genera el módulo Python (openjarvis_rust)
└── ...                # 17 crates adicionales
```

## Las 14 primitivas del framework

| # | Primitiva | Estado base |
|---|-----------|-------------|
| 1 | Tools | Activa (18 herramientas: web_search, code_interpreter, file_read, shell_exec, think, calculator, etc.) |
| 2 | Memory (SQLite/Hybrid) | Activa — memory.db se recrea limpia en el próximo inicio |
| 3 | Learning Loop | Activa (LoRA requiere `orchestrator-training`) |
| 4 | Operators | Activa |
| 5 | Engines | Activa (Ollama por defecto, 127.0.0.1:11434) |
| 6 | Intelligence/Router | Activa |
| 7 | Agents | Activa (orchestrator en función_calling mode) |
| 8 | Traces | Activa (traces.db — 1 traza de sesión actual) |
| 9 | Channels | Código presente, requiere extras por canal |
| 10 | Security | Activa |
| 11 | Evals | Parcial |
| 12 | Telemetry | Activa |
| 13 | Scheduler | Activa |
| 14 | Recipes | Activa |

## Fixes aplicados

### 30 Mar 2026 — Correcciones core (traces, web_search, config)

| Archivo | Fix |
|---------|-----|
| `src/openjarvis/cli/serve.py` | Añadido bloque de inicialización de `TraceStore` + `trace_store` pasado a `create_app` |
| `src/openjarvis/server/app.py` | Añadido parámetro `trace_store=None` a `create_app` + `app.state.trace_store` |
| `src/openjarvis/server/routes.py` | `_handle_agent` envuelve `agent.run()` con `TraceCollector` cuando hay store |
| `src/openjarvis/server/api_routes.py` | Rutas `/v1/traces` usan `app.state.trace_store` con path explícito |
| `config.toml` (raíz) | Limpiado — removida Tavily key expuesta, nota que el framework NO lee este archivo |
| `~/.openjarvis/config.toml` | `[learning] enabled = true`, `update_interval = 50` |

### 03 Abr 2026 — Reparación botón de inicio + Tauri commands

| Archivo | Fix |
|---------|-----|
| `Iniciar_TEO.bat` | Recreado (se había perdido) — sets `TAVILY_API_KEY`, arranca backend + frontend + browser |
| `start_openjarvis.ps1` | Reconstruido desde cero (estaba vacío) — equivalente PowerShell |
| `Iniciar TEO.lnk` (Desktop) | Redirigido de `cmd.exe` → `Iniciar_TEO.bat` |
| `frontend/src-tauri/src/lib.rs` | Añadidos 7 comandos Tauri faltantes: `get_setup_status`, `get_api_base`, `fetch_models`, `pull_ollama_model`, `delete_ollama_model`, `transcribe_audio`, `speech_health` |
| `frontend/src-tauri/Cargo.toml` | Añadida feature `multipart` a reqwest (requerida por `transcribe_audio`) |
| `.gitignore` | Añadido `Iniciar_TEO.bat` (contiene API key, no debe ir a git) |

### 04 Abr 2026 — Fix rendimiento + alucinaciones + memoria corrupta

| Archivo | Fix |
|---------|-----|
| `src/openjarvis/cli/serve.py` | `agent_kwargs` ahora incluye `max_tokens` y `temperature` desde `config.intelligence` — causa raíz de respuestas truncadas (era siempre 1024, ahora 4096) |
| `src/openjarvis/engine/ollama.py` | `keep_alive="2h"` en constructor y en payloads de generate+stream — modelo permanece en VRAM entre sesiones |
| `src/openjarvis/server/api_routes.py` | `store.recent()` → `store.list_traces()` — fix error 500 en `/v1/traces` |
| `Modelfile_TEO.txt` | Fecha de referencia (04 Abr 2026) + instrucción explícita de usar `web_search` para fechas/eventos actuales |
| `~/.openjarvis/config.toml` | Herramientas extendidas: `think`, `calculator` añadidos; memoria optimizada: `context_top_k 5→3`, `context_max_tokens 2048→1024` |
| `~/.openjarvis/memory.db` | **ELIMINADO** — contenía 3 GB de binarios Rust compilados (91,279 "documentos" de `rust/target/`) ingresados accidentalmente. Era la causa raíz de alucinaciones y latencia de 5-7 min. Se recrea limpia automáticamente. |

## Advertencias importantes

- **No modificar el proyecto original** — es un framework de Stanford, los cambios deben ir en config o en archivos propios (TEO, operadores, channels)
- **No instalar dependencias manualmente** — siempre usar `uv sync --extra <nombre>`
- **El módulo Rust** (`openjarvis_rust`) es requerido para funcionalidad completa — si se pierde, reconstruir con el comando exacto del paso 3
- **CONDA_PREFIX** conflicta con maturin — hacer `unset CONDA_PREFIX` si aparece el error de entornos duales
- **`jarvis ask` en terminal no interactiva** — el output de streaming no se captura con `2>&1` solo; usar `| cat` o la API de Ollama directamente
- **NUNCA ejecutar `jarvis memory` sobre el directorio raíz** — indexa `rust/target/` (GBs de binarios compilados). Usar solo sobre directorios de documentos específicos.
- **Ollama host** — SIEMPRE `http://127.0.0.1:11434`, NUNCA `localhost` (Docker intercepta IPv6)

## Tests

```bash
uv run pytest tests/ -v                    # suite completa
uv run pytest tests/engine/ -v             # solo engine
uv run pytest tests/learning/ -v           # solo learning
uv run pytest -m "not live and not cloud"  # excluir tests que requieren servicios
```

Markers disponibles: `live`, `cloud`, `nvidia`, `amd`, `apple`, `slow`
