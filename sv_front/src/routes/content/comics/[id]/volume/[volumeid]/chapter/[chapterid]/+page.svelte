<script lang="ts">
	import { page } from "$app/stores";
	import Banner from "$lib/components/studio/std/comps/Banner.svelte";
	import type { Chapter } from "$lib/types/content";
	import { PageError } from "$lib/types/error";
	import { Result } from "$lib/types/result";
	import type { FormState } from "$lib/types/state/form_state";
	import { updatePageState, type PageState } from "$lib/types/state/page_state";
	import { onMount } from "svelte";


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


    let chapter: Chapter|null = $state(null);

	let pages: any[] = $state([]);

    onMount(async()=>{      
		let res_chapter = await fetch(`http://localhost:7878/content/comic/volume/chapter/show/${chapterId}`, {
			method: 'GET',
			credentials: 'include',
			headers: {
				'Content-Type': 'application/json'
			}
		});

		if (res_chapter.ok == true) {
			//UNIMPLEMENTED
			console.log(res_chapter);
			let res = await res_chapter.json();
			chapter = res.chapter;
			console.log($state.snapshot(chapter));
			pageState.loading = false;
			pages = Array.from({ length: chapter?.pages }, (_, i) => i);
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
    })
	// endsection:  --- State

    //TODO: Retrieve chapter
</script>

<main class="page">
	{#if pageState.loading}
		<center>Loading chapter...</center>
	{:else if pageState.loading == false && pageState.inner_state == Result.Ok}
	<Banner text={chapter?.title}></Banner><br>
		<ul class="flex_col h-fit w-full p-1">
			<li class=""><b class="">Pages: &nbsp;</b>{chapter?.pages}</li>
			<li class=""><b class="">Number in volume: &nbsp;</b>{chapter?.relative_chapter}</li>
			<li class=""><b class="">Number in comic: &nbsp;</b>{chapter?.absolute_chapter}</li>
			<br>
			<li class=""><b class="">Synopsis<br>&nbsp;</b>{chapter?.synopsis}</li>
		</ul>

		<iframe
			src={chapter?.file ? `http://localhost:7878/content/comic/volume/chapter/image/${chapter.file}` : ''}
			title =" {chapter?.file}"
			width="281px"
			height="500px"
			class="profile secondary_border mx-auto mb-2 rounded-md"
		></iframe>

		{#each pages as page}
  <img
			src={chapter?.file ? `http://localhost:7878/content/comic/volume/chapter/file/${chapterId}/${chapter.file}/${page}` : ''}
    alt={`Page ${page + 1}`}
    loading="lazy"
    style="width: 100%; margin-bottom: 1rem;"
  />
	{/each}

	{:else}
				<div class="back_btn flex_center w-full p-3">
					<a
						class="  primary_txt_hover secondary_bg_hover w-auto rounded-xl border border-yellow-300 p-1 text-center font-semibold"
						href="/content/comics/create/{chapter?.id.id.String}/">Attach comic</a
					>
				</div>
	{/if}

</main>
