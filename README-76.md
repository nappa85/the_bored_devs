# Cypress SFCC Boilerplate

This project aims to give a common starting point to test SFCC using Cypress.<br/>
Every project has little-to-big frontend differences, so little-to-big will be needed to pass the tests.

## Docker

You can run Cypress from a Docker container, this way you don't need to install anything on your local machine, and you can assure everything is at the right version.<br/>
This command assumes the current folder is the project folder:
```bash
docker run --rm -ti --ipc=host -v $(pwd):/mnt --workdir /mnt cypress/base:10 bash
```

## How to install

First time Cypress needs to be installed before it can be used:
```bash
npm install
```
**DOCKER NOTE**:
Once you have the `node_modules` folder, you can simply run this command after launching the container:
```bash
$(npm bin)/cypress install
```

## How to run

There are several ways to use Cypress

### Interactive

```bash
$(npm bin)/cypress open
```
**DOCKER NOTE**:
This won't work inside Docker until you share the X11 Socket, how to do it (and if it's possible) vary based on your OS.

### Headless

```bash
$(npm bin)/cypress run
```
This will run the tests inside Electron headless.<br/>
You can specify which browser to use using the `--browser` flag.
```bash
$(npm bin)/cypress run --browser chrome
```
**DOCKER NOTE**:
The `cypress/base` container doesn't contain any other browser than Electron, you can use another Cypress container, e.g. `cypress/browsers:node10.16.3-chrome80-ff73`.

## Configuration

To configure Cypress, rename the file `cypress.json.example` to `cypress.json` and edit it to set your sandbox url.

## File structure

### Fixtures

Fixtures are test data, like product name, size, color, etc...<br/>
It's really important to not hardcode those data inside the tests, but instead group it in a single file to have a single point of edit to customize the tests for every project.

Location: `cypress/fixtures/metadata.json`

Example:
```json
{
    "user": {
        "fName": "Test1",
        "lName": "User1",
        "address1": "Via Verdi",
        "country": "Italy",
        "state": "Veneto",
        "stateAbr": "VN",
        "city": "Venezia",
        "zip": "01801",
        "phone": "781-555-1212",
        "email": "testuser1@demandware.com",
        "password": "Test123!"
    },
    "product": {
        "name": "Elbow Sleeve Ribbed Sweater",
        "quantity": "2",
        "editCartQuantity": "1",
        "itemPrice": "$65.99",
        "totalItemPrice": "$131.98",
        "shipping": "$7.99",
        "tax": "$14.00",
        "taxCheckout": "$7.00",
        "estimatedTotal": "$153.97",
        "estimatedTotalCheckout": "$146.97"
    }
}
```

### Gherkin Test Definition

Cypress allows to use Gherkin file format to define the test workflow.<br/>
Gherkin files must be saved as `feature` files and grouped in folders representing the test type.

Location: `cypress/integration/features`

Example `cypress/integration/features/homePage/landHomePage.feature`:
```gherkin
Feature: Land Home Page
    As a shopper, I want to land on the home Page

@homePage
    Scenario: Shopper is able to land on the home Page
        When shopper selects yes or no for tracking consent
```

### Page Object Model (POM) files

Cypress allows to grops page definitions, defining selectors and methods specific for the pages.

Location: `cypress/integration/common/pages`

Example `cypress/integration/common/pages/home.page.js`:
```json
module.exports = {
    locators: {
        searchField: 'div.site-search input.form-control.search-field',
        searchedImage: 'div.site-search button[name="search-button"]'
    },
    search(product) {
        cy.fillField(this.locators.searchField, product);
        cy.get(this.locators.searchedImage).first().click();
    }
}
```

### Step files

Step files are the core of Cypress tests, here the single steps described in Gherkin files are translated in real code.<br/>
As Gherkin files, step files must be grouped by test type.

Location: `cypress/integration/common`

Example `cypress/integration/common/homePage/landHomePage.steps.js`:
```json
import When from "cypress-cucumber-preprocessor/steps";
import homePage from '../pages/home.page';
import data from '../../../fixtures/metadata.json'

When('shopper selects yes or no for tracking consent', () => {
    cy.viewport('macbook-15');
    cy.amOnPage(data.login.homePage);
    homePage.accept();
});

```
