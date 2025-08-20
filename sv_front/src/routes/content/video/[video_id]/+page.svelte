<script lang="ts">
	import { page } from "$app/stores";
	import SeasonForm from "$lib/components/content/forms/seasonForm.svelte";
	import { stringToSurrealId } from "$lib/helper_functions.ts/converters";
	import type { SeasonForCreate } from "$lib/types/content";
	import { FormError, PageError } from "$lib/types/error";
	import { Result } from "$lib/types/result";
	import { updateFormState, type FormState } from "$lib/types/state/form_state";
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

	let video_id = $page.params.video_id;

	let form_on = $state(false);
	// endsection:  --- State



	// section:     --- Variables
	let video: any | null = $state(null);
	let seasons: any[] = $state([]);

    let season_form: SeasonForCreate=  $state({
		no_of_episodes: 0,
		synopsis: "",
		video: {
			id: {
				String: ""
			},
			tb: ""
		}
	});

	// endsection:  --- Variables

	onMount(async()=>{

		season_form.video = stringToSurrealId(`video:${video_id}`);
		let response = await fetch(`http://localhost:7878/content/video/index?video=${video_id}`, {
			method: 'GET',
			credentials: 'include',
			headers: {
				'Content-Type': 'application/json'
			}
		});

		if (response.ok == true) {
			//UNIMPLEMENTED
			let res = await response.json();
			// console.log($state.snapshot(res));
			video = res.video;
			console.log($state.snapshot(video));
			pageState.loading = false;
		} else if (response.ok == false) {
			console.error('failed');
			if (response.status == 401) {
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
			} else if (response.status == 404) {
				updatePageState(
					pageState,
					Result.Ok,
					PageError.NotFoundError,
					false,
					"The requested video doesn't exist"
				);
				console.log($state.snapshot(pageState));
			}
		}


		let res_seasons = await fetch(`http://localhost:7878/content/video/season/${video_id}`, {
			method: 'GET',
			credentials: 'include',
			headers: {
				'Content-Type': 'application/json'
			}
		});

		if (res_seasons.ok == true) {
			//UNIMPLEMENTED
			console.log(res_seasons);
			let res = await res_seasons.json();
			seasons = res.season_list;
			seasons = seasons.sort((a,b) => a.vol_no - b.vol_no);
			console.log($state.snapshot(seasons));
			pageState.loading = false;
		} else if (res_seasons.ok == false) {
			console.error('failed');
			console.log(res_seasons);
			if (res_seasons.status == 401) {
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
			} else if (res_seasons.status == 404) {
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
</script>

<main>
	{#if pageState.loading}
		<center>Loading content...</center>
	{:else if pageState.loading == false && pageState.inner_state == Result.Ok && pageState.error == null}
		<h1 class="mb-7 mt-4 text-center text-3xl font-extrabold">Season list</h1>
		<ul class="content_list flex_col" id="content_list">
			{#each seasons as season}
				<article class="seasons secondary_bg_hover primary_txt_hover secondary_border  m-2 p-1 rounded cursor-default">
					<li><a class="tertiary_txt font-bold underline " href="/content/videos/{video_id}/season/{season.id.id.String}">Season {season.season_no}</a></li>
					<p class="text-sm">
						{season.synopsis}
					</p>
				</article>
			{/each}
		</ul>
		<br><br>
		<center>
			<button
				class="btn primary_btn"
				onclick={() => {
					form_on = !form_on;
					console.log($state.snapshot(form_on));
				}}>Toggle season form</button
			>
		</center>
		{#if form_on == true}
			<center class="w-full">
                <SeasonForm {video_id} {season_form}/>
            </center>
		{/if}
	{:else if pageState.loading == false && pageState.inner_state == Result.Ok && pageState.error == PageError.NotFoundError}
		<center class="w-full">
                <SeasonForm {video_id} {season_form}/>
		</center>
	{:else}
		<h3>Not found</h3>
	{/if}
</main>

<style>
</style>