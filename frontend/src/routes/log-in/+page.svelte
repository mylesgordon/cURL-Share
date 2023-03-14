<script lang="ts">
	import { Button, ButtonGroup, ChoiceInput, Input } from 'agnostic-svelte';
	import { goto } from '$app/navigation';
	import { isLoggedIn } from '$lib/stores';
	import { logInRequest } from '$lib/api.page';
	import Meta from '$lib/components/Meta.svelte';

	let errorText: string;
	let username: string;
	let password: string;

	let checkedOption: any[] = [];
	let cookieAgreed: boolean;

	$: if (checkedOption[0] !== undefined) {
		cookieAgreed = checkedOption[0] === 'agrees';
	} else {
		cookieAgreed = false;
	}

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

	<p class="text-center">
		cURL Share uses cookies to store the user's current session. No information is stored within
		this cookie apart from the user's unique ID.
	</p>

	<div class="m-auto">
		<ChoiceInput
			type="checkbox"
			id="cookie-checkbox"
			data-testid="cookie-checkbox"
			options={[
				{ name: 'cookie-agreed', value: 'agrees', label: "I accept cURL Share's cookie policy" }
			]}
			bind:checked={checkedOption}
			isFieldset={false}
		/>
	</div>

	{#if errorText}
		<p class="text-center" id="error-text">{errorText}</p>
	{/if}

	<ButtonGroup ariaLabel="Form submission buttons" css="self-center">
		<Button isGrouped isBordered mode="primary" type="submit" isDisabled={!cookieAgreed}
			>Log In</Button
		>
		<Button isGrouped isBordered mode="primary" type="submit" isDisabled={!cookieAgreed}
			>Sign Up</Button
		>
	</ButtonGroup>
</form>
