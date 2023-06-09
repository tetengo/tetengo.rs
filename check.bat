@echo off
rem Checks this package
rem Copyright 2023 kaoru  https://www.tetengo.org/

cargo verify-project
if errorlevel 1 exit /b 1

cargo fmt --check
if errorlevel 1 exit /b 1

cargo clippy --tests
if errorlevel 1 exit /b 1

cargo doc
if errorlevel 1 exit /b 1

cargo build --all-targets
if errorlevel 1 exit /b 1

cargo test --all-targets
if errorlevel 1 exit /b 1

