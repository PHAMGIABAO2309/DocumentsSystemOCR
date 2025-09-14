import fitz  # PyMuPDF
from PIL import Image
from io import BytesIO
import os
import cv2
import numpy as np
import easyocr
import re
from fastapi import FastAPI, UploadFile, File
from fastapi.responses import JSONResponse
from fastapi.middleware.cors import CORSMiddleware

# ======= OCR Reader =======
reader = easyocr.Reader(['vi'], gpu=True)  # gpu=False nếu không có CUDA

# ======= Helper functions =======
def pdf_first_half_only(pdf_bytes: bytes, output_dir="img", dpi=600):
    """
    Chuyển đổi nửa trên của trang đầu tiên của file PDF (dùng bytes) thành ảnh PNG.
    """
    # Mở tài liệu PDF từ bytes
    pdf_document = fitz.open(stream=pdf_bytes, filetype="pdf")

    os.makedirs(output_dir, exist_ok=True)

    # Lấy trang đầu tiên
    page = pdf_document[0]
    zoom = dpi / 72
    mat = fitz.Matrix(zoom, zoom)
    pix = page.get_pixmap(matrix=mat, alpha=False)

    # Chuyển pixmap thành ảnh PIL
    img = Image.open(BytesIO(pix.tobytes("png")))
    width, height = img.size
    img_top = img.crop((0, 0, width, height // 2))

    output_path_half = os.path.join(output_dir, "upload_preview.png")
    img_top.save(output_path_half)

    pdf_document.close()
    return output_path_half


def sort_and_merge_lines(results, y_threshold=60, x_threshold=120):
    results = [r for r in results if len(r[1].strip()) >= 3]
    results.sort(key=lambda r: r[0][0][1])
    lines, current_line, last_y = [], [], None

    for bbox, text, conf in results:
        y_top = bbox[0][1]
        if last_y is None or abs(y_top - last_y) <= y_threshold:
            current_line.append((bbox, text, conf))
        else:
            current_line.sort(key=lambda r: r[0][0][0])
            lines.append(current_line)
            current_line = [(bbox, text, conf)]
        last_y = y_top
    if current_line:
        current_line.sort(key=lambda r: r[0][0][0])
        lines.append(current_line)

    split_lines = []
    for line in lines:
        temp_line = [line[0]]
        for i in range(1, len(line)):
            gap = line[i][0][0][0] - line[i-1][0][1][0]
            if gap > x_threshold:
                split_lines.append(temp_line)
                temp_line = [line[i]]
            else:
                temp_line.append(line[i])
        if temp_line:
            split_lines.append(temp_line)

    merged_lines = []
    for line in split_lines:
        merged_text = " ".join([t for _, t, _ in line]).strip()
        avg_conf = np.mean([c for _, _, c in line])
        y_top = line[0][0][0][1]
        merged_lines.append((merged_text, avg_conf, y_top))
    return merged_lines


def extract_info(image_path):
    image = cv2.imread(image_path)
    h, w = image.shape[:2]
    top_image = image[0:int(h * 3/5), :]

    results = reader.readtext(top_image, detail=1, paragraph=False)
    top_lines = sort_and_merge_lines(results)

    m_origin, m_number, m_day, category, left_m_cited = None, None, None, None, None
    m_day_idx, category_idx = None, None

    # --- m_origin ---
    co_hoa_idx = None
    for idx, (text, _, _) in enumerate(top_lines):
        if "CỘNG HÒA" in text.upper():
            co_hoa_idx = idx
            break
    if co_hoa_idx is not None:
        start_keep = max(co_hoa_idx - 1, 0)
        top_lines = top_lines[start_keep:]
        if co_hoa_idx - 1 >= 0:
            m_origin = top_lines[0][0]
        rel_idx = 1
        if rel_idx + 1 < len(top_lines) and top_lines[rel_idx + 1][0].isupper():
            m_origin = (m_origin or "") + " " + top_lines[rel_idx + 1][0]

    # --- số hiệu & ngày tháng ---
    for i, (text, conf, y) in enumerate(top_lines):
        if re.match(r"^\s*Số", text, re.IGNORECASE):
            m = re.search(r"Số[:\.]?\s*(.+)", text, re.IGNORECASE)
            if m:
                m_number = m.group(1).strip()

        day_pattern = r"ngày\s+\D*?(\d{1,2})\D*?tháng\s+\D*?(\d{1,2})\D*?năm\s+\D*?(\d{4})"
        m = re.search(day_pattern, text, re.IGNORECASE)
        if m and m_day is None:
            m_day = f"{m.group(1)}/{m.group(2)}/{m.group(3)}"
            m_day_idx = i
            if i + 1 < len(top_lines):
                next_line = top_lines[i+1][0]
                if next_line.isupper():
                    category = next_line
                    category_idx = i+1

    # --- left_m_cited ---
    def extract_left_m_cited(start_idx):
        left_lines, base_y = [], None
        for j in range(start_idx+1, len(top_lines)):
            text, conf, y = top_lines[j]
            if base_y is None:
                base_y = y
                left_lines.append(text)
            else:
                if abs(y - base_y) <= 160:
                    left_lines.append(text)
                    base_y = y
                else:
                    break
        return " ".join(left_lines) if left_lines else None

    if category_idx is not None:
        left_m_cited = extract_left_m_cited(category_idx)
    elif m_day_idx is not None:
        left_m_cited = extract_left_m_cited(m_day_idx)

    return {
        "m_origin": m_origin,
        "m_number": m_number,
        "m_day": m_day,
        "category": category,
        "left_m_cited": left_m_cited
    }


# ======= FASTAPI APP =======
app = FastAPI()

app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

@app.post("/ocr")
async def upload_and_ocr(file: UploadFile = File(...)):
    # Đọc file PDF từ memory
    pdf_bytes = await file.read()

    # Trích ảnh nửa trên của trang 1
    img_path = pdf_first_half_only(pdf_bytes)

    # OCR & trích xuất
    data = extract_info(img_path)

    # Xóa ảnh tạm
    os.remove(img_path)

    return JSONResponse(content=data)

# chay bằng lệnh: uvicorn ocr_api:app --reload --host 0.0.0.0 --port 8000