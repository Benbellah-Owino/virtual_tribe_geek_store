<script lang="ts">
	import { clearState, updateFormState, type FormState } from '$lib/types/state/form_state';
	import type { StudioForCreate } from '$lib/types/studio';
	import { FormError } from '$lib/types/error';
	import { Result } from '$lib/types/result';

	let formState: FormState = $state({
		inner_state: Result.Ok,
		error: null,
		message: '',
		target: '',
		locked: false
	});

	let studio_form: StudioForCreate = $state({
		name: '',
		email: '',
		description: '',
        owner: ''
	});

	async function submit(e: Event) {
		e.preventDefault();
		console.log('submit');

		for (const key in studio_form) {
			if (studio_form[key] == '' && key!='owner') {
				updateFormState(
					formState,
					Result.Err,
					FormError.PasswordsDontMatch,
					'Missing Field',
					key,
					false
				);
				console.log(formState.target);
				return;
			}
		}

		console.log($state.snapshot(studio_form));

		try {
			let response = await fetch(`http://localhost:7878/studio`, {
				method: 'POST',
				credentials: 'include',
				body: JSON.stringify(studio_form),
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
				console.log('created')
				window.open('/user/creator/studios','_self');
			} else if (response.status == 500) {
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
		console.log('hey');
	}
</script>

<main class="page main_bg flex_col mt-5 h-full w-full">
	<h1 class="mb-7 text-center text-3xl font-extrabold">STUDIO CREATION PAGE</h1>
	<center>
		<form class="form alt_bg md:w-96 rounded-lg p-3" onsubmit={submit}>
			<h3 class="float-left mb-4 text-3xl font-extrabold">CREATE STUDIO</h3>
			<br />
			{#if formState.inner_state == Result.Ok && formState.target == 'form'}
				<center><p class="error main_txt text-lg font-semibold">{formState.message}</p></center>
			{:else if formState.inner_state == Result.Err && formState.target == 'form'}
				<center><p class="error text-red-400 text-lg font-semibold">{formState.message}</p></center>
			{/if}
			<div class="form_div">
				<label for="name">name</label>
				<input type="text" name="name" id="name" bind:value={studio_form.name} />
				{#if formState.inner_state == Result.Err && formState.target == 'name'}
					<p class="error text-red-500">{formState.message}</p>
				{/if}
			</div>
			<div class="form_div">
				<label for="email">Email</label>
				<input type="email" name="email" id="email" bind:value={studio_form.email} />
				{#if formState.inner_state == Result.Err && formState.target == 'email'}
					<p class="error text-red-500">{formState.message}</p>
				{/if}
			</div>
			<div class="form_div">
				<label for="description">Description</label>
				<!--TODO: Add word limit to description field on server side -->
				<textarea
					name="description"
					id="description"
					class="w-11/12"
					rows="10"
					bind:value={studio_form.description}
				></textarea>
			</div>

			<button type="submit" class="btn primary_btn w-11/12">submit</button>
			<br /><br />
		</form>
	</center>
</main>
