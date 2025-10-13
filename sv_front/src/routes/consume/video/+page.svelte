<script lang="ts">
	import { updateFormState, type FormState } from '$lib/types/state/form_state';
	import { FormError } from '$lib/types/error';
	import { Result } from '$lib/types/result';
	import type { VideoForCreate } from '$lib/types/content';
	import { page } from '$app/stores';
	import { onMount } from 'svelte';
	import { stringToSurrealId, surrealIdToString } from '$lib/helper_functions.ts/converters';


	onMount(async () => {
	});
</script>

<main class="page main_bg flex_col mt-5 h-full w-full">
</main>
