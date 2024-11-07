<script lang="ts">
	import { clearState, updateFormState, type FormState } from '$lib/types/state/form_state';
	import type { Creator, CreatorForCreate } from '$lib/types/creator';
	import { FormError, type FieldError } from '$lib/types/error';
	import { Result } from '$lib/types/result';
	import { error } from '@sveltejs/kit';
	import { onMount } from 'svelte';
	import type { PageState } from '$lib/types/state/page_state';
	import stockProfilePic from '$lib/assets/stock_pp.png';
	let pageState: PageState = $state({
		inner_state: Result.Ok,
		error: null,
		loading: true,
		message: 'Loading...'
	});

	let creator: Creator = $state({
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
	});

	let creator2: Creator;
	let formState: FormState = {
		inner_state: Result.Ok,
		error: null,
		message: '',
		target: '',
		locked: false
	};
	let socials: string[] = [];
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

			console.log(res.creator);
			if (res.creator.socials == null) {
				res.creator.socials = { facebook: '', twitter: '', instagram: '' };
			}
			let db_creator = res.creator;

			console.log(db_creator);
			creator = db_creator;
			creator2 = db_creator;
			pageState.loading = false;
		} else if (response.ok == false) {
		}
	});
	let errors: FieldError[] = $state([]);
	async function register(e: Event) {
		for (const key in creator) {
			// if (creator[key] == '') {
			// 	updateFormState(formState, Result.Err, FormError.PasswordsDontMatch, 'Missing Field', key, false);
			// 	console.log(formState.target);
			// 	return;
			// }
			if (creator[key] != creator2[key]) {
				let field = key;
				let value = creator[key];

				if (key == 'socials') {
					console.log('social');
					for (const sk in creator.socials) {
						if (creator.socials[sk] == null) {
							continue;
						}
						if (creator.socials[sk] != creator2[sk]) {
							field = `socials`;
							value = `${sk};${creator.socials[sk]}`;
						}
						console.log(`{\n${field}: ${value}\n}`);
					}
				}
				let response = await fetch(`http://localhost:7878/creator`, {
					method: 'PATCH',
					credentials: 'include',

					body: JSON.stringify({ field, value }),
					headers: {
						'Content-Type': 'application/json'
					}
				});

				if (response.status == 201) {
					console.log(creator[key] + ' updated');
				} else if (response.status == 500) {
					console.error(creator[key] + ' update error');
					updateFormState(
						formState,
						Result.Err,
						FormError.UpdateFailed,
						key,
						'Update Failed',
						false
					);
					errors.push({ field: key, message: 'Update Failed' });
				}
			}
		}
	}

	async function upload_avatar(e:any) {
		const fileInput: any = document.getElementById('avatar');
		if (fileInput == null) return;

		const file = fileInput.files[0];
		console.log(file)
		if (!file) {
			alert('Please select a file to upload.');
			return;
		}

		let formData = new FormData();
		formData.append('file', file);
		console.log(formData.values);


		let response = await fetch(`http://localhost:7878/creator/upload/${creator.email}`, {
			method: 'PATCH',
			credentials: 'include',
			body: formData,
		});

		if (response.status == 201) {
			console.log('updloaded');
		} else if (response.status == 500) {
			updateFormState(
				formState,
				Result.Err,
				FormError.UpdateFailed,
				'avatar',
				'Update Failed',
				false
			);
			errors.push({ field: 'avatar', message: 'Update Failed' });
		}
	}
</script>

<main class="page main_bg flex_col mt-5 h-full w-full">
	<form
		enctype="multipart/form-data"
		class=" flex_col secondary_border mb-9 w-full rounded-md"
		onsubmit={upload_avatar}
	>
		<img
			src={creator.avatar ? `http://localhost:7878/creator/image/${creator.avatar}` : stockProfilePic}
			alt="Profile picture of {creator.username}"
			width="128px"
			height="128px"
			class="profile secondary_border mx-auto mb-2 rounded-full"
		/>
		<input type="file" name="avatar" id="avatar" /><br />
		<button type="submit" class="btn primary_btn">upload photo</button>
	</form>
	<h1 class="mb-7 text-center text-3xl font-extrabold">CREATOR REGISTER PAGE</h1>
	<center>
		<form class=" form alt_bg w-11/12 rounded-lg p-3" onsubmit={register}>
			<h3 class="float-left mb-4 text-2xl font-bold">Register</h3>
			{#if formState.inner_state == Result.Err}
				{#each errors as e}
					<p class="error text-red-500">{e.field} {e.message}</p>
				{/each}
			{/if}
			<br />
			<div class="form_div">
				<label for="username">Username</label>
				<input type="text" name="username" id="username" bind:value={creator.username} />
				{#if formState.inner_state == Result.Err && formState.target == 'username'}
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
					bind:value={creator.description}
				></textarea>
			</div>
			<article class="socials primary_border main_shadow">
				<div class="form_div">
					<label for="facebook">Facebook</label>
					<input type="text" name=" facebook" id="facebook" bind:value={creator.socials.facebook} />
				</div>
				<div class="form_div">
					<label for="instagram">Instagram</label>
					<input
						type="text"
						name="instagram"
						id="instagram"
						bind:value={creator.socials.instagram}
					/>
				</div>
				<div class="form_div">
					<label for="twitter">Twitter/X</label>
					<input type="text" name="twitter" id="twitter" bind:value={creator.socials.twitter} />
				</div>
			</article>
			<br />
			<button type="submit" class="btn primary_btn w-11/12">submit</button>
			<br /><br />
			If you already have an account with us
			<u class="t text-amber-700"><a href="login">click here to login.</a></u>
		</form>
	</center>
</main>
