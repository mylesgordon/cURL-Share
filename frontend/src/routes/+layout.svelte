<script lang="ts">
	import '../app.css';
	import 'agnostic-svelte/css/common.min.css';
	import { onMount } from 'svelte';
	import Header from '$lib/components/Header.svelte';
	import type { LayoutServerData } from './$types';

	export let data: LayoutServerData;

	let svelteStatus = 'svelte-not-hydrated';

	onMount(() => {
		svelteStatus = 'svelte-hydrated';
	});
</script>

<Header isLoggedInResponse={data.isLoggedIn} />
<span data-testid={svelteStatus} />

<main class="max-w-4xl m-auto pt-4">
	<slot />
</main>
