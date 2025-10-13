<script lang="ts">
	import { updateFormState, type FormState } from '$lib/types/state/form_state';
	import { FormError } from '$lib/types/error';
	import { Result } from '$lib/types/result';
	import type { VideoForCreate } from '$lib/types/content';
	import { page } from '$app/stores';
	import { onMount } from 'svelte';
	import { stringToSurrealId, surrealIdToString } from '$lib/helper_functions.ts/converters';

	// Creators
	let creators: any[];
	let proc_c = $state('');
	let found_creators: any[] = $state([]);
	('');
	let video_creators: string[];
	let search_c: any[] = $state([]);

	// Writers
	let writers: { id: number; name: string }[] = $state([]);
	let writerStr: string = $state('');
	let writerId = 0;

	// artists
	let artists: { id: number; name: string }[] = $state([]);
	let artistStr: string = $state('');
	let artistId = 0;
	$effect(() => {
		if (proc_c == '') {
			search_c = [];
		}
	});

	onMount(async () => {
		let response = await fetch(`http://localhost:7878/creator/list`, {
			method: 'GET',
			credentials: 'include',
			headers: {
				'Content-Type': 'application/json'
			}
		});

		if (response.ok == true) {
			let res = await response.json();
			creators = res.creators;

			// console.log(creators);
		} else if (response.ok == false) {
		}
	});

	let formState: FormState = $state({
		inner_state: Result.Ok,
		error: null,
		message: '',
		target: '',
		locked: false
	});


	let video_form: VideoForCreate = $state({
		content: '',
		writer: [],
		creator: [],
		video_content: '',
		average_run_length: 0
	});

	async function submit(e: Event) {
		e.preventDefault();
		console.log('-----------------------------------------');
		console.log('submit');
		video_form.writer = writers.map((m) => {
			return m.name;
		});
        
		video_form.creator = found_creators.map((m) => {
			return m.id;
		});
		// video_form.creator = video_form.creator.map((c)=>{
		// 	console.log($state.snapshot(c))
		// 	if(typeof(c)== 'object'){
		// 		console.info("converting")
		// 		let cs = surrealIdToString(c);
		// 		console.log(cs)
		// 		return cs
		// 	}else{
		// 		return "ERROR"
		// 	}
		// })
		if(typeof(video_form.content)=='string'){
			video_form.content = stringToSurrealId(video_form.content);
		}
		console.log($state.snapshot(video_form));
		// console.log($state.snapshot(found_creators));
		console.log('-----------------------------------------');

		//TODO: Send to backend
		try {
			// TODO" Fix this route
			let response = await fetch(`http://localhost:7878/content/video`, {
				method: 'POST',
				credentials: 'include',
				body: JSON.stringify(video_form),
				headers: {
					'Content-Type': 'application/json'
				}
			});
			if (response.status == 201) {
				//UNIMPLEMENTED
				setTimeout(() => {
					updateFormState(formState, Result.Ok, null, 'Registration success', 'form', true);
					console.log($state.snapshot(formState));
				}, 3000);
				console.log('created');
				window.open(`/content/${$page.params.content}`, '_self');
			} else if (response.status == 500) {
				updateFormState(
					formState,
					Result.Err,
					FormError.SubmissionFailed,
					'Submission Failed',
					'form',
					false
				);
			}else if (response.ok == false) {
				console.log(response.statusText)
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



	function pf(e: Event) {
		e.preventDefault();
	}
</script>

<main class="page main_bg flex_col mt-5 h-full w-full">
	<h1 class="mb-7 text-center text-3xl font-extrabold">VIDEO SERIES CREATION PAGE</h1>
	<center class="w-full">
		<form
			enctype="multipart/form-data"
			class=" flex_col secondary_border mb-9 w-full rounded-md"
			onsubmit={pf}
		>
			<input type="file" name="cover" id="cover" /><br />
		</form>
		<form class="form alt_bg rounded-lg p-3 md:w-96 lg:w-5/6" onsubmit={submit}>
			<h3 class="float-left mb-4 text-3xl font-extrabold">CREATE VIDEO SERIES</h3>
			<br />
			{#if formState.inner_state == Result.Ok && formState.target == 'form'}
				<center><p class="error main_txt text-lg font-semibold">{formState.message}</p></center>
			{:else if formState.inner_state == Result.Err && formState.target == 'form'}
				<center><p class="error text-lg font-semibold text-red-400">{formState.message}</p></center>
			{/if}
			<div class="form_div">
				<label for="name">Writers</label>
				<div class="creators flex_row m-2" id="creators">
					{#each writers as w (w.id)}
						<div
							class="creator_tag secondary_bg_hover primary_txt_hover secondary_border mx-1 cursor-pointer rounded-full p-1 text-xs"
							id={`${w.id}_${w.name}`}
						>
							{w.name}
							<button
								onclick={() =>
									(writers = writers.filter((wr) => {
										return wr.id != w.id;
									}))}>x</button
							>
						</div>
					{/each}
				</div>
				<input
					type="text"
					name="writers"
					id="writers"
					bind:value={writerStr}
					oninput={(e) => {
						if (writerStr.charAt(writerStr.length - 1) == ',') {
							writers.push({ id: writerId, name: writerStr.substring(0, writerStr.length - 1) });
							writerStr = '';
							writerId++;
							console.log($state.snapshot(writers));
						}
						// writers= writerStr.split(',');
						// console.log($state.snapshot(writers))
					}}
				/>
				{#if formState.inner_state == Result.Err && formState.target == 'writers'}
					<p class="error text-red-500">{formState.message}</p>
				{/if}
			</div>
			<div class="form_div">
				<label for="email">Artists</label>
				<div class="creators flex_row m-2" id="creators">
					{#each artists as a (a.id)}
						<div
							class="creator_tag secondary_bg_hover primary_txt_hover secondary_border mx-1 cursor-pointer rounded-full p-1 text-xs"
							id={`${a.id}_${a.name}`}
						>
							{a.name}
							<button
								onclick={() =>
									(artists = artists.filter((ar) => {
										return ar.id != a.id;
									}))}>x</button
							>
						</div>
					{/each}
				</div>
				<input
					type="text"
					name="artists"
					id="artists"
					bind:value={artistStr}
					oninput={(e) => {
						if (artistStr.charAt(artistStr.length - 1) == ',') {
							artists.push({ id: artistId, name: artistStr.substring(0, artistStr.length - 1) });
							artistStr = '';
							artistId++;
							console.log($state.snapshot(artists));
						}
					}}
				/>
				{#if formState.inner_state == Result.Err && formState.target == 'artists'}
					<p class="error text-red-500">{formState.message}</p>
				{/if}
			</div>

			<div class="form_div">
				<label for="creators">Creators</label>
				<div class="creators flex_row m-2" id="creators">
					{#each found_creators as s (s.username)}
						<div
							class="creator_tag main_bg_hover secondary_border mx-1 cursor-pointer rounded-full p-1 text-xs"
							id={s.username}
						>
							{s.username}
							<button
								onclick={() =>
									(found_creators = found_creators.filter((c) => {
										return c.username != s.username;
									}))}>x</button
							>
						</div>
					{/each}
				</div>
				<input
					type="search"
					name="creator_search"
					id="creator_search"
					placeholder="search for creators"
					oninput={(e) => {
						search_c = [];
						proc_c = e.target?.value;
						search_c = creators.filter((c: { username: string }) => {
							return c.username.includes(proc_c);
						});

						// console.log($state.snapshot(search_c));
					}}
				/>
				<!-- TODO: Create tags with creators names to give user ability to delete creators -->
				<div class="found_creators flex_col secondary_borderb m-auto bg-black">
					{#each search_c as s (s.username)}
						<button
							class="secondary_border_tlr main_bg_hover w-72 rounded-sm p-1"
							id={s}
							onclick={(e) => {
								e.preventDefault();
								if (!found_creators.some((c) => c.username === s.username)) {
									found_creators.push(s);
								}
							}}>{s.username}</button
						>
					{/each}
				</div>
				<!-- <button
					class="primary_btn"
					onclick={(e) => {
						e.preventDefault;
						console.log($state.snapshot(search_c));
					}}>click</button
				> -->
				{#if formState.inner_state == Result.Err && formState.target == 'artists'}
					<p class="error text-red-500">{formState.message}</p>
				{/if}
			</div>
			<button type="submit" class="btn primary_btn w-11/12">submit</button>
			<br /><br />
		</form>
	</center>
</main>
