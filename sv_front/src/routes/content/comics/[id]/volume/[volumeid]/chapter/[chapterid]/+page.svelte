<script lang="ts">
	import { page } from "$app/stores";
	import { Result } from "$lib/types/result";
	import type { FormState } from "$lib/types/state/form_state";
	import type { PageState } from "$lib/types/state/page_state";
	import { onMount } from "svelte";


	// section:     --- State
	let pageState: PageState = $state({
		inner_state: Result.Ok,
		error: null,
		loading: true,
		message: ''
	});

	let formState: FormState = $state({
		inner_state: Result.Ok,
		error: null,
		message: '',
		target: '',
		locked: false
	});

	let chapterId = $page.params.chapterid;
	let formOn = $state(false);

    let chapter;

    onMount(async()=>{
        
    })
	// endsection:  --- State

    //TODO: Retrieve chapter
</script>