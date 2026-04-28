# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Fixed

- **Elixir bindings now respect the `:force_build` configuration.** Previously, setting `config :rustler_precompiled, :force_build, kreuzcrawl: true` in Elixir projects was ignored by the native loader. This made it impossible to bypass precompiled binary checksum failures by building from source as suggested in the error message. The loader now correctly checks `Application.compile_env/3`.
