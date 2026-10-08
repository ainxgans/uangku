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
<div class="mb-8">
    <h1 class="text-3xl font-semibold tracking-tight text-slate-900">Dashboard</h1>
    <p class="text-slate-500 mt-1">This month's financial summary</p>
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
