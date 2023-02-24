<script lang="ts">
	import { Header, HeaderNav, HeaderNavItem } from 'agnostic-svelte';
	import { isLoggedIn } from '$lib/stores';

	export let isLoggedInResponse: boolean;
	isLoggedIn.set(isLoggedInResponse);
</script>

<Header>
	<a href="/">cURL Share</a>
	<HeaderNav>
		<HeaderNavItem>
			{#if $isLoggedIn}
				<a href="/log-out">Log Out</a>
			{:else}
				<a href="/log-in">Log In</a>
			{/if}
		</HeaderNavItem>
	</HeaderNav>
</Header>
