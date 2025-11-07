@echo off
REM Plugin deployment script for Rust plugin system (Windows)
REM This script copies built plugin libraries to the runtime plugins directory

setlocal enabledelayedexpansion

REM Determine the build profile (default to debug)
set PROFILE=%1
if "%PROFILE%"=="" set PROFILE=debug

set TARGET_DIR=target\%PROFILE%
set PLUGINS_DIR=%TARGET_DIR%\plugins

echo Deploying plugins for %PROFILE% build...
echo Platform: Windows
echo Library extension: .dll

REM Create the plugins directory if it doesn't exist
if not exist "%PLUGINS_DIR%" (
    mkdir "%PLUGINS_DIR%"
    echo Created plugins directory: %PLUGINS_DIR%
) else (
    echo Verified plugins directory: %PLUGINS_DIR%
)

REM Counter for deployed plugins
set DEPLOYED=0

REM Find and copy all plugin DLL files
for %%f in ("%TARGET_DIR%\*plugin*.dll") do (
    if exist "%%f" (
        echo Deploying: %%~nxf
        copy /Y "%%f" "%PLUGINS_DIR%\" >nul
        set /a DEPLOYED+=1
    )
)

if %DEPLOYED%==0 (
    echo Warning: No plugin libraries found in %TARGET_DIR%
    echo Make sure you have built the project with: cargo build
    exit /b 1
)

echo.
echo Successfully deployed %DEPLOYED% plugin(s) to %PLUGINS_DIR%
echo.
echo Plugin libraries:
dir /B "%PLUGINS_DIR%\*.dll"

echo.
echo You can now run the core application with: cargo run --bin cli

endlocal
