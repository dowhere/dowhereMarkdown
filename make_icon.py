# 将 logo.png 的深色背景抠为透明，内容居中放到正方形画布，供 tauri icon 使用
from PIL import Image, ImageFilter

SRC = r'e:/test/markdown-editor-tools/logo.png'
OUT = r'e:/test/markdown-editor-tools/src-tauri/logo-source.png'
SIZE = 1024
CONTENT_RATIO = 0.84  # 内容占画布比例，四周留边

im = Image.open(SRC).convert('RGB')
gray = im.convert('L')

# 1) 笔画核心：只保留明显亮于背景的像素（去掉暗阴影与辉光）
CORE = 78
core = gray.point(lambda v: 255 if v >= CORE else 0)
# 2) 核心向外膨胀覆盖笔画暗侧面，再高斯平滑消除锯齿边界
body = core
for _ in range(3):
    body = body.filter(ImageFilter.MaxFilter(3))
body = body.filter(ImageFilter.GaussianBlur(1.6)).point(lambda v: 255 if v >= 128 else 0)

# 3) body 范围内按亮度平滑生成 alpha，保留 3D 暗侧面
LOW, HIGH = 26, 72
lut = []
for v in range(256):
    if v <= LOW:
        a = 0
    elif v >= HIGH:
        a = 255
    else:
        a = round((v - LOW) / (HIGH - LOW) * 255)
    lut.append(a)
inner_alpha = gray.point(lut)

# body 外 alpha 强制为 0；同时保留极亮的贴边辉光（深底上仍有一点霓虹感）
GLOW = 160
glow = gray.point(lambda v: 0 if v < GLOW else round((v - GLOW) / (255 - GLOW) * 200))
alpha = Image.composite(inner_alpha, glow, body)
# 羽化 alpha 边缘，去掉残余锯齿
alpha = alpha.filter(ImageFilter.GaussianBlur(0.7))

im.putalpha(alpha)

# 裁掉透明边缘
bbox = alpha.getbbox()
content = im.crop(bbox)

# 等比缩放并居中放到正方形透明画布
target = int(SIZE * CONTENT_RATIO)
w, h = content.size
scale = min(target / w, target / h)
content = content.resize((round(w * scale), round(h * scale)), Image.LANCZOS)

canvas = Image.new('RGBA', (SIZE, SIZE), (0, 0, 0, 0))
canvas.paste(content, ((SIZE - content.size[0]) // 2, (SIZE - content.size[1]) // 2), content)
canvas.save(OUT)
print('已生成:', OUT, canvas.size)
