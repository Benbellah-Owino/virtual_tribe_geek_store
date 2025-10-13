<script lang="ts">
	import { page } from '$app/stores';
	import EpisodeForm from '$lib/components/content/forms/episodeForm.svelte';
	import { stringToSurrealId } from '$lib/helper_functions.ts/converters';
	import type { EpisodeForCreate } from '$lib/types/content';
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


	let seasonId = $page.params.season_id;
	let videoId = $page.params.video_id;
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
					window.open('user/login', '_self');
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
	<!-- TODO: This page will show latest or suggested episodes for the user -->
	Episode Creation Page
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
