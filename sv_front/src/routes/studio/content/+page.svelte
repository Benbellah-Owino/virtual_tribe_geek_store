<script lang="ts">
	import type { Content } from "$lib/types/content";
	import { onMount } from "svelte";

    let contents: Content[] = $state([]);


	onMount(async () => {
		let response = await fetch(`http://localhost:7878/content/studio:2xq3qx3qhay7ada22wna`, {
			method: 'GET',
			credentials: 'include',
			headers: {
				'Content-Type': 'application/json'
			}
		});

		if (response.ok == true) {
			let res = await response.json();
            console.log(res.content_list)
            contents = res.content_list

		} else if (response.ok == false) {
		}
	});
</script>

<main class="page">
    <section class="content_list" id="content_list">
        {#each contents as content }
            <h1>{content.title}</h1>
        {/each}

        <a href="/content/create/studio:2xq3qx3qhay7ada22wna" class="main_border rounded">Add content</a>
    </section>
</main>