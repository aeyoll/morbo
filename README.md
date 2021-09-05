# Morbo, a CSP Reporter

[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Build Status](https://github.com/aeyoll/morbo/actions/workflows/rust.yml/badge.svg)](https://github.com/aeyoll/morbo/actions/workflows/ci.yml)
[![Crates.io Version](https://img.shields.io/crates/v/morbo.svg)](https://crates.io/crates/morbo)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.48.0+-lightgray.svg)](#rust-version-requirements)

![Morbo, a CSP Reporter](https://github.com/aeyoll/morbo/blob/main/.github/logo.jpg?raw=true)

The HTTP Content-Security-Policy `report-to` (and the deprecated `report-uri`) response header directive instructs the user agent to report attempts to violate the Content Security Policy. These violation reports consist of JSON documents sent via an HTTP POST request to the specified URI.

This Rust crate is an endpoint to receive those reports and send them to an email address.

Install
---

First, install using cargo:

```
cargo install morbo
```

Then, setup some environment variables:

```
MORBO_MAILER_FROM_NAME=Example
MORBO_MAILER_FROM_EMAIL=example@example.org
MORBO_MAILER_TO_NAME=Example
MORBO_MAILER_TO_EMAIL=example@example.org
MORBO_MAILER_SMTP_HOSTNAME=localhost
MORBO_MAILER_SMTP_PORT=1025
MORBO_MAILER_SMTP_USERNAME=
MORBO_MAILER_SMTP_PASSWORD=
```

Usage
---

```
morbo --port=8080 # (port is optionnal, default 8080)
```

Then, setup a reverse proxy in your webserver. For example, in nginx:

```
location /_/csp-reports {
    proxy_pass http://127.0.0.1:8080;
}
```

In the website you want to report CPS, set the following headers:

```
Report-To: {"group": "csp-endpoint","max_age": 10886400,"endpoints": [{ "url": "http://example.org/_/csp-reports" }]});
Content-Security-Policy: default-src 'self'; report-to csp-endpoint; report-uri http://example.org/_/csp-reports;');
```

Event though `report-uri` is deprecated, `report-to` is not supported in every browser.
