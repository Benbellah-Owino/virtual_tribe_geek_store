<script lang="ts">
	import { page } from '$app/stores';
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

	let formState: FormState = $state({
		inner_state: Result.Ok,
		error: null,
		message: '',
		target: '',
		locked: false
	});

	let comic_id = $page.params.id;
	let form_on = $state(false);
	// endsection:  --- State

	// section:     --- Variables
	let comic: any | null = $state(null);
	let volumes: any[] = $state([]);

	let volume_form: VolumeForCreate = $state({
		synopsis: '',
		comic: {
			id: {
				String: ''
			},
			tb: ''
		},
		cover: null,
		no_of_chapters: 0
	});

	// endsection:  --- Variables

	onMount(async () => {
		volume_form.comic = stringToSurrealId(`comic:${comic_id}`);
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

	async function submit(e: Event) {
		e.preventDefault();

		try {
			console.log($state.snapshot(volume_form));
			// TODO" Fix this route
			let response = await fetch(`http://localhost:7878/content/comic/volume`, {
				method: 'POST',
				credentials: 'include',
				body: JSON.stringify(volume_form),
				headers: {
					'Content-Type': 'application/json'
				}
			});
			if (response.status == 201) {
				//UNIMPLEMENTED
				setTimeout(() => {
					updateFormState(formState, Result.Ok, null, 'Registration success', 'form', true);
					console.log($state.snapshot(formState));
				}, 3000);
				console.log('created');
				window.open(`/content/comics/${comic_id}`, '_self');
			} else if (response.status == 500) {
				updateFormState(
					formState,
					Result.Err,
					FormError.SubmissionFailed,
					'Submission Failed',
					'form',
					false
				);
			} else if (response.ok == false) {
				console.log(response.statusText);
				updateFormState(
					formState,
					Result.Err,
					FormError.SubmissionFailed,
					'Submission Failed',
					'form',
					false
				);
			}
		} catch (error) {
			console.error(error);
			updateFormState(
				formState,
				Result.Err,
				FormError.SubmissionFailed,
				'Submission Failed',
				'form',
				false
			);
		}
	}
</script>

<main>
	{#if pageState.loading}
		<center>Loading content...</center>
	{:else if pageState.loading == false && pageState.inner_state == Result.Ok && pageState.error == null}
		<h1 class="mb-7 mt-4 text-center text-3xl font-extrabold">Volumes list</h1>
		<ul class="content_list flex_col" id="content_list">
			{#each volumes as volume}
				<article class="volumes secondary_bg_hover primary_txt_hover secondary_border  m-2 p-1 rounded cursor-default">
					<li><a class="tertiary_txt font-bold underline " href="/content/comics/{comic_id}/volume/{volume.id.id.String}">volume {volume.vol_no}</a></li>
					<p class="text-sm">
						{volume.synopsis}
					</p>
				</article>
			{/each}
		</ul>
		<br><br>
		<center>
			<button
				class="btn primary_btn"
				onclick={() => {
					form_on = !form_on;
					console.log($state.snapshot(form_on));
				}}>Toggle Volume form</button
			>
		</center>
		{#if form_on == true}
			<center class="w-full">
				<form class="form alt_bg mt-3 rounded-lg p-3 md:w-96 lg:w-5/6" onsubmit={submit}>
					<h3 class="float-left mb-4 text-3xl font-extrabold">ADD VOLUME</h3>
					<br />
					{#if formState.inner_state == Result.Ok && formState.target == 'form'}
						<center><p class="error main_txt text-lg font-semibold">{formState.message}</p></center>
					{:else if formState.inner_state == Result.Err && formState.target == 'form'}
						<center
							><p class="error text-lg font-semibold text-red-400">{formState.message}</p></center
						>
					{/if}
					<div class="form_div">
						<label for="description">Synopsis</label>
						<!--TODO: Add word limit to description field on server side -->
						<textarea
							name="description"
							id="description"
							class="w-11/12"
							rows="10"
							bind:value={volume_form.synopsis}
						></textarea>
					</div>
					<button type="submit" class="btn primary_btn w-11/12">submit</button>
					<!-- TODO: Next time number the volumes -->
				</form>
			</center>
		{/if}
	{:else if pageState.loading == false && pageState.inner_state == Result.Ok && pageState.error == PageError.NotFoundError}
		<center class="w-full">
			<form class="form alt_bg mt-3 rounded-lg p-3 md:w-96 lg:w-5/6" onsubmit={submit}>
				<h3 class="float-left mb-4 text-3xl font-extrabold">ADD VOLUME</h3>
				<br />
				{#if formState.inner_state == Result.Ok && formState.target == 'form'}
					<center><p class="error main_txt text-lg font-semibold">{formState.message}</p></center>
				{:else if formState.inner_state == Result.Err && formState.target == 'form'}
					<center
						><p class="error text-lg font-semibold text-red-400">{formState.message}</p></center
					>
				{/if}
				<div class="form_div">
					<label for="description">Synopsis</label>
					<!--TODO: Add word limit to description field on server side -->
					<textarea
						name="description"
						id="description"
						class="w-11/12"
						rows="10"
						bind:value={volume_form.synopsis}
					></textarea>
				</div>
				<button type="submit" class="btn primary_btn w-11/12">submit</button>
				<!-- TODO: Next time number the volumes -->
			</form>
		</center>
	{:else}
		<h3>Not found</h3>
	{/if}
</main>

<style>
</style>
