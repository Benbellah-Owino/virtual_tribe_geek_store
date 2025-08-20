<script lang="ts">
	import type { VolumeForCreate } from "$lib/types/content";
	import { FormError } from "$lib/types/error";
	import { Result } from "$lib/types/result";
	import { updateFormState, type FormState } from "$lib/types/state/form_state";

    let {volume_form, comic_id}: {volume_form: VolumeForCreate, comic_id: string} = $props();

	let formState: FormState = $state({
		inner_state: Result.Ok,
		error: null,
		message: '',
		target: '',
		locked: false
	});


	async function submit(e: Event) {
		e.preventDefault();

		try {
			console.log($state.snapshot(volume_form));
			// TODO" Fix this route
			let response = await fetch(`http://localhost:7878/content/comic/volume`, {
				method: 'POST',
				credentials: 'include',
				body: JSON.stringify(volume_form),
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
				window.open(`/content/comics/${comic_id}`, '_self');
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
</script>

<form class="form alt_bg mt-3 rounded-lg p-3 md:w-96 lg:w-5/6" onsubmit={submit}>
	<h3 class="float-left mb-4 text-3xl font-extrabold">ADD VOLUME</h3>
	<br />
	{#if formState.inner_state == Result.Ok && formState.target == 'form'}
		<center><p class="error main_txt text-lg font-semibold">{formState.message}</p></center>
	{:else if formState.inner_state == Result.Err && formState.target == 'form'}
		<center><p class="error text-lg font-semibold text-red-400">{formState.message}</p></center>
	{/if}
	<div class="form_div">
		<label for="description">Synopsis</label>
		<!--TODO: Add word limit to description field on server side -->
		<textarea
			name="description"
			id="description"
			class="w-11/12"
			rows="10"
			bind:value={volume_form.synopsis}
		></textarea>
	</div>
	<button type="submit" class="btn primary_btn w-11/12">submit</button>
	<!-- TODO: Next time number the volumes -->
</form>
