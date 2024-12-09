
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
		studio: {
			id: {
				String: ''
			},
			tb: ''
		},
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
	onMount(async () => {
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
			console.log(db_content);
			content = db_content;
			console.log($state.snapshot(content

			)
			)
			pageState.loading = false;
		} else if (response.ok == false) {
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
			}
		}
	});
    // TODO: Finish this page
</script>
<main class="page">
	{#if pageState.loading}
		<center>Loading content...</center>
	{:else if pageState.loading == false && pageState.inner_state == Result.Ok}
		<center> content</center>
		<img
			src={content.cover
				? `http://localhost:7878/content/image/${content.cover}`
				: ''}
			alt="Profile picture of {content.name}"
			width="128px"
			height="128px"
			class="profile secondary_border mx-auto mb-2 rounded-full"
		/>
		<center><Banner text={content.name} /></center>
		<ul class="secondary_border mt-8 flex h-fit w-full flex-col items-start justify-center p-4">
			<li class="tertiary_txt"><b class="main_txt">Name: &nbsp </b>{content.name}</li>
			<li class="tertiary_txt"><b class="main_txt">Email: &nbsp</b>{content.email}</li>
			<li class="tertiary_txt w-10/12 border border-gray-500">
				<h3 class="main_txt underline">Description</h3>
				<p class="text-sm">{content.description}</p>
			</li>
		</ul>

        <a href="/content/{content_id}/content" class="">Click here to view this contents content</a>
	{:else if pageState.inner_state == Result.Err}
		<h3 class="error">{pageState.message}</h3>
	{/if}
</main>