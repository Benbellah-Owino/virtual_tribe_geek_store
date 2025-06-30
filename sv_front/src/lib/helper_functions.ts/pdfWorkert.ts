// src/lib/pdfWorkerFactory.ts
import workerSource from 'pdfjs-dist/build/pdf.worker.mjs?raw';

export function createPdfWorkerBlobUrl(): string {
  const blob = new Blob([workerSource], { type: 'application/javascript' }); // NOT module
  return URL.createObjectURL(blob);
}
