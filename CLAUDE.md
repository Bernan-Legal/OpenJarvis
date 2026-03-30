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

- **Modelo base:** `qwen2.5:14b-instruct` (9.0 GB VRAM) — cambiado 30 Mar 2026 desde qwen3.5:35b
- **Tag Ollama:** `teo:latest`
- **Modelfile base:** `Modelfile` (FROM qwen2.5:14b-instruct) — backup en `Modelfile.bak.35b`
- **Modelfile con system prompt:** `Modelfile_TEO.txt` (FROM teo + identidad ADN Legal)
- **Identidad:** Coordinador operativo de Bernan — análisis legal, educación IA, código, investigación
- **Capacidades:** Derecho penal CR, forense digital, instrucción IA, automatización Python, redacción, razonamiento general

**Rollback al 35B:** `ollama create teo -f Modelfile.bak.35b && ollama create teo -f Modelfile_TEO.txt`

El warning `teo not found` en `jarvis doctor` es esperado — Ollama registra el modelo como `teo:latest` pero la config dice `teo`. En práctica ambos nombres funcionan.

## Entorno

- **OS:** Windows 11 Pro, shell Git Bash
- **Python:** 3.10.18 (venv en `.venv/`)
- **uv:** 0.10.10
- **GPU:** NVIDIA RTX 5090, CUDA 12.8, Blackwell (compute 12.0)
- **Ollama:** `http://localhost:11434`
- **Modelos disponibles:** `teo:latest`, `qwen3.5:35b`, `qwen2.5:14b-instruct`, `qwen2.5:7b-instruct`
- **Node.js:** v24.14.0 (requerido para WhatsApp Baileys bridge)

## Configuración

Hay dos archivos de config:

| Archivo | Propósito |
|---------|-----------|
| `C:\proyectos_5090\OpenJarvis\config.toml` | Config del proyecto (TEO, RTX 5090) |
| `C:\Users\USUARIO\.openjarvis\config.toml` | Config de usuario (home) |

**Formato correcto** para herramientas (según referencia oficial):
```toml
[agent]
tools = "code_interpreter,web_search,file_read,shell_exec"  # string, no array
```

## Estructura del proyecto

```
src/openjarvis/
├── system.py          # JarvisSystem — composición principal
├── agents/            # Simple, React, Orchestrator, RLM, Monitor
├── tools/             # 18+ herramientas registradas
├── tools/storage/     # Backends de memoria (SQLite, FAISS, ColBERT, BM25)
├── learning/          # Learning loop: traces → SFT → LoRA → eval
├── operators/         # Agentes autónomos con manifests TOML
├── engine/            # Backends de inferencia (Ollama, vLLM, cloud)
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
| 1 | Tools | Activa |
| 2 | Memory (SQLite/Hybrid) | Activa |
| 3 | Learning Loop | Activa (LoRA requiere `orchestrator-training`) |
| 4 | Operators | Activa |
| 5 | Engines | Activa (Ollama por defecto) |
| 6 | Intelligence/Router | Activa |
| 7 | Agents | Activa |
| 8 | Traces | Activa |
| 9 | Channels | Código presente, requiere extras por canal |
| 10 | Security | Activa |
| 11 | Evals | Parcial |
| 12 | Telemetry | Activa |
| 13 | Scheduler | Activa |
| 14 | Recipes | Activa |

## Advertencias importantes

- **No modificar el proyecto original** — es un framework de Stanford, los cambios deben ir en config o en archivos propios (TEO, operadores, channels)
- **No instalar dependencias manualmente** — siempre usar `uv sync --extra <nombre>`
- **El módulo Rust** (`openjarvis_rust`) es requerido para funcionalidad completa — si se pierde, reconstruir con el comando exacto del paso 3
- **CONDA_PREFIX** conflicta con maturin — hacer `unset CONDA_PREFIX` si aparece el error de entornos duales
- **`jarvis ask` en terminal no interactiva** — el output de streaming no se captura con `2>&1` solo; usar `| cat` o la API de Ollama directamente

## Tests

```bash
uv run pytest tests/ -v                    # suite completa
uv run pytest tests/engine/ -v             # solo engine
uv run pytest tests/learning/ -v           # solo learning
uv run pytest -m "not live and not cloud"  # excluir tests que requieren servicios
```

Markers disponibles: `live`, `cloud`, `nvidia`, `amd`, `apple`, `slow`
