describe('User session', () => {
	beforeEach(() => {
		cy.deleteTestUser();
	});

	it('redirects use to the home page after a successful sign up', () => {
		cy.signUpTestUser();
		cy.url().should('eq', `${Cypress.config().baseUrl}/`);
	});

	it('redirects user to home page after a successful log in', () => {
		cy.signUpTestUser();
		cy.logout();
		cy.loginAsTestUser();
		cy.url().should('eq', `${Cypress.config().baseUrl}/`);
	});
});
