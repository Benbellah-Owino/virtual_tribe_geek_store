<script lang="ts">
	import { page } from "$app/stores";
    
	let seasonId = $page.params.season_id;
	let videoId = $page.params.video_id;
</script>

<main class="page">
	<a
		class="tertiary_txt secondary_txt_hover font-bold underline"
		href="/content/video/{videoId}/season/{seasonId}/episode"
		>Add Episode</a
	>
</main>
