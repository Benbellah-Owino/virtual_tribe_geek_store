<script lang="ts">
    import type { EpisodeForCreate } from '$lib/types/content';
    import { FormError } from '$lib/types/error';
    import { Result } from '$lib/types/result';
    import { updateFormState, type FormState } from '$lib/types/state/form_state';
    import { fly } from 'svelte/transition';

    let { episode_form }: { episode_form: EpisodeForCreate } = $props();

    let openIntro = $state(false);
    let openClose = $state(false);
    let formState: FormState = $state({
        inner_state: Result.Ok,
        error: null,
        message: '',
        target: '',
        locked: false
    });

    async function submit(e: Event) {
        e.preventDefault();
        console.log($state.snapshot(episode_form));
        try {
            let response = await fetch(`http://localhost:7878/content/video/season/episode`, {
                method: 'POST',
                credentials: 'include',
                body: JSON.stringify(episode_form),
                headers: {
                    'Content-Type': 'application/json'
                }
            });
            if (response.status == 201) {
                //UNIMPLEMENTED
                setTimeout(() => {
                    updateFormState(formState, Result.Ok, null, 'Chapter creation success', 'form', true);
                    console.log($state.snapshot(formState));
                }, 3000);
                console.log('created');
                let res = await response.json();
                console.log(res);
                // console.log('Uploading cover');
                // upload_cover(res.episode.id);

                // console.log('Uploading file');
                // upload_file(res.episode.id);
                //window.open(`/content/videos/${seasonId}`, '_self');
            } else if (response.status == 500) {
                updateFormState(
                    formState,
                    Result.Err,
                    FormError.SubmissionFailed,
                    'Submission Failed',
                    'form',
                    false
                );
            } else if (response.ok == false) {
                console.log(response.statusText);
                console.log(response);
                updateFormState(
                    formState,
                    Result.Err,
                    FormError.SubmissionFailed,
                    'Submission Failed',
                    'form',
                    false
                );
            }
        } catch (error) {
            console.error(error);
            updateFormState(
                formState,
                Result.Err,
                FormError.SubmissionFailed,
                'Submission Failed',
                'form',
                false
            );
        }
    }
</script>

