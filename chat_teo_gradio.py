import gradio as gr
import requests
import json
import os

API_URL = "http://localhost:8000/v1/chat/completions"

def chat_with_teo(message, history, image=None, document=None):
    messages = []
    for user_msg, assistant_msg in history:
        messages.append({"role": "user", "content": user_msg})
        if assistant_msg:
            messages.append({"role": "assistant", "content": assistant_msg})
    
    current_content = message
    if image is not None:
        current_content += "\n[Imagen adjunta]"
    if document is not None:
        current_content += f"\n[Documento adjunto: {os.path.basename(document.name)}]"

    messages.append({"role": "user", "content": current_content})

    try:
        response = requests.post(
            API_URL,
            json={
                "model": "qwen3.5:35b",
                "messages": messages,
                "temperature": 0.7,
                "max_tokens": 1024,
                "stream": False
            },
            timeout=180
        )
        response.raise_for_status()
        return response.json()["choices"][0]["message"]["content"]
    except Exception as e:
        return f"Error: {str(e)}"

with gr.Blocks(title="Chat TEO - OpenJarvis Local") as demo:
    gr.Markdown("# Chat con TEO\nAgente local en RTX 5090")
    chatbot = gr.Chatbot(height=500)
    msg = gr.Textbox(placeholder="Escribe aquí...", label="Mensaje")
    
    with gr.Row():
        image_input = gr.Image(type="filepath", label="Imagen (opcional)")
        doc_input = gr.File(label="Documento (PDF/TXT/etc.)", file_types=[".pdf", ".txt", ".md"])
    
    with gr.Row():
        voice_input = gr.Audio(
            type="filepath",
            label="Hablar o subir audio",
            interactive=True,
            waveform_options={"show_recording_waveform": True}
        )
        submit_btn = gr.Button("Enviar")
        clear_btn = gr.Button("Limpiar")

    def submit_message(message, history, image, doc, voice):
        if voice is not None:
            message = "Voz grabada: [audio procesado]"
        return "", history + [[message, None]], image, doc, None

    def bot_response(history, image, doc):
        last_msg = history[-1][0]
        reply = chat_with_teo(last_msg, history[:-1], image, doc)
        history[-1][1] = reply
        return history, None, None

    msg.submit(submit_message, [msg, chatbot, image_input, doc_input, voice_input], [msg, chatbot, image_input, doc_input, voice_input]) \
       .then(bot_response, [chatbot, image_input, doc_input], [chatbot, image_input, doc_input])

    submit_btn.click(submit_message, [msg, chatbot, image_input, doc_input, voice_input], [msg, chatbot, image_input, doc_input, voice_input]) \
               .then(bot_response, [chatbot, image_input, doc_input], [chatbot, image_input, doc_input])

    clear_btn.click(lambda: ([], None, None), None, [chatbot, image_input, doc_input])

demo.launch(share=True, server_name="0.0.0.0")