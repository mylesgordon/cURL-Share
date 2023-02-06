<script lang="ts">
	import type { SvelteComponentDev } from 'svelte/internal';

	export let description: string;
	export let href: string;
</script>

<a {href} aria-label={description}>
	<slot />
</a>
