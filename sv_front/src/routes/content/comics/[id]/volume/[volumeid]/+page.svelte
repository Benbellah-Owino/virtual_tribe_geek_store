<script lang="ts">
	import { page } from '$app/stores';
	import ChapterForm from '$lib/components/content/forms/chapterForm.svelte';
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

		let res_chapters = await fetch(
			`http://localhost:7878/content/comic/volume/chapter/${volumeId}`,
			{
				method: 'GET',
				credentials: 'include',
				headers: {
					'Content-Type': 'application/json'
				}
			}
		);

		if (res_chapters.ok == true) {
			//UNIMPLEMENTED
			console.log(res_chapters);
			let res = await res_chapters.json();
			chapters = res.chapters_list;
			chapters = chapters.sort((a, b) => a.relative_chapter - b.relative_chapter);
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

</script>

<main class="page">
	<center><h1 class="text-6xl font-extrabold">Chapters</h1></center>
	{#if pageState.loading}
		<center>Loading content...</center>
	{:else if pageState.loading == false && pageState.inner_state == Result.Ok && pageState.error == null}
	
		<ul class="contentList  my-5 w-full h-fit p-2 flex flex-col justify-center items-center md:grid md:grid-cols-3" id="content_list">
			{#each chapters as chapter}
				<!-- svelte-ignore a11y_click_events_have_key_events -->
				<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
				<article
					class="chapter color2_bg_hover bg-black  w-72 m-2  cursor-pointer hover:scale-105 duration-300"
					onclick={()=>{
						window.open(`/content/comics/${comicId}/volume/${volumeId}/chapter/${chapter.id.id.String}`)
					}}
				>
					<img
						src={chapter?.cover
							? `http://localhost:7878/content/comic/volume/chapter/cover/${chapter.cover}`
							: ''}
						alt="Picture of {chapter?.title}"
						height="384px"
						class="chapterCover w-72 mx-auto object-contain"
					/>
					<div class="pl-2 w-full">
						<a
							class="tertiary_txt secondary_txt_hover font-bold underline"
							href="/content/comics/{comicId}/volume/{volumeId}/chapter/{chapter.id.id.String}"
							>{chapter.relative_chapter}. {chapter.title}</a
						>
					</div>
				</article>
			{/each}
		</ul>
		<br /><br />
		<center>
			<button
				class="btn primary_btn mt-5"
				onclick={() => {
					form_on = !form_on;
					console.log($state.snapshot(form_on));
				}}>Toggle Chapter form</button
			>
		</center>

		{#if form_on == true}
			<br />
			<center class="p-4">
				<ChapterForm {chapter_form} />
			</center>
		{/if}
	{:else if (pageState.loading == false && pageState.inner_state == Result.Ok && pageState.error == PageError.NotFoundError) || form_on == true}
		<center class="mt-3">
			<h2 class="mb-2 mt-8 text-3xl">Add new chapter to this volume below</h2>
			<ChapterForm {chapter_form} />
		</center>
	{:else}
		<h3>Not found</h3>
	{/if}
</main>

<style>
	.contentList{
		min-height: 30rem;
	}
	.chapter{
		height: 26rem;
	}

	.chapterCover{
		object-fit: cover;
		height:22rem;
		margin-bottom: 6px;
	}
</style>
