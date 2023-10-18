@echo off

set EXAMPLE_NAME=%1
set FOLDER=.bin

if not exist %FOLDER% (mkdir %FOLDER%)
cargo objcopy --release --example %EXAMPLE_NAME% -- -O binary %FOLDER%/%EXAMPLE_NAME%.bin

FOR /F "tokens=*" %%g IN ('python -m site --user-site') do (SET PYTHON_PATH=%%g)
python "%PYTHON_PATH%\bflb_mcu_tool" --chipname bl702 --firmware %FOLDER%/%EXAMPLE_NAME%.bin