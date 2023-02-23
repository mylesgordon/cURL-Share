<script lang="ts">
	import { Header, HeaderNav, HeaderNavItem } from 'agnostic-svelte';
	import { page } from '$app/stores';
</script>

<Header>
	<a href="/">cURL Share</a>
	<HeaderNav>
		<HeaderNavItem>
			{#if $page.data.isLoggedIn}
				<a href="/log-out">Log Out</a>
			{:else}
				<a href="/log-in">Log In</a>
			{/if}
		</HeaderNavItem>
	</HeaderNav>
</Header>
