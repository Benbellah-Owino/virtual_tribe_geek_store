<script lang="ts">
	import { updateFormState, type FormState } from '$lib/types/state/form_state';
	import { FormError } from '$lib/types/error';
	import { Result } from '$lib/types/result';
	import type { ComicForCreate } from '$lib/types/content';
	import { page } from '$app/stores';

	let formState: FormState = $state({
		inner_state: Result.Ok,
		error: null,
		message: '',
		target: '',
		locked: false
	});

	let comic_form: ComicForCreate = $state({
		content: $page.params.content,
		writer: [],
		artist: [],
		creator: []
	});

	async function submit(e: Event) {
		e.preventDefault();
		console.log('submit');
		console.log($state.snapshot(comic_form))
	}

	// async function submit(e: Event) {
	// 	e.preventDefault();
	// 	console.log('submit');
	// 	let content = $page.params.content;
	// 	console.log(content)
	// 	for (const key in comic_form) {
	// 		if (Array.isArray(comic_form[key])) { //Check if field is an array
	// 			if (comic_form[key].length == 0) { //Check if array is empty
	// 				updateFormState(
	// 					formState,
	// 					Result.Err,
	// 					FormError.PasswordsDontMatch,
	// 					'Missing Field',
	// 					key,
	// 					false
	// 				);
	// 				console.log(formState.target);
	// 				return;
	// 			}
	// 		}
	// 	}

	// 	try {
	// 		// TODO" Fix this route
	// 		let response = await fetch(`http://localhost:7878/content/comic`, {
	// 			method: 'POST',
	// 			credentials: 'include',
	// 			body: JSON.stringify(comic_form),
	// 			headers: {
	// 				'Content-Type': 'application/json'
	// 			}
	// 		});
	// 		if (response.status == 201) {
	// 			//UNIMPLEMENTED
	// 			setTimeout(() => {
	// 				updateFormState(formState, Result.Ok, null, 'Registration success', 'form', true);
	// 				console.log($state.snapshot(formState));
	// 			}, 3000);
	// 			console.log('created');
	// 			window.open('/user/creator/studios', '_self');
	// 		} else if (response.status == 500) {
	// 			updateFormState(
	// 				formState,
	// 				Result.Err,
	// 				FormError.SubmissionFailed,
	// 				'Submission Failed',
	// 				'form',
	// 				false
	// 			);
	// 		}
	// 	} catch (error) {
	// 		console.error(error);
	// 		updateFormState(
	// 			formState,
	// 			Result.Err,
	// 			FormError.SubmissionFailed,
	// 			'Submission Failed',
	// 			'form',
	// 			false
	// 		);
	// 	}
	// 	console.log('hey');
	// }

	function pf(e: Event) {
		e.preventDefault();
	}
</script>

<main class="page main_bg flex_col mt-5 h-full w-full">
	<h1 class="mb-7 text-center text-3xl font-extrabold">STUDIO CREATION PAGE</h1>
	<center>
		<form
			enctype="multipart/form-data"
			class=" flex_col secondary_border mb-9 w-full rounded-md"
			onsubmit={pf}
		>
			<input type="file" name="cover" id="cover" /><br />
		</form>
		<form class="form alt_bg rounded-lg p-3 md:w-96" onsubmit={submit}>
			<h3 class="float-left mb-4 text-3xl font-extrabold">CREATE STUDIO</h3>
			<br />
			{#if formState.inner_state == Result.Ok && formState.target == 'form'}
				<center><p class="error main_txt text-lg font-semibold">{formState.message}</p></center>
			{:else if formState.inner_state == Result.Err && formState.target == 'form'}
				<center><p class="error text-lg font-semibold text-red-400">{formState.message}</p></center>
			{/if}
			<div class="form_div">
				<label for="name">Writers</label>
				<input type="text" name="writers" id="writers" bind:value={comic_form.writer} />
				{#if formState.inner_state == Result.Err && formState.target == 'writers'}
					<p class="error text-red-500">{formState.message}</p>
				{/if}
			</div>
			<div class="form_div">
				<label for="email">Artists</label>
				<input type="text" name="artists" id="artists" bind:value={comic_form.artist} />
				{#if formState.inner_state == Result.Err && formState.target == 'artists'}
					<p class="error text-red-500">{formState.message}</p>
				{/if}
			</div>

			<button type="submit" class="btn primary_btn w-11/12">submit</button>
			<br /><br />
		</form>
	</center>
</main>
