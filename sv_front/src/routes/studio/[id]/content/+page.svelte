<script lang="ts">
	import { page } from "$app/stores";
	import type { Content } from "$lib/types/content";
	import { onMount } from "svelte";

    let contents: Content[] = $state([]);

	let studio_id = $page.params.id;

	console.log(studio_id)
	onMount(async () => {
		let response = await fetch(`http://localhost:7878/content/${studio_id}`, {
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

        <a href="/content/create/{studio_id}" class="main_border rounded">Add content</a>
    </section>
</main>