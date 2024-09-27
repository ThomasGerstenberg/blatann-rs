use crate::ffi;

#[repr(u8)]
#[derive(FromPrimitive, Copy, Clone, Debug)]
pub enum BleGapAddressType {
    Public = ffi::BLE_GAP_ADDR_TYPE_PUBLIC as u8,
    Static = ffi::BLE_GAP_ADDR_TYPE_RANDOM_STATIC as u8,
    PrivateResolvable = ffi::BLE_GAP_ADDR_TYPE_RANDOM_PRIVATE_RESOLVABLE as u8,
    PrivateNonresolvable = ffi::BLE_GAP_ADDR_TYPE_RANDOM_PRIVATE_NON_RESOLVABLE as u8,
}

#[repr(u8)]
#[derive(FromPrimitive, Copy, Clone, Debug)]
pub enum BleGapAdvertisingType {
    ConnectableUndirected = ffi::BLE_GAP_ADV_TYPE_ADV_IND as u8,
    ConnectableDirected = ffi::BLE_GAP_ADV_TYPE_ADV_DIRECT_IND as u8,
    ScannableUndirected = ffi::BLE_GAP_ADV_TYPE_ADV_SCAN_IND as u8,
    NonconnectableUndirected = ffi::BLE_GAP_ADV_TYPE_ADV_NONCONN_IND as u8,
    ScanResponse = 0xFF,
}

#[repr(u8)]
#[derive(FromPrimitive, Copy, Clone, Debug)]
pub enum BleGapTimeoutSource {
    Advertising = ffi::BLE_GAP_TIMEOUT_SRC_ADVERTISING as u8,
    Scan = ffi::BLE_GAP_TIMEOUT_SRC_SCAN as u8,
    Conn = ffi::BLE_GAP_TIMEOUT_SRC_CONN as u8,
    AuthPayload = ffi::BLE_GAP_TIMEOUT_SRC_AUTH_PAYLOAD as u8,
}

#[repr(u8)]
#[derive(FromPrimitive, Copy, Clone, Debug)]
pub enum BleAdvDataType {
    Flags = ffi::BLE_GAP_AD_TYPE_FLAGS as u8,
    Service16bitUuidMoreAvailable = ffi::BLE_GAP_AD_TYPE_16BIT_SERVICE_UUID_MORE_AVAILABLE as u8,
    Service16bitUuidComplete = ffi::BLE_GAP_AD_TYPE_16BIT_SERVICE_UUID_COMPLETE as u8,
    Service32bitUuidMoreAvailable = ffi::BLE_GAP_AD_TYPE_32BIT_SERVICE_UUID_MORE_AVAILABLE as u8,
    Service32bitUuidComplete = ffi::BLE_GAP_AD_TYPE_32BIT_SERVICE_UUID_COMPLETE as u8,
    Service128bitUuidMoreAvailable = ffi::BLE_GAP_AD_TYPE_128BIT_SERVICE_UUID_MORE_AVAILABLE as u8,
    Service128bitUuidComplete = ffi::BLE_GAP_AD_TYPE_128BIT_SERVICE_UUID_COMPLETE as u8,
    ShortLocalName = ffi::BLE_GAP_AD_TYPE_SHORT_LOCAL_NAME as u8,
    CompleteLocalName = ffi::BLE_GAP_AD_TYPE_COMPLETE_LOCAL_NAME as u8,
    TxPowerLevel = ffi::BLE_GAP_AD_TYPE_TX_POWER_LEVEL as u8,
    ClassOfDevice = ffi::BLE_GAP_AD_TYPE_CLASS_OF_DEVICE as u8,
    SimplePairingHashC = ffi::BLE_GAP_AD_TYPE_SIMPLE_PAIRING_HASH_C as u8,
    SimplePairingRandomizerR = ffi::BLE_GAP_AD_TYPE_SIMPLE_PAIRING_RANDOMIZER_R as u8,
    SecurityManagerTkValue = ffi::BLE_GAP_AD_TYPE_SECURITY_MANAGER_TK_VALUE as u8,
    SecurityManagerOobFlags = ffi::BLE_GAP_AD_TYPE_SECURITY_MANAGER_OOB_FLAGS as u8,
    SlaveConnectionIntervalRange = ffi::BLE_GAP_AD_TYPE_SLAVE_CONNECTION_INTERVAL_RANGE as u8,
    SolicitedSeviceUuids16bit = ffi::BLE_GAP_AD_TYPE_SOLICITED_SERVICE_UUIDS_16BIT as u8,
    SolicitedSeviceUuids128bit = ffi::BLE_GAP_AD_TYPE_SOLICITED_SERVICE_UUIDS_128BIT as u8,
    ServiceData = ffi::BLE_GAP_AD_TYPE_SERVICE_DATA as u8,
    PublicTargetAddress = ffi::BLE_GAP_AD_TYPE_PUBLIC_TARGET_ADDRESS as u8,
    RandomTargetAddress = ffi::BLE_GAP_AD_TYPE_RANDOM_TARGET_ADDRESS as u8,
    Appearance = ffi::BLE_GAP_AD_TYPE_APPEARANCE as u8,
    AdvertisingInterval = ffi::BLE_GAP_AD_TYPE_ADVERTISING_INTERVAL as u8,
    LeBluetoothDeviceAddress = ffi::BLE_GAP_AD_TYPE_LE_BLUETOOTH_DEVICE_ADDRESS as u8,
    LeRole = ffi::BLE_GAP_AD_TYPE_LE_ROLE as u8,
    SimplePairngHashC256 = ffi::BLE_GAP_AD_TYPE_SIMPLE_PAIRING_HASH_C256 as u8,
    SimplePairngRandomizerR256 = ffi::BLE_GAP_AD_TYPE_SIMPLE_PAIRING_RANDOMIZER_R256 as u8,
    ServiceData32bitUuid = ffi::BLE_GAP_AD_TYPE_SERVICE_DATA_32BIT_UUID as u8,
    ServiceData128bitUuid = ffi::BLE_GAP_AD_TYPE_SERVICE_DATA_128BIT_UUID as u8,
    Uri = ffi::BLE_GAP_AD_TYPE_URI as u8,
    Information3dData = ffi::BLE_GAP_AD_TYPE_3D_INFORMATION_DATA as u8,
    ManufacturerSpecificData = ffi::BLE_GAP_AD_TYPE_MANUFACTURER_SPECIFIC_DATA as u8,
}

impl From<BleAdvDataType> for u8 {
    fn from(value: BleAdvDataType) -> Self {
        value as u8
    }
}

#[repr(u8)]
#[derive(FromPrimitive, Copy, Clone, Debug)]
pub enum BleGapRole {
    Invalid = ffi::BLE_GAP_ROLE_INVALID as u8,
    Peripheral = ffi::BLE_GAP_ROLE_PERIPH as u8,
    Central = ffi::BLE_GAP_ROLE_CENTRAL as u8,
}

bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct BleGapPhy: u8 {
        const AUTO = ffi::BLE_GAP_PHY_AUTO as u8;
        const ONE_MBPS = ffi::BLE_GAP_PHY_1MBPS as u8;
        const TWO_MBPS = ffi::BLE_GAP_PHY_2MBPS as u8;
        const CODED = ffi::BLE_GAP_PHY_CODED as u8;
    }
}

impl BleGapPhy {
    pub fn from_bits_or_default(value: u8) -> Self {
        Self::from_bits(value).unwrap_or_else(|| BleGapPhy::AUTO)
    }
}
