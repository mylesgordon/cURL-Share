<script lang="ts">
	import CurlGroupForm from '$lib/components/CurlGroupForm.svelte';
	import Meta from '$lib/components/Meta.svelte';
	import type { PageData } from '../$types';

	export let data: PageData;
	const { success, curlGroup } = data;
</script>

<Meta title="Editing a cURL group" />

{#if success}
	<h2>Editing cURL Group '{curlGroup.name}'</h2>

	<CurlGroupForm projectId={curlGroup.project_id} {curlGroup} editing />
{:else}
	<p>Something went wrong. Please try again.</p>
{/if}
