<script>
    import { resolve } from '$app/paths';
    let email = $state('');
    let otp = $state('');
    let step = $state(1);
    let loading = $state(false);
    let error = $state('');

    async function req(e) {
        e.preventDefault();
        loading = true;
        error = '';
        try {
            const res = await fetch(resolve('/api/auth/request-otp'), {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ email }),
                credentials: 'include'
            });
            if (!res.ok) {
                const data = await res.json().catch(() => ({}));
                error = data.message || data.error || 'Failed to send OTP';
            } else {
                step = 2;
            }
        } catch (err) {
            error = 'Network error: ' + err.message;
        } finally {
            loading = false;
        }
    }

    async function ver(e) {
        e.preventDefault();
        loading = true;
        error = '';
        try {
            const res = await fetch(resolve('/api/auth/verify-otp'), {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ email, code: otp }),
                credentials: 'include'
            });
            if (!res.ok) {
                const data = await res.json().catch(() => ({}));
                error = data.message || data.error || 'Invalid OTP';
            } else {
                window.location.href = resolve('/dashboard');
            }
        } catch (err) {
            error = 'Network error: ' + err.message;
        } finally {
            loading = false;
        }
    }
</script>
<div class="min-h-[80vh] flex flex-col justify-center max-w-sm mx-auto">
    <div class="bg-white p-8 rounded-2xl shadow-sm border border-slate-100 relative overflow-hidden">
        <h2 class="text-3xl font-semibold mb-2 tracking-tight text-slate-900">Sign In</h2>
        <p class="text-slate-500 text-sm mb-8">Uangku – Personal Finance Tracker</p>

        {#if error}
            <div class="mb-6 p-3 bg-red-50 text-red-600 text-sm rounded-lg font-medium border border-red-100">
                {error}
            </div>
        {/if}

        {#if step === 1}
        <form onsubmit={req} class="flex flex-col gap-4 relative z-10">
            <div>
                <label for="email" class="block text-sm font-medium text-slate-700 mb-1.5">Email address</label>
                <input id="email" type="email" bind:value={email} placeholder="you@example.com" required 
                    class="w-full px-4 py-3 rounded-xl border border-slate-200 focus:outline-none focus:ring-2 focus:ring-slate-900 focus:border-transparent transition-all bg-slate-50 text-slate-900" />
            </div>
            <button disabled={loading} class="mt-2 w-full bg-slate-900 text-white py-3 rounded-xl font-medium hover:bg-slate-800 disabled:opacity-50 transition-all active:scale-[0.98]">
                {loading ? 'Sending code...' : 'Continue'}
            </button>
        </form>
        {:else}
        <form onsubmit={ver} class="flex flex-col gap-4 relative z-10">
            <p class="text-sm text-slate-600">Code sent to <span class="font-medium text-slate-900">{email}</span></p>
            <div>
                <label for="otp" class="block text-sm font-medium text-slate-700 mb-1.5">6-digit code</label>
                <input id="otp" type="text" bind:value={otp} placeholder="000000" maxlength="6" required 
                    class="w-full px-4 py-3 text-center tracking-widest text-xl font-medium rounded-xl border border-slate-200 focus:outline-none focus:ring-2 focus:ring-slate-900 focus:border-transparent transition-all bg-slate-50 text-slate-900" />
            </div>
            <button disabled={loading} class="mt-2 w-full bg-slate-900 text-white py-3 rounded-xl font-medium hover:bg-slate-800 disabled:opacity-50 transition-all active:scale-[0.98]">
                {loading ? 'Verifying...' : 'Sign In'}
            </button>
            <button type="button" onclick={() => { step = 1; error = ''; }} class="mt-2 py-2 text-sm font-medium text-slate-500 hover:text-slate-900 transition-colors">
                Use a different email
            </button>
        </form>
        {/if}
    </div>
</div>
