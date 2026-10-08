# Landing Page & Dashboard Quick Add

## Landing Page (Marketing)

Akan menggantikan route `/` yang saat ini hanya redirect ke dashboard.
Jika user sudah login (punya JWT), akan tetap diredirect ke dashboard. Jika belum, tampilkan landing page.

- **Hero Section**: Headline besar "Track every penny, simply." dengan typography Outfit yang bold dan berkarakter. Tombol CTA "Start Tracking — Free".
- **Features Grid**: Tiga pilar utama:
  1. Quick Logging: "Input transactions in seconds, not minutes."
  2. Smart Budgets: "Set limits per category and get warned before you overspend."
  3. Clear Insights: "See exactly where your money goes with minimal, actionable summaries."
- **Style**: Sesuai dengan pedoman redesign: background off-white (`bg-slate-50`), teks abu gelap/hitam, desain bersih tanpa elemen berlebihan.

## Dashboard Quick Add Modal

Menambahkan fitur CRUD (Create) langsung dari halaman Dashboard untuk mempercepat alur kerja pengguna tanpa harus pindah ke halaman `/transactions`.

- **Trigger**: Tombol primer "+ New Transaction" diletakkan sejajar dengan judul halaman "Dashboard".
- **Modal Component**:
  - Backdrop blur atau gelap tipis.
  - Kartu modal di tengah dengan border tipis dan shadow halus.
- **Form Data**:
  - `type`: income / expense (radio buttons atau tabs).
  - `category_id`: select box (perlu `fetch` list kategori saat dashboard load).
  - `amount`: number input.
  - `description`: text input (opsional).
  - `date`: date picker (default ke hari ini).
- **Aksi Setelah Submit**:
  - Panggil `POST /api/transactions`.
  - Jika sukses, tutup modal, bersihkan form, dan muat ulang ringkasan dashboard (income/expense/balance).
  - Jika gagal, tampilkan pesan error inline.
