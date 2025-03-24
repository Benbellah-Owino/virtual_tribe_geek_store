<script lang="ts">
	import { page } from '$app/stores';
	import Banner from '$lib/components/studio/std/comps/Banner.svelte';
	import { PageError } from '$lib/types/error';
	import { Result } from '$lib/types/result';
	import { updatePageState, type PageState } from '$lib/types/state/page_state';
	import type { Studio } from '$lib/types/studio';
	import { onMount } from 'svelte';

	let studio_id = $page.params.id;

	let studio: Studio = $state({
		id: { tb: '', id: { String: '' } },
		name: '',
		owner: { tb: '', id: { String: '' } },
		email: '',
		description: ''
	});

	let pageState: PageState = $state({
		inner_state: Result.Ok,
		error: null,
		loading: true,
		message: ''
	});
	onMount(async () => {
		let response = await fetch(`http://localhost:7878/studio/${studio_id}`, {
			method: 'GET',
			credentials: 'include',
			headers: {
				'Content-Type': 'application/json'
			}
		});

		if (response.ok == true) {
			//UNIMPLEMENTED
			let res = await response.json();
			let db_studio = res.studio;
			console.log(db_studio);
			studio = db_studio;
			pageState.loading = false;
		} else if (response.ok == false) {
			if (response.status == 401) {
				updatePageState(
					pageState,
					Result.Err,
					PageError.Unauthorized,
					false,
					'You are not authorized! Redirecting you to login page...'
				);
				setTimeout(() => {
					window.open('studio/login', '_self');
				}, 5000);
			}
		}
	});
</script>

<main class="page">
	{#if pageState.loading}
		<center>Loading studio...</center>
	{:else if pageState.loading == false && pageState.inner_state == Result.Ok}
		<center> studio</center>
		<center><Banner text={studio.name} /></center>
		<ul class="secondary_border mt-8 flex h-fit w-full flex-col items-start justify-center p-4">
			<li class="tertiary_txt"><b class="main_txt">Name: &nbsp </b>{studio.name}</li>
			<li class="tertiary_txt"><b class="main_txt">Email: &nbsp</b>{studio.email}</li>
			<li class="tertiary_txt w-10/12 border border-gray-500">
				<h3 class="main_txt underline">Description</h3>
				<p class="text-sm">{studio.description}</p>
			</li>
		</ul>
		<br>
        <a href="/studio/{studio_id}/content" class="tertiary_txt_hover mt-3 p-3 ml-1 text-lg font-bold">Click here to view this studios content</a>
	{:else if pageState.inner_state == Result.Err}
		<h3 class="error">{pageState.message}</h3>
	{/if}
</main>
