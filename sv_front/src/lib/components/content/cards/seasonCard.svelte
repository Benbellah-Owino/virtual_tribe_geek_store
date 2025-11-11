<script lang="ts">
	import { onMount } from 'svelte';

	let {
		videoId,
		seasonId,
		season_synopsis,
		season_no
	}: { videoId: string; seasonId: string; season_synopsis: string; season_no: number } = $props();
	let episodes: any[] = $state([]);
	let episodesFetched: boolean = $state(false);
	let episodesShow: boolean = $state(false);

	async function showEpisodes() {
		if (!episodesFetched) {
            console.log("RUN")
			let res_episodes = await fetch(
				`http://localhost:7878/content/video/season/episode/${seasonId}`,
				{
					method: 'GET',
					credentials: 'include',
					headers: {
						'Content-Type': 'application/json'
					}
				}
			);

			if (res_episodes.ok == true) {
				//UNIMPLEMENTED
				let res = await res_episodes.json();
				episodes = res.episode_list;
				episodes = episodes.sort((a, b) => a.relative_episode - b.relative_episode);
				episodes.forEach((e) => {
					console.log($state.snapshot(e));
				});

				episodesFetched = true;

				// pageState.loading = false;
			} else if (res_episodes.ok == false) {
				console.error('failed');
				console.log(res_episodes);
				if (res_episodes.status == 401) {
					setTimeout(() => {
						window.open('content/login', '_self');
					}, 5000);
				} else if (res_episodes.status == 404) {
					console.error('not found');
                    episodesFetched = true
				}
			}
		}
	}
</script>

<article
	class="seasons secondary_bg_hover primary_txt_hover secondary_border m-2 cursor-default rounded p-1"
    onmouseenter={()=> {
        episodesShow = true
        showEpisodes()
        }}
    onmouseleave={()=> episodesShow = false}
>
	<li>
		<a class="tertiary_txt font-bold underline" href="/content/video/{videoId}/season/{seasonId}"
			>Season {season_no}</a
		>
	</li>
	<p class="text-sm">
		{season_synopsis}
	</p>
	<br />
	<hr />
	{#if episodesShow}
		<ul class="sn_epi_list border-t border-t-yellow-300 p-1">
			{#each episodes as episode}
				<li class="mb-1">
					<a href="/content/video/${videoId}/season/${seasonId}/episode/${episode.id.id.String} "
						>Episode {episode.relative_episode}: {episode.title}</a
					>
				</li>
			{/each}
		</ul>
	{/if}
</article>
