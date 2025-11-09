<script lang="ts">
    import { updateFormState, type FormState } from '$lib/types/state/form_state';
    import { FormError } from '$lib/types/error';
    import { Result } from '$lib/types/result';
    import type { VideoForCreate } from '$lib/types/content';
    import { page } from '$app/stores';
    import { onMount } from 'svelte';
    import { stringToSurrealId, surrealIdToString } from '$lib/helper_functions.ts/converters';
    import type { PageState } from '$lib/types/state/page_state';
    // import { ComicType } from '$lib/types/state/video_type';

    let videos: any[] = $state([]);
    // section:     --- State
    let pageState: PageState = $state({
        inner_state: Result.Ok,
        error: null,
        loading: true,
        message: ''
    });

    onMount(async () => {
            let response = await fetch(`http://localhost:7878/content/video`, {
                method: 'GET',
                credentials: 'include',
                headers: {
                    'Content-Type': 'application/json'
                }
            });

            if (response.ok == true){
                let res =  await response.json()

                console.log(res)
                videos = res.video_list

                console.log($state.snapshot(videos))
				pageState.loading = false;
            }

    });
</script>

<main class="page main_bg flex_col mt-5 h-full w-full">

    {#if pageState.loading}
        <center>Loading content...</center>
    {:else if pageState.loading == false && pageState.inner_state == Result.Ok && pageState.error == null}
        <!--COMIC ASPECT RATIO IS 2:3  -->
    <section class="videoList w-full flex flex-col justify-center items-center md:flex-start md:flex-row">
            {#each videos as video}
                <article
                    class="videoTile secondary_shadow_hover main_txt flex_col m-4 bg-black"
                    id="videoTile{video?.id.id.String}"
                >
                    <a
                        href="/content/{surrealIdToString(video.content.id)}"
                        target="_blank"
                        rel="noopener noreferrer"
                    >
                        <img
                            src={video.content.cover
                                ? `http://localhost:7878/content/image/${video.content.cover}`
                                : ''}
                            alt="Cover for video"
                            class="cover object-cover"
                        />
                    </a>

                    <a href="/studio/{surrealIdToString(video.content.studio.id)}" target="_blank">
                        <div
                            class="studioPill secondary_bg primary_txt w-fit rounded-2xl p-1 text-sm font-semibold"
                        >
                            {video.content.studio?.name}
                        </div>
                    </a>

                    <a
                        target="_blank"
                        href="/consume/{surrealIdToString(video.content.id)}"
                        class="videoTitle text-xl font-bold"
                    >
                        {`${video.content.title}(${video.created_at.year})`}
                    </a>

                    <p class="videoDescription max-h-28 w-full cursor-default overflow-hidden p-2 text-sm">
                        {video.content.description}
                    </p>
                </article>
            {/each}
        </section>
    {/if}
</main>

<style>
    /* Tablets */
    .videoTile {
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

    .videoDescription {
        overflow: hidden;
        white-space: nowrap;
        text-overflow: ellipsis;
    }

    /* Tablets */
    @media (min-width: 768px) {
        .videoTile {
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

        .videoDescription {
            white-space: inherit;
            display: -webkit-box;
            -webkit-line-clamp: 5; /* Number of lines */
            -webkit-box-orient: vertical;
            overflow: hidden;
            text-overflow: ellipsis;
        }
    }
</style>
