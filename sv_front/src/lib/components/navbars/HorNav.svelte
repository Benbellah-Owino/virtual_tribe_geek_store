<script>
	import { onMount } from 'svelte';

	let isLoggedIn = $state(false);
	onMount(async () => {
		let cookie = await window.cookieStore.get('auth_token');

		isLoggedIn = cookie ? true : false;
	});
</script>

<nav class="secondary_borderb flex min-h-20 items-center justify-between p-2">
	<div class="logo">
		<h1 class="text text-6xl font-bold">VRT</h1>
	</div>
	<ul class="links flex items-center justify-around text-sm md:text-base">
		<li><a href="/content/comics/" class="nav_link underline">Comics</a></li>
		<li><a href="/user/creator/" class="nav_link underline">My Page</a></li>
		{#if isLoggedIn}
			<li><a href="/user/logout" class="nav_link underline">Logout</a></li>
		{:else}
			<li><a href="/user/login" class="nav_link underline">Login</a></li>
		{/if}
	</ul>
</nav>

<style>
	nav {
		display: flex;
	}

	.links {
		min-width: 30%;
	}
</style>
