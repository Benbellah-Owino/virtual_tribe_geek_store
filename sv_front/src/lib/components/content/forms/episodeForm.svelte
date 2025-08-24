<script lang="ts">
	import type { EpisodeForCreate } from '$lib/types/content';
	import { Result } from '$lib/types/result';
	import type { FormState } from '$lib/types/state/form_state';

	let { episode_form }: { episode_form: EpisodeForCreate } = $props();

    let openSide = $state(false)
	let formState: FormState = $state({
		inner_state: Result.Ok,
		error: null,
		message: '',
		target: '',
		locked: false
	});

	function submit() {}
</script>

<section class="flex_row w-full bg-red-100">
	<form class="form alt_bg mt-3 rounded-lg p-3 md:w-96 lg:w-5/6" onsubmit={submit}>
		<h3>ADD EPISODE</h3>

		<br />
		{#if formState.inner_state == Result.Ok && formState.target == 'form'}
			<center><p class="error main_txt text-lg font-semibold">{formState.message}</p></center>
		{:else if formState.inner_state == Result.Err && formState.target == 'form'}
			<center><p class="error text-lg font-semibold text-red-400">{formState.message}</p></center>
		{/if}

		<div class="form_div">
			<label for="title">Title</label>
			<input type="text" name="title" id="title" bind:value={episode_form.title} />
			{#if formState.inner_state == Result.Err && formState.target == 'title'}
				<p class="error text-red-500">{formState.message}</p>
			{/if}
		</div>

        <!-- -->
		<div class="form_div">
			<label for="runlength">Episode Run length</label>
			<p class="text-xs">Hours</p>
			<input
				type="number"
				name="hrs"
				id="hrs"
				placeholder="enter hours"
				bind:value={episode_form.runlength.hours}
			/>

			<p class="text-xs">Minutes</p>
			<input
				type="number"
				name="mins"
				id="mins"
				placeholder="enter minutes"
				bind:value={episode_form.runlength.minutes}
			/>

			<p class="text-xs">Seconds</p>
			<input
				type="number"
				name="secs"
				id="secs"
				placeholder="enter seconds"
				bind:value={episode_form.runlength.seconds}
			/>
			{#if (formState.inner_state == Result.Err && formState.target == 'runlength') || formState.target == 'runlength&confirm_runlength'}
				<p class="error text-red-500">{formState.message}</p>
			{/if}
		</div>
	</form>
<!-- TODO: Match the frontend form episode to backend one -->
    {#if openSide}
        <form>
            <h3>Enter the point where the opening</h3>
        </form>
    {/if}
</section>
