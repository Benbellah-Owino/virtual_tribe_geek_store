<script lang="ts">
	import HorNav from '$lib/components/navbars/HorNav.svelte';
	import '../app.css';
	let { children } = $props();
</script>

<HorNav />
{@render children()}
