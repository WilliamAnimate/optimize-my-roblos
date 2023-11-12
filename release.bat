@echo off

set /p "option=build with self-built STD? [Y/n] "

if not defined option (
	exit 1
)

echo %option%
set o=2

if %option% EQU y (
	set o=1
) else if %option% EQU Y (
	set o=1
) else if %option% EQU n (
	set o=0
) else if %option% EQU N (
	set o=0
) else (
	exit 0
)


echo %o%

if o EQU 1 (
	cd src-tauri
	cargo +nightly build -Z build-std=std,panic_abort --target x86_64-pc-windows-msvc --release
) else if o EQU 0 (
	npm run tauri build
)

:: this does not build with tauri... probably fine?