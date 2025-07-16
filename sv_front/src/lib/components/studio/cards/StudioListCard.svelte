<script lang="ts">
	import { suridToString } from "$lib/types/server";
import type { Studio } from "$lib/types/studio";

    let {studio} : {studio: Studio} = $props();
    let id :string = suridToString(studio.id);
</script>

<a href="/studio/{id}" class="text-base w-11/12 borderb1_hover">
<article class="studio_card tertiary_bg_hover tertiary_borderb txt1_hover w-full h-16 md:h-12 p-2 mt-2 flex flex-col md:flex-row md:justify-evenly md:items-center md:text-center font-bold cursor-pointer" id="{id}card">
   {studio.name}
<h5 class="text-sm w-2/6">{studio.email}</h5>
</article>
</a>