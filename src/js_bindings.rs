use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "pdfBridge"], js_name = loadPdfFromUrl, catch)]
    pub async fn load_pdf_from_url(url: &str) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_namespace = ["window", "pdfBridge"], js_name = loadPdfFromData, catch)]
    pub async fn load_pdf_from_data(data: &js_sys::Uint8Array) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_namespace = ["window", "pdfBridge"], js_name = renderPage, catch)]
    pub async fn render_page(
        page_num: u32,
        canvas_id: &str,
        scale: f64,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_namespace = ["window", "pdfBridge"], js_name = getNumPages)]
    pub fn get_num_pages() -> u32;

    #[wasm_bindgen(js_namespace = ["window", "pdfBridge"], js_name = isLoaded)]
    pub fn is_loaded() -> bool;

    #[wasm_bindgen(js_namespace = ["window", "pdfBridge"], js_name = computeHash, catch)]
    pub async fn compute_hash(data: &js_sys::Uint8Array) -> Result<JsValue, JsValue>;
}
