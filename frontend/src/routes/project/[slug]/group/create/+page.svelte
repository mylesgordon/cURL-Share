<script lang="ts">
	import CurlGroupForm from '$lib/components/CurlGroupForm.svelte';
	import Meta from '$lib/components/Meta.svelte';
	import type { CurlGroup } from '$lib/types';
	import type { PageData } from './$types';

	export let data: PageData;
	const { isLoggedIn, projectId } = data;

	const newCurlGroup: CurlGroup = {
		id: -1,
		curls: '[]',
		description: 'Insert description',
		labels: '',
		name: 'New cURL Group',
		project_id: projectId
	};
</script>

<Meta title="Create new cURL group" />

{#if projectId != -1 && isLoggedIn}
	<h1>Create new cURL group</h1>

	<CurlGroupForm {projectId} curlGroup={newCurlGroup} />
{:else}
	<p>Unable to load page. Please sign in or try again.</p>
{/if}
