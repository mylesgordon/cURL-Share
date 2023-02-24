/// <reference types="cypress" />

Cypress.Commands.add('waitForHydration', () => {
	cy.get('[data-testid="svelte-hydrated"]', { timeout: 10000 }).should('exist');
});

Cypress.Commands.add('signUpTestUser', () => {
	cy.visit('/log-in');
	cy.waitForHydration();

	cy.get('#username').type('test');
	cy.get('#password').type('user');
	cy.get('button').contains('Sign Up').click();
	cy.get('#log-out-link').should('exist');
});

Cypress.Commands.add('loginAsTestUser', () => {
	cy.visit('/log-in');
	cy.waitForHydration();

	cy.get('#username').type('test');
	cy.get('#password').type('user');
	cy.get('button').contains('Log In').click();
	cy.get('#log-out-link').should('exist');
});

Cypress.Commands.add('logout', () => {
	cy.get('#log-out-link').then(($element) => $element.trigger('click'));
});

Cypress.Commands.add('deleteTestUser', () => {
	let testUserDoesExist = true;

	cy.loginAsTestUser();
	cy.get('body').then((body) => {
		testUserDoesExist = body.find('#error-text').length !== 0;
		cy.log(`${body.find('#error-text').length}`);
	});

	if (testUserDoesExist) {
		cy.log('User exists - deleting');
		cy.request('POST', 'http://localhost:8080/api/v1/delete-user');
		cy.reload();
		cy.get('#log-in-link').should('exist');
	} else {
		cy.log("Nothing to do - test user doesn't exist!");
	}
});

declare global {
	// eslint-disable-next-line @typescript-eslint/no-namespace
	namespace Cypress {
		// eslint-disable-next-line @typescript-eslint/no-unused-vars
		interface Chainable<Subject> {
			waitForHydration(): Chainable<void>;
			deleteTestUser(): Chainable<void>;
			loginAsTestUser(): Chainable<void>;
			logout(): Chainable<void>;
			signUpTestUser(): Chainable<void>;
		}
	}
}

export {};
