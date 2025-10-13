<script lang="ts">
	import { page } from '$app/stores';
	import VolumeForm from '$lib/components/content/forms/volumeForm.svelte';
	import { stringToSurrealId } from '$lib/helper_functions.ts/converters';
	import type { VolumeForCreate } from '$lib/types/content';
	import { FormError, PageError } from '$lib/types/error';
	import { Result } from '$lib/types/result';
	import { updateFormState, type FormState } from '$lib/types/state/form_state';
	import { updatePageState, type PageState } from '$lib/types/state/page_state';
	import { error } from '@sveltejs/kit';
	import { onMount } from 'svelte';

	// section:     --- State
	let pageState: PageState = $state({
		inner_state: Result.Ok,
		error: null,
		loading: true,
		message: ''
	});

	let comic_id = $page.params.id;
	// endsection:  --- State

	// section:     --- Variables
	let comic: any | null = $state(null);
	let volumes: any[] = $state([]);

	// endsection:  --- Variables

	onMount(async () => {
		console.log(comic_id)
		let response = await fetch(`http://localhost:7878/content/comic/index?comic=${comic_id}`, {
			method: 'GET',
			credentials: 'include',
			headers: {
				'Content-Type': 'application/json'
			}
		});

		if (response.ok == true) {
			//UNIMPLEMENTED
			let res = await response.json();
			// console.log($state.snapshot(res));
			comic = res.comic;
			console.log($state.snapshot(comic));
			pageState.loading = false;
		} else if (response.ok == false) {
			console.error('failed');
			if (response.status == 401) {
				updatePageState(
					pageState,
					Result.Err,
					PageError.Unauthorized,
					false,
					'You are not authorized! Redirecting you to login page...'
				);
				setTimeout(() => {
					window.open('content/login', '_self');
				}, 5000);
			} else if (response.status == 404) {
				updatePageState(
					pageState,
					Result.Ok,
					PageError.NotFoundError,
					false,
					"The requested comic doesn't exist"
				);
				console.log($state.snapshot(pageState));
			}
		}

		let res_volumes = await fetch(`http://localhost:7878/content/comic/volume/${comic_id}`, {
			method: 'GET',
			credentials: 'include',
			headers: {
				'Content-Type': 'application/json'
			}
		});

		if (res_volumes.ok == true) {
			//UNIMPLEMENTED
			console.log(res_volumes);
			let res = await res_volumes.json();
			volumes = res.volume_list;
			volumes = volumes.sort((a,b) => a.vol_no - b.vol_no);
			console.log($state.snapshot(volumes));
			pageState.loading = false;
		} else if (res_volumes.ok == false) {
			console.error('failed');
			console.log(res_volumes);
			if (res_volumes.status == 401) {
				updatePageState(
					pageState,
					Result.Err,
					PageError.Unauthorized,
					false,
					'You are not authorized! Redirecting you to login page...'
				);
				setTimeout(() => {
					window.open('content/login', '_self');
				}, 5000);
			} else if (res_volumes.status == 404) {
				console.error('not found');
				updatePageState(
					pageState,
					Result.Ok,
					PageError.NotFoundError,
					false,
					"The requested volumrd doesn't exist"
				);
				console.log($state.snapshot(pageState));
			}
		}
	});

	
</script>

<main>
	{#if pageState.loading}
		<center>Loading content...</center>
	{:else if pageState.loading == false && pageState.inner_state == Result.Ok && pageState.error == null}
		<h1 class="mb-7 mt-4 text-center text-3xl font-extrabold">Volumes list</h1>
		<ul class="content_list flex_col" id="content_list">
			{#each volumes as volume}
				<article class="volumes secondary_bg_hover primary_txt_hover secondary_border  m-2 p-1 rounded cursor-default">
					<li><a class="tertiary_txt font-bold underline " href="/consume/comics/{comic_id}/volume/{volume.id.id.String}">volume {volume.vol_no}</a></li>
					<p class="text-sm">
						{volume.synopsis}
					</p>
				</article>
			{/each}
		</ul>
		<br><br>
	
	{:else if pageState.loading == false && pageState.inner_state == Result.Ok && pageState.error == PageError.NotFoundError}
		<center class="w-full">
			<h2 class="font-bold">Comic Not Found</h2>
		</center>
	{:else}
		<h3>Not found</h3>
	{/if}
</main>

<style>
</style>
