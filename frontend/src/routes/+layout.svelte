<script>
    import '../app.css';
    import { page } from '$app/stores';
    let { children } = $props();
</script>
<div class="flex h-screen bg-gray-50 text-gray-900">
    {#if $page.url.pathname !== '/login'}
    <aside class="w-64 bg-white border-r p-4 flex flex-col gap-2">
        <h1 class="text-xl font-bold mb-6">Uangku</h1>
        <a href="/dashboard" class="p-2 hover:bg-gray-100 rounded">Dashboard</a>
        <a href="/transactions" class="p-2 hover:bg-gray-100 rounded">Transactions</a>
        <a href="/budgets" class="p-2 hover:bg-gray-100 rounded">Budgets</a>
        <a href="/categories" class="p-2 hover:bg-gray-100 rounded">Categories</a>
        <a href="/settings" class="p-2 hover:bg-gray-100 rounded">Settings</a>
        <button onclick={() => window.location.href='/login'} class="p-2 hover:bg-red-50 text-red-600 rounded mt-auto text-left">Logout</button>
    </aside>
    {/if}
    <main class="flex-1 p-8 overflow-auto">
        {@render children()}
    </main>
</div>
