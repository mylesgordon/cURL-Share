import { fireEvent, render } from '@testing-library/svelte';
import { goto } from '$app/navigation';
import { logInRequest } from '$lib/api.page';
import { screen } from '@testing-library/dom';
import { testMatchingSnapshot } from '../common';
import LogIn from '../../routes/log-in/+page.svelte';
import userEvent from '@testing-library/user-event';

vi.mock('$app/navigation');
vi.mock('$lib/api.page');

describe('Log In page', () => {
	function getPageElements() {
		const usernameField = screen.getByLabelText('Username');
		const passwordField = screen.getByLabelText('Password');
		const logInButton = screen.getByRole('button', { name: /Log In/i });
		const signUpButton = screen.getByRole('button', { name: /Sign Up/i });

		return { usernameField, passwordField, logInButton, signUpButton };
	}

	testMatchingSnapshot(LogIn);

	it('should display the correct title', () => {
		render(LogIn);
		expect(document.title).toEqual('Log In | cURL Share');
	});

	it('shows correct error message when username or password are empty', async () => {
		render(LogIn);
		const { logInButton, signUpButton, passwordField, usernameField } = getPageElements();
		const table = [
			{ a: usernameField, b: passwordField },
			{ a: passwordField, b: usernameField }
		];

		for (const { a, b } of table) {
			await userEvent.clear(a);
			await userEvent.type(b, 'notempty');
			await fireEvent.click(logInButton);
			screen.getByText('Username or password fields empty - please try again.');
			await fireEvent.click(signUpButton);
			screen.getByText('Username or password fields empty - please try again.');
		}
	});

	it('shows the correct error message depending on server response', async () => {
		render(LogIn);
		const { logInButton, signUpButton, passwordField, usernameField } = getPageElements();
		const table = [
			{
				statusCode: 401,
				expectedMessage: 'Incorrect username and/or password - please try again.',
				buttonElement: logInButton
			},
			{
				statusCode: 409,
				expectedMessage: 'User with that username already exists - please try again.',
				buttonElement: signUpButton
			},
			{
				statusCode: 404,
				expectedMessage: 'Unknown error - please try again.',
				buttonElement: logInButton
			}
		];

		for (const { statusCode, expectedMessage, buttonElement } of table) {
			logInRequest.mockReturnValue(statusCode);

			await userEvent.type(usernameField, 'user');
			await userEvent.type(passwordField, 'password');
			await fireEvent.click(buttonElement);
			await screen.findByText(expectedMessage);
		}
	});

	it('creates a correct API call', async () => {
		render(LogIn);
		const { logInButton, signUpButton, passwordField, usernameField } = getPageElements();
		const table = [
			{
				buttonElement: logInButton,
				endpoint: 'log-in'
			},
			{
				buttonElement: signUpButton,
				endpoint: 'sign-up'
			}
		];

		for (const { buttonElement, endpoint } of table) {
			await userEvent.clear(usernameField);
			await userEvent.clear(passwordField);

			await userEvent.type(usernameField, 'user');
			await userEvent.type(passwordField, 'password');
			await fireEvent.click(buttonElement);

			expect(logInRequest).toHaveBeenCalledWith(window.fetch, endpoint, 'user', 'password');
		}
	});

	it('redirects the user on succesful log in/sign up', async () => {
		render(LogIn);
		const { logInButton, signUpButton, passwordField, usernameField } = getPageElements();
		const table = [
			{
				buttonElement: logInButton,
				statusCode: 200
			},
			{
				buttonElement: signUpButton,
				statusCode: 201
			}
		];

		for (const { buttonElement, statusCode } of table) {
			render(LogIn);
			logInRequest.mockReturnValue(statusCode);

			await userEvent.type(usernameField, 'user');
			await userEvent.type(passwordField, 'password');
			await fireEvent.click(buttonElement);

			expect(goto).toHaveBeenCalledWith('/');
		}
	});
});
