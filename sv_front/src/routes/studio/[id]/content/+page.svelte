<script lang="ts">
	import { page } from "$app/stores";
	import { surrealIdToString } from "$lib/helper_functions.ts/converters";
	import type { Content } from "$lib/types/content";
	import { onMount } from "svelte";

    let contents: Content[] = $state([]);

	let studio_id = $page.params.id;

	console.log(studio_id)
	
	onMount(async () => {
		let response = await fetch(`http://localhost:7878/content/studio/${studio_id}`, {
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
			console.error("Content not found");
		}
	});
</script>

<main class="page">

	<h1 class="mb-7 mt-4 text-center text-3xl font-extrabold">Studio Content list</h1>
    <section class="content_list" id="content_list">
		<table class=" table_border tbl_txt w-11/12 mx-auto p-0 table-auto border-separate border-spacing-0 rounded text-xs">
			<thead>
				<tr class="text-center">
					<th>Title</th>
					<th>Recommended price</th>
					<th>Rating</th>
					<th></th>
				</tr>
			</thead>
			<tbody>
				{#each contents as content }
					<tr class="md:p-1">
						<td class=" md:p-2 text-left">{content.title}</td>
						<td class=" md:p-2 text-left">ksh {content.recom_price}</td>
						<td class=" md:p-2 text-left">{content.rating}</td>
						<td class="md:p-2 text-center">
            				<a class="primary_txt secondary_bg tertiary_bg_hover md:p-1 m-auto w-fit md:w-14 border border-yellow-300 rounded-xl text-center text-sm font-semibold md:font-bold"
							target="_blank" href="/content/{surrealIdToString(content.id)}">view</a><br>
						</td>

					</tr>
				{/each}
			</tbody>
		</table>
		<div class="back_btn p-3 w-full flex_center mt-5">
			<a class="  primary_txt_hover secondary_bg_hover p-1  border border-yellow-300 rounded-xl text-center font-semibold "
			href="/content/create/{studio_id}"
			>Add Content</a>
		</div>
    </section>
</main>