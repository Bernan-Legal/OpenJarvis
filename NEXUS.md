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
| **8020** | MULTIRAG_ADN | FastAPI + JWT | RAG legal CR — iniciar con `MULTIRAG ADN.bat` |
| **8521** | MULTIRAG_ADN | Streamlit UI | Interfaz web de MULTIRAG |
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

3. Bernan envía mensaje a TEO (texto, voz, o documento adjunto)
   │
   ├── POST /v1/chat/completions
   ├── orchestrator agent maneja el turno (máx 8)
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
| `src/openjarvis/speech/subprocess_whisper.py` | Backend STT local vía Miniconda3 |

---

## Estado del proyecto — 22 Mayo 2026 (actualizado)

### Funcionando correctamente

- TEO responde via `/v1/chat/completions` en puerto 8222
- Extensión Rust (`openjarvis_rust`) cargada
- **25 herramientas activas** — ver lista completa en CLAUDE.md (incluye `multirag_adn`)
- **Micrófono / Speech-to-text FUNCIONAL** — backend `subprocess-whisper` vía Miniconda3
- Browser automation (Playwright/Chromium) instalado y operativo
- PDF extraction (`pdfplumber 0.11.9`) instalado via `memory-pdf`
- **20 skills instalados** en `~/.openjarvis/skills/`
- Frontend React compila sin errores, proxy configurado a 8222 (timeout 180s)
- Ollama con `teo:latest` disponible — `num_ctx 16384`
- Memoria SQLite activa con recuperación selectiva
- Trazas registrándose en `~/.openjarvis/traces.db`
- **Subida de documentos** via botón Paperclip → POST `/v1/files/upload` → `~/.openjarvis/uploads/`
- **TEO invoca `file_read`/`pdf_extract`** cuando mensaje contiene `[Archivo adjunto: ruta]`
- `web_search` invocado correctamente para personas reales y datos actuales
- `shell_exec` funciona en Windows con `subprocess` + `env=None` (bypass Rust bridge)
- Fecha/hora correcta en system_prompt via `{FECHA_ACTUAL}` / `{HORA_ACTUAL}` en `serve.py`
- `Iniciar_TEO.bat` verifica puertos 8222 y 5173 antes de arrancar (evita duplicados)

### Detalles técnicos de features implementados

#### Speech-to-text (micrófono) — FUNCIONAL ✓

**Estado:** el micrófono funciona. Bernan puede dictar mensajes a TEO.

**Causa del bug anterior:** `backend.transcribe()` era una llamada bloqueante (`subprocess.run`)
dentro de un handler `async` de FastAPI. La primera transcripción carga el modelo Whisper (~30s),
lo que congelaba el event loop de uvicorn y hacía que el proxy de Vite agotara su timeout (30s),
causando `TypeError: Failed to fetch` en el browser.

**Fix aplicado (commit `96856b7`):**
- `api_routes.py` — transcripción corre en `loop.run_in_executor(None, ...)` (thread pool)
- `vite.config.ts` — timeout del proxy `/v1` aumentado de 30s a 180s
- `subprocess_whisper.py` — error explícito si el subprocess no produce output (`lines[-1]` IndexError)

**Advertencia:** la **primera** transcripción toma 20-35s (carga del modelo Whisper a RAM).
Las siguientes son inmediatas (modelo en caché del proceso mientras TEO esté corriendo).

**Configuración activa en `~/.openjarvis/config.toml`:**
```toml
[speech]
backend = "subprocess-whisper"
model = "base"
language = "es"
python_exe = "C:/Users/USUARIO/miniconda3/python.exe"
```

### Cambios recientes significativos

| Commit | Cambio | Descripción |
|--------|--------|-------------|
| (22 May) | fix: startup + executor behavior | Botón .lnk corregido; `max_turns→8`; `poll_tool_budget→15`; reglas DEBES en system_prompt; `multirag_adn` activo |
| `96856b7` | fix: speech transcription run_in_executor + proxy timeout | Micrófono "Failed to fetch" resuelto |
| `997741e` | docs: README + NEXUS update | Notas sesión 20 May 2026 noche |
| `e3d2e25` | fix: system_prompt en function_calling | `_run_function_calling` no pasaba system_prompt |
| `8f170e0` | fix: wiring system_prompt en serve.py | Leer `config.agent.system_prompt` y pasar al agente |
| `ce3a212` | fix: restaurar tavily+ddgs | Comando canónico con 4 extras juntos |
| `15ad11f` | fix: retry model fetch en startup | Prefiere `teo:latest` como default |
| `705a905` | fix: file_read rules en prompts | DEBES/PROHIBIDO para file_read/pdf_extract |
| `802f20a` | feat: microphone + document upload | Web Speech API + Paperclip + upload backend |

### Cambios anteriores significativos

