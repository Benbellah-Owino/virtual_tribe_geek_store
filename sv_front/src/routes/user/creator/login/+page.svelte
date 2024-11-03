<script lang="ts">
	import { clearState, updateFormState, type FormState } from '$lib/types/app_state';
	import type { Creator, CreatorForCreate } from '$lib/types/creator';
	import { FormError } from '$lib/types/error';
	import { Result } from '$lib/types/result';
	import type { UserForLogin } from '$lib/types/user';
	import { error } from '@sveltejs/kit';

	let formState: FormState = $state({
		inner_state: Result.Ok,
		error: null,
		message: '',
		target: ''
	});

	let login_form: UserForLogin = $state({	
		email: '',
		password: '',
	});

	let role: string[] = $state([]);

	function addRole() {
		// Add writer to role if writer has comma
	}

	async function register(e: Event) {
		e.preventDefault();
		console.log('submit');
		console.log(formState);
		for (const key in login_form) {
			if (login_form[key] == '') {
				updateFormState(formState, Result.Err, FormError.PasswordsDontMatch, 'Missing Field', key);
				console.log(formState.target);
				return;
			}
		}

		let response = await fetch(`http://localhost:7878/creator/login`, {
			method: 'POST',
			credentials: 'include',
			body: JSON.stringify(login_form),
			headers: {
				'Content-Type': 'application/json'
			}
		});

		if (response.status == 201) {
			//UNIMPLEMENTED
		} else if (response.status == 500) {
			error(500, 'Registraition failure');
		}
	}
</script>

<main class="page main_bg flex_col mt-5 h-full w-full">
	<h1 class="mb-7 text-center text-3xl font-extrabold">CREATOR LOGIN PAGE</h1>
	<center>
		<form class="form alt_bg w-11/12 rounded-lg p-3" onsubmit={register}>
			<h3 class="float-left mb-4 text-2xl font-bold">Login</h3>
			
			<div class="form_div">
				<label for="email">Email</label>
				<input type="email" name="email" id="email" bind:value={login_form.email} />
				{#if formState.inner_state == Result.Err && formState.target == 'email'}
					<p class="error text-red-500">{formState.message}</p>
				{/if}
			</div>
			<div class="form_div">
				<label for="password">password</label>
				<input type="password" name="password" id="password" bind:value={login_form.password} />

				{#if (formState.inner_state == Result.Err && formState.target == 'password') || formState.target == 'password&confirm_password'}
					<p class="error text-red-500">{formState.message}</p>
				{/if}
			</div>


			<button type="submit" class="btn primary_btn w-11/12">login</button>
			<br /><br />
			If you don't have an account with us
			<u class="t text-amber-700"><a href="register">click here to register with us.</a></u>
		</form>
	</center>
</main>
