/// <reference types="cypress" />

describe('User session', () => {
	beforeEach(() => {
		cy.deleteTestUser();
	});

	it('prevents a user from creating a user with a duplicate username', () => {
		cy.signUpTestUser(true);
		cy.visitAndWaitForHydration('/log-in');

		cy.get('#username').type('test');
		cy.get('#password').type('4nyPassword!');
		cy.get('button').contains('Sign Up').click();

		cy.get('#error-text').should('contain.text', 'User with that username already exists - please try again.');
	});

	it('prevents a user from logging in with an incorrect username and/or password', () => {
		cy.signUpTestUser(true);
		cy.visitAndWaitForHydration('/log-in');

		cy.get('#username').type('test');
		cy.get('#password').type('4nyPassword!');
		cy.get('button').contains('Log In').click();

		cy.get('#error-text').should('contain.text', 'Incorrect username and/or password - please try again.');

		cy.get('#username').type('i-dont-exist');
		cy.get('#password').type('password');
		cy.get('button').contains('Log In').click();

		cy.get('#error-text').should('contain.text', 'Incorrect username and/or password - please try again.');
	});

	it('redirects user to the page they were last on after a successful sign up', () => {
		cy.getCookie('id').should('not.exist');
		cy.signUpTestUser(true);
		// FIXME: implement 'last page' functionality
		cy.url().should('eq', `${Cypress.config().baseUrl}/`);
		cy.getCookie('id').should('exist');
	});

	it('redirects user to the page they were last on after a succesful log out', () => {
		cy.signUpTestUser(true);
		cy.getCookie('id').should('exist');
		cy.logout(true);
		// FIXME: implement 'last page' functionality
		cy.url().should('eq', `${Cypress.config().baseUrl}/`);
		cy.getCookie('id').should('be.null');
	});

	it('redirects user to the page they were last on after a successful log in', () => {
		cy.signUpTestUser(true);
		cy.logout(true);
		cy.loginAsTestUser(true);
		// FIXME: implement 'last page' functionality
		cy.url().should('eq', `${Cypress.config().baseUrl}/`);
		cy.getCookie('id').should('exist');
	});
});
