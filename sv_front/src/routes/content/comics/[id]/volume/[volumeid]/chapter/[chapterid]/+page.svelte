<script lang="ts">
	import { page } from '$app/stores';
	import Banner from '$lib/components/studio/std/comps/Banner.svelte';
	import type { Chapter } from '$lib/types/content';
	import { PageError } from '$lib/types/error';
	import { Result } from '$lib/types/result';
	import type { FormState } from '$lib/types/state/form_state';
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

	let chapterId = $page.params.chapterid;
	let formOn = $state(false);

	let chapter: Chapter | null = $state(null);

	let pages: any[] = $state([]);

	let pageInFocus = $state(0);

	onMount(async () => {
		let res_chapter = await fetch(
			`http://localhost:7878/content/comic/volume/chapter/show/${chapterId}`,
			{
				method: 'GET',
				credentials: 'include',
				headers: {
					'Content-Type': 'application/json'
				}
			}
		);

		if (res_chapter.ok == true) {
			//UNIMPLEMENTED
			console.log(res_chapter);
			let res = await res_chapter.json();
			chapter = res.chapter;
			console.log($state.snapshot(chapter));
			pageState.loading = false;
		} else if (res_chapter.ok == false) {
			console.error('failed');
			console.log(res_chapter);
			if (res_chapter.status == 401) {
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
			} else if (res_chapter.status == 404) {
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

		const res = await fetch(
			`http://localhost:7878/content/comic/volume/chapter/file/count/${chapter?.file}`
		);
		const count = await res.json();
		pages = Array.from({ length: count + 1 }, (_, i) => i);
	});
	// endsection:  --- State

	//TODO: Retrieve chapter
</script>

<main class="page">
	<div
		class="pageInFocus secondary_border flex_center sticky right-2 top-2 float-right w-16 rounded-md p-1"
	>
		{pageInFocus} / {pages.length - 1}
	</div>
	{#if pageState.loading}
		<center>Loading chapter...</center>
	{:else if pageState.loading == false && pageState.inner_state == Result.Ok}
		<Banner text={chapter?.title}></Banner><br />
		<ul class="flex_col h-fit w-full p-1">
			<li class=""><b class="">Pages: &nbsp;</b>{chapter?.pages}</li>
			<li class=""><b class="">Number in volume: &nbsp;</b>{chapter?.relative_chapter}</li>
			<li class=""><b class="">Number in comic: &nbsp;</b>{chapter?.absolute_chapter}</li>
			<br />
			<li class=""><b class="">Synopsis<br />&nbsp;</b>{chapter?.synopsis}</li>
		</ul>

		<section class="book flex_col w-full">
			{#each pages as page}
				{#if page > 0}
					<!-- svelte-ignore a11y_no_static_element_interactions -->
					<article
						class="comicPage secondary_border flex_col my-4 w-full rounded p-1 md:w-10/12"
						onmouseenter={() => (pageInFocus = page)}
					>
						<img
							src={chapter?.file
								? `http://localhost:7878/content/comic/volume/chapter/file/${page}/${chapter.file}`
								: ''}
							alt={`Page ${page + 1}`}
							loading="lazy"
							class=" md:10/12 w-full"
						/>
						<!-- style="width: 90%; margin-bottom: 1rem;" -->
						<p>{page}</p>
					</article>
				{/if}
			{/each}
		</section>
	{:else}
		<div class="back_btn flex_center w-full p-3">
			<!-- <a
						class="  primary_txt_hover secondary_bg_hover w-auto rounded-xl border border-yellow-300 p-1 text-center font-semibold"
						href="/content/comics/create/{chapter?.id.id.String}/">Attach comic</a
					> -->
		</div>
	{/if}
</main>

<style>
	.pageInFocus {
		font-weight: bolder;
		text-align: center;
		color: black;
		background-color: var(--secondary);
		opacity: 0.7;
	}
	.comicPage:hover {
		cursor: pointer;
		box-shadow: 3px 3px 3px 3px gold;
		transition: all ease-in-out 0.3s;
	}
</style>
