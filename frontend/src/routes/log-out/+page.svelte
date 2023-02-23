<script lang="ts">
	import { goto, invalidateAll } from '$app/navigation';
	import Meta from '$lib/components/Meta.svelte';

	async function logOut() {
		try {
			await fetch('http://localhost:8080/api/v1/log-out', {
				method: 'POST',
				mode: 'cors',
				headers: { 'Access-Control-Allow-Origin': 'http://localhost:8080' },
				credentials: 'include'
			});
			await invalidateAll();
			await goto('/');
		} catch (e) {
			console.error(e);
		}
	}

	logOut();
</script>

<Meta title="Log Out" />
