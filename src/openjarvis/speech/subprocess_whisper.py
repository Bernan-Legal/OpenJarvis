"""Subprocess Whisper backend — transcribes using an external Python with faster-whisper.

Useful when the uv venv can't install faster-whisper (e.g. onnxruntime Python 3.10
incompatibility) but another Python environment (Miniconda, system) already has it.
"""

from __future__ import annotations

import json
import os
import subprocess
import tempfile
from typing import List, Optional

from openjarvis.core.registry import SpeechRegistry
from openjarvis.speech._stubs import SpeechBackend, TranscriptionResult

_TRANSCRIBE_SCRIPT = """
import sys, json, warnings
warnings.filterwarnings("ignore")
import faster_whisper

model_size = sys.argv[1]
audio_path = sys.argv[2]
language   = sys.argv[3] if sys.argv[3] != "None" else None

model = faster_whisper.WhisperModel(model_size, device="auto", compute_type="int8")
segs, info = model.transcribe(audio_path, language=language)
text = " ".join(s.text.strip() for s in segs)
print(json.dumps({"text": text, "language": getattr(info, "language", None) or "es"}))
"""


@SpeechRegistry.register("subprocess-whisper")
class SubprocessWhisperBackend(SpeechBackend):
    """Local STT via subprocess call to an external Python with faster-whisper.

    Configure via config.toml:

        [speech]
        backend = "subprocess-whisper"
        model = "base"          # tiny | base | small | medium | large-v3
        language = "es"
        python_exe = "C:/Users/USUARIO/miniconda3/python.exe"
    """

    backend_id = "subprocess-whisper"

    def __init__(
        self,
        python_exe: str = "",
        model_size: str = "base",
        language: str = "",
    ) -> None:
        self._python_exe = python_exe or "python"
        self._model_size = model_size
        self._language = language or None

    def transcribe(
        self,
        audio: bytes,
        *,
        format: str = "webm",
        language: Optional[str] = None,
    ) -> TranscriptionResult:
        lang = language or self._language

        suffix = f".{format}" if not format.startswith(".") else format
        with tempfile.NamedTemporaryFile(suffix=suffix, delete=False) as tmp:
            tmp.write(audio)
            tmp_path = tmp.name

        try:
            result = subprocess.run(
                [
                    self._python_exe, "-c", _TRANSCRIBE_SCRIPT,
                    self._model_size,
                    tmp_path.replace("\\", "/"),
                    str(lang),
                ],
                capture_output=True,
                text=True,
                timeout=120,
            )
            if result.returncode != 0:
                raise RuntimeError(result.stderr.strip() or "Subprocess transcription failed")

            # Last non-empty line is the JSON output
            lines = [l for l in result.stdout.strip().splitlines() if l.strip()]
            data = json.loads(lines[-1])
            return TranscriptionResult(
                text=data.get("text", ""),
                language=data.get("language"),
                confidence=None,
                duration_seconds=0.0,
                segments=[],
            )
        finally:
            try:
                os.unlink(tmp_path)
            except OSError:
                pass

    def health(self) -> bool:
        try:
            result = subprocess.run(
                [self._python_exe, "-c", "import faster_whisper; print('ok')"],
                capture_output=True,
                text=True,
                timeout=10,
            )
            return result.returncode == 0 and "ok" in result.stdout
        except Exception:
            return False

    def supported_formats(self) -> List[str]:
        return ["wav", "mp3", "m4a", "ogg", "flac", "webm"]
