<script lang="ts">
	import { goto } from '$app/navigation';
	import { isLoggedIn } from '$lib/stores';
	import { logOutRequest } from '$lib/api.page';
	import Meta from '$lib/components/Meta.svelte';

	async function logOut() {
		try {
			await logOutRequest(fetch);
			isLoggedIn.set(false);
			await goto('/');
		} catch (e) {
			console.error(e);
		}
	}

	logOut();
</script>

<Meta title="Log Out" />
