<script>
    import { resolve } from '$app/paths';
    let list = $state([]);
    let name = $state('');
    let type = $state('expense');

    const load = () =>
        fetch(resolve('/api/categories'), { credentials: 'include' })
            .then(r => {
                if (r.status === 401) {
                    window.location.href = resolve('/login');
                    return null;
                }
                return r.json();
            })
            .then(d => { if (d) list = d; })
            .catch(() => {});

    $effect(() => { load(); });

    async function add(e) {
        e.preventDefault();
        await fetch(resolve('/api/categories'), {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ name, type }),
            credentials: 'include'
        });
        name = '';
        load();
    }
</script>
<h1 class="text-2xl font-bold mb-6">Categories</h1>
<form onsubmit={add} class="flex gap-2 mb-6">
    <input bind:value={name} placeholder="Name" required class="border p-2 rounded flex-1" />
    <select bind:value={type} class="border p-2 rounded">
        <option value="expense">Expense</option>
        <option value="income">Income</option>
    </select>
    <button class="bg-black text-white px-4 rounded">Add</button>
</form>
<div class="bg-white shadow rounded">
    {#if list.length === 0}
        <div class="p-4 text-center text-gray-500">No categories found</div>
    {:else}
        {#each list as c}
        <div class="flex justify-between p-3 border-b last:border-0">
            <span>{c.name}</span>
            <span class={c.type === 'income' ? 'text-green-600' : 'text-red-600'}>{c.type}</span>
        </div>
        {/each}
    {/if}
</div>