| Fecha | Cambio | Descripción |
|-------|--------|-------------|
| 19 May 2026 | `config.toml` system_prompt completo | Placeholders `{FECHA_ACTUAL}` / `{HORA_ACTUAL}` dinámicos |
| 11 May 2026 | Expansión de herramientas | 6 → 24 herramientas activas |
| 11 May 2026 | Skills instalados | 20 skills built-in en `~/.openjarvis/skills/` |
| 11 May 2026 | `memory-pdf` + `browser` extras | pdfplumber + Playwright/Chromium |
| Abr 2026 | `127.0.0.1` en Ollama | Docker interceptaba `::1:11434` con `localhost` |

---

## Restricciones de entorno críticas

| Restricción | Detalle |
|-------------|---------|
| Python 3.10 en uv venv | `onnxruntime>=1.24` solo cp311+; `faster-whisper>=1.1` bloqueado en uv |
| Miniconda3 = Python 3.10 también | Pero con paquetes conda que sí tienen wheels para 3.10 |
| PyTorch en Miniconda3 | `2.10.0.dev20251001+cu130` — instalado, NO en uv venv |
| Docker Desktop activo | Intercepta `localhost:11434` como `::1` — SIEMPRE usar `127.0.0.1` |
| Puerto 8000 permanente | DeepBernan Anonymizer — TEO usa 8222 |
| Rust bridge en Windows | Falla con Python `-c "..."` → bypass via subprocess |
| `uv sync --extra X` reemplazante | Siempre usar los 4 extras juntos: `server tools-search memory-pdf browser` |

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

# 4. ¿El upload de archivos funciona? (TEO debe estar reiniciado con nuevo código)
curl -X POST http://127.0.0.1:8222/v1/files/upload -F "file=@cualquier.txt"

# 5. ¿El speech backend está activo?
curl http://127.0.0.1:8222/v1/speech/health

# 6. ¿Las herramientas locales funcionan? (Rust ext + file_read + shell_exec)
uv run python scripts/teo_tool_diagnostic.py .

# 7. ¿Qué ocupa el puerto 8000?
netstat -ano | findstr :8000
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
├── Chat responde pero sin web_search / dice "no tengo internet"
│   ├── TAVILY_API_KEY no cargada → iniciar con .secrets.ps1 / Iniciar_TEO.bat
│   └── system_prompt no inyectado → verificar que config.toml tiene system_prompt y reiniciar TEO
│
├── Upload de archivos da 405
│   └── TEO corriendo con código viejo → REINICIAR TEO con nuevo código
│
├── Micrófono gris o "Failed to fetch"
│   ├── TEO no reiniciado → Ctrl+F5 en browser; esperar hasta 35s la primera vez (carga modelo)
│   ├── Speech backend no activo → verificar [speech] en config.toml y reiniciar TEO
│   └── Puerto 8222 ocupado por instancia anterior → matar PID, reiniciar con Iniciar_TEO.bat
│
└── Frontend no carga en :5173
    └── npm run dev no está corriendo → revisar la ventana del script PS1
```

---

## Ideas para próximas sesiones

### TEO responde con voz (Text-to-Speech) — IDEA PENDIENTE DE DISEÑO

**Objetivo:** que TEO no solo reciba audio (ya funciona con Whisper), sino que devuelva sus respuestas habladas.

**Idea original (TEO la sugirió):** usar `gTTS` + `simpleaudio`.

**Problemas con ese enfoque:**
- `gTTS` manda el texto a los servidores de Google → no es local-first
- `simpleaudio.WaveObject.from_wave_file("output.mp3")` es un bug — simpleaudio no lee MP3, solo WAV
- Integrar TTS como script standalone rompe la arquitectura del framework

**Enfoque correcto para este ecosistema:**

El ecosistema ya tiene **DBS_Audios** con XTTS/OpenVoice instalado y operativo.
La integración correcta tiene dos vías, de menor a mayor complejidad:

**Opción A — http_request desde TEO a DBS_Audios (rápido, sin tocar el framework)**
```
TEO genera texto → tool http_request → POST DBS_Audios/tts → audio WAV → reproduce en frontend
```
- No requiere modificar OpenJarvis
- Solo necesita que DBS_Audios exponga un endpoint `/tts`
- Frontend reproduce el audio via `<audio>` tag o Web Audio API

**Opción B — Canal de voz nativo en OpenJarvis**
```
OpenJarvis channels/ → nuevo channel "voice_output" con backend XTTS local
```
- Más limpio arquitectónicamente
- Requiere implementar `src/openjarvis/channels/voice_output.py`
- XTTS clona la voz de Bernan (ya tiene el modelo entrenado en DBS_Audios)

**Para la próxima sesión:**
1. Verificar que DBS_Audios tiene un endpoint TTS activo (o activarlo)
2. Decidir: Opción A (rápida) o Opción B (correcta)
3. Si Opción A: añadir instrucción en system_prompt de TEO para invocar `http_request` al TTS cuando el usuario active "modo voz"
4. Si Opción B: implementar `voice_output.py` usando el mismo patrón de `subprocess_whisper.py` (subprocess a Python XTTS)
