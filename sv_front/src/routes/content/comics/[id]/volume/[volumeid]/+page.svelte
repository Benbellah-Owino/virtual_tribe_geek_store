<script lang="ts">
	import { page } from '$app/stores';
	import { stringToSurrealId } from '$lib/helper_functions.ts/converters';
	import type { ChapterForCreate, VolumeForCreate } from '$lib/types/content';
	import { FormError, PageError } from '$lib/types/error';
	import { Result } from '$lib/types/result';
	import { suridToString, type SurrealId } from '$lib/types/server';
	import { updateFormState, type FormState } from '$lib/types/state/form_state';
	import { updatePageState, type PageState } from '$lib/types/state/page_state';
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

	let volumeId = $page.params.volumeid;
	let comicId = $page.params.id;
	let form_on = $state(false);
	// endsection:  --- State

	// section:     --- Variables
	let comic: any | null = $state(null);
	let chapters: any[] = $state([]);

	let chapter_form: ChapterForCreate = $state({
		title: '',
		pages: 0,
		synopsis: '',
		volume: {
			id: {
				String: ''
			},
			tb: ''
		}
	});

	// endsection:  --- Variables

	onMount(async () => {
		chapter_form.volume = stringToSurrealId(`volume:${volumeId}`);

		let res_chapters = await fetch(`http://localhost:7878/content/comic/volume/chapter/${volumeId}`, {
			method: 'GET',
			credentials: 'include',
			headers: {
				'Content-Type': 'application/json'
			}
		});

		if (res_chapters.ok == true) {
			//UNIMPLEMENTED
			console.log(res_chapters);
			let res = await res_chapters.json();
			chapters = res.chapters_list;
			chapters = chapters.sort((a,b) => a.relative_chapter - b.relative_chapter);
			console.log($state.snapshot(chapters));
			pageState.loading = false;
		} else if (res_chapters.ok == false) {
			console.error('failed');
			console.log(res_chapters);
			if (res_chapters.status == 401) {
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
			} else if (res_chapters.status == 404) {
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
		console.log($state.snapshot(chapter_form))
		try {
			let response = await fetch(`http://localhost:7878/content/comic/volume/chapter`, {
				method: 'POST',
				credentials: 'include',
				body: JSON.stringify(chapter_form),
				headers: {
					'Content-Type': 'application/json'
				}
			});
			if (response.status == 201) {
				//UNIMPLEMENTED
				setTimeout(() => {
					updateFormState(formState, Result.Ok, null, 'Chapter creation success', 'form', true);
					console.log($state.snapshot(formState));
				}, 3000);
				console.log('created');
				let res = await response.json();
				console.log(res)
				console.log("Uploading file")
				upload_file(res.chapter.id)
				//window.open(`/content/comics/${volumeId}`, '_self');
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
				console.log(response)
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

	async function pf(e:Event) {
		e.preventDefault()
	}

	async function upload_file(id: SurrealId) {

		const fileInput: any = document.getElementById('comic');
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

		let response = await fetch(`http://localhost:7878/content/comic/volume/chapter/file/upload/${suridToString(id)}`, {
			method: 'POST',
			credentials: 'include',
			body: formData
		});

		if (response.ok == true) {
			console.log(await response.json())
			setTimeout(()=>{
				updateFormState(formState, Result.Ok, null, 'Uploading success!', 'form', true);
			}, 3000)
			console.log('Uploading file')
			//open(`/studio/${$page.params.studio}/content`)
		} else if (response.ok == false) {
			console.log('Cover update failed');
			updateFormState(
				formState,
				Result.Err,
				FormError.UpdateFailed,
				'avatar',
				'Update Failed',
				false
			);
		}else{
			console.log('Cover update failed');
			console.log(response.ok);
			console.log(response.status);
		}
	}
</script>

<main>
    <center><h1 class="text-6xl font-extrabold">Chapters</h1></center>
	{#if pageState.loading}
		<center>Loading content...</center>
	{:else if pageState.loading == false && pageState.inner_state == Result.Ok && pageState.error == null}
		<h1 class="mb-7 mt-4 text-center text-3xl font-extrabold">Chapter list</h1>
		<ul class="content_list flex_col" id="content_list">
			{#each chapters as chapter}
				<li><a href="/content/comics/{comicId}/volume/{volumeId}/chapter/{chapter.id.id.String}">{chapter.relative_chapter}. {chapter.title}</a></li>
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
		<!-- TODO: Remove this shit -->
			<center class="w-full">
				FORM
			</center>
		{/if}
	{:else if pageState.loading == false && pageState.inner_state == Result.Ok && pageState.error == PageError.NotFoundError}
		<center class="w-full">
	<form
		enctype="multipart/form-data"
		class=" flex_col secondary_border mb-9 w-full rounded-md"
		onsubmit={pf}
	>
		<input type="file" name="comic" id="comic" /><br />

	</form>
			<form class="form alt_bg mt-3 rounded-lg p-3 md:w-96 lg:w-5/6" onsubmit={submit}>
				<h3 class="float-left mb-4 text-3xl font-extrabold">ADD CHAPTER</h3>
				<br />
				{#if formState.inner_state == Result.Ok && formState.target == 'form'}
					<center><p class="error main_txt text-lg font-semibold">{formState.message}</p></center>
				{:else if formState.inner_state == Result.Err && formState.target == 'form'}
					<center
						><p class="error text-lg font-semibold text-red-400">{formState.message}</p></center
					>
				{/if}
			<div class="form_div">
				<label for="title">Title</label>
				<input type="text" name="title" id="title" bind:value={chapter_form.title} />
				{#if formState.inner_state == Result.Err && formState.target == 'title'}
					<p class="error text-red-500">{formState.message}</p>
				{/if}
			</div>
			<div class="form_div">
				<label for="pages">Pages</label>
				<input type="number" name="pages" id="pages" bind:value={chapter_form.pages} />
				{#if formState.inner_state == Result.Err && formState.target == 'pages'}
					<p class="error text-red-500">{formState.message}</p>
				{/if}
			</div>
				<div class="form_div">
					<label for="description">Synopsis</label>
					<!--TODO: Add word limit to description field on server side -->
					<textarea
						name="description"
						id="description"
						class="w-11/12"
						rows="10"
						bind:value={chapter_form.synopsis}
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
