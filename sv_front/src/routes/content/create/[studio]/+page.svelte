<script lang="ts">
	import { clearState, updateFormState, type FormState } from '$lib/types/state/form_state';
	import type { ContentForCreate, Genre } from '$lib/types/content';
	import { FormError } from '$lib/types/error';
	import { Result } from '$lib/types/result';
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { suridToString, type SurrealId } from '$lib/types/server';


	let genres: Genre[] = $state([]);

	onMount(async () => {
		let response = await fetch(`http://localhost:7878/content/genre`, {
			method: 'GET',
			credentials: 'include',
			headers: {
				'Content-Type': 'application/json'
			}
		});

		if (response.ok == true) {
			let res = await response.json();
			genres = res.genres;
		} else if (response.ok == false) {
		}
	});
	let formState: FormState = $state({
		inner_state: Result.Ok,
		error: null,
		message: '',
		target: '',
		locked: false
	});

	let content_form: ContentForCreate = $state({
		title: '',
		studio: $page.params.studio,
		description: '',
		audiences: '',
		recom_price: 0,
		genre: []
	});

	let audiences = $state(['G', 'PG', 'PG-13', 'R', 'AO']);


	async function submit(e: Event) {
		e.preventDefault();
		if(typeof(content_form.recom_price) == "string"){
			content_form.recom_price = parseFloat(content_form.recom_price)
		}
		for (const key in content_form) {
			if (content_form[key] == '') {
				updateFormState(
					formState,
					Result.Err,
					FormError.PasswordsDontMatch,
					'Missing Field',
					key,
					false
				);
				console.log(formState.target);
				return;
			}
		}

		console.log($state.snapshot(content_form));


		try {
			console.log('submitting...');
			let response = await fetch(`http://localhost:7878/content`, {
				method: 'POST',
				credentials: 'include',
				body: JSON.stringify(content_form),
				headers: {
					'Content-Type': 'application/json'
				}
			});
			if (response.status == 201) {
				console.log('OK');
				//UNIMPLEMENTED
				updateFormState(formState, Result.Ok, null, 'Created, Uploading avatar....', 'form', true);
				console.log($state.snapshot(formState));
				let res = await response.json()
				console.log(res)
				upload_cover(res.content.id);
			} else if (response.status == 500) {
				console.log('SERVER ERROR');
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
			console.log('CLIENT ERROR');
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
		console.log('hey');
	}


	async function upload_cover(id: SurrealId) {
		console.log(id)
		const fileInput: any = document.getElementById('cover');
		if (fileInput == null) return;

		const file = fileInput.files[0];
		console.log(file);
		if (!file) {
			alert('Please select a file to upload.');
			return;
		}

		let formData = new FormData();
		formData.append('file', file);
		console.log(formData.values);

		let response = await fetch(`http://localhost:7878/content/cover/upload/${suridToString(id)}`, {
			method: 'POST',
			credentials: 'include',
			body: formData
		});

		if (response.ok == true) {
			console.log(await response.json())
			setTimeout(()=>{
				updateFormState(formState, Result.Ok, null, 'Uploading success!', 'form', true);
			}, 3000)
			open(`/studio/${$page.params.studio}/content`)
		} else if (response.ok == false) {
			updateFormState(
				formState,
				Result.Err,
				FormError.UpdateFailed,
				'avatar',
				'Update Failed',
				false
			);
		}
	}

	function pf(e: Event){
		e.preventDefault()
	}
</script>

<main class="page main_bg flex_col mt-5 h-full w-full">
	<h1 class="mb-7 text-center text-3xl font-extrabold">CONTENT CREATION PAGE</h1>
	<center>
	<form
		enctype="multipart/form-data"
		class=" flex_col secondary_border mb-9 w-full rounded-md"
		onsubmit={pf}
	>
		<input type="file" name="cover" id="cover" /><br />

	</form>

		<form class="form alt_bg rounded-lg p-3 md:w-96" onsubmit={submit}>
			<h3 class="float-left mb-4 text-3xl font-extrabold">CREATE CONTENT</h3>
			<br />
			{#if formState.inner_state == Result.Ok && formState.target == 'form'}
				<center><p class="error main_txt text-lg font-semibold">{formState.message}</p></center>
			{:else if formState.inner_state == Result.Err && formState.target == 'form'}
				<center><p class="error text-lg font-semibold text-red-400">{formState.message}</p></center>
			{/if}

			<div class="form_div">
				<label for="title">title</label>
				<input type="text" name="title" id="title" bind:value={content_form.title} />
				{#if formState.inner_state == Result.Err && formState.target == 'title'}
					<p class="error text-red-500">{formState.message}</p>
				{/if}
			</div>

			<div class="form_div">
				<label for="audiences">audiences</label>
				<select name="audiences" id="audiences" bind:value={content_form.audiences}>
					{#each audiences as audience}
						<option class="m-0" value={audience}>{audience}</option>
					{/each}
				</select>
				{#if formState.inner_state == Result.Err && formState.target == 'audiences'}
					<p class="error text-red-500">{formState.message}</p>
				{/if}
			</div>

			<div class="form_div">
				<!-- TODO: ITs supposed to be an array -->
				<label for="genres">genre</label>
					{#if genres.length == 0}
					No genre available
				{:else}
					{#each genres as genre}
						<div class="check_div flex_row">
							<p>{genre.name}</p> 
							<input
								type="checkbox"
								class="m-0"
								name="genres"
								bind:group={content_form.genre}
								value={`${genre.id.tb}:${genre.id.id.String}`}
							/>
						</div>
					{/each}
				{/if}
				{#if formState.inner_state == Result.Err && formState.target == 'genre'}
					<p class="error text-red-500">{formState.message}</p>
				{/if}
			</div>

			<div class="form_div">
				<label for="recom_price">recom_price</label>
				<input
					type="text"
					name="recom_price"
					id="recom_price"
					bind:value={content_form.recom_price}
				/>
				{#if formState.inner_state == Result.Err && formState.target == 'recom_price'}
					<p class="error text-red-500">{formState.message}</p>
				{/if}
			</div>

			<div class="form_div">
				<label for="description">Description</label>
				<!--TODO: Add word limit to description field on server side -->
				<textarea
					name="description"
					id="description"
					class="w-11/12"
					rows="10"
					bind:value={content_form.description}
				></textarea>
				{#if formState.inner_state == Result.Err && formState.target == 'description'}
					<p class="error text-red-500">{formState.message}</p>
				{/if}
			</div>

			<button type="submit" class="btn primary_btn w-11/12">submit</button>
			<br /><br />
		</form>
	</center>
</main>

<style>
	select {
		background-color: var(--primary);
		color: var(--text_color);
		border: var(--text_color) 1px solid;
		border-radius: 0.25rem;
		cursor: pointer;
		height: 36px;
		align-items: center;
		font-size: 16px;
		min-width: 200px;
		width: 98%;
		padding: 0px, 16px;
	}
</style>
