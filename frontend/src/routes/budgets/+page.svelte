<script>
    let list = $state([]);
    $effect(() => {
        fetch('http://localhost:3000/api/budgets', { credentials: 'include' })
            .then(r=>r.json()).then(d=>list=d).catch(()=>{});
    });
</script>
<h1 class="text-2xl font-bold mb-6">Budgets</h1>
<div class="bg-white shadow rounded">
    {#each list as b}
    <div class="flex justify-between p-3 border-b last:border-0">
        <span>{b.category_name}</span>
        <span class="font-medium">${b.amount}</span>
    </div>
    {/each}
</div>
