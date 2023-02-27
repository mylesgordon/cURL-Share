/// <reference types="cypress" />

Cypress.Commands.add('visitAndWaitForHydration', (url: string) => {
	cy.visit(url);	
	cy.get('[data-testid="svelte-hydrated"]', { timeout: 10000 }).should('exist');
});

Cypress.Commands.add('signUpTestUser', (verifySuccess: boolean) => {
	cy.visitAndWaitForHydration('/log-in');

	cy.get('#username').type('test');
	cy.get('#password').type('user');
	cy.get('button').contains('Sign Up').click();
	if (verifySuccess) {
		cy.get('#log-out-link').should('exist');
	}
});

Cypress.Commands.add('loginAsTestUser', (verifySuccess: boolean) => {
	cy.visitAndWaitForHydration('/log-in');

	cy.get('#username').type('test');
	cy.get('#password').type('user');
	cy.get('button').contains('Log In').click();
	if (verifySuccess) {
		cy.get('#log-out-link').should('exist');
	}
});

Cypress.Commands.add('logout', (verifySuccess: boolean) => {
	cy.get('#log-out-link').click();
	if (verifySuccess) {
		cy.get('#log-in-link').should('exist');
	}
});

Cypress.Commands.add('deleteTestUser', () => {
	cy.request({ url: 'http://localhost:8080/api/v1/log-in', method: "POST", body: { username: "test", password: "user" } , failOnStatusCode: false }).then((response) => {
		if (response.isOkStatusCode) {
			cy.request({ url: 'http://localhost:8080/api/v1/delete-user', method: "POST" });
		}
	});
	cy.reload();
});

declare global {
	// eslint-disable-next-line @typescript-eslint/no-namespace
	namespace Cypress {
		// eslint-disable-next-line @typescript-eslint/no-unused-vars
		interface Chainable<Subject> {
			visitAndWaitForHydration(url: string): Chainable<void>;
			deleteTestUser(): Chainable<void>;
			loginAsTestUser(verifySuccess: boolean): Chainable<void>;
			logout(verifySuccess): Chainable<void>;
			signUpTestUser(verifySuccess: boolean): Chainable<void>;
		}
	}
}

export {};
