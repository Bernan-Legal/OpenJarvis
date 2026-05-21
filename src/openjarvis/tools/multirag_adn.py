"""MULTIRAG_ADN tool — legal research against ADN Legal's RAG system (port 8020)."""

from __future__ import annotations

import os
from typing import Any, Optional

from openjarvis.core.registry import ToolRegistry
from openjarvis.core.types import ToolResult
from openjarvis.tools._stubs import BaseTool, ToolSpec

_DEFAULT_URL = "http://127.0.0.1:8020"
_TOKEN_CACHE: dict[str, str] = {}


def _get_token(base_url: str, username: str, password: str) -> str:
    cache_key = f"{base_url}:{username}"
    if cache_key in _TOKEN_CACHE:
        return _TOKEN_CACHE[cache_key]

    import urllib.request, json as _json

    payload = _json.dumps({"username": username, "password": password}).encode()
    req = urllib.request.Request(
        f"{base_url}/auth/token",
        data=payload,
        headers={"Content-Type": "application/json"},
        method="POST",
    )
    with urllib.request.urlopen(req, timeout=10) as resp:
        data = _json.loads(resp.read())

    token = data["access_token"]
    _TOKEN_CACHE[cache_key] = token
    return token


@ToolRegistry.register("multirag_adn")
class MultiragAdnTool(BaseTool):
    """Query the MULTIRAG_ADN legal RAG system for Costa Rican legal research."""

    tool_id = "multirag_adn"

    @property
    def spec(self) -> ToolSpec:
        return ToolSpec(
            name="multirag_adn",
            description=(
                "Consulta el sistema MULTIRAG_ADN de investigación legal costarricense. "
                "Usa 9 agentes especializados con acceso a expedientes y legislación CR indexada. "
                "Úsalo para: jurisprudencia CR, análisis de expedientes, normas legales costarricenses."
            ),
            parameters={
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "Consulta legal en español (ej: 'delito de estafa artículo 216 CP').",
                    },
                    "expediente_id": {
                        "type": "string",
                        "description": "ID de expediente específico a consultar (opcional).",
                    },
                },
                "required": ["query"],
            },
            category="legal",
        )

    def execute(self, **params: Any) -> ToolResult:
        query: str = params.get("query", "").strip()
        if not query:
            return ToolResult(tool_name="multirag_adn", content="No query provided.", success=False)

        expediente_id: Optional[str] = params.get("expediente_id") or None

        base_url = os.environ.get("MULTIRAG_ADN_URL", _DEFAULT_URL).rstrip("/")
        username = os.environ.get("MULTIRAG_ADN_USERNAME", "bernan")
        password = os.environ.get("MULTIRAG_ADN_PASSWORD", "")

        if not password:
            return ToolResult(
                tool_name="multirag_adn",
                content="MULTIRAG_ADN_PASSWORD no configurada. Agrega la variable de entorno en Iniciar_TEO.bat.",
                success=False,
            )

        import urllib.request, json as _json, urllib.error

        try:
            token = _get_token(base_url, username, password)
        except Exception as exc:
            _TOKEN_CACHE.clear()
            return ToolResult(
                tool_name="multirag_adn",
                content=f"Error de autenticación con MULTIRAG_ADN: {exc}",
                success=False,
            )

        body: dict = {"query": query, "bypass_cache": False}
        if expediente_id:
            body["expediente_id"] = expediente_id

        payload = _json.dumps(body).encode()
        req = urllib.request.Request(
            f"{base_url}/consultar",
            data=payload,
            headers={
                "Content-Type": "application/json",
                "Authorization": f"Bearer {token}",
            },
            method="POST",
        )

        try:
            with urllib.request.urlopen(req, timeout=120) as resp:
                data = _json.loads(resp.read())
        except urllib.error.HTTPError as exc:
            if exc.code == 401:
                _TOKEN_CACHE.clear()
            return ToolResult(
                tool_name="multirag_adn",
                content=f"Error HTTP {exc.code} de MULTIRAG_ADN: {exc.reason}",
                success=False,
            )
        except Exception as exc:
            return ToolResult(
                tool_name="multirag_adn",
                content=f"Error al conectar con MULTIRAG_ADN ({base_url}): {exc}",
                success=False,
            )

        respuesta = data.get("respuesta", "")
        fuentes = data.get("fuentes", [])
        intent = data.get("intent", "")
        equipos = data.get("equipos_activados", [])

        lines = [respuesta]
        if fuentes:
            lines.append(f"\n**Fuentes ({len(fuentes)}):** {', '.join(str(f) for f in fuentes[:5])}")
        if intent:
            lines.append(f"**Intent:** {intent} | **Equipos:** {', '.join(equipos) if equipos else '—'}")

        return ToolResult(
            tool_name="multirag_adn",
            content="\n".join(lines),
            success=True,
            metadata={"intent": intent, "fuentes": len(fuentes), "from_cache": data.get("from_cache", False)},
        )


__all__ = ["MultiragAdnTool"]
