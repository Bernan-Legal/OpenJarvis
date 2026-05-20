# CLAUDE.md — OpenJarvis / TEO

Instrucciones para Claude Code trabajando en este repositorio.

---

## Qué es este proyecto

OpenJarvis es un framework **local-first** para construir agentes de IA on-device,
desarrollado por el Stanford Scaling Intelligence Lab y Hazy Research.
**No es una aplicación — es una plataforma de infraestructura.**

Este repositorio contiene la instalación activa con **TEO** — el agente operativo
personalizado de Bernan Salazar / ADN Legal, corriendo sobre RTX 5090 en Windows 11.

---

## Reglas de trabajo en este repo

- No instalar dependencias con `pip install` directo — usar `uv sync --extra <nombre>`.
- No correr `maturin develop` desde `rust/` — siempre con el flag `-m` desde la raíz.
- No modificar `Iniciar_TEO.bat` — está gitignored y contiene la API key de Tavily.
- La config activa de jarvis está en `C:\Users\USUARIO\.openjarvis\config.toml`, NO en `config.toml` del repo (ese es solo referencia).
- Puerto 8000 está **permanentemente ocupado** por DeepBernan Anonymizer. TEO usa **8222**.

---

## Instalación

```bash
# 1. Dependencias
uv sync

# 2. Con extras de desarrollo
uv sync --extra dev

# 3. Extensión Rust — SIEMPRE desde la raíz con -m
uv run maturin develop -m rust/crates/openjarvis-python/Cargo.toml

# 4. Verificar (ignora el UnicodeEncodeError de Rich en consola Windows — es cosmético)
uv run jarvis doctor
```

### Backends opcionales

```bash
uv sync --extra memory-faiss          # búsqueda vectorial FAISS
uv sync --extra memory-pdf            # ingesta de PDFs — INSTALADO (pdfplumber 0.11.9)
uv sync --extra memory-colbert        # dense retrieval
uv sync --extra memory-bm25           # ranking sparse
uv sync --extra orchestrator-training # torch + LoRA fine-tuning
uv sync --extra gpu-metrics           # métricas RTX 5090
uv sync --extra channel-telegram      # bot Telegram
uv sync --extra server                # FastAPI REST API
uv sync --extra browser               # automatización web Playwright/Chromium — INSTALADO
# uv sync --extra speech              # audio_transcribe (faster-whisper) — requiere Python 3.11+
```

Combinar en un solo comando: `uv sync --extra memory-faiss --extra memory-pdf`

---

## Puertos del sistema

| Puerto | Servicio | Notas |
|--------|----------|-------|
| **8222** | `jarvis serve` — TEO backend API | Puerto activo de TEO |
| **5173** | Frontend React (dev) | Solo cuando corre `npm run dev` |
| **11434** | Ollama | Siempre corriendo en background |
| **8000** | DeepBernan Anonymizer | Ocupado permanentemente — NO usar para TEO |
| **8222** | Desktop Tauri standalone | Coincide intencionalmente con jarvis serve |

---

## Arrancar TEO

```
Iniciar_TEO.bat              ← acceso directo en escritorio (contiene TAVILY_API_KEY)
.\start_openjarvis.ps1       ← alternativa sin key hardcoded (lee .secrets.ps1)
```

Lo que hace el script:
1. Dot-sourcea `.secrets.ps1` → carga `TAVILY_API_KEY`
2. Inicia `jarvis serve --host 127.0.0.1 --port 8222` en ventana separada
3. Inicia `npm run dev` en `frontend/` → React en localhost:5173
4. Abre el navegador en http://localhost:5173

**No usar `--host 0.0.0.0`** para el servidor — solo exponer en loopback.

### Variables de entorno necesarias

| Variable | Dónde | Para qué |
|----------|-------|----------|
| `TAVILY_API_KEY` | `.secrets.ps1` (gitignored) | `web_search` tool |
| `OLLAMA_HOST` | Opcional | Override del host de Ollama |
| `OPENJARVIS_CONFIG` | Opcional | Override de la ruta del config |

---

## Config activo de TEO

**Ruta:** `C:\Users\USUARIO\.openjarvis\config.toml`

Secciones clave:

```toml
[engine.ollama]
host = "http://127.0.0.1:11434"   # NO localhost — Docker intercepta ::1
keep_alive = "2h"

[intelligence]
default_model = "teo"              # Modelfile en Ollama basado en qwen2.5:14b
max_tokens = 4096

[agent]
default_agent = "orchestrator"
tools = "think,calculator,web_search,code_interpreter,file_read,shell_exec,file_write,http_request,pdf_extract,retrieval,memory_store,memory_retrieve,memory_search,git_status,git_diff,git_log,agent_spawn,repl,llm,browser_navigate,browser_extract,browser_screenshot,browser_click,browser_type"

[server]
host = "127.0.0.1"
port = 8222
```

El `config.toml` en la raíz del repo es **solo referencia** — el framework no lo carga.

---

## Arquitectura del backend

```
src/openjarvis/
├── engine/         # Backends de inferencia (ollama.py, openai, anthropic…)
├── agents/         # orchestrator, react, operative, claude_code…
├── tools/          # web_search, code_interpreter, file_read, shell_exec, think…
├── channels/       # telegram, whatsapp, slack, discord…
├── server/         # FastAPI — endpoints /v1/chat/completions, /v1/models…
├── intelligence/   # routing, model selection
├── learning/       # traces → SFT feedback loop
├── memory/         # SQLite + FAISS + ColBERT backends
├── operators/      # agentes autónomos programados (cron/event)
└── traces/         # registro de ejecuciones para debugging y aprendizaje
```

