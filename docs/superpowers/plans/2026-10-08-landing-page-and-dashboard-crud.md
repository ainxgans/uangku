# Landing Page & Dashboard Quick Add Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Create a marketing landing page and add a Quick Add Transaction modal to the dashboard.

**Architecture:** SvelteKit frontend communicating with the Loco (Rust) backend. The landing page is public. The dashboard modal uses Svelte state to toggle visibility and fetches categories on mount.

**Tech Stack:** Svelte 5 (`$state`, `$effect`), TailwindCSS v4, fetch API.

## Global Constraints

- Use Svelte 5 syntax (`$state`, `onclick`, `onsubmit`).
- Design must follow the established Tailwind styling (rounded-2xl borders, no heavy shadows, Outfit font).
- API calls must include `credentials: 'include'`.

---

### Task 1: Landing Page (Marketing)

**Files:**
- Create/Modify: `frontend/src/routes/+page.svelte` (currently just redirecting)

**Interfaces:**
- Consumes: None (pure UI).
- Produces: A public landing page.

- [ ] **Step 1: Write the landing page component**

Replace the current redirect logic with the marketing UI. Check if user is logged in via an API call in `onMount` (or `$effect`) and redirect if they are.

```svelte
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
```

- [ ] **Step 2: Verify visually**
Since we don't write automated UI tests for the landing page in this MVP, start the dev server and verify the page looks correct and the redirect works if logged in.

Run: `npm run dev` in frontend.

- [ ] **Step 3: Commit**

```bash
git add frontend/src/routes/+page.svelte
git commit -m "feat: implement marketing landing page"
```

---

### Task 2: Dashboard Quick Add Modal

**Files:**
- Modify: `frontend/src/routes/dashboard/+page.svelte`

**Interfaces:**
- Consumes: `GET /api/categories`, `POST /api/transactions`.
- Produces: Updated dashboard summary.

- [ ] **Step 1: Implement the Modal & Logic**

Update `frontend/src/routes/dashboard/+page.svelte` to include the modal state, category fetching, transaction submission, and UI overlay.

