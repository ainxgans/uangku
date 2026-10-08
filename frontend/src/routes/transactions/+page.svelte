<script>
    let list = $state([]);
    $effect(() => {
        fetch('http://localhost:3000/api/transactions', { credentials: 'include' })
            .then(r=>r.json()).then(d=>list=d).catch(()=>{});
    });
</script>
<h1 class="text-2xl font-bold mb-6">Transactions</h1>
<div class="bg-white shadow rounded">
    {#each list as t}
    <div class="flex justify-between p-3 border-b last:border-0">
        <div>
            <div class="font-medium">{t.description}</div>
            <div class="text-xs text-gray-500">{t.date}</div>
        </div>
        <div class={t.amount > 0 ? 'text-green-600' : 'text-red-600'}>${Math.abs(t.amount)}</div>
    </div>
    {/each}
</div>
