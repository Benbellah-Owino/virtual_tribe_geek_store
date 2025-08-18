<script lang="ts">
	import { page } from '$app/stores';
	import { PageError } from '$lib/types/error';
	import { Result } from '$lib/types/result';
	import { updatePageState, type PageState } from '$lib/types/state/page_state';
	import type { Content } from '$lib/types/content';
	import { onMount } from 'svelte';
	import Banner from '$lib/components/studio/std/comps/Banner.svelte';

	let content_id = $page.params.id;

	let content: Content = $state({
		id: {
			id: {
				String: ''
			},
			tb: ''
		},
		rating: 0,
		title: '',
		studio: null,
		description: '',
		audiences: '',
		recom_price: 0,
		genre: []
	});

	let pageState: PageState = $state({
		inner_state: Result.Ok,
		error: null,
		loading: true,
		message: ''
	});

	let comic: any|null = $state(null);
	let video: any|null = $state(null);

	onMount(async () => {
		// Get content
		let response = await fetch(`http://localhost:7878/content/${content_id}`, {
			method: 'GET',
			credentials: 'include',
			headers: {
				'Content-Type': 'application/json'
			}
		});

		if (response.ok == true) {
			//UNIMPLEMENTED
			let res = await response.json();
			let db_content = res.content;
			content = db_content;
			console.log($state.snapshot(content));
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
					Result.Err,
					PageError.Unauthorized,
					false,
					"The requested content doesn't exist"
				);
			}
		}

		// GET COMIC DETAILS
		let cont = content_id.split(':');
		getComic(cont)
		getVideo(cont)
	});
	
	async function getComic(cont:string[]) {
		let response2 = await fetch(`http://localhost:7878/content/comic/index?content=${cont[1]}`, {
			method: 'GET',
			credentials: 'include',
			headers: {
				'Content-Type': 'application/json'
			}
		});

		if (response2.ok == true) {
			//UNIMPLEMENTED
			let res = await response2.json();
			// console.log($state.snapshot(res));
			comic = res.comic;
			console.log($state.snapshot(comic));
			pageState.loading = false;
		} else if (response2.ok == false) {
			console.error('failed');
			if (response2.status == 401) {
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
			} else if (response2.status == 404) {
				updatePageState(
					pageState,
					Result.Ok,
					PageError.NotFoundError,
					false,
					"The requested comic doesn't exist"
				);
				console.log($state.snapshot(pageState));
			}
		}
	}
	// TODO: Finish this page
	async function getVideo(cont:string[]) {
		let response2 = await fetch(`http://localhost:7878/content/video/index?content=${cont[1]}`, {
			method: 'GET',
			credentials: 'include',
			headers: {
				'Content-Type': 'application/json'
			}
		});

		if (response2.ok == true) {
			//UNIMPLEMENTED
			let res = await response2.json();
			// console.log($state.snapshot(res));
			video = res.video;
			console.log($state.snapshot(video));
			pageState.loading = false;
		} else if (response2.ok == false) {
			console.error('failed');
			if (response2.status == 401) {
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
			} else if (response2.status == 404) {
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
	}
</script>

<main class="page">
	{#if pageState.loading}
		<center>Loading content...</center>
	{:else if pageState.loading == false && pageState.inner_state == Result.Ok}
		<center> content</center>
		<img
			src={content.cover ? `http://localhost:7878/content/image/${content.cover}` : ''}
			alt="Profile picture of {content.name}"
			width="281px"
			height="500px"
			class="profile secondary_border mx-auto mb-2 rounded-md"
		/>
		<center><Banner text={content.name} /></center>
		<hr class="main_txt">
		<ul class="mt-8 flex h-fit w-full flex-col items-start justify-center p-4">
			<li class="tertiary_txt"><b class="main_txt">Title: &nbsp </b>{content.title}</li>
			<li class="tertiary_txt"><b class="main_txt">Rating: &nbsp </b>{content.rating}</li>
			<li class="tertiary_txt">
				<b class="main_txt">Audience Rating: &nbsp </b><span class="font-extrabold"
					>{content.audiences}</span
				>
			</li>
			<h3><b>Genres:</b></h3>
			<div class="genres flex_row">
				{#each content.genre as genre}
					<span
						class="secondary_bg tertiary_bg_hover primary_txt tertiary_txt_hover tertiary_border m-1 cursor-pointer rounded-full border-2 p-1 text-sm font-bold"
						>{genre.name}</span
					>
				{/each}
			</div>
			<li class="tertiary_txt"><b class="main_txt">Studio: &nbsp </b>{content.studio?.name}</li>

			<li class="tertiary_txt w-10/12 p-1 border border-gray-500 rounded">
				<h3 class="main_txt underline">Description</h3>
				<p class="text-sm">{content.description}</p>
			</li>
		</ul>

		<!-- TODO Add authoirization logic to this component -->

		<!-- COMIC INFO -->
		<section class="comic mt-5 p-2">
			{#if comic}
				<h2 class="my text-2xl"><b><u>COMIC</u></b></h2>
				<h4><b>Writers</b></h4>
				<ul class="style pl-4">
					{#each comic.writer as writer}
						<li>{writer}</li>
					{/each}
				</ul>
				<br>

				<h4><b>Artists</b></h4>
				<ul class="style pl-4">
					{#each comic.artist as artist}
						<li>{artist}</li>
					{/each}
				</ul>
				<br>
				
				<h4><b>creators</b></h4>
				<ul class="style pl-4">
					{#each comic.creator as creator}
						<li>{creator.username}</li>
					{/each}
				</ul>

				<div class="back_btn flex_center w-full p-3">
					<a
						class="  primary_txt_hover secondary_bg_hover w-auto rounded-xl border border-yellow-300 p-1 text-center font-semibold"
						href="/content/comics/{comic.id.id.String}/">View comic</a
					>
				</div>
				
				<br>
				
				<!-- TODO: Add routing to creator given an id in the URL. This is to make it enable
					the usernames above to be links to creator page
				-->

			{:else}
				<div class="back_btn flex_center w-full p-3">
					<a
						class="  primary_txt_hover secondary_bg_hover w-auto rounded-xl border border-yellow-300 p-1 text-center font-semibold"
						href="/content/comics/create/{content_id}/">Attach comic</a
					>
				</div>

			{/if}
			{#if video}
				<h2 class="my text-2xl"><b><u>VIDEO</u></b></h2>
				<h4><b>Writers</b></h4>
				<ul class="style pl-4">
					{#each video.writer as writer}
						<li>{writer}</li>
					{/each}
				</ul>
				<br>
				
				<h4><b>creators</b></h4>
				<ul class="style pl-4">
					{#each video.creator as creator}
						<li>{creator.username}</li>
					{/each}
				</ul>

				<div class="back_btn flex_center w-full p-3">
					<a
						class="  primary_txt_hover secondary_bg_hover w-auto rounded-xl border border-yellow-300 p-1 text-center font-semibold"
						href="/content/videos/{video.id.id.String}/">View Video</a
					>
				</div>
				
				<br>
				
				
			{:else}
				<div class="back_btn flex_center w-full p-3">
					<a
						class="  primary_txt_hover secondary_bg_hover w-auto rounded-xl border border-yellow-300 p-1 text-center font-semibold"
						href="/content/video/create/{content_id}/">Attach video</a
					>
				</div>
			{/if}
		</section>
		<!--  -->
		<div class="back_btn flex_center w-full p-3">
			<a
				class="  primary_txt_hover secondary_bg_hover w-14 rounded-xl border border-yellow-300 p-1 text-center font-semibold"
				href="/studio/{content.studio?.id.tb}:{content.studio?.id.id.String}/content">back</a
			>
		</div>
		<!-- <a href="/content/{content_id}/content" class="">Click here to view this contents content</a> -->
	{:else if pageState.inner_state == Result.Err}
		<h3 class="error">{pageState.message}</h3>
	{/if}
</main>
