# مشروع موازن الأحمال المتعدد (Multi-Protocol Load Balancer Project)

هذا المشروع عبارة عن مجموعة أدوات متكاملة لدراسة وتطبيق مفاهيم موازنة الأحمال (Load Balancing) في الأنظمة الموزعة. يتضمن المشروع موازنات أحمال متطورة بلغات مختلفة وتطبيقات خلفية (Backend) لمحاكاة سيناريوهات التشغيل الحقيقية.

---

## 🏗️ مكونات المشروع (Sub-Projects)

### 1. موازن أحمال Rust المتطور (`/load-balncer-with-rust`)
هو المكون الرئيسي والأكثر تقدماً في المشروع.
- **الطبقات (Layers)**: يدعم L4 (TCP) و L7 (HTTP).
- **الخوارزميات**: Weighted Round Robin, Least Connections, Round Robin.
- **ACL Routing**: توجيه الطلبات بناءً على المسار (مثلاً `/even` و `/odd`).
- **Health Checks**: فحص دوري لصحة السيرفرات.
- **Config**: يتم التحكم به عبر `config.yml`.

### 2. موازن أحمال Go المبسط (`/go-load-balancing`)
نسخة تعليمية بسيطة تركز على موازنة HTTP.
- **الخوارزمية**: Least Connections.
- **الميزات**: فحص حالة السيرفرات، معالجة أخطاء 503.
- **Config**: يتم التحكم به عبر `config.json`.

### 3. تطبيق Java الخلفي المطوّر (`/java-web-app`)
مصمم خصيصاً لاختبار خوارزمية Least Connections.
- **المميزات**: يحتوي على واجهة رسومية وعداد تنازلي (Countdown Timer).
- **نقطة وصول `/hold`**: تسمح بإبقاء الاتصال مفتوحاً لمدة محددة لمحاكاة ضغط الاتصالات.
- **الملف الجاهز**: `out/artifacts/simpleWebApp_v2.jar`.

### 4. تطبيق Go الخلفي الخفيف (`/go-web-app`)
خادم ويب بسيط بلغة Go لأغراض الاختبار السريع.

---

## 🚀 طريقة التشغيل (Setup & Execution)

### أولاً: تشغيل السيرفرات الخلفية (Backends)
يمكنك اختيار أي من السيرفرين (أو كلاهما معاً):

**خيار Java (الموصى به للاختبار المتقدم):**
```powershell
# كرر هذا الأمر مع تغيير المنفذ (9001، 9002، 9003) واسم السيرفر في كل نافذة تيرمينال
java -jar .\java-web-app\out\artifacts\simpleWebApp_v2.jar 9001 "Server_1"
```

**خيار Go:**
```powershell
cd go-web-app
go run main.go 9001 "Server_1"
```

### ثانياً: تشغيل موازنات الأحمال (Load Balancers)

**تشغيل موازن Rust (الأقوى):**
1. عدل ملف `config.yml` داخل المجلد لضبط السيرفرات والخوارزمية.
2. نفذ الأمر:
```powershell
cd load-balncer-with-rust
cargo run
```

**تشغيل موازن Go:**
1. عدل ملف `config.json` لضبط المنافذ.
2. نفذ الأمر:
```powershell
cd go-load-balancing
go run main.go
```

---

## 🧪 اختبار الخوارزميات (Testing Guide)

### اختبار Least Connections (عبر Java UI)
1. افتح المتصفح على موازن الأحمال (مثلاً: `http://localhost:8000`).
2. في خانة "Hold Connection"، أدخل **15** ثانية في Chrome واضغط Submit.
3. افتح Edge فوراً وادخل لنفس الرابط؛ ستلاحظ أن الموازن نقلك لسيرفر مختلف لأن السيرفر الأول مشغول حالياً.

### اختبار الـ ACL (في نسخة Rust)
- اطلب `http://localhost:8000/even`: سيتم توجيهك دائماً للسيرفر 2.
- اطلب `http://localhost:8000/odd`: سيتم التوزيع بين السيرفر 1 و 3.

---

## 🛠️ المتطلبات التقنية
- **Rust**: 1.70+
- **Go**: 1.21+
- **Java**: JRE 17+
