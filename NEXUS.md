# NEXUS.md — OpenJarvis / TEO

Mapa de integración del proyecto dentro del ecosistema RTX 5090 de Bernan Salazar.

---

## Qué es OpenJarvis en este ecosistema

OpenJarvis no corre aislado — es el **cerebro central de razonamiento** del ecosistema.
TEO es la instancia personalizada que coordina tareas, procesa lenguaje natural,
ejecuta herramientas y conecta con los demás proyectos del stack.

```
┌─────────────────────────────────────────────────────────────┐
│                     RTX 5090  /  Windows 11                 │
│                                                             │
│  ┌──────────────────┐    ┌──────────────────┐              │
│  │  OpenJarvis/TEO  │    │      Ollama       │              │
│  │   (jarvis serve) │◄───│  teo:latest       │              │
│  │   :8222          │    │  qwen2.5:14b      │              │
│  └────────┬─────────┘    │  :11434           │              │
│           │              └──────────────────┘              │
│           │ API REST /v1/                                   │
│           ▼                                                 │
│  ┌──────────────────┐    ┌──────────────────┐              │
│  │  Frontend React  │    │  DeepBernan      │              │
│  │  (Vite dev)      │    │  Anonymizer      │              │
│  │  :5173           │    │  :8000           │              │
│  └──────────────────┘    └──────────────────┘              │
│                                                             │
│  ┌──────────────────┐    ┌──────────────────┐              │
│  │  NanoBot Penal   │    │  ADN Legal Nexus │              │
│  │  (@NanoTheo_bot) │    │  RAG jurídico    │              │
│  │  Telegram        │    │  Streamlit       │              │
│  └──────────────────┘    └──────────────────┘              │
│                                                             │
│  ┌──────────────────┐    ┌──────────────────┐              │
│  │  DBSVideos Pro   │    │  DBS_Audios      │              │
│  │  ComfyUI+Studio  │    │  XTTS/OpenVoice  │              │
│  └──────────────────┘    └──────────────────┘              │
└─────────────────────────────────────────────────────────────┘
```

---

## Registro de puertos — ecosistema completo

| Puerto | Proyecto | Proceso | Notas |
|--------|----------|---------|-------|
| **8222** | OpenJarvis/TEO | `jarvis serve` | Backend API activo |
| **5173** | OpenJarvis/TEO | `npm run dev` (Vite) | Frontend dev server |
| **11434** | Ollama | `ollama serve` | Siempre activo — todos los modelos locales |
| **8000** | DeepBernan Anonymizer | FastAPI | Permanentemente ocupado |
| **8010** | DeepBernan Anonymizer | FastAPI (fallback) | Cuando hay conflicto con DBLab |
| **8530** | DBS Beauty Try-On | Streamlit | UI de prueba virtual |
| **8030** | DBS Beauty Try-On | FastAPI + MediaPipe | Backend visión |
| **8188** | DBSVideos Pro | ComfyUI | Generación de video |

---

## Puntos de integración de TEO

### Con Ollama

TEO delega toda la inferencia a Ollama via HTTP en `http://127.0.0.1:11434`.
El modelo `teo:latest` es un Modelfile personalizado sobre `qwen2.5:14b`.

```toml
# ~/.openjarvis/config.toml
[engine.ollama]
host = "http://127.0.0.1:11434"   # 127.0.0.1, NO localhost (Docker intercepta ::1)
keep_alive = "2h"
```

Modelos locales disponibles relevantes para el ecosistema:

| Modelo | Uso principal |
|--------|---------------|
| `teo:latest` | Agente operativo general (TEO) |
| `qwen2.5:14b` | Fallback / base de TEO |
| `qwen2.5-coder:14b` | Tareas de código pesado |
| `nomic-embed-text:latest` | Embeddings para RAG |
| `gemma4:26b` | Tareas de razonamiento alternativo |
| `glm-ocr:latest` | OCR de documentos |

### Con NanoBot Penal CR

