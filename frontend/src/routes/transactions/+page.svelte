<script>
    import { resolve } from '$app/paths';
    let list = $state([]);
    $effect(() => {
        fetch(resolve('/api/transactions'), { credentials: 'include' })
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
<h1 class="text-2xl font-bold mb-6">Transactions</h1>
<div class="bg-white shadow rounded">
    {#if list.length === 0}
        <div class="p-4 text-center text-gray-500">No transactions yet</div>
    {:else}
        {#each list as t}
        <div class="flex justify-between p-3 border-b last:border-0">
            <div>
                <div class="font-medium">{t.description || 'Transaction'}</div>
                <div class="text-xs text-gray-500">{t.date}</div>
            </div>
            <div class={t.amount > 0 ? 'text-green-600' : 'text-red-600'}>
                {t.amount > 0 ? '+' : '-'} Rp {Math.abs(t.amount).toLocaleString('id-ID')}
            </div>
        </div>
        {/each}
    {/if}
</div>
