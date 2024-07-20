@echo off
rem Checks the scripts
rem Copyright (C) 2023-2024 kaoru  <https://www.tetengo.org/>

for %%f in (%~dp0*.py) do (
    echo Checking %%~nxf...
    mypy %%f
    if errorlevel 1 exit /b 1
    black %%f
    if errorlevel 1 exit /b 1
    isort --profile black %%f
    if errorlevel 1 exit /b 1
)
