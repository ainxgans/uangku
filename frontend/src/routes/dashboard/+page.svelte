<script>
    import { resolve } from '$app/paths';
    let s = $state({ income: 0, expense: 0, balance: 0 });
    $effect(() => {
        fetch(resolve('/api/dashboard/summary'), { credentials: 'include' })
            .then(r => {
                if (r.status === 401) {
                    window.location.href = resolve('/login');
                    return null;
                }
                return r.json();
            })
            .then(d => {
                if (d) {
                    s = d.summary ? d.summary : d;
                }
            })
            .catch(() => {});
    });
</script>
<h1 class="text-2xl font-bold mb-6">Dashboard</h1>
<div class="grid grid-cols-3 gap-4">
    <div class="bg-white p-4 shadow rounded">
        <div class="text-sm text-gray-500">Income</div>
        <div class="text-xl font-bold text-green-600">Rp {s.income?.toLocaleString('id-ID') ?? 0}</div>
    </div>
    <div class="bg-white p-4 shadow rounded">
        <div class="text-sm text-gray-500">Expense</div>
        <div class="text-xl font-bold text-red-600">Rp {s.expense?.toLocaleString('id-ID') ?? 0}</div>
    </div>
    <div class="bg-white p-4 shadow rounded">
        <div class="text-sm text-gray-500">Balance</div>
        <div class="text-xl font-bold">Rp {s.balance?.toLocaleString('id-ID') ?? 0}</div>
    </div>
</div>
