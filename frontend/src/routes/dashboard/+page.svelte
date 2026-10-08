<script>
    let s = $state({ income: 0, expense: 0, balance: 0 });
    $effect(() => {
        fetch('http://localhost:3000/api/dashboard/summary', { credentials: 'include' })
            .then(r => r.json()).then(d => s = d).catch(()=>{});
    });
</script>
<h1 class="text-2xl font-bold mb-6">Dashboard</h1>
<div class="grid grid-cols-3 gap-4">
    <div class="bg-white p-4 shadow rounded">
        <div class="text-sm text-gray-500">Income</div>
        <div class="text-xl font-bold text-green-600">${s.income}</div>
    </div>
    <div class="bg-white p-4 shadow rounded">
        <div class="text-sm text-gray-500">Expense</div>
        <div class="text-xl font-bold text-red-600">${s.expense}</div>
    </div>
    <div class="bg-white p-4 shadow rounded">
        <div class="text-sm text-gray-500">Balance</div>
        <div class="text-xl font-bold">${s.balance}</div>
    </div>
</div>