```svelte
<script lang="ts">
    import { resolve } from '$app/paths';
    
    let s = $state({ income: 0, expense: 0, balance: 0 });
    let categories = $state([]);
    let isModalOpen = $state(false);
    
    // Form state
    let type = $state('expense');
    let category_id = $state('');
    let amount = $state('');
    let description = $state('');
    let date = $state(new Date().toISOString().split('T')[0]);
    let submitting = $state(false);
    let error = $state('');

    function loadSummary() {
        fetch(resolve('/api/dashboard/summary'), { credentials: 'include' })
            .then(r => {
                if (r.status === 401) {
                    window.location.href = resolve('/login');
                    return null;
                }
                return r.json();
            })
            .then(d => {
                if (d) s = d.summary ? d.summary : d;
            })
            .catch(() => {});
    }

    function loadCategories() {
        fetch(resolve('/api/categories'), { credentials: 'include' })
            .then(r => r.json())
            .then(d => {
                // Assuming API returns array or { items: [] }
                categories = d.items || d || [];
                if (categories.length > 0) {
                    const filtered = categories.filter(c => c.type === type);
                    if (filtered.length > 0) category_id = filtered[0].id;
                }
            })
            .catch(() => {});
    }

    $effect(() => {
        loadSummary();
        loadCategories();
    });

    $effect(() => {
        // Auto-select first category when type changes
        const filtered = categories.filter(c => c.type === type);
        if (filtered.length > 0 && !filtered.find(c => c.id === category_id)) {
            category_id = filtered[0].id;
        }
    });

    async function handleSubmit(e: Event) {
        e.preventDefault();
        submitting = true;
        error = '';

        try {
            const res = await fetch(resolve('/api/transactions'), {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                credentials: 'include',
                body: JSON.stringify({
                    category_id: parseInt(category_id), // Handle numeric vs uuid depending on backend
                    type,
                    amount: parseInt(amount),
                    description,
                    date
                })
            });

            if (!res.ok) throw new Error('Failed to create transaction');
            
            // Success
            isModalOpen = false;
            amount = '';
            description = '';
            loadSummary(); // Refresh dashboard
        } catch (err: any) {
            error = err.message || 'Error saving transaction';
        } finally {
            submitting = false;
        }
    }
</script>

<div class="mb-8 flex justify-between items-end">
    <div>
        <h1 class="text-3xl font-semibold tracking-tight text-slate-900">Dashboard</h1>
        <p class="text-slate-500 mt-1">This month's financial summary</p>
    </div>
    <button onclick={() => isModalOpen = true} class="bg-slate-900 text-white px-5 py-2.5 rounded-xl font-medium hover:bg-slate-800 transition-colors active:scale-[0.98]">
        + New Transaction
    </button>
</div>

<div class="grid grid-cols-1 md:grid-cols-3 gap-6">
    <div class="bg-white p-6 rounded-2xl shadow-sm border border-slate-100 hover:border-slate-200 transition-colors">
        <div class="text-sm font-medium text-slate-500 mb-2">Income</div>
        <div class="text-3xl font-semibold text-emerald-600 tracking-tight">Rp {s.income?.toLocaleString('id-ID') ?? 0}</div>
    </div>
    <div class="bg-white p-6 rounded-2xl shadow-sm border border-slate-100 hover:border-slate-200 transition-colors">
        <div class="text-sm font-medium text-slate-500 mb-2">Expense</div>
        <div class="text-3xl font-semibold text-rose-600 tracking-tight">Rp {s.expense?.toLocaleString('id-ID') ?? 0}</div>
    </div>
    <div class="bg-white p-6 rounded-2xl shadow-sm border border-slate-100 hover:border-slate-200 transition-colors">
        <div class="text-sm font-medium text-slate-500 mb-2">Balance</div>
        <div class="text-3xl font-semibold text-slate-900 tracking-tight">Rp {s.balance?.toLocaleString('id-ID') ?? 0}</div>
    </div>
</div>

{#if isModalOpen}
    <!-- Backdrop -->
    <div class="fixed inset-0 bg-slate-900/20 backdrop-blur-sm z-40 flex items-center justify-center p-4">
        <!-- Modal -->
        <div class="bg-white w-full max-w-md rounded-2xl shadow-xl border border-slate-100 p-6 relative">
            <button onclick={() => isModalOpen = false} class="absolute top-4 right-4 text-slate-400 hover:text-slate-600">
                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M18 6 6 18"/><path d="m6 6 12 12"/></svg>
            </button>
            
            <h2 class="text-xl font-semibold mb-6 text-slate-900 tracking-tight">New Transaction</h2>

            {#if error}
                <div class="mb-4 p-3 bg-red-50 text-red-600 text-sm rounded-lg border border-red-100">
                    {error}
                </div>
            {/if}

            <form onsubmit={handleSubmit} class="flex flex-col gap-4">
                <!-- Type Toggle -->
                <div class="flex bg-slate-100 p-1 rounded-xl">
                    <button type="button" class="flex-1 py-2 text-sm font-medium rounded-lg transition-colors {type === 'expense' ? 'bg-white text-rose-600 shadow-sm' : 'text-slate-500 hover:text-slate-700'}" onclick={() => type = 'expense'}>Expense</button>
                    <button type="button" class="flex-1 py-2 text-sm font-medium rounded-lg transition-colors {type === 'income' ? 'bg-white text-emerald-600 shadow-sm' : 'text-slate-500 hover:text-slate-700'}" onclick={() => type = 'income'}>Income</button>
                </div>

                <!-- Category -->
                <div>
                    <label class="block text-sm font-medium text-slate-700 mb-1.5">Category</label>
                    <select bind:value={category_id} required class="w-full px-4 py-3 rounded-xl border border-slate-200 focus:ring-2 focus:ring-slate-900 focus:border-transparent bg-slate-50 text-slate-900">
                        {#each categories.filter(c => c.type === type) as cat}
                            <option value={cat.id}>{cat.name}</option>
                        {/each}
                    </select>
                </div>

                <!-- Amount -->
                <div>
                    <label class="block text-sm font-medium text-slate-700 mb-1.5">Amount (Rp)</label>
                    <input type="number" bind:value={amount} required min="1" placeholder="50000" class="w-full px-4 py-3 rounded-xl border border-slate-200 focus:ring-2 focus:ring-slate-900 focus:border-transparent bg-slate-50 text-slate-900" />
                </div>

                <!-- Date -->
                <div>
                    <label class="block text-sm font-medium text-slate-700 mb-1.5">Date</label>
                    <input type="date" bind:value={date} required class="w-full px-4 py-3 rounded-xl border border-slate-200 focus:ring-2 focus:ring-slate-900 focus:border-transparent bg-slate-50 text-slate-900" />
                </div>

                <!-- Description -->
                <div>
                    <label class="block text-sm font-medium text-slate-700 mb-1.5">Description (Optional)</label>
                    <input type="text" bind:value={description} placeholder="Lunch at..." class="w-full px-4 py-3 rounded-xl border border-slate-200 focus:ring-2 focus:ring-slate-900 focus:border-transparent bg-slate-50 text-slate-900" />
                </div>

                <button type="submit" disabled={submitting} class="mt-4 w-full bg-slate-900 text-white py-3 rounded-xl font-medium hover:bg-slate-800 disabled:opacity-50 transition-colors active:scale-[0.98]">
                    {submitting ? 'Saving...' : 'Save Transaction'}
                </button>
            </form>
        </div>
    </div>
{/if}
```

- [ ] **Step 2: Commit**

```bash
git add frontend/src/routes/dashboard/+page.svelte
git commit -m "feat: add quick transaction modal to dashboard"
```
