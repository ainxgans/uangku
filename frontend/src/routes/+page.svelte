<script lang="ts">
    import { resolve } from '$app/paths';

    let checking = $state(true);

    $effect(() => {
        // Simple check if logged in
        fetch(resolve('/api/auth/me'), { credentials: 'include' })
            .then(res => {
                if (res.ok) {
                    window.location.href = resolve('/dashboard');
                } else {
                    checking = false;
                }
            })
            .catch(() => {
                checking = false;
            });
    });
</script>

{#if checking}
    <div class="min-h-screen flex items-center justify-center bg-slate-50">
        <p class="text-slate-500 font-medium">Memuat...</p>
    </div>
{:else}
    <div class="min-h-screen bg-slate-50 text-slate-900 font-sans selection:bg-slate-900 selection:text-white">
        <!-- Nav -->
        <header class="max-w-6xl mx-auto px-6 py-6 flex justify-between items-center sticky top-0 bg-slate-50/80 backdrop-blur-md z-50">
            <div class="text-2xl font-bold tracking-tight">Uangku.</div>
            <a href="/login" class="px-6 py-2.5 bg-slate-900 text-white text-sm font-medium rounded-2xl hover:bg-slate-800 transition-colors active:scale-[0.98]">Masuk / Daftar</a>
        </header>

        <!-- Hero -->
        <main>
            <section class="max-w-6xl mx-auto px-6 pt-24 pb-20 md:pt-32 md:pb-32">
                <div class="max-w-4xl">
                    <h1 class="text-5xl md:text-7xl font-bold tracking-tight mb-8 leading-[1.1]">Catat setiap sen. <br /><span class="text-slate-400">Tanpa ribet.</span></h1>
                    <p class="text-xl md:text-2xl text-slate-500 mb-12 leading-relaxed max-w-2xl font-medium">Berhenti menebak-nebak ke mana uang Anda pergi. Uangku adalah cara paling minimalis, cepat, dan aman untuk mengambil kendali atas keuangan pribadi Anda.</p>
                    <div class="flex flex-col sm:flex-row gap-4">
                        <a href="/login" class="inline-flex justify-center items-center px-8 py-4 bg-slate-900 text-white font-medium rounded-2xl hover:bg-slate-800 transition-colors active:scale-[0.98] text-lg">Mulai Mencatat — Gratis</a>
                        <a href="#fitur" class="inline-flex justify-center items-center px-8 py-4 bg-white text-slate-900 font-medium rounded-2xl border border-slate-200 hover:border-slate-300 hover:bg-slate-50 transition-colors active:scale-[0.98] text-lg">Pelajari Fitur</a>
                    </div>
                </div>
            </section>

            <!-- Value Props -->
            <section id="fitur" class="border-t border-slate-200 bg-white">
                <div class="max-w-6xl mx-auto px-6 py-24 md:py-32">
                    <div class="mb-16 max-w-2xl">
                        <h2 class="text-3xl md:text-4xl font-bold tracking-tight mb-4">Dirancang untuk kecepatan.</h2>
                        <p class="text-lg text-slate-500">Mencatat keuangan seharusnya tidak menjadi beban. Uangku membuang semua grafik rumit yang tidak Anda butuhkan, menyisakan apa yang benar-benar penting.</p>
                    </div>

                    <div class="grid grid-cols-1 md:grid-cols-3 gap-12">
                        <div class="flex flex-col">
                            <div class="h-12 w-12 bg-slate-100 rounded-2xl flex items-center justify-center mb-6 text-xl">⚡</div>
                            <h3 class="text-xl font-semibold mb-3 tracking-tight">Input Kilat</h3>
                            <p class="text-slate-500 leading-relaxed">Tambahkan pemasukan atau pengeluaran dalam hitungan detik. Tersedia fitur Quick Add yang memotong langkah berlebihan sehingga kebiasaan mencatat Anda tetap terjaga.</p>
                        </div>
                        <div class="flex flex-col">
                            <div class="h-12 w-12 bg-slate-100 rounded-2xl flex items-center justify-center mb-6 text-xl">🎯</div>
                            <h3 class="text-xl font-semibold mb-3 tracking-tight">Batas Anggaran</h3>
                            <p class="text-slate-500 leading-relaxed">Atur limit bulanan untuk makanan, transportasi, hingga hiburan. Dapatkan peringatan sebelum Anda melewati batas budget yang sudah ditentukan.</p>
                        </div>
                        <div class="flex flex-col">
                            <div class="h-12 w-12 bg-slate-100 rounded-2xl flex items-center justify-center mb-6 text-xl">📊</div>
                            <h3 class="text-xl font-semibold mb-3 tracking-tight">Laporan Bersih</h3>
                            <p class="text-slate-500 leading-relaxed">Lihat persis ke mana uang Anda mengalir. Dasbor kami hanya menampilkan rangkuman paling krusial—sisa saldo, tren harian, dan kategori terbesar.</p>
                        </div>
                    </div>
                </div>
            </section>

            <!-- How it works / Steps -->
            <section class="border-t border-slate-200 bg-slate-50">
                <div class="max-w-6xl mx-auto px-6 py-24 md:py-32">
                    <h2 class="text-3xl md:text-4xl font-bold tracking-tight mb-16 text-center">Tiga Langkah Sederhana</h2>
                    
                    <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
                        <div class="bg-white p-8 rounded-3xl border border-slate-100 shadow-sm relative overflow-hidden">
                            <div class="text-8xl font-bold text-slate-50 absolute -top-4 -right-4 -z-0 select-none">1</div>
                            <div class="relative z-10">
                                <h3 class="text-xl font-semibold mb-3">Daftar Tanpa Password</h3>
                                <p class="text-slate-500">Cukup masukkan email Anda. Kami menggunakan sistem OTP (One-Time Password) sehingga Anda tidak perlu mengingat kata sandi baru.</p>
                            </div>
                        </div>
                        <div class="bg-white p-8 rounded-3xl border border-slate-100 shadow-sm relative overflow-hidden mt-0 md:mt-8">
                            <div class="text-8xl font-bold text-slate-50 absolute -top-4 -right-4 -z-0 select-none">2</div>
                            <div class="relative z-10">
                                <h3 class="text-xl font-semibold mb-3">Atur Kategori</h3>
                                <p class="text-slate-500">Sesuaikan pos pengeluaran dengan gaya hidup Anda. Kami sudah menyiapkan kategori standar, tapi Anda punya kendali penuh untuk menambahkannya.</p>
                            </div>
                        </div>
                        <div class="bg-white p-8 rounded-3xl border border-slate-100 shadow-sm relative overflow-hidden mt-0 md:mt-16">
                            <div class="text-8xl font-bold text-slate-50 absolute -top-4 -right-4 -z-0 select-none">3</div>
                            <div class="relative z-10">
                                <h3 class="text-xl font-semibold mb-3">Raih Ketenangan</h3>
                                <p class="text-slate-500">Tidak ada lagi kejutan di akhir bulan. Anda akan selalu tahu sisa dana Anda, tagihan apa yang belum dibayar, dan seberapa hemat Anda bulan ini.</p>
                            </div>
                        </div>
                    </div>
                </div>
            </section>

            <!-- Bottom CTA -->
            <section class="border-t border-slate-200 bg-slate-900 text-white">
                <div class="max-w-4xl mx-auto px-6 py-24 md:py-32 text-center">
                    <h2 class="text-4xl md:text-5xl font-bold tracking-tight mb-6">Siap membenahi keuanganmu?</h2>
                    <p class="text-xl text-slate-400 mb-10 max-w-2xl mx-auto">Bergabunglah dan mulai bangun kebiasaan finansial yang lebih baik hari ini juga.</p>
                    <a href="/login" class="inline-block px-10 py-5 bg-white text-slate-900 font-bold rounded-2xl hover:bg-slate-100 transition-colors active:scale-[0.98] text-lg">Buat Akun Gratis</a>
                </div>
            </section>
        </main>

        <!-- Footer -->
        <footer class="bg-slate-900 text-slate-500 border-t border-slate-800">
            <div class="max-w-6xl mx-auto px-6 py-12 flex flex-col md:flex-row justify-between items-center gap-4">
                <div class="text-xl font-bold text-slate-300 tracking-tight">Uangku.</div>
                <div class="text-sm">© 2026 Uangku. All rights reserved.</div>
            </div>
        </footer>
    </div>
{/if}
