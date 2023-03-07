<script lang="ts">
	import { goto } from '$app/navigation';
	import { isLoggedIn } from '$lib/stores';
	import Meta from '$lib/components/Meta.svelte';

	async function logOut() {
		try {
			await fetch(`${import.meta.env.VITE_BACKEND_BASE_URL}/api/v1/log-out`, {
				method: 'POST',
				mode: 'cors',
				headers: { 'Access-Control-Allow-Origin': 'http://localhost:8080' },
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
