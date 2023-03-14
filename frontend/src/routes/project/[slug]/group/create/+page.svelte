<script lang="ts">
	import CurlGroupForm from '$lib/components/CurlGroupForm.svelte';
	import Meta from '$lib/components/Meta.svelte';
	import type { PageData } from './$types';

	export let data: PageData;
	const { isLoggedIn, projectId } = data;
</script>

<Meta title="Create new cURL group" />

{#if projectId != -1 && isLoggedIn}
	<h1>Create new cURL group</h1>

	<CurlGroupForm {projectId} />
{:else}
	<p>Unable to load page. Please sign in or try again.</p>
{/if}
