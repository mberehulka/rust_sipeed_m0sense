EXAMPLE_NAME=$1
FOLDER=.bin

mkdir -p $FOLDER
export RUSTFLAGS="-Z macro-backtrace"
cargo objcopy --release --example $EXAMPLE_NAME -- -O binary $FOLDER/$EXAMPLE_NAME.bin

PYTHON_PATH=$(pip show bflb_mcu_tool | grep -oP "Location: \K[^\n]+")
sudo python3 $PYTHON_PATH/bflb_mcu_tool --chipname bl702 --firmware $FOLDER/$EXAMPLE_NAME.bin