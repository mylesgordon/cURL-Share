<script lang="ts">
	import { goto } from '$app/navigation';
	import { backendUrl, isLoggedIn } from '$lib/stores';
	import Meta from '$lib/components/Meta.svelte';

	async function logOut() {
		try {
			await fetch(`${$backendUrl}/api/v1/log-out`, {
				method: 'POST',
				mode: 'cors',
				headers: { 'Access-Control-Allow-Origin': $backendUrl },
				credentials: 'include'
			});
			isLoggedIn.set(false);
			await goto('/');
		} catch (e) {
			console.error(e);
		}
	}

	logOut();
</script>

<Meta title="Log Out" />
