@echo off
rem Checks this package
rem Copyright (C) 2023-2025 kaoru  <https://www.tetengo.org/>

call kogyan\scripts\check.bat
if errorlevel 1 exit /b 1

kogyan\scripts\sort_derive_attributes.py .
if errorlevel 1 exit /b 1

kogyan\scripts\check_use_order.py .
if errorlevel 1 exit /b 1

cargo verify-project
if errorlevel 1 exit /b 1

cargo fmt --check
if errorlevel 1 exit /b 1

cargo clippy --all-targets
if errorlevel 1 exit /b 1

cargo doc
if errorlevel 1 exit /b 1

cargo build --all-targets
if errorlevel 1 exit /b 1

cargo test --all-targets --quiet
if errorlevel 1 exit /b 1

