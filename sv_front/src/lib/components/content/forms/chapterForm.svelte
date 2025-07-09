<script lang="ts">
	import { page } from "$app/stores";
	import type { ChapterForCreate } from "$lib/types/content";
	import { FormError } from "$lib/types/error";
	import { Result } from "$lib/types/result";
	import { suridToString, type SurrealId } from "$lib/types/server";
	import { updateFormState, type FormState } from "$lib/types/state/form_state";

    let {chapter_form}:{chapter_form: ChapterForCreate} = $props()

	let formState: FormState = $state({
		inner_state: Result.Ok,
		error: null,
		message: '',
		target: '',
		locked: false
	});
	async function submit(e: Event) {
		e.preventDefault();
		console.log($state.snapshot(chapter_form));
		try {
			let response = await fetch(`http://localhost:7878/content/comic/volume/chapter`, {
				method: 'POST',
				credentials: 'include',
				body: JSON.stringify(chapter_form),
				headers: {
					'Content-Type': 'application/json'
				}
			});
			if (response.status == 201) {
				//UNIMPLEMENTED
				setTimeout(() => {
					updateFormState(formState, Result.Ok, null, 'Chapter creation success', 'form', true);
					console.log($state.snapshot(formState));
				}, 3000);
				console.log('created');
				let res = await response.json();
				console.log(res);
				console.log('Uploading cover');
				upload_cover(res.chapter.id);

				console.log('Uploading file');
				upload_file(res.chapter.id);
				//window.open(`/content/comics/${volumeId}`, '_self');
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
				console.log(response);
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

	async function pf(e: Event) {
		e.preventDefault();
	}

	async function upload_file(id: SurrealId) {
		const fileInput: any = document.getElementById('comic');
		if (fileInput == null) return;

		const file = fileInput.files[0];
		console.log(file);
		if (!file) {
			alert('Please select a file to upload.');
			return;
		}

		let formData = new FormData();
		formData.append('file', file);
		console.log(formData.values);

		let response = await fetch(
			`http://localhost:7878/content/comic/volume/chapter/file/upload/${suridToString(id)}`,
			{
				method: 'POST',
				credentials: 'include',
				body: formData
			}
		);

		if (response.ok == true) {
			console.log(await response.json());
			setTimeout(() => {
				updateFormState(formState, Result.Ok, null, 'Uploading success!', 'form', true);
			}, 3000);
			console.log('Uploading file');
			//open(`/studio/${$page.params.studio}/content`)
		} else if (response.ok == false) {
			console.log('Comic upload failed');
			updateFormState(
				formState,
				Result.Err,
				FormError.UpdateFailed,
				'comic',
				'Upload failed',
				false
			);
		} else {
			console.log('Cover upload failed');
			console.log(response.ok);
			console.log(response.status);
		}
	}
	

    // Uploading cover
	async function upload_cover(id: SurrealId) {
		console.log(id)
		const fileInput: any = document.getElementById('comic_cover');
		if (fileInput == null) return;

		const file = fileInput.files[0];
		console.log(file);
		if (!file) {
			alert('Please select a file to upload.');
			return;
		}

		let formData = new FormData();
		formData.append('file', file);
		console.log(formData.values);

		let response = await fetch(`http://localhost:7878/content/comic/volume/chapter/cover/upload/${suridToString(id)}`, {
			method: 'POST',
			credentials: 'include',
			body: formData
		});

		if (response.ok == true) {
			console.log(await response.json())
			setTimeout(()=>{
				updateFormState(formState, Result.Ok, null, 'Uploading success!', 'form', true);
			}, 3000)
		} else if (response.ok == false) {
			console.log('Cover update failed');
			updateFormState(
				formState,
				Result.Err,
				FormError.UpdateFailed,
				'comic_cover',
				'Update Failed',
				false
			);
		}else{
			console.log('Cover update failed');
			console.log(response.ok);
			console.log(response.status);
		}
	}
</script>
			<form
				enctype="multipart/form-data"
				class=" flex_col secondary_border mb-9 w-full rounded-md"
				onsubmit={pf}
			>
				<h3 class="mb-4 text-base font-semibold">Select comic cover</h3>
				<input type="file" name="comic_cover" id="comic_cover" /><br />
			</form>
			<!-- Comic file input -->
			<form
				enctype="multipart/form-data"
				class=" flex_col secondary_border mb-9 w-full rounded-md"
				onsubmit={pf}
			>
				<h3 class="mb-4 text-base font-semibold">Select comic book file</h3>
				<input type="file" name="comic" id="comic" /><br />
			</form>


			<form class="form alt_bg mt-3 rounded-lg p-3 md:w-96 lg:w-5/6" onsubmit={submit}>
				<h3 class="float-left mb-4 text-3xl font-extrabold">ADD CHAPTER</h3>
				<br />
				{#if formState.inner_state == Result.Ok && formState.target == 'form'}
					<center><p class="error main_txt text-lg font-semibold">{formState.message}</p></center>
				{:else if formState.inner_state == Result.Err && formState.target == 'form'}
					<center
						><p class="error text-lg font-semibold text-red-400">{formState.message}</p></center
					>
				{/if}
				<div class="form_div">
					<label for="title">Title</label>
					<input type="text" name="title" id="title" bind:value={chapter_form.title} />
					{#if formState.inner_state == Result.Err && formState.target == 'title'}
						<p class="error text-red-500">{formState.message}</p>
					{/if}
				</div>
				<div class="form_div">
					<label for="pages">Pages</label>
					<input type="number" name="pages" id="pages" bind:value={chapter_form.pages} />
					{#if formState.inner_state == Result.Err && formState.target == 'pages'}
						<p class="error text-red-500">{formState.message}</p>
					{/if}
				</div>
				<div class="form_div">
					<label for="description">Synopsis</label>
					<!--TODO: Add word limit to description field on server side -->
					<textarea
						name="description"
						id="description"
						class="w-11/12"
						rows="10"
						bind:value={chapter_form.synopsis}
					></textarea>
				</div>
				<button type="submit" class="btn primary_btn w-11/12">submit</button>
				<!-- TODO: Next time number the volumes -->
			</form>