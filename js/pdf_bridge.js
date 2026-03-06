// PDF.js bridge for Leptos WASM interop
const pdfjsLib = await import("https://cdnjs.cloudflare.com/ajax/libs/pdf.js/4.9.155/pdf.min.mjs");

pdfjsLib.GlobalWorkerOptions.workerSrc =
    "https://cdnjs.cloudflare.com/ajax/libs/pdf.js/4.9.155/pdf.worker.min.mjs";

let currentPdf = null;
let currentPage = null;

window.pdfBridge = {
    async loadPdfFromUrl(url) {
        const loadingTask = pdfjsLib.getDocument(url);
        currentPdf = await loadingTask.promise;
        return currentPdf.numPages;
    },

    async loadPdfFromData(data) {
        const loadingTask = pdfjsLib.getDocument({ data });
        currentPdf = await loadingTask.promise;
        return currentPdf.numPages;
    },

    async renderPage(pageNum, canvasId, scale) {
        if (!currentPdf) return { width: 0, height: 0 };

        const page = await currentPdf.getPage(pageNum);
        currentPage = page;
        const viewport = page.getViewport({ scale });

        const canvas = document.getElementById(canvasId);
        if (!canvas) return { width: 0, height: 0 };

        const context = canvas.getContext("2d");
        canvas.width = viewport.width;
        canvas.height = viewport.height;

        const renderContext = {
            canvasContext: context,
            viewport: viewport,
        };

        await page.render(renderContext).promise;
        return { width: viewport.width, height: viewport.height };
    },

    getNumPages() {
        return currentPdf ? currentPdf.numPages : 0;
    },

    isLoaded() {
        return currentPdf !== null;
    },

    async computeHash(data) {
        const hashBuffer = await crypto.subtle.digest("SHA-256", data);
        const hashArray = Array.from(new Uint8Array(hashBuffer));
        return hashArray.map((b) => b.toString(16).padStart(2, "0")).join("");
    },
};
