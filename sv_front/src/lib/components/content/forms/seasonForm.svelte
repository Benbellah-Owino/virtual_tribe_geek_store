<script lang="ts">
    import type { SeasonForCreate } from '$lib/types/content';
    import { FormError } from '$lib/types/error';
    import { Result } from '$lib/types/result';
    import { updateFormState, type FormState } from '$lib/types/state/form_state';

    let { season_form, videoId }: { season_form: SeasonForCreate; videoId: string } = $props();
    let formState: FormState = $state({
        inner_state: Result.Ok,
        error: null,
        message: '',
        target: '',
        locked: false
    });

    async function submit(e: Event) {
        e.preventDefault();

        try {
            console.log($state.snapshot(season_form));
            // TODO" Fix this route
            let response = await fetch(`http://localhost:7878/content/video/season`, {
                method: 'POST',
                credentials: 'include',
                body: JSON.stringify(season_form),
                headers: {
                    'Content-Type': 'application/json'
                }
            });
            if (response.status == 201) {
                //UNIMPLEMENTED
                setTimeout(() => {
                    updateFormState(formState, Result.Ok, null, 'Registration success', 'form', true);
                    console.log($state.snapshot(formState));
                }, 3000);
                console.log('created');
                window.open(`/content/video/${videoId}`, '_self');
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
<form class="form alt_bg mt-3 rounded-lg p-3 md:w-96 lg:w-5/6" onsubmit={submit}>
    <h3 class="float-left mb-4 text-3xl font-extrabold">ADD SEASON</h3>
    <br />
    {#if formState.inner_state == Result.Ok && formState.target == 'form'}
        <center><p class="error main_txt text-lg font-semibold">{formState.message}</p></center>
    {:else if formState.inner_state == Result.Err && formState.target == 'form'}
        <center><p class="error text-lg font-semibold text-red-400">{formState.message}</p></center>
    {/if}
    <div class="form_div">
        <div class="form_div">
            <label for="number_of_episodes">Number of Episodes</label>
            <input
                type="number"
                name="number_of_episodes"
                id="number_of_episodes"
                bind:value={season_form.number_of_seasons}
            />
        </div>

        <label for="synopsis">Synopsis</label>
        <!--TODO: Add word limit to synopsis field on server side -->
        <textarea
            name="synopsis"
            id="synopsis"
            class="w-11/12"
            rows="10"
            bind:value={season_form.synopsis}
        ></textarea>
    </div>

    <button type="submit" class="btn primary_btn w-11/12">submit</button>
    <!-- TODO: Next time number the seasons -->
</form>
