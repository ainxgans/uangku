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
                body: JSON.stringify({ email, otp }),
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
<div class="max-w-sm mx-auto mt-32 bg-white p-6 rounded shadow">
    <h2 class="text-2xl font-bold mb-4">Login Uangku</h2>
    {#if error}
        <div class="mb-4 p-2 bg-red-100 text-red-700 text-sm rounded">{error}</div>
    {/if}
    {#if step === 1}
    <form onsubmit={req} class="flex flex-col gap-3">
        <input type="email" bind:value={email} placeholder="Email" required class="border p-2 rounded" />
        <button disabled={loading} class="bg-black text-white p-2 rounded disabled:opacity-50">
            {loading ? 'Sending...' : 'Get OTP'}
        </button>
    </form>
    {:else}
    <form onsubmit={ver} class="flex flex-col gap-3">
        <p class="text-xs text-gray-500">OTP code sent for {email}</p>
        <input type="text" bind:value={otp} placeholder="OTP Code" required class="border p-2 rounded" />
        <button disabled={loading} class="bg-black text-white p-2 rounded disabled:opacity-50">
            {loading ? 'Verifying...' : 'Verify'}
        </button>
        <button type="button" onclick={() => { step = 1; error = ''; }} class="text-xs text-gray-600 underline text-center">Back</button>
    </form>
    {/if}
</div>