NanoBot (`@NanoTheo_bot`) corre independiente en Telegram pero puede redirigir
consultas complejas a TEO via su API si se configura el webhook.

Integración planificada: `channel-telegram` en OpenJarvis apuntando al mismo bot.

```bash
# Activar canal Telegram en OpenJarvis
uv sync --extra channel-telegram
```

### Con ADN Legal Nexus (RAG jurídico)

ADN Legal Nexus es el sistema RAG de jurisprudencia costarricense (CIJ + SCIJ).
TEO puede consumir sus respuestas via `web_search` (Tavily + Tor) o directamente
si se expone un endpoint REST.

Integración planificada: tool `retrieval` de OpenJarvis apuntando al índice ColBERT
del RAG unificado.

### Con DBS_Audios / XTTS

TEO puede generar texto que DBS_Audios convierte a voz con la clonación XTTS.
Flujo: TEO → respuesta texto → llamada HTTP a DBS_Audios → audio con voz de Bernan.

---

## Flujo de una sesión de trabajo con TEO

```
1. Bernan ejecuta Iniciar_TEO.bat
   │
   ├── Carga TAVILY_API_KEY desde .secrets.ps1
   ├── Inicia jarvis serve en 127.0.0.1:8222
   └── Inicia npm run dev en frontend/ → localhost:5173

2. Browser abre http://localhost:5173
   │
   └── React frontend conecta a /v1/* proxy → 127.0.0.1:8222

3. Bernan envía mensaje a TEO
   │
   ├── POST /v1/chat/completions
   ├── orchestrator agent maneja el turno (máx 5)
   ├── TEO decide qué herramientas usar (24 activas):
   │   ├── think / calculator    → razonamiento y cálculo
   │   ├── web_search            → Tavily API (info actual)
   │   ├── code_interpreter/repl → Python en sandbox
   │   ├── file_read / file_write → documentos locales
   │   ├── pdf_extract           → leer PDFs (expedientes, contratos)
   │   ├── shell_exec            → comandos del sistema
   │   ├── http_request          → APIs externas directas
   │   ├── browser_navigate/extract → automatización web Playwright
   │   ├── git_status/diff/log   → operaciones git
   │   ├── memory_store/retrieve/search → memoria explícita SQLite
   │   ├── retrieval             → búsqueda en memoria por similitud
   │   ├── agent_spawn           → sub-agentes paralelos
   │   └── llm                   → delegar a otro modelo
   └── Respuesta streaming → frontend

4. La traza queda en ~/.openjarvis/traces.db
   └── Disponible en /v1/traces para debugging
```

---

## Archivos clave del proyecto

| Archivo / Ruta | Propósito |
|----------------|-----------|
| `Iniciar_TEO.bat` | Script de inicio principal (gitignored, contiene API key) |
| `start_openjarvis.ps1` | Alternativa sin key hardcoded |
| `.secrets.ps1` | Variables sensibles (gitignored — no commitear) |
| `.secrets.bat` | Equivalente .bat (gitignored) |
| `Modelfile_TEO.txt` | Definición del modelo TEO para Ollama |
| `~/.openjarvis/config.toml` | Config activa (fuera del repo) |
| `config.toml` | Config de referencia del repo (no la lee jarvis) |
| `src/openjarvis/engine/ollama.py` | Backend de inferencia Ollama |
| `frontend/src/lib/api.ts` | Todas las llamadas HTTP del frontend |
| `frontend/vite.config.ts` | Proxy dev server (apunta a 8222) |
| `scripts/teo_tool_diagnostic.py` | Diagnóstico rápido de herramientas locales |

---

## Estado del proyecto (mayo 2026)

### Funcionando correctamente

