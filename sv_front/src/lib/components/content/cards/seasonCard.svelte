<script lang="ts">
	import { onMount } from 'svelte';

	let {
		videoId,
		seasonId,
		season_synopsis,
		season_no,
		path
	}: { videoId: string; seasonId: string; season_synopsis: string; season_no: number, path:string } = $props();
	let episodes: any[] = $state([]);
	let episodesFetched: boolean = $state(false);
	let episodesShow: boolean = $state(false);

	onMount(showEpisodes)
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
	class="seasons secondary_bg_hover primary_txt_hover secondary_borderb p-2 pb-4 cursor-default"
>
	<li>
		<a class="tertiary_txt font-bold text-2xl underline" href="/content/video/{videoId}/season/{seasonId}"
			>Season {season_no}</a
		>
	</li>
	<p class="text-sm mt-2">
		{season_synopsis}
	</p>
	<br />
	<hr />
		<ul class="sn_epi_list  p-1">
			<h5 class="text-xs underline">Episodes</h5>
			{#each episodes as episode}
				<li class="mb-1">
					<a href="/{path}/video/{videoId}/season/{seasonId}/episode/{episode.id.id.String} "
						>Episode {episode.relative_episode}: {episode.title}</a
					>
				</li>
			{/each}
		</ul>
</article>
