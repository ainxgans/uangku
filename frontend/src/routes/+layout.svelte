<script>
    import '../app.css';
    import { page } from '$app/state';
    import { resolve } from '$app/paths';
    let { children } = $props();

    async function handleLogout() {
        try {
            await fetch(resolve('/api/auth/logout'), { method: 'POST', credentials: 'include' });
        } catch (e) {}
        window.location.href = resolve('/login');
    }

    let isPublic = $derived(page.url.pathname === '/' || page.url.pathname === '/login' || page.url.pathname === resolve('/') || page.url.pathname === resolve('/login'));
</script>

{#if isPublic}
    {@render children()}
{:else}
    <div class="flex h-screen bg-slate-50 text-slate-900 font-sans">
        <aside class="w-64 bg-white border-r border-slate-100 flex flex-col pt-8 pb-4">
            <h1 class="text-xl font-bold mb-8 px-6 tracking-tight text-slate-900">Uangku.</h1>
            <nav class="flex flex-col gap-1 px-4">
                <a href={resolve('/dashboard')} class="px-4 py-2.5 text-sm font-medium rounded-lg transition-colors {page.url.pathname === resolve('/dashboard') ? 'bg-slate-900 text-white' : 'text-slate-600 hover:bg-slate-100 hover:text-slate-900'}">Dashboard</a>
                <a href={resolve('/transactions')} class="px-4 py-2.5 text-sm font-medium rounded-lg transition-colors {page.url.pathname === resolve('/transactions') ? 'bg-slate-900 text-white' : 'text-slate-600 hover:bg-slate-100 hover:text-slate-900'}">Transactions</a>
                <a href={resolve('/budgets')} class="px-4 py-2.5 text-sm font-medium rounded-lg transition-colors {page.url.pathname === resolve('/budgets') ? 'bg-slate-900 text-white' : 'text-slate-600 hover:bg-slate-100 hover:text-slate-900'}">Budgets</a>
                <a href={resolve('/categories')} class="px-4 py-2.5 text-sm font-medium rounded-lg transition-colors {page.url.pathname === resolve('/categories') ? 'bg-slate-900 text-white' : 'text-slate-600 hover:bg-slate-100 hover:text-slate-900'}">Categories</a>
            </nav>
            <div class="mt-auto px-4">
                <button onclick={handleLogout} class="w-full text-left px-4 py-2.5 text-sm font-medium text-slate-500 hover:text-red-600 hover:bg-red-50 rounded-lg transition-colors">Logout</button>
            </div>
        </aside>
        <main class="flex-1 p-10 overflow-auto">
            <div class="max-w-5xl mx-auto">
                {@render children()}
            </div>
        </main>
    </div>
{/if}
