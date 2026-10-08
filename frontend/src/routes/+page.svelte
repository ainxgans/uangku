<script>
    import { resolve } from '$app/paths';
    import { onMount } from 'svelte';

    onMount(() => {
        window.location.replace(resolve('/dashboard'));
    });
</script>

<div class="p-8 text-center text-gray-500">
    <p>Redirecting to dashboard...</p>
</div>
