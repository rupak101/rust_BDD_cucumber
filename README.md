# Automation of API calls

This project have API tests using BDD(Gherkin) and also dockerize the project.

# Tools, Framework, Programming Language used:

- Rust, BDD, Gherkin, Docker, Cucumber

# Development environment :

- All development and execution done on Mac OS. It should work on other OS as well.

## Dependencies

To run the tests, please ensure you have the following installed:

- Rust
- Docker (for running in container)

## Installation

- Clone the repo

## Run the tests

### Standard Mode
- Run tests.

```
cargo test
```

### Run the tests in Docker container

* build docker image 
```
docker build -t api_test .
```
* run docker image to run tests
```
docker run --env-file .env api_test
```

# Future Enhancement :
  * Keep all sensitive data in secure vault, add integration to fetch the value from the vault.Benefit - we can change the key anytime without changing any code.
  * Make more modular and optimize the code(As This is my first project in Rust language)
  * Deep response validation for API requests.
  * Negative scenarios can be added to the test suite.
  * Error handling for API requests.

# Offical Documents links:

- Rust Docs: https://www.rust-lang.org/
- Cucumber : https://cucumber.io/
- Gherkin : https://cucumber.io/docs/gherkin/reference/
