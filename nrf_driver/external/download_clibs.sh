DRIVER_DIR=nrf-ble-driver
VERSION=4.1.2
WIN_PACKAGE=nrf-ble-driver-$VERSION-win_x86_64.zip
LINUX_PACKAGE=nrf-ble-driver-$VERSION-linux_x86_64.tar.gz
MAC_PACKAGE=nrf-ble-driver-$VERSION-macos_x86_64.tar.gz

echo "Making $DRIVER_DIR"
mkdir $DRIVER_DIR
echo "Getting packages..."
wget -P $DRIVER_DIR \
    https://github.com/NordicSemiconductor/pc-ble-driver/releases/download/v$VERSION/$WIN_PACKAGE \
    https://github.com/NordicSemiconductor/pc-ble-driver/releases/download/v$VERSION/$LINUX_PACKAGE \
    https://github.com/NordicSemiconductor/pc-ble-driver/releases/download/v$VERSION/$MAC_PACKAGE \

echo "Unzipping packages"
unzip $DRIVER_DIR/$WIN_PACKAGE -d $DRIVER_DIR
tar xf $DRIVER_DIR/$LINUX_PACKAGE -C $DRIVER_DIR
tar xf $DRIVER_DIR/$MAC_PACKAGE -C $DRIVER_DIR

echo "Cleaning up"
rm $DRIVER_DIR/$WIN_PACKAGE
rm $DRIVER_DIR/$LINUX_PACKAGE
rm $DRIVER_DIR/$MAC_PACKAGE
