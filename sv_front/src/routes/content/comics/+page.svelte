<script lang="ts">
	import { Result } from '$lib/types/result';
	import type { PageState } from '$lib/types/state/page_state';
	import { onMount } from 'svelte';

	let comics = $state([]);
	// section:     --- State
	let pageState: PageState = $state({
		inner_state: Result.Ok,
		error: null,
		loading: true,
		message: ''
	});

	let form_on = $state(false);
	// endsection:  --- State

	onMount(async () => {
		try {
			let response = await fetch(`http://localhost:7878/content/comic`, {
				method: 'GET',
				credentials: 'include',
				headers: {
					'Content-Type': 'application/json'
				}
			});

            if (response.ok == true) {
			//UNIMPLEMENTED
			let res = await response.json();
			console.log($state.snapshot(res));
			

			pageState.loading = false;
		} else if (response.ok == false) {
            console.error('not found')
        }
	 
		} catch (error) {}
	});
</script>

<main>
	{#if pageState.loading}
		<center>Loading content...</center>
	{:else if pageState.loading == false && pageState.inner_state == Result.Ok && pageState.error == null}{/if}
</main>

<style>
</style>
