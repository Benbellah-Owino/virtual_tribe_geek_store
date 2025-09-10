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
		title: '',
		synopsis: '',
		runlength: {
			seconds: 0,
			minutes: 0,
			hours: 0
		},
		opening_length: {
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
		closing_length: {
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
	<center><h1 class="text-6xl font-extrabold">EPISODE CREATION FORM</h1></center>

	<br />
	<center class="p-4">
		<EpisodeForm {episode_form} />
	</center>
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
