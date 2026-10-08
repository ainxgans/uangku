<script>
    let email = $state('');
    let otp = $state('');
    let step = $state(1);

    async function req(e) {
        e.preventDefault();
        await fetch('http://localhost:3000/api/auth/request-otp', {
            method: 'POST', headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ email }), credentials: 'include'
        });
        step = 2;
    }

    async function ver(e) {
        e.preventDefault();
        await fetch('http://localhost:3000/api/auth/verify-otp', {
            method: 'POST', headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ email, otp }), credentials: 'include'
        });
        window.location.href = '/dashboard';
    }
</script>
<div class="max-w-sm mx-auto mt-32 bg-white p-6 rounded shadow">
    <h2 class="text-2xl font-bold mb-4">Login</h2>
    {#if step === 1}
    <form onsubmit={req} class="flex flex-col gap-3">
        <input type="email" bind:value={email} placeholder="Email" required class="border p-2 rounded" />
        <button class="bg-black text-white p-2 rounded">Get OTP</button>
    </form>
    {:else}
    <form onsubmit={ver} class="flex flex-col gap-3">
        <input type="text" bind:value={otp} placeholder="OTP Code" required class="border p-2 rounded" />
        <button class="bg-black text-white p-2 rounded">Verify</button>
    </form>
    {/if}
</div>