### Endpoints principales de la API

| Método | Ruta | Uso |
|--------|------|-----|
| POST | `/v1/chat/completions` | Chat estilo OpenAI compatible |
| GET | `/v1/models` | Modelos disponibles |
| GET | `/health` | Liveness check |
| GET | `/v1/info` | Modelo y agente activos |
| GET | `/v1/traces` | Historial de ejecuciones |
| GET | `/v1/telemetry/stats` | Métricas de uso |
| POST | `/v1/managed-agents` | Crear agente autónomo |

---

## Frontend (React + Vite)

```
frontend/src/
├── pages/          # ChatPage, SettingsPage, OperatorsPage…
├── components/     # Dashboard, TraceDebugger, ModelSelector…
├── lib/
│   ├── api.ts      # Todas las llamadas HTTP al backend
│   └── store.ts    # Estado global (Zustand)
└── App.tsx
```

El proxy de Vite redirige `/v1/*` y `/health` a `http://127.0.0.1:8222`.
`getBase()` en `api.ts` es la función que resuelve la URL base — siempre usarla.

### Dos apps Tauri (no confundir)

| Directorio | Propósito | Puerto |
|------------|-----------|--------|
| `frontend/src-tauri/` | Wrapper Tauri del React dev mode | 8222 |
| `desktop/src-tauri/` | App standalone con auto-boot de jarvis | 8222 |

**Modo normal (recomendado):** browser en localhost:5173 via `Iniciar_TEO.bat`
**Modo Tauri desktop:** `cd desktop && npm run tauri dev`

---

## TEO — Agente personalizado

- **Modelo Ollama:** `teo:latest` (Modelfile sobre `qwen2.5:14b`)
- **Parámetros:** `num_ctx 8192`, `temperature 0.65`, `top_p 0.9`, `repeat_penalty 1.12`
- **Agente:** `orchestrator` con hasta 5 turnos
- **Herramientas activas (24):** `think`, `calculator`, `web_search`, `code_interpreter`, `file_read`, `shell_exec`, `file_write`, `http_request`, `pdf_extract`, `retrieval`, `memory_store`, `memory_retrieve`, `memory_search`, `git_status`, `git_diff`, `git_log`, `agent_spawn`, `repl`, `llm`, `browser_navigate`, `browser_extract`, `browser_screenshot`, `browser_click`, `browser_type`
- **Skills instalados (20):** `~/.openjarvis/skills/` — `pdf-summarize`, `topic-research`, `data-analyze`, `translate-doc`, `daily-digest`, `compare-docs`, `security-scan`, `code-test-gen`, y 12 más
- **Memoria:** SQLite, `top_k=3`, `min_score=0.15`, max 1024 tokens de contexto
- **Modelfile:** `Modelfile_TEO.txt` en la raíz del repo

Para recrear el modelo en Ollama:
```bash
ollama create teo -f Modelfile_TEO.txt
```

### Cómo activar herramientas adicionales

Las herramientas disponibles se registran en `src/openjarvis/tools/__init__.py` (importación con `try/except`).
Para activar una herramienta registrada, agregar su nombre a la línea `tools = "..."` en `~/.openjarvis/config.toml`.
Para activar una herramienta no registrada, agregar primero el import en `__init__.py`.

Herramientas registradas pero no activas actualmente: `agent_kill`, `agent_list`, `agent_send`, `channel_send`, `code_interpreter_docker`, `git_commit`, `memory_index`

### Cómo crear un skill personalizado

Crear un archivo TOML en `~/.openjarvis/skills/mi-skill.toml`:
```toml
[skill]
name = "mi-skill"
description = "Descripción del skill"

[[skill.steps]]
tool_name = "web_search"
arguments_template = '{"query": "{tema}"}'
output_key = "resultados"

[[skill.steps]]
tool_name = "think"
arguments_template = '{"thought": "Resumir: {resultados}"}'
output_key = "resumen"
```

---

## Comandos útiles de diagnóstico

```bash
# Test rápido del backend
curl http://127.0.0.1:8222/health
curl http://127.0.0.1:8222/v1/info

# Test de chat directo a TEO
curl -X POST http://127.0.0.1:8222/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{"model":"teo","messages":[{"role":"user","content":"test"}],"stream":false}'

# Modelos disponibles en Ollama
curl http://127.0.0.1:11434/api/tags | python -m json.tool

# Diagnóstico de herramientas locales (Rust ext + file_read + shell_exec)
uv run python scripts/teo_tool_diagnostic.py .

# Suite de tests
uv run pytest tests/ -v
```

---

## Gotchas conocidos

| Problema | Causa | Solución |
|----------|-------|----------|
| `jarvis doctor` falla con UnicodeEncodeError | Consola Windows cp1252 no maneja el checkmark de Rich | Es cosmético — el sistema funciona igual |
| Puerto 8000 rechazado | DeepBernan Anonymizer lo ocupa siempre | Usar 8222 (ya está configurado) |
| Ollama en `localhost` vs `127.0.0.1` | Docker intercepta `::1:11434` | Config usa `127.0.0.1` explícitamente |
| `maturin develop` sin flag `-m` | Instala en el venv equivocado | Siempre correr desde raíz con `-m` |
| TAVILY_API_KEY no disponible | `.secrets.ps1` no cargado | Iniciar via `Iniciar_TEO.bat` o el script PS1 |
| `audio_transcribe` no disponible | `faster-whisper` requiere Python 3.11+; entorno es 3.10 | Pendiente hasta migrar Python o usar Whisper API |
| Browser tools sin respuesta | Playwright no instalado o binarios faltantes | `uv sync --extra browser && uv run playwright install chromium` |
