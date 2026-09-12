# Continuous Integration (CI) Pipeline

In trunk-based development we should be able to deploy our `main` branch at any point in time. Every member of the team can branch of from `main`,
develop a small feature or fix a bug, merge back into `main` and release to our users.

```
Continuous Integration empowers each member of the team to integrate 
their changes into the `main` branch multiple times a day.
```
This has powerful ripple effects. 
Some are tangible and easy to spot: it reduces the chances of having to sort out messy merge conflicts due to long-lived branches. Nobody likes merge conflicts.

Some are subtler: **Continuous Integration tightens the feedback loop.** You are less likely to go off on your own and develop or weeks just to find the approach
you have chosen is not endorsed by the rest of the team, or it would not integrate well with the rest of the project.
It forces you to engage with your teammates earlier than when it feels comfortable, course-correcting if necessary
when it is still necessary when it is still easy to do so (and nobody is likely to get offended).

**How do we make it possible?**
With a collection of automated checks running on every commit - our **CI Pipeline**.
If one of the checks fails you cannot merge to `main` - as simple as that.

CI Pipeline often go beyond ensuring code health: they are a good place to perform a series of additional important checks - e.g.,
scanning our dependency tree for known vulnerabilities, linting, formatting, etc.

We will run through the different checks that you might want to run as part of the CI Pipeline of your Rust projects,
introducing the associated tools as go along. We will then provide a set of ready-made CI pipelines for some of the major CI providers.

## CI Steps
### [-] Tests
If your CI Pipeline had single step, it should be testing. Tests are a first-class concept in the Rust ecosystem
and you can leverage `cargo` to run your unit and integration tests:
```
cargo test
```
`cargo test` also takes care of building the project before running tests, hence you do not need to
run `cargo build` beforehand (even though the most pipelines will invoke `cargo build` before running tests to cache dependencies).

### [-] Code Coverage

### [-] Linting

### [-] Formatting

### [-] Security Vulnerabilities

## Ready-to-go CI Pipelines


