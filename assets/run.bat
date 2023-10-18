@echo off

set EXAMPLE_NAME=%1

if not exist bin (mkdir bin)
cargo objcopy --release --example %EXAMPLE_NAME% -- -O binary bin/%EXAMPLE_NAME%.bin

FOR /F "tokens=*" %%g IN ('python -m site --user-site') do (SET PYTHON_PATH=%%g)
python "%PYTHON_PATH%\bflb_mcu_tool" --chipname bl702 --firmware bin/%EXAMPLE_NAME%.bin