import { useState, useCallback, useRef } from 'react';

export type SpeechState = 'idle' | 'recording' | 'transcribing';

interface SpeechRecognitionLike {
  continuous: boolean;
  interimResults: boolean;
  lang: string;
  onresult: ((event: { results: ArrayLike<{ 0: { transcript: string } }> }) => void) | null;
  onerror: ((event: { error: string }) => void) | null;
  onend: (() => void) | null;
  start(): void;
  stop(): void;
}

function getSpeechCtor(): (new () => SpeechRecognitionLike) | undefined {
  if (typeof window === 'undefined') return undefined;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  return (window as any).SpeechRecognition || (window as any).webkitSpeechRecognition;
}

export function useSpeech() {
  const [state, setState] = useState<SpeechState>('idle');
  const [error, setError] = useState<string | null>(null);
  const recognitionRef = useRef<SpeechRecognitionLike | null>(null);
  const transcriptRef = useRef<string>('');
  const resolveRef = useRef<((text: string) => void) | null>(null);

  const available = !!getSpeechCtor();

  const startRecording = useCallback(async (): Promise<void> => {
    const Ctor = getSpeechCtor();
    if (!Ctor) {
      setError('Reconocimiento de voz no disponible (usa Chrome o Edge)');
      return;
    }
    setError(null);
    transcriptRef.current = '';

    const recognition = new Ctor();
    recognition.continuous = true;
    recognition.interimResults = false;
    recognition.lang = 'es-CR';

    recognition.onresult = (event) => {
      transcriptRef.current = Array.from(event.results as ArrayLike<{ 0: { transcript: string } }>)
        .map((r) => r[0].transcript)
        .join(' ');
    };

    recognition.onerror = (event) => {
      if (event.error !== 'no-speech') setError(event.error);
      setState('idle');
      recognitionRef.current = null;
      resolveRef.current?.(transcriptRef.current);
      resolveRef.current = null;
    };

    recognition.onend = () => {
      setState('idle');
      recognitionRef.current = null;
      resolveRef.current?.(transcriptRef.current);
      resolveRef.current = null;
    };

    recognitionRef.current = recognition;
    recognition.start();
    setState('recording');
  }, []);

  const stopRecording = useCallback((): Promise<string> => {
    return new Promise((resolve) => {
      const recognition = recognitionRef.current;
      if (!recognition) {
        resolve(transcriptRef.current);
        return;
      }
      resolveRef.current = resolve;
      recognition.stop();
    });
  }, []);

  return {
    state,
    error,
    available,
    startRecording,
    stopRecording,
    isRecording: state === 'recording',
    isTranscribing: state === 'transcribing',
  };
}
