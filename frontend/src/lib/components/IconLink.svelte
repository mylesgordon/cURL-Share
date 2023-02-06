<script lang="ts">
	export let description: string;
	export let href: string;
</script>

<a {href} aria-label={description}>
	<slot />
</a>
