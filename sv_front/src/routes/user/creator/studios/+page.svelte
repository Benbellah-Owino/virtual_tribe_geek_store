
<script lang="ts">
	import { page } from "$app/stores";
	import StudioListCard from "$lib/components/studio/cards/StudioListCard.svelte";
	import { PageError } from "$lib/types/error";
	import { Result } from "$lib/types/result";
	import { updatePageState, type PageState } from "$lib/types/state/page_state";
	import type { Studio } from "$lib/types/studio";
	import { onMount } from "svelte";


let studios: Studio[] = $state([])

let pageState: PageState = $state({
	inner_state: Result.Ok,
	error: null,
	loading: true,
	message: ""
})
	onMount(async () => {
        //TODO: Add creator ID to browser
		let response = await fetch(`http://localhost:7878/creator/studio/owner`, {
			method: 'GET',
			credentials: 'include',
			headers: {
				'Content-Type': 'application/json'
			}
		});

		if (response.ok == true) {
			//UNIMPLEMENTED
			let res = await response.json();
			let db_studios = res.studios;
			console.log(res);
			studios = db_studios;
			pageState.loading = false;
		} else if (response.ok == false) {
			if(response.status == 401){
				updatePageState(pageState, Result.Err,PageError.Unauthorized, false, 'You are not authorized! Redirecting you to login page...',);
				setTimeout(()=>{
				window.open('studios/login', '_self')
				}, 5000)
			}
		}
	});
</script>

<main class="page">
    {#if pageState.loading}
        <center>Loading studios...</center>
    {:else if pageState.loading == false && pageState.inner_state == Result.Ok}
        <section class="studios flex_col w-full h-fit mt-2 ">
            {#each studios as studio }
                <StudioListCard {studio}/>
            {/each}
        </section>
    {:else if pageState.inner_state == Result.Err}
        <h3 class="error">{pageState.message}</h3>
    {/if}
</main>