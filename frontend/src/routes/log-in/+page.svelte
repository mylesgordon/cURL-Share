<script lang="ts">
	import { Button, ButtonGroup, Input } from 'agnostic-svelte';
	import { goto } from '$app/navigation';
	import { isLoggedIn } from '$lib/stores';
	import { logInRequest } from '$lib/api.page';
	import Meta from '$lib/components/Meta.svelte';

	let errorText: string;
	let username: string;
	let password: string;

	function getEndpointFromSubmitter(submitter: HTMLElement | null): string | null {
		switch (submitter?.textContent) {
			case 'Log In':
				return 'log-in';
			case 'Sign Up':
				return 'sign-up';
			default:
				return null;
		}
	}

	async function handleServerResponse(responseStatus: number) {
		switch (responseStatus) {
			case 200:
			case 201:
				isLoggedIn.set(true);
				await goto('/');
				break;
			case 401:
				errorText = 'Incorrect username and/or password - please try again.';
				break;
			case 409:
				errorText = 'User with that username already exists - please try again.';
				break;
			default:
				errorText = 'Unknown error - please try again.';
				break;
		}
	}

	async function onSubmit(e: SubmitEvent) {
		username = username.trim();
		password = password.trim();

		if (!username || !password) {
			errorText = 'Username or password fields empty - please try again.';
			return;
		}

		const endpoint = getEndpointFromSubmitter(e.submitter);
		if (!endpoint) {
			console.error('Submitter not identified.');
			return;
		}

		try {
			const requestStatus = await logInRequest(fetch, endpoint, username, password);
			console.log(requestStatus);
			handleServerResponse(requestStatus);
		} catch (err) {
			errorText = 'Unhandled error - please try again.';
			console.error(err);
		}
	}
</script>

<Meta title="Log In" />

<form class="flex flex-col space-y-3" on:submit|preventDefault={onSubmit}>
	<Input isRounded id="username" label="Username" bind:value={username} />
	<Input isRounded id="password" type="password" label="Password" bind:value={password} />

	{#if errorText}
		<p id="error-text">{errorText}</p>
	{/if}

	<ButtonGroup ariaLabel="Form submission buttons" css="self-center">
		<Button isGrouped isBordered mode="primary" type="submit">Log In</Button>
		<Button isGrouped isBordered mode="primary" type="submit">Sign Up</Button>
	</ButtonGroup>
</form>
