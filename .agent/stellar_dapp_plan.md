# 🚀 Kế Hoạch Chi Tiết: Build dApp trên Stellar Blockchain

> Dựa trên hướng dẫn **Rise In x Stellar University Tour** — Tác giả: Verner Huang (DevRel)

---

## Tóm Tắt Trang Notion

Trang Notion hướng dẫn học viên tham gia bootcamp **Stellar Blockchain** của Rise In, từ chuẩn bị môi trường cho đến nộp dự án. Nội dung chính:

- **Công cụ chính**: [Soroban Studio](https://soroban.studio/) — IDE online, không cần cài đặt
- **Ví**: Freighter Wallet trên Testnet
- **Ngôn ngữ smart contract**: Rust (biên dịch sang WASM, deploy lên Stellar)
- **Frontend** (optional): HTML/JS thuần hoặc React/Next.js
- **Nộp bài**: Qua [Rise In](https://www.risein.com/programs/-stellar-vietnam-unitour)

---

## Phase 1: Chuẩn Bị Tài Khoản & Công Cụ

> [!IMPORTANT]
> Phase này là nền tảng — không hoàn thành sẽ không thể tiếp tục các bước sau.

### Step 1.1 — Tạo/Kết nối tài khoản GitHub
- [ ] Truy cập [github.com](https://github.com) → tạo tài khoản nếu chưa có
- [ ] Đảm bảo repo sẽ được đặt **PUBLIC** (yêu cầu bắt buộc khi nộp bài)

### Step 1.2 — Kết nối GitHub với Soroban Studio
- [ ] Truy cập [soroban.studio](https://soroban.studio/)
- [ ] Đăng nhập bằng GitHub → xác thực qua Device Activation
- [ ] Xác nhận đã kết nối thành công

### Step 1.3 — Cài đặt ví Freighter
- [ ] Tải extension Freighter tại [freighter.app](https://www.freighter.app/)
- [ ] Cài đặt vào trình duyệt (Chrome/Brave/Firefox)
- [ ] Tạo ví mới → **lưu secret key an toàn** (KHÔNG BAO GIỜ chia sẻ)

### Step 1.4 — Chuyển sang Testnet & nhận XLM thử nghiệm
- [ ] Mở extension Freighter → **Settings** → chuyển mạng sang **Testnet**
- [ ] Bấm nút yêu cầu nhận tiền thử nghiệm (Friendbot)
- [ ] Chờ giây lát → xác nhận ví đã có XLM (dùng để trả phí giao dịch)

---

## Phase 2: Chọn Ý Tưởng Dự Án

> [!TIP]
> Giữ ý tưởng **đơn giản** và có thể hoàn thành trong ~20 phút. Đừng nghĩ quá phức tạp.

### Step 2.1 — Brainstorm ý tưởng
- [ ] Sử dụng AI (Claude/Gemini/GPT/Grok) để thảo luận ý tưởng
- [ ] **Quan trọng**: Ý tưởng phải **độc đáo** — dùng ý tưởng có sẵn sẽ bị trùng và bị từ chối

### Step 2.2 — Chọn lĩnh vực phù hợp

Các lĩnh vực gợi ý:

| Lĩnh vực | Ví dụ ý tưởng |
|---|---|
| 💸 **Thanh toán & Chuyển tiền** | Chia hóa đơn, tip box, cổng thanh toán freelancer |
| 🪙 **Tài sản Token hóa** | Token vé sự kiện, chứng chỉ đại học, token cà phê |
| 🎓 **Cuộc sống Sinh viên** | Token loyalty trường, marketplace sách, DAO học bổng |
| 🗳️ **Bỏ phiếu & Quản trị** | Bỏ phiếu cộng đồng, bảng xếp hạng hackathon |
| 💰 **DeFi** | CLB tiết kiệm, escrow freelance, vesting token |

### Step 2.3 — Định phạm vi (scope)
- [ ] Nếu ý tưởng quá lớn → chỉ build phần thanh toán/chuyển tiền hôm nay
- [ ] Xác nhận ý tưởng có **liên quan blockchain** (dùng token hoặc thanh toán Stellar)
- [ ] Viết mô tả ngắn gọn (1-2 câu) về dự án

---

## Phase 3: Viết Smart Contract (Rust/Soroban)

### Step 3.1 — Dùng AI để viết code `lib.rs`
- [ ] Mở AI chatbot (Claude/Gemini/GPT)
- [ ] Mô tả ý tưởng dự án cho AI
- [ ] Yêu cầu AI viết code cho file `lib.rs` (smart contract Soroban bằng Rust)

> [!TIP]
> Prompt mẫu: *"Viết một Soroban smart contract bằng Rust cho dự án [TÊN DỰ ÁN]. Contract cần có các hàm: [liệt kê chức năng]. Sử dụng soroban-sdk."*

### Step 3.2 — Dán code vào Soroban Studio
- [ ] Mở file `lib.rs` trong Soroban Studio
- [ ] Paste code từ AI vào

### Step 3.3 — Sửa lỗi (nếu có)
- [ ] Nếu code có lỗi khi build → chụp ảnh màn hình lỗi
- [ ] Gửi ảnh lỗi cho AI để fix
- [ ] Hoặc hỏi DevRel/người hỗ trợ tại buổi học

> [!WARNING]
> Đây là bước **dễ xảy ra lỗi nhất**. Đừng ngại hỏi trợ giúp!

---

## Phase 4: Build & Deploy Contract lên Testnet

### Step 4.1 — Build (Compile) contract
- [ ] Trong Soroban Studio, bấm nút **"Build Contract"**
- [ ] Chờ quá trình biên dịch hoàn tất (Rust → WASM)
- [ ] Xác nhận build thành công (không có lỗi)

### Step 4.2 — Deploy lên Testnet
- [ ] Bấm nút **"Deploy to Testnet"**
- [ ] Kết nối ví Freighter khi được yêu cầu
- [ ] Ký giao dịch deploy
- [ ] **Lưu lại Contract ID** (quan trọng — cần cho bước sau)

> [!IMPORTANT]
> Contract ID có dạng: `CBT5F3UMMV3MVWAU34YCIKXB6AMDAO7YXU3P2FITUVACVAFGRMZVYA5X`
> Lưu lại cẩn thận — bạn sẽ cần nó cho README và frontend.

---

## Phase 5: Tương Tác (Invoke) Contract

### Step 5.1 — Gọi hàm contract
- [ ] Trong Soroban Studio, chọn hàm muốn gọi (ví dụ: `initialize()`)
- [ ] Nhập các tham số cần thiết
- [ ] Bấm **Invoke** → ký giao dịch bằng Freighter

### Step 5.2 — Xác nhận giao dịch thành công
- [ ] Kiểm tra thông báo **"Transaction submitted successfully!"**
- [ ] **Lưu lại link giao dịch** trên [stellar.expert](https://stellar.expert)
- [ ] Đảm bảo có **ít nhất 1 giao dịch invoke thành công** (bắt buộc)

> [!IMPORTANT]
> Bạn cần **ít nhất 2 giao dịch** trên stellar.expert:
> 1. Deploy contract
> 2. Invoke contract (tương tác)

---

## Phase 6: Xây Dựng Frontend *(Không Bắt Buộc)*

> Phase này là optional nhưng làm dự án **hoàn chỉnh hơn nhiều**.

### Step 6.1 — Tải công cụ phát triển
- [ ] Tải [Google Antigravity](https://antigravity.google/download) (ưu tiên) hoặc VSCode

### Step 6.2 — Clone starter code
```bash
git clone https://github.com/hien17/soroban-bootcamp.git
```

### Step 6.3 — Chọn phương pháp xây dựng UI
#### Tùy Chọn A: Dùng AI tạo file HTML/JS duy nhất
Prompt mẫu cho AI:
> *Tạo UI web hiện đại cho dApp [TÊN DỰ ÁN] trên Stellar. Cần có:*
> - *Header với tên dự án + nút "Kết nối Ví"*
> - *Phần chính với [MÔ TẢ TÍNH NĂNG]*
> - *Bảng trạng thái hiển thị địa chỉ ví*
> - *Khu vực kết quả cho transaction hash*
> - *Giao diện tối, thiết kế tối giản*
> *Dùng HTML, CSS, JS thuần. KHÔNG bao gồm logic blockchain.*

#### Tùy Chọn B: Dùng scaffold starter
```bash
cd soroban-bootcamp/frontend
```

### Step 6.4 — Kết nối Ví + Contract (code quan trọng)
- [ ] Thêm code kết nối Freighter Wallet vào JS
- [ ] Thay `YOUR_CONTRACT_ID_HERE` bằng Contract ID thực

```javascript
import * as StellarSdk from "@stellar/stellar-sdk";
import { isConnected, getAddress, signTransaction } from "@stellar/freighter-api";

const CONTRACT_ID = "YOUR_CONTRACT_ID_HERE"; // ← thay bằng ID thực
const RPC_URL = "https://soroban-testnet.stellar.org";
const NETWORK_PASSPHRASE = "Test SDF Network ; September 2015";

async function connectWallet() {
  if (!(await isConnected())) return alert("Cài Freighter!");
  const { address } = await getAddress();
  document.getElementById("wallet-address").textContent = address;
  return address;
}

async function callContract(funcName, ...args) {
  const address = await connectWallet();
  const server = new StellarSdk.SorobanRpc.Server(RPC_URL);
  const account = await server.getAccount(address);
  const contract = new StellarSdk.Contract(CONTRACT_ID);

  const tx = new StellarSdk.TransactionBuilder(account, {
    fee: StellarSdk.BASE_FEE,
    networkPassphrase: NETWORK_PASSPHRASE,
  })
    .addOperation(contract.call(funcName, ...args))
    .setTimeout(30)
    .build();

  const prepared = await server.prepareTransaction(tx);
  const { signedTxXdr } = await signTransaction(prepared.toXDR(), {
    networkPassphrase: NETWORK_PASSPHRASE,
  });
  const signed = StellarSdk.TransactionBuilder.fromXDR(signedTxXdr, NETWORK_PASSPHRASE);
  const result = await server.sendTransaction(signed);
  return result;
}
```

---

## Phase 7: Hoàn Thiện README.md

### Step 7.1 — Viết README (bắt buộc bằng tiếng Anh)
- [ ] Viết theo template bắt buộc:

```markdown
# Title
Name of the project

# Description
Why you do this project

# Features
Detailed features

# Contract
Contract link (e.g. https://stellar.expert/explorer/testnet/contract/YOUR_CONTRACT_ID?filter=history)

Contract's screenshot

# Future scopes
What are your future plan

# Profile
Your name, skills, ...
```

### Step 7.2 — Thêm ảnh chụp màn hình
- [ ] Chụp ảnh contract trên stellar.expert (phải thấy ít nhất 2 giao dịch)
- [ ] Thêm ảnh vào README bằng cách: mở GitHub editor → **Ctrl+V** để paste ảnh

> [!CAUTION]
> - Contract phải được tạo từ **ý tưởng của bạn**, KHÔNG phải contract mẫu
> - KHÔNG đưa private key hoặc file `.env` lên GitHub

---

## Phase 8: Validate & Nộp Dự Án

### Step 8.1 — Validate dự án
- [ ] Trong Soroban Studio, bấm nút **"VALIDATE PROJECT"**
- [ ] Xác nhận tất cả test đều pass

### Step 8.2 — Kiểm tra checklist cuối
- [ ] ✅ Repo GitHub là **PUBLIC**
- [ ] ✅ README có contract ID + link giao dịch + ảnh contract
- [ ] ✅ Link giao dịch mở đúng trên Stellar Expert
- [ ] ✅ Có thể giải thích code (DevRel có thể hỏi)
- [ ] ✅ Không có private key hoặc `.env` trong repo

### Step 8.3 — Nộp bài trên Rise In
- [ ] Tạo tài khoản LinkedIn (nếu chưa có)
- [ ] Truy cập [risein.com/programs/-stellar-vietnam-unitour](https://www.risein.com/programs/-stellar-vietnam-unitour)
- [ ] Điền form nộp dự án với đầy đủ thông tin

---

## Hệ Thống Milestone

| | Milestone 1 (Hôm nay) | Milestone 2 (2 tuần sau) |
|---|---|---|
| **Yêu cầu** | GitHub repo + 1 tx hash trên Testnet | Demo đầy đủ + README + video 2 phút |
| **Hỗ trợ** | DevRel trực tiếp | Nhóm cộng đồng + mentorship |

### Milestone 2 — Phát triển tiếp (Optional)
1. Hoàn thiện contract — thêm hàm, xử lý lỗi, test
2. Xây frontend thực sự — React/Next.js với scaffold starter
3. Quay video demo 2 phút
4. Viết README chi tiết hơn

### Phần thưởng Milestone 2
- 🏆 Top 3: giới thiệu trên mạng xã hội Rise In
- 🎯 Dự án xuất sắc: mentorship 1:1 + giới thiệu đối tác Stellar
- 📜 Tất cả hoàn thành: Chứng chỉ Rise In
- 🚀 Fast-track hackathon: ưu tiên cho các nhóm được Rise In tài trợ
- 💰 Hướng dẫn grant SDF Community Fund

---

## Xử Lý Sự Cố Nhanh

| Vấn đề | Giải pháp |
|---|---|
| Không cài được Rust | `brew install rust` (Mac) hoặc dùng Gitpod |
| Lệnh `stellar` không tìm thấy | `cargo install --locked stellar-cli` |
| Build thất bại | Đọc lỗi — Rust cho biết chính xác dòng lỗi |
| Deploy thất bại | Nạp tiền bằng Friendbot rồi thử lại |
| Freighter không kết nối | Đảm bảo đang ở mạng **Testnet** |
| Không biết xây gì | Chọn bất kỳ ý tưởng nào từ danh sách gợi ý |
| Sắp hết thời gian | Deploy contract mẫu — 1 tx hash = 1 KPI |

---

## Tài Nguyên Hữu Ích

| Tài nguyên | Link |
|---|---|
| Stellar Developer Docs | [developers.stellar.org](https://developers.stellar.org) |
| Soroban Smart Contracts | [developers.stellar.org/docs/build/smart-contracts](https://developers.stellar.org/docs/build/smart-contracts) |
| Scaffold Stellar (Full-Stack) | [scaffoldstellar.org](https://scaffoldstellar.org) |
| Stellar Expert (Explorer) | [stellar.expert](https://stellar.expert) |
| Stellar Laboratory | [laboratory.stellar.org](https://laboratory.stellar.org) |
| Stellar Community Fund | [communityfund.stellar.org](https://communityfund.stellar.org) |

> [!NOTE]
> **Chính sách AI**: Dùng AI như công cụ tăng tốc, không phải thay thế. AI tạo boilerplate → bạn viết logic. DevRel có thể hỏi bạn giải thích code — hãy đảm bảo **hiểu mỗi dòng làm gì**.

---

## Open Questions

> [!IMPORTANT]
> Trước khi bắt tay vào làm, bạn cần xác nhận:
> 1. **Bạn muốn build ý tưởng gì?** — Bạn đã có ý tưởng dự án cụ thể chưa, hay cần tôi giúp brainstorm?
> 2. **Mức độ hoàn thiện** — Bạn muốn làm đến đâu: chỉ smart contract (Milestone 1) hay full-stack với frontend (Milestone 2)?
> 3. **Môi trường phát triển** — Bạn muốn dùng Soroban Studio (online, không cần cài đặt) hay cài đặt thủ công trên máy?
> 4. **Kinh nghiệm Rust** — Bạn đã biết Rust chưa? Điều này ảnh hưởng đến cách tiếp cận viết smart contract.
