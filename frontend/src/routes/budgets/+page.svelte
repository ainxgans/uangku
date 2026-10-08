<script>
    import { resolve } from '$app/paths';
    let list = $state([]);
    $effect(() => {
        fetch(resolve('/api/budgets'), { credentials: 'include' })
            .then(r => {
                if (r.status === 401) {
                    window.location.href = resolve('/login');
                    return null;
                }
                return r.json();
            })
            .then(d => { if (d) list = d; })
            .catch(() => {});
    });
</script>
<h1 class="text-2xl font-bold mb-6">Budgets</h1>
<div class="bg-white shadow rounded">
    {#if list.length === 0}
        <div class="p-4 text-center text-gray-500">No budgets set yet</div>
    {:else}
        {#each list as b}
        <div class="flex justify-between p-3 border-b last:border-0">
            <span>{b.category_name || 'Category'}</span>
            <span class="font-medium">Rp {b.amount?.toLocaleString('id-ID') ?? 0}</span>
        </div>
        {/each}
    {/if}
</div>
