<script lang="ts">
	import { suridToString } from "$lib/types/server";
import type { Studio } from "$lib/types/studio";

    let {studio} : {studio: Studio} = $props();
    let id :string = suridToString(studio.id);
</script>

<article class="studio_card tertiary_bg_hover tertiary_border w-11/12 h-12 p-2 mt-2 flex justify-evenly items-center cursor-pointer" id="{id}card">
    <a href="/studio/{id}" class="text-base secondary_txt_hover txt1_hover borderb1_hover">{studio.name}</a>
<h5 class="text-sm w-2/6">{studio.email}</h5>
</article>