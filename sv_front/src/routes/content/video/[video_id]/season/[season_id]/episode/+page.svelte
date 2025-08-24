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

	let formState: FormState = $state({
		inner_state: Result.Ok,
		error: null,
		message: '',
		target: '',
		locked: false
	});

	let seasonId = $page.params.season_id;
	let videoId = $page.params.video_id;
	let form_on = $state(false);
	// endsection:  --- State

	// section:     --- Variables
	let video: any | null = $state(null);
	let episodes: any[] = $state([]);

	let episode_form: EpisodeForCreate = $state({
        title: "",
		runlength: {
			seconds: 0,
			minutes: 0,
			hours: 0
		},
		skiplength: {
			start: {
				seconds: 0,
				minutes: 0,
				hours: 0
			},
			end: {
				seconds: 0,
				minutes: 0,
				hours: 0
			}
		},
		sypnosis: '',
		season: {
			id: {
				String: ''
			},
			tb: ''
		}
	});

	// endsection:  --- Variables

	onMount(async () => {
		episode_form.season = stringToSurrealId(`season:${seasonId}`);

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
			console.log(res_episodes);
			let res = await res_episodes.json();
			episodes = res.episodes_list;
			episodes = episodes.sort((a, b) => a.relative_episode - b.relative_episode);
			console.log($state.snapshot(episodes));
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

	async function submit(e: Event) {
		e.preventDefault();
		console.log($state.snapshot(episode_form));
		try {
			let response = await fetch(`http://localhost:7878/content/video/season/episode`, {
				method: 'POST',
				credentials: 'include',
				body: JSON.stringify(episode_form),
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
				console.log(res);
				console.log('Uploading file');
				upload_file(res.episode.id);
				//window.open(`/content/videos/${seasonId}`, '_self');
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
				console.log(response);
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

	async function pf(e: Event) {
		e.preventDefault();
	}

	async function upload_file(id: SurrealId) {
		const fileInput: any = document.getElementById('video');
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

		let response = await fetch(
			`http://localhost:7878/content/video/season/episode/file/upload/${suridToString(id)}`,
			{
				method: 'POST',
				credentials: 'include',
				body: formData
			}
		);

		if (response.ok == true) {
			console.log(await response.json());
			setTimeout(() => {
				updateFormState(formState, Result.Ok, null, 'Uploading success!', 'form', true);
			}, 3000);
			console.log('Uploading file');
			//open(`/studio/${$page.params.studio}/content`)
		} else if (response.ok == false) {
			console.log('Comic upload failed');
			updateFormState(
				formState,
				Result.Err,
				FormError.UpdateFailed,
				'video',
				'Upload failed',
				false
			);
		} else {
			console.log('Cover upload failed');
			console.log(response.ok);
			console.log(response.status);
		}
	}

	async function upload_cover(id: SurrealId) {
		console.log(id);
		const fileInput: any = document.getElementById('video_cover');
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
			console.log(await response.json());
			setTimeout(() => {
				updateFormState(formState, Result.Ok, null, 'Uploading success!', 'form', true);
			}, 3000);
			open(`/studio/${$page.params.studio}/content`);
		} else if (response.ok == false) {
			console.log('Cover update failed');
			updateFormState(
				formState,
				Result.Err,
				FormError.UpdateFailed,
				'video_cover',
				'Update Failed',
				false
			);
		} else {
			console.log('Cover update failed');
			console.log(response.ok);
			console.log(response.status);
		}
	}
</script>

<main class="page">
	<center><h1 class="text-6xl font-extrabold">Chapters</h1></center>
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
							`/content/videos/${videoId}/season/${seasonId}/episode/${episode.id.id.String}`
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
							href="/content/videos/{videoId}/season/{seasonId}/episode/{episode.id.id.String}"
							>{episode.relative_episode}. {episode.title}</a
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
				<EpisodeForm {episode_form} />
			</center>
		{/if}
	{:else if (pageState.loading == false && pageState.inner_state == Result.Ok && pageState.error == PageError.NotFoundError) || form_on == true}
		<center class="mt-3">
			<h2 class="mb-2 mt-8 text-3xl">Add new episode to this season below</h2>
			<EpisodeForm {episode_form} />
		</center>
	{:else}
		<h3>Not found</h3>
	{/if}
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
