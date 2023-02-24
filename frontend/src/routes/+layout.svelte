<script lang="ts">
	import '../app.css';
	import 'agnostic-svelte/css/common.min.css';
	import Header from '$lib/components/Header.svelte';
	import type { LayoutServerData } from './$types';

	export let data: LayoutServerData;
</script>

<Header isLoggedInResponse={data.isLoggedIn} />
<main class="max-w-4xl m-auto pt-4">
	<slot />
</main>
