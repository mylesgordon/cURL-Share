<script>
	import '../app.css';
	import 'agnostic-svelte/css/common.min.css';
	import Header from '$lib/components/Header.svelte';
</script>

<Header />
<main class="max-w-4xl m-auto pt-4">
	<slot />
</main>
