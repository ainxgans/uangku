<script lang="ts">
    import { resolve } from '$app/paths';
    import { goto } from '$app/navigation';

    let checking = $state(true);

    $effect(() => {
        // Simple check if logged in
        fetch(resolve('/api/auth/me'), { credentials: 'include' })
            .then(res => {
                if (res.ok) {
                    goto('/dashboard');
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
        <p class="text-slate-500">Loading...</p>
    </div>
{:else}
    <div class="min-h-screen bg-slate-50 text-slate-900 font-sans">
        <header class="max-w-6xl mx-auto px-6 py-6 flex justify-between items-center">
            <div class="text-xl font-bold tracking-tight">Uangku.</div>
            <a href="/login" class="px-5 py-2.5 bg-slate-900 text-white text-sm font-medium rounded-xl hover:bg-slate-800 transition-colors active:scale-[0.98]">Sign In</a>
        </header>

        <main class="max-w-6xl mx-auto px-6 py-24 md:py-32">
            <div class="max-w-3xl">
                <h1 class="text-5xl md:text-7xl font-bold tracking-tight mb-6 leading-tight">Track every penny, <span class="text-slate-400">simply.</span></h1>
                <p class="text-xl text-slate-500 mb-10 leading-relaxed max-w-2xl">Stop guessing where your money goes. Uangku is the minimal, fast, and secure way to take control of your personal finances.</p>
                <a href="/login" class="inline-block px-8 py-4 bg-slate-900 text-white font-medium rounded-xl hover:bg-slate-800 transition-colors active:scale-[0.98] text-lg">Start Tracking — Free</a>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-3 gap-8 mt-32 border-t border-slate-200 pt-16">
                <div>
                    <h3 class="text-xl font-semibold mb-3 tracking-tight">Quick Logging</h3>
                    <p class="text-slate-500">Input transactions in seconds, not minutes. Built for speed so tracking doesn't become a chore.</p>
                </div>
                <div>
                    <h3 class="text-xl font-semibold mb-3 tracking-tight">Smart Budgets</h3>
                    <p class="text-slate-500">Set limits per category and get warned before you overspend. Stay on top of your financial goals.</p>
                </div>
                <div>
                    <h3 class="text-xl font-semibold mb-3 tracking-tight">Clear Insights</h3>
                    <p class="text-slate-500">See exactly where your money goes with minimal, actionable summaries that make sense at a glance.</p>
                </div>
            </div>
        </main>
    </div>
{/if}