<section class="flex_center w-full">
    <form class="form alt_bg m-3 rounded-lg p-3 md:w-96 lg:w-5/6" onsubmit={submit}>
        <h3>ADD EPISODE</h3>

        <br />
        {#if formState.inner_state == Result.Ok && formState.target == 'form'}
            <center><p class="error main_txt text-lg font-semibold">{formState.message}</p></center>
        {:else if formState.inner_state == Result.Err && formState.target == 'form'}
            <center><p class="error text-lg font-semibold text-red-400">{formState.message}</p></center>
        {/if}

        <div class="form_div">
            <label for="title">Title</label>
            <input type="text" name="title" id="title" bind:value={episode_form.title} />
            {#if formState.inner_state == Result.Err && formState.target == 'title'}
                <p class="error text-red-500">{formState.message}</p>
            {/if}
        </div>

        <!-- TODO: Force the times to be 2 digits-->
        <div class="form_div">
            <label for="runlength">Episode Run length</label>
            <p class="text-xs">Hours</p>
            <input
                type="number"
                name="hrs"
                id="hrs"
                placeholder="enter hours"
                bind:value={episode_form.runlength.hours}
            />

            <p class="text-xs">Minutes</p>
            <input
                type="number"
                name="mins"
                id="mins"
                placeholder="enter minutes"
                bind:value={episode_form.runlength.minutes}
            />

            <p class="text-xs">Seconds</p>
            <input
                type="number"
                name="secs"
                id="secs"
                placeholder="enter seconds"
                bind:value={episode_form.runlength.seconds}
            />
            {#if (formState.inner_state == Result.Err && formState.target == 'runlength') || formState.target == 'runlength&confirm_runlength'}
                <p class="error text-red-500">{formState.message}</p>
            {/if}
        </div>

        <div class="form_div">
            <label for="opening_length">Intro timestamp</label>
            <p>
                <b>Start : </b> &nbsp;
                {('0' + episode_form.opening_length.start.hours).slice(-2)} :
                {('0' + episode_form.opening_length.start.minutes).slice(-2)} :
                {('0' + episode_form.opening_length.start.seconds).slice(-2)}
            </p>
            <p>
                <b>End&nbsp; &nbsp;: </b> &nbsp;
                {('0' + episode_form.opening_length.end.hours).slice(-2)} :
                {('0' + episode_form.opening_length.end.minutes).slice(-2)} :
                {('0' + episode_form.opening_length.end.seconds).slice(-2)}
            </p>
            <button class="btn skip_times mt-1" onclick={() => (openIntro = !openIntro)}>
                Edit intro timestamps</button
            >
        </div>
        <hr class="secondary_bg secondary_txt" />

        <div class="form_div">
            <label for="closing_length">Credits timestamp</label>
            <p>
                <b>Start : </b> &nbsp;
                {episode_form.closing_length.start.hours} :
                {episode_form.closing_length.start.minutes} :
                {episode_form.closing_length.start.seconds}
            </p>
            <p>
                <b>End&nbsp; &nbsp;: </b> &nbsp;
                {episode_form.closing_length.end.hours} :
                {episode_form.closing_length.end.minutes} :
                {episode_form.closing_length.end.seconds}
            </p>
            <button class="btn skip_times mt-1" onclick={() => (openClose = !openClose)}>
                Edit credits timestamps</button
            >
        </div>
        <div class="form_div">
            <label for="synopsis">Synopsis</label>
            <!--TODO: Add word limit to synopsis field on server side -->
            <textarea
                name="synopsis"
                id="synopsis"
                class="w-11/12"
                rows="10"
                bind:value={episode_form.synopsis}
            ></textarea>
            {#if (formState.inner_state == Result.Err && formState.target == 'synopsis') || formState.target == 'synopsis'}
                <p class="error text-red-500">{formState.message}</p>
            {/if}
        </div>
        <button type="submit" class="btn primary_btn w-11/12">submit</button>
    </form>

    <!-- TODO: Match the frontend form episode to backend one -->
    <div class="time_stamp_editors flex_col">
        {#if openIntro}
            <div
                id="intro_times"
                class="alt_bg m-2 w-80 p-2"
                in:fly={{ x: 200, duration: 500 }}
                out:fly={{ x: 200, duration: 500 }}
            >
                <h2 class="text-xl">Intro Time Stamps</h2>
                <h3>Enter when the intro starts</h3>
                <div class="intro flex_row">
                    <input
                        type="number"
                        name="hrs"
                        id="hrs"
                        class="skip_time"
                        bind:value={episode_form.opening_length.start.hours}
                    />
                    <p class="text-xs">Hrs</p>
                    &nbsp;

                    <input
                        type="number"
                        name="mins"
                        id="mins"
                        class="skip_time"
                        bind:value={episode_form.opening_length.start.minutes}
                    />
                    <p class="text-xs">Mins</p>
                    &nbsp;

                    <input
                        type="number"
                        name="secs"
                        id="secs"
                        class="skip_time"
                        bind:value={episode_form.opening_length.start.seconds}
                    />
                    <p class="text-xs">Secs</p>
                    &nbsp;
                </div>

                <hr />
                <br />
                <h3>Enter when the intro ends</h3>
                <div class="intro flex_row">
                    <input
                        type="number"
                        name="hrs"
                        id="hrs"
                        class="skip_time"
                        bind:value={episode_form.opening_length.end.hours}
                    />
                    <p class="text-xs">Hrs</p>
                    &nbsp;

                    <input
                        type="number"
                        name="mins"
                        id="mins"
                        class="skip_time"
                        bind:value={episode_form.opening_length.end.minutes}
                    />
                    <p class="text-xs">Mins</p>
                    &nbsp;

                    <input
                        type="number"
                        name="secs"
                        id="secs"
                        class="skip_time"
                        bind:value={episode_form.opening_length.end.seconds}
                    />
                    <p class="text-xs">Secs</p>
                    &nbsp;
                </div>
                {#if (formState.inner_state == Result.Err && formState.target == 'runlength') || formState.target == 'runlength&confirm_runlength'}
                    <p class="error text-red-500">{formState.message}</p>
                {/if}
            </div>
        {/if}

        {#if openClose}
            <div
                id="credit_times"
                class="alt_bg w-80 p-2"
                in:fly={{ x: 200, duration: 500 }}
                out:fly={{ x: 200, duration: 500 }}
            >
                <h2 class="text-xl">Credits Time Stamps</h2>
                <h3>Enter when the closing credits starts</h3>
                <div class="intro flex_row">
                    <input
                        type="number"
                        name="hrs"
                        id="hrs"
                        class="skip_time"
                        bind:value={episode_form.closing_length.start.hours}
                    />
                    <p class="text-xs">Hrs</p>
                    &nbsp;

                    <input
                        type="number"
                        name="mins"
                        id="mins"
                        class="skip_time"
                        bind:value={episode_form.closing_length.start.minutes}
                    />
                    <p class="text-xs">Mins</p>
                    &nbsp;

                    <input
                        type="number"
                        name="secs"
                        id="secs"
                        class="skip_time"
                        bind:value={episode_form.closing_length.start.seconds}
                    />
                    <p class="text-xs">Secs</p>
                    &nbsp;
                </div>

                <hr />
                <br />
                <h3>Enter when the closing credits end</h3>
                <div class="intro flex_row">
                    <input
                        type="number"
                        name="hrs"
                        id="hrs"
                        class="skip_time"
                        bind:value={episode_form.closing_length.end.hours}
                    />
                    <p class="text-xs">Hrs</p>
                    &nbsp;

                    <input
                        type="number"
                        name="mins"
                        id="mins"
                        class="skip_time"
                        bind:value={episode_form.closing_length.end.minutes}
                    />
                    <p class="text-xs">Mins</p>
                    &nbsp;

                    <input
                        type="number"
                        name="secs"
                        id="secs"
                        class="skip_time"
                        bind:value={episode_form.closing_length.end.seconds}
                    />
                    <p class="text-xs">Secs</p>
                    &nbsp;
                </div>
                {#if (formState.inner_state == Result.Err && formState.target == 'runlength') || formState.target == 'runlength&confirm_runlength'}
                    <p class="error text-red-500">{formState.message}</p>
                {/if}
            </div>
        {/if}
    </div>
</section>

<style>
    .skip_time {
        width: 3rem;
        height: 24px;
        font-size: 14px;
        background-color: var(--primary);
    }

    .skip_times {
        color: var(--secondary);
    }

    input[type='number']::-webkit-inner-spin-button,
    input[type='number']::-webkit-outer-spin-button {
        -webkit-appearance: none;
        margin: 0;
    }

    /* For Firefox */
    input[type='number'] {
        -moz-appearance: textfield;
    }
</style>
