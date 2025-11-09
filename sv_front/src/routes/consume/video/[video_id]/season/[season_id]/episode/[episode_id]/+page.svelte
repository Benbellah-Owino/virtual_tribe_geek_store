<script lang="ts">
	import { page } from '$app/stores';
	import Banner from '$lib/components/studio/std/comps/Banner.svelte';
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


	//let videoType: VideoType = $state(VideoType.UNKNOWN);

	let episodeId = $page.params.episode_id;

	let episode: any | null = $state(null);

	// let video_file_details: VideoFileDetails = $state({
	//     count: 50,
	//     content_type: ''
	// });

	onMount(async () => {
		let res_episode = await fetch(
			`http://localhost:7878/content/video/season/episode/show/${episodeId}`,
			{
				method: 'GET',
				credentials: 'include',
				headers: {
					'Content-Type': 'application/json'
				}
			}
		);

		if (res_episode.ok == true) {
			//UNIMPLEMENTED
			console.log(res_episode);
			let res = await res_episode.json();
			episode = res.episode;
			console.log($state.snapshot(episode));
			pageState.loading = false;
		} else if (res_episode.ok == false) {
			console.error('failed');
			console.log(res_episode);
			if (res_episode.status == 401) {
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
			} else if (res_episode.status == 404) {
				console.error('not found');
				updatePageState(
					pageState,
					Result.Ok,
					PageError.NotFoundError,
					false,
					"The requested volumrd does't exist"
				);
				console.log($state.snapshot(pageState));
			}
		}

		// try {
		//     const res = await fetch(
		//         `http://localhost:7878/content/video/season/episode/file/count/${episode?.file}`
		//     );
		//     video_file_details = await res.json();
		//     videoType = ret_video_type(video_file_details.content_type);
		//     console.log(video_file_details);
		//     if (video_file_details.content_type == 'pdf') {
		//         console.log('Fetching pdf ' + episode?.file);
		//         let url = `http://localhost:7878/content/video/season/episode/file/0/${episode?.file}`;
		//         renderPdf(url);
		//     }
		// } catch (error) {
		//     console.error(error);
		// }
		// pages = Array.from({ length: video_file_details.count + 1 }, (_, i) => i);
	});
	// endsection:  --- State
</script>

<main class="page">
	<center>Hello</center>
	{#if pageState.loading}
		<center>Loading episode...</center>
	{:else if pageState.loading == false && pageState.inner_state == Result.Ok}
		<Banner text={episode?.title}></Banner><br />

		<!-- svelte-ignore a11y_media_has_caption -->
		<center class="w-full">
			<video class="main_border h-96 w-11/12 rounded" controls>
				<source
					src="http://localhost:7878/content/video/season/episode/file/{episode.file}"
					type="video/mp4"
				/>
			</video>
		</center>
	{:else}
		<div class="back_btn flex_center w-full p-3">
			<!-- <a
                        class="  primary_txt_hover secondary_bg_hover w-auto rounded-xl border border-yellow-300 p-1 text-center font-semibold"
                        href="/content/videos/create/{episode?.id.id.String}/">Attach video</a
                    > -->
		</div>
	{/if}
</main>