- TEO responde via `/v1/chat/completions` en puerto 8222
- Extensión Rust (`openjarvis_rust`) cargada
- **24 herramientas activas** — ver lista completa en CLAUDE.md
- Browser automation (Playwright/Chromium) instalado y operativo
- PDF extraction (`pdfplumber 0.11.9`) instalado via `memory-pdf`
- **20 skills instalados** en `~/.openjarvis/skills/`
- Frontend React compila sin errores, proxy configurado a 8222
- Ollama con `teo:latest` disponible y respondiendo
- Memoria SQLite activa con recuperación selectiva
- Trazas registrándose en `~/.openjarvis/traces.db`

### Pendiente / En progreso

- **Extensión Rust de análisis de proyectos:** `teo_tool_diagnostic.py` verifica que funcione
- **Canal Telegram:** código presente en `channels/telegram.py`, pendiente de activar con `uv sync --extra channel-telegram`
- **Integración con RAG jurídico:** planificada via tool `retrieval` + índice ColBERT
- **Fine-tuning SFT:** traces acumulándose, `auto_update = false` por ahora
- **App Tauri standalone (`desktop/`):** funcional pero no es el modo principal de uso

### Cambios recientes significativos

| Fecha | Cambio | Descripción |
|-------|--------|-------------|
| May 2026 | Expansión de herramientas | 6 → 24 herramientas activas; browser, pdf_extract, file_write, git, memory, agent_spawn |
| May 2026 | Skills instalados | 20 skills built-in en `~/.openjarvis/skills/`; `src/openjarvis/tools/__init__.py` extendido |
| May 2026 | `memory-pdf` + `browser` extras | `pdfplumber 0.11.9` y `playwright 1.58.0` / Chromium instalados |
| May 2026 | README fix | Puerto corregido `0.0.0.0:8000` → `127.0.0.1:8222`; URL frontend → `127.0.0.1:5173` |
| May 2026 | Fix puerto 8222 | Puerto oficial de TEO — 8000 siempre ocupado por DeepBernan |
| Abr 2026 | `127.0.0.1` en Ollama | Docker interceptaba `::1:11434` con `localhost` |
| Abr 2026 | `max_tokens` desde config | Ya no hardcodeado — respeta el valor del config.toml |
| Abr 2026 | `keep_alive = "2h"` | Evita que Ollama descargue el modelo entre turnos |
| Abr 2026 | `TraceStore.recent()` fix | Corregido bug en recuperación de trazas recientes |
| Mar 2026 | Modelfile `FROM qwen2.5:14b` | `qwen2.5:14b-instruct` ya no existe en Ollama Hub |

---

## Cómo diagnosticar problemas

```bash
# 1. ¿Ollama está corriendo?
curl http://127.0.0.1:11434/api/tags

# 2. ¿jarvis serve está corriendo en 8222?
curl http://127.0.0.1:8222/health
curl http://127.0.0.1:8222/v1/info

# 3. ¿TEO responde?
curl -X POST http://127.0.0.1:8222/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{"model":"teo","messages":[{"role":"user","content":"test"}],"stream":false}'

# 4. ¿Qué ocupa el puerto 8000?
netstat -ano | findstr :8000

# 5. ¿Las herramientas locales funcionan? (Rust ext + file_read + shell_exec)
uv run python scripts/teo_tool_diagnostic.py .

# 6. ¿El frontend compila?
cd frontend && npm run build
```

### Árbol de decisión ante fallo

```
TEO no responde
│
├── curl /health falla en 8222
│   ├── jarvis serve no está corriendo → ejecutar Iniciar_TEO.bat
│   └── Puerto 8222 ocupado → revisar netstat, matar proceso extraño
│
├── /health OK pero chat falla
│   ├── Ollama no corre → iniciar Ollama, verificar en 11434
│   └── Modelo teo:latest no existe → ollama create teo -f Modelfile_TEO.txt
│
├── Chat responde pero sin web_search
│   └── TAVILY_API_KEY no cargada → iniciar con .secrets.ps1 / Iniciar_TEO.bat
│
└── Frontend no carga en :5173
    └── npm run dev no está corriendo → revisar la ventana del script PS1
```
