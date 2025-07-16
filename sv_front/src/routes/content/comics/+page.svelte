<script lang="ts">
	import { surrealIdToString } from '$lib/helper_functions.ts/converters';
	import { getChartDatesObject } from '$lib/helper_functions.ts/date_time';
	import { Result } from '$lib/types/result';
	import type { PageState } from '$lib/types/state/page_state';
	import { onMount } from 'svelte';

	let comics: any[] = $state([]);
	// section:     --- State
	let pageState: PageState = $state({
		inner_state: Result.Ok,
		error: null,
		loading: true,
		message: ''
	});

	let form_on = $state(false);
	// endsection:  --- State

	onMount(async () => {
		try {
			let response = await fetch(`http://localhost:7878/content/comic`, {
				method: 'GET',
				credentials: 'include',
				headers: {
					'Content-Type': 'application/json'
				}
			});

			if (response.ok == true) {
				//UNIMPLEMENTED
				let res = await response.json();
				//console.log($state.snapshot(res.comic_list));
				comics = res.comic_list;

				pageState.loading = false;
				// comics.forEach((comic) => {
				// 	console.log($state.snapshot(comic));
				// });
				comics = comics.map((comic) => {
					comic.created_at = getChartDatesObject(comic.created_at);
					return comic;
				});
				console.log($state.snapshot(comics));
			} else if (response.ok == false) {
				console.error('not found');
			}
		} catch (error) {}
	});
</script>

<main class="page">
	{#if pageState.loading}
		<center>Loading content...</center>
	{:else if pageState.loading == false && pageState.inner_state == Result.Ok && pageState.error == null}
		<!--COMIC ASPECT RATIO IS 2:3  -->
	<section class="comicList w-full flex flex-col justify-center items-center md:flex-start md:flex-row">
			{#each comics as comic}
				<article
					class="comicTile secondary_shadow_hover main_txt flex_col m-4 bg-black"
					id="comicTile{comic?.id.id.String}"
				>
					<a
						href="/content/{surrealIdToString(comic.content.id)}"
						target="_blank"
						rel="noopener noreferrer"
					>
						<img
							src={comic.content.cover
								? `http://localhost:7878/content/image/${comic.content.cover}`
								: ''}
							alt="Cover for comic"
							class="cover object-cover"
						/>
					</a>

					<a href="/studio/{surrealIdToString(comic.content.studio.id)}" target="_blank">
						<div
							class="studioPill secondary_bg primary_txt w-fit rounded-2xl p-1 text-sm font-semibold"
						>
							{comic.content.studio?.name}
						</div>
					</a>

					<a
						target="_blank"
						href="/content/{surrealIdToString(comic.content.id)}"
						class="comicTitle text-xl font-bold"
					>
						{`${comic.content.title}(${comic.created_at.year})`}
					</a>

					<p class="comicDescription max-h-28 w-full cursor-default overflow-hidden p-2 text-sm">
						{comic.content.description}
					</p>
				</article>
			{/each}
		</section>
	{/if}
</main>

<style>
	/* Tablets */
	.comicTile {
		height: 38rem;
		width: 22rem;
	}

	.cover {
		object-fit: cover;
		height: 33rem;
		width: 22rem;
	}

	.studioPill {
		margin-top: -16px;
	}

	.comicDescription {
		overflow: hidden;
		white-space: nowrap;
		text-overflow: ellipsis;
	}

	/* Tablets */
	@media (min-width: 768px) {
		.comicTile {
			height: 50rem;
			width: 24rem;
		}

		.cover {
			object-fit: cover;
			height: 36rem;
			width: 24rem;
		}

		.studioPill {
			margin-top: -16px;
		}

		.comicDescription {
			white-space: inherit;
			display: -webkit-box;
			-webkit-line-clamp: 5; /* Number of lines */
			-webkit-box-orient: vertical;
			overflow: hidden;
			text-overflow: ellipsis;
		}
	}
</style>
