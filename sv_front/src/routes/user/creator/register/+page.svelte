<script lang="ts">
	import { clearState, updateFormState, type FormState } from '$lib/types/state/form_state';
	import type { CreatorForCreate } from '$lib/types/creator';
	import { FormError } from '$lib/types/error';
	import { Result } from '$lib/types/result';
	import { error } from '@sveltejs/kit';

	let formState: FormState = $state({
		inner_state: Result.Ok,
		error: null,
		message: '',
		target: '',
		locked: false
	});

	let register_form: CreatorForCreate = $state({
		role: 'writer',
		username: '',
		email: '',
		password: '',
		confirm_password: ''
	});

	let role: string[] = $state([]);

	function addRole() {
		// Add writer to role if writer has comma
	}

	function checkPasswordMatch() {
		if (register_form.password != register_form.confirm_password) {
			console.log('err');
			updateFormState(
				formState,
				Result.Err,
				FormError.PasswordsDontMatch,
				"Passwords don't match",
				'password&confirm_password',
				false
			);
		} else {
			clearState(formState);
		}
	}

	async function register(e: Event) {
		e.preventDefault();
		console.log('submit');

		checkPasswordMatch();
		for (const key in register_form) {
			if (register_form[key] == '') {
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

		try {
			let response = await fetch(`http://localhost:7878/creator`, {
				method: 'POST',
				credentials: 'include',
				body: JSON.stringify(register_form),
				headers: {
					'Content-Type': 'application/json'
				}
			});

			if (response.status == 201) {
				//UNIMPLEMENTED
				setTimeout(() => {
					updateFormState(formState, Result.Ok, null, 'Registration success', 'form', true);
					console.log($state.snapshot(formState));
					window.open('/user/creator/login', '_self');
				}, 3000);
			} else if (response.status == 500) {
				console.error("Failed");
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

<main class="page main_bg mt-5 h-full w-full">
	<h1 class="mb-7 text-center text-3xl font-extrabold">CREATOR REGISTER PAGE</h1>
	<center>
		<form class="form alt_bg w-8/12 rounded-lg p-3 flex_col" onsubmit={register}>
			<h3 class="float-left mb-4 text-2xl font-bold">Register</h3>
			<br />
			{#if formState.inner_state == Result.Ok && formState.target == 'form'}
				<center><p class="error main_txt text-lg font-semibold">{formState.message}</p></center>
			{:else if formState.inner_state == Result.Err && formState.target == 'form'}
				<center><p class="error text-red-400 text-lg font-semibold">{formState.message}</p></center>
			{/if}
			<div class="form_div">
				<label for="username">Username</label>
				<input type="text" name="username" id="username" bind:value={register_form.username} />
				{#if formState.inner_state == Result.Err && formState.target == 'username'}
					<p class="error text-red-500">{formState.message}</p>
				{/if}
			</div>
			<div class="form_div">
				<label for="email">Email</label>
				<input type="email" name="email" id="email" bind:value={register_form.email} />
				{#if formState.inner_state == Result.Err && formState.target == 'email'}
					<p class="error text-red-500">{formState.message}</p>
				{/if}
			</div>
			<div class="form_div">
				<label for="password">password</label>
				<input type="password" name="password" id="password" bind:value={register_form.password} />

				{#if (formState.inner_state == Result.Err && formState.target == 'password') || formState.target == 'password&confirm_password'}
					<p class="error text-red-500">{formState.message}</p>
				{/if}
			</div>
			<div class="form_div">
				<label for="confirm_password">confirm_password</label>
				<input
					type="password"
					name="confirm_password"
					id="confirm_password"
					bind:value={register_form.confirm_password}
				/>
				{#if (formState.inner_state == Result.Err && formState.target == 'confirm_password') || formState.target == 'password&confirm_password'}
					{console.log(formState.message)}
					<p class="error text-red-500">{formState.message}</p>
				{/if}
			</div>

			<button type="submit" class="btn primary_btn w-11/12">submit</button>
			<br /><br />
			If you already have an account with us
			<u class="t text-amber-700"><a href="login">click here to login.</a></u>
		</form>
	</center>
</main>
