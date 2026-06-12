# 🚀 Kế Hoạch Triển Khai Chuyên Sâu: Pay-Per-Use API Gate trên Stellar

Kế hoạch này được nâng cấp với **logic Smart Contract chuyên sâu** (sử dụng Token thực tế thay vì điểm mô phỏng) và **yêu cầu UI/UX cao cấp**. Bảng kế hoạch này hướng tới việc tự code và thiết lập hệ thống chuẩn chỉnh thay vì chỉ dùng các tool có sẵn.

---

## 🌟 Kiến Trúc Hệ Thống (Architecture)
1. **Smart Contract (Soroban/Rust):** Quản lý nạp/rút Token chuẩn (XLM) và lưu trữ Credit thông qua thư viện Token gốc của mạng Stellar.
2. **Frontend (React/Next.js):** Giao diện tương tác người dùng (Nạp tiền, Test API) với thiết kế hiện đại (Glassmorphism, Animations).
3. **Backend (Node.js):** Lắng nghe API Request, gọi hàm trừ Credit trên Smart Contract bằng quyền của Dev, sau đó trả về dữ liệu API mô phỏng.

---

## Phase 1: Chuẩn Bị Môi Trường Nâng Cao

### Step 1.1 — Khởi tạo & Cài đặt
- [ ] Khởi tạo Repo GitHub: `stellar-advanced-api-gate` (PUBLIC).
- [ ] Cài đặt [Soroban CLI](https://developers.stellar.org/docs/build/smart-contracts/getting-started/setup) trên máy thật (không dùng bản online để có thể lập trình logic token và backend độc lập).
- [ ] Cài đặt Node.js & Yarn/npm cho Frontend và Backend.

### Step 1.2 — Setup Freighter Wallet & Testnet Token
- [ ] Tạo **2 tài khoản** trên Freighter:
  - `Admin/Dev Account`: Dùng để quản lý Contract và thu lợi nhuận.
  - `User Account`: Dùng để đóng vai trò người mua API.
- [ ] Dùng Friendbot để nạp tiền (XLM) vào cả 2 ví.
- [ ] **Lưu ý:** Ghi nhớ địa chỉ của Native Token (XLM) trên Testnet là `CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC`.

---

## Phase 2: Phát Triển Smart Contract Chuyên Sâu

> [!IMPORTANT]
> Logic chuyên sâu: Chúng ta không dùng điểm "ảo", mà gọi hàm `transfer` thông qua thư viện `soroban_sdk::token` để chuyển tiền thật (XLM Testnet) vào Contract.

### Step 2.1 — Cấu trúc Contract
- `initialize(admin: Address, token: Address)`: Gán admin và cấu hình địa chỉ Token sẽ chấp nhận thanh toán (ví dụ địa chỉ của XLM).
- `deposit(user: Address, amount: i128)`: Dùng `token::Client` chuyển `amount` từ ví `user` vào Contract. Tự động quy đổi ra Credit (ví dụ: 1 XLM = 10 API call). Yêu cầu `user.require_auth()`.
- `consume_credit(dev: Address, user: Address)`: Trừ Credit của user đi 1 đơn vị. Bắt buộc phải do Dev ký (`dev.require_auth()`).
- `withdraw(dev: Address, amount: i128)`: Dev rút Token doanh thu từ Contract về ví cá nhân.

### Step 2.2 — Code & Build
- [ ] Mở editor (VSCode), viết code trong `src/lib.rs` sử dụng `soroban_sdk::token`.
- [ ] Viết **Unit Test (Tùy chọn)** để test luồng nạp/rút tiền local.
- [ ] Build sang định dạng `.wasm`: `cargo build --target wasm32-unknown-unknown --release`.

### Step 2.3 — Deploy Lên Testnet
- [ ] Sử dụng Soroban CLI để deploy.
- [ ] **Lưu lại Contract ID** sau khi deploy thành công.

---

## Phase 3: Xây Dựng Backend (Trạm Giao Tiếp)

### Step 3.1 — Setup Express.js
- [ ] Tạo một server Node.js nhỏ (`npm init -y`, `npm i express @stellar/stellar-sdk`).
- [ ] Gắn Secret Key của tài khoản Dev vào file `.env` của Backend.

### Step 3.2 — Logic gọi Soroban từ Backend
- [ ] Viết API endpoint: `POST /api/generate-data`
- [ ] Khi nhận request từ giao diện, Backend sẽ:
  1. Dùng Secret Key của Dev để ký giao dịch gọi hàm `consume_credit` đẩy lên mạng Stellar Testnet.
  2. Nếu Transaction trả về Status = OK (Trừ credit thành công), Backend trả về dữ liệu thật (Ví dụ: đoạn text AI) cho Frontend.
  3. Nếu báo lỗi (Do User hết Credit), Backend trả mã HTTP `403 Payment Required`.

---

## Phase 4: Xây Dựng Giao Diện UI/UX Cao Cấp

### Step 4.1 — Công nghệ Frontend
- Dùng **React / Vite / Next.js**.
- Styling bằng **Tailwind CSS**.
- Cài đặt thư viện Wallet: `@stellar/freighter-api`.

### Step 4.2 — Thiết kế UI (Mockup)
- [ ] **Theme:** Dark mode hiện đại, sử dụng hiệu ứng kính mờ (Glassmorphism), bóng đổ màu Neon (Glow effect) tím/xanh.
- [ ] **Layout chia 2 khối chính:**
  - **Khối 1 (Wallet & Balance):** Hiện địa chỉ ví đang kết nối. Hiện số dư Credit khổng lồ. Form nhập số token XLM muốn nạp kèm nút bấm "Nạp tiền mua Credit".
  - **Khối 2 (API Testing Sandbox):** Bảng hiển thị API (Thời tiết/AI/Phân tích Data). Có nút "GỌI API TỪ BACKEND". Cần có hiệu ứng quay Loading thật đẹp khi bấm vào chờ Stellar duyệt.
- [ ] **Terminal Log UI (Optional):** Có một khung nhỏ dưới góc màn hình chạy text giống console của hacker báo cáo các tiến trình để người xem hiểu chuyện gì đang xảy ra (Đang kết nối ví -> Gọi hàm deposit trên chuỗi -> Contract cập nhật Credit...).

### Step 4.3 — Tích hợp Ký Giao Dịch
- [ ] Viết hàm `connectWallet` để liên kết Freighter.
- [ ] Viết hàm `buyCreditsTransaction` dùng thư viện SDK để tạo payload gọi `deposit`, bật ví Freighter lên bắt người dùng bấm Xác nhận.

---

## Phase 5: Test & Validate Luồng Toàn Diện

- [ ] **Kịch bản 1:** Ví User chưa nạp tiền -> Bấm gọi API -> Trình duyệt báo lỗi "Không đủ Credit, hãy nạp XLM".
- [ ] **Kịch bản 2:** Bấm nút nạp 1 XLM -> Ký ví Freighter -> Stellar trừ 1 XLM -> Contract cấp 10 Credit.
- [ ] **Kịch bản 3:** Ví User bấm gọi API -> Giao diện quay Loading -> Backend báo Stellar trừ 1 Credit -> Trả về kết quả JSON màu xanh -> Bảng Credit trên UI tụt xuống còn 9.
- [ ] **Kịch bản 4:** Dùng CLI hoặc Script riêng, đóng vai Admin rút toàn bộ XLM từ Contract về ví thành công.

---

## Phase 6: Hoàn Thiện & Nộp Bài

- [ ] **Hoàn thiện README:** Chụp ảnh màn hình giao diện UI/UX "khủng" của bạn lên. Vẽ lại sơ đồ luồng (Frontend -> Backend -> Smart Contract) vì nó cực kỳ chuyên sâu.
- [ ] Cung cấp các link giao dịch tiêu biểu trên `stellar.expert`.
- [ ] Nếu có thời gian, hãy quay màn hình 2 phút trình diễn luồng nạp và trừ tiền tự động.
- [ ] Nộp Form dự án cho Ban tổ chức Rise In.
