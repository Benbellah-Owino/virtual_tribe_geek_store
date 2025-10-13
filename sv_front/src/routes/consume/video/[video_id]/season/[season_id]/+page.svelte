<script lang="ts">
	import { page } from '$app/stores';
	import { stringToSurrealId } from '$lib/helper_functions.ts/converters';
	import type { EpisodeForCreate } from '$lib/types/content';
	import { PageError } from '$lib/types/error';
	import { Result } from '$lib/types/result';
	import type { FormState } from '$lib/types/state/form_state';
	import { updatePageState, type PageState } from '$lib/types/state/page_state';
	import { onMount } from 'svelte';

	let seasonId = $page.params.season_id;
	let videoId = $page.params.video_id;

	// section:     --- State
	let pageState: PageState = $state({
		inner_state: Result.Ok,
		error: null,
		loading: true,
		message: ''
	});

	// endsection:  --- State

	// section:     --- Variables
	let video: any | null = $state(null);
	let episodes: any[] = $state([]);

	// endsection:  --- Variables

	onMount(async () => {
		let res_episodes = await fetch(
			`http://localhost:7878/content/video/season/episode/${seasonId}`,
			{
				method: 'GET',
				credentials: 'include',
				headers: {
					'Content-Type': 'application/json'
				}
			}
		);

		if (res_episodes.ok == true) {
			//UNIMPLEMENTED
			let res = await res_episodes.json();
			episodes = res.episode_list;
			episodes = episodes.sort((a, b) => a.relative_episode - b.relative_episode);
			episodes.forEach((e) => {
				console.log($state.snapshot(e));
			});

			pageState.loading = false;
		} else if (res_episodes.ok == false) {
			console.error('failed');
			console.log(res_episodes);
			if (res_episodes.status == 401) {
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
			} else if (res_episodes.status == 404) {
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
	{#if pageState.loading}
		<center>Loading content...</center>
	{:else if pageState.loading == false && pageState.inner_state == Result.Ok && pageState.error == null}
		<ul
			class="contentList my-5 flex h-fit w-full flex-col items-center justify-center p-2 md:grid md:grid-cols-3"
			id="content_list"
		>
			{#each episodes as episode}
				<!-- svelte-ignore a11y_click_events_have_key_events -->
				<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
				<article
					class="episode color2_bg_hover m-2 w-72 cursor-pointer bg-black duration-300 hover:scale-105"
					onclick={() => {
						window.open(
							`/consume/video/${videoId}/season/${seasonId}/episode/${episode.id.id.String}`
						);
					}}
				>
					<img
						src={episode?.cover
							? `http://localhost:7878/content/video/season/episode/cover/${episode.cover}`
							: ''}
						alt="Picture of {episode?.title}"
						height="384px"
						class="episodeCover mx-auto w-72 object-contain"
					/>
					<div class="w-full pl-2">
						<a
							class="tertiary_txt secondary_txt_hover font-bold underline"
							href="/consume/video/{videoId}/season/{seasonId}/episode/{episode.id.id.String}"
							>{episode.relative_episode}. {episode.title}</a
						>
					</div>
				</article>
			{/each}
		</ul>
		<br /><br />
	{/if}
	<a
		class="tertiary_txt secondary_txt_hover font-bold underline"
		href="/content/video/{videoId}/season/{seasonId}/episode">Add Episode</a
	>
</main>

<style>
	.contentList {
		min-height: 30rem;
	}
	.episode {
		height: 26rem;
	}

	.episodeCover {
		object-fit: cover;
		height: 22rem;
		margin-bottom: 6px;
	}
</style>
