<script lang="ts">
	import type { Creator } from '$lib/types/creator';
	import stockProfilePic from '$lib/assets/stock_pp.png';
	import { onMount } from 'svelte';
	import { updatePageState, type PageState } from '$lib/types/state/page_state';
	import { Result } from '$lib/types/result';
	import { page } from '$app/stores';
	import { convertIsoToDate } from '$lib/helper_functions.ts/date_time';
	import { error, redirect } from '@sveltejs/kit';
	import { PageError } from '$lib/types/error';

	let pageState: PageState = $state({
		inner_state: Result.Ok,
		error: null,
		loading: true,
		message: 'Loading...'
	});

	let creator: Creator = $state(  {
		role: '',
		socials: {
			twitter: null,
			instagram: null,
			facebook: null
		},
		username: '',
		email: '',
		description: null,
		avatar: null,
		joined_at: null
	} );

	onMount(async () => {
		let response = await fetch(`http://localhost:7878/creator`, {
			method: 'GET',
			credentials: 'include',
			headers: {
				'Content-Type': 'application/json'
			}
		});

		if (response.ok == true) {
			//UNIMPLEMENTED
			let res = await response.json();
			let db_creator = res.creator;
			console.log(db_creator);
			creator = db_creator;
			pageState.loading = false;
		} else if (response.ok == false) {
			if(response.status == 401){
				updatePageState(pageState, Result.Err,PageError.Unauthorized, false, 'You are not authorized! Redirecting you to login page...',);
				setTimeout(()=>{
				window.open('creator/login', '_self')
				}, 5000)
			}
		}
	});
	let sample_text = "No description was provided"
</script>

<main class="page flex_col p-6">
	{#if pageState.inner_state == Result.Err}
		<h1 class="text-6xl font-extrabold text-red-600">{pageState.message}</h1>
	{:else if pageState.inner_state == Result.Ok && pageState.loading == true}
		<h1 class="text-6xl font-extrabold">{pageState.message}</h1>
	{:else if pageState.inner_state == Result.Ok && pageState.loading == false}
		<section class="personal_info border1 flex_col w-11/12 rounded p-3 cursor-pointer">
			<img
				src={creator.avatar ? creator.avatar : stockProfilePic}
				alt="Profile picture of {creator.username}"
				width="128px"
				height="128px"
				class="profile secondary_border rounded-full"
			/>

			<article class="text_info flex_col mt-4 items-center text-center">
				{creator.username ? creator.username : 'Usermame'} <br />
				{creator.email ? creator.email : 'Email'}
			</article>

			<!-- TODO: Convert the roles field to an array -->
			<ul class="roles">
				<li>{creator.role}</li>
			</ul>
        <h5 class="text-xs">Joined at {creator.joined_at ? convertIsoToDate(creator.joined_at.toString()) : 'None' }</h5>
		</section>

		<article class="description border1 flex_col mt-4 flex w-11/12 rounded p-3 cursor-pointer">
			<h3 class="float-left w-full font-bold underline">Bio description</h3>
			<hr />
			<p class="text-sm">
				{creator.description ? creator.description : sample_text}
			</p>
		</article>

		<ul class="socials color1_txt border1 mt-4 w-11/12 rounded p-3 text-sm cursor-pointer">
			<h3 class="float-left w-full text-base font-bold underline">Socials</h3>
			{#if creator.socials}
				{#if creator.socials.facebook}
					<li><a href={creator.socials.facebook} target="_blank" class="txt1_hover">Facebook</a></li>
				{/if}
				{#if creator.socials.instagram}
					<li><a href={creator.socials.instagram} target="_blank" class="txt1_hover">Instagram</a></li>
				{/if}
				{#if creator.socials.twitter}
					<li><a href={creator.socials.twitter} target="_blank" class="txt1_hover">Twitter / X</a></li>
				{/if}

            {:else}
                No social media links were provided
			{/if}
		</ul>
	{/if}
</main>
