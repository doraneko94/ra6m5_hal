pub struct Descriptor {
    descriptor: DescriptorType,
    is_little_endian: bool,
}

impl Descriptor {
    pub fn data(&self) -> [u8; 255] {
        let mut data = [0u8; 255];
        data[1] = self.descriptor.to_u8();

        match self.descriptor {
            DescriptorType::Device { bcd_usb, device_class } => {
                data[0] = 18;
                _set(&mut data[2..4], bcd_usb as u16, self.is_little_endian);
                device_class.set(&mut data[4..6]);
            }
            _ => {}
        }
        data
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DescriptorType {
    Device {
        bcd_usb: BcdUsb,
        device_class: DeviceClass,
    },
    Configuration,
    String,
    Interface,
    Endpoint,
    DeviceQualifier,
    OtherSpeedConfiguration,
    InterfacePower,
    Otg,
    Debug,
    InterfaceAssociationDescriptor,
    Bos,
    DeviceCapability
}
impl DescriptorType {
    pub fn to_u8(&self) -> u8 {
        match self {
            Self::Device{ .. }=>0x01, Self::Configuration=>0x02, Self::String=>0x03,
            Self::Interface=>0x04, Self::Endpoint=>0x05, Self::DeviceQualifier=>0x06,
            Self::OtherSpeedConfiguration=>0x07, Self::InterfacePower=>0x08, Self::Otg=>0x09,
            Self::Debug=>0x10, Self::InterfaceAssociationDescriptor=>0x11, Self::Bos=>0x15,
            Self::DeviceCapability=>0x16
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BcdUsb {
    Usb1_0 = 0x100,
    Usb1_1 = 0x110,
    Usb2_0 = 0x200,
    Usb2_0Bos = 0x201,
    Usb3_0To2_0 = 0x210,
    Usb3_0 = 0x300,
    EnhancedSuperSpeed = 0x310
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DeviceClass {
    InterfaceDefined,
    Audio(AudioSubClass),
    Cdc(CdcSubClass),
    Hid(HidSubClass),
    Physical(PhysicalSubClass),
    Imaging(ImagingSubClass),
    Printer(PrinterSubClass),
    MassStorage(MassStorageSubClass),
    Hub(HubSubClass),
    CdcData(CdcDataSubClass),
    SmartCard(SmartCardSubClass),
    ContentSecurity(ContentSecuritySubClass),
    Video(VideoSubClass),
    PersonalHealthcare(PersonalHealthcareSubClass),
    Av(AvSubClass),
    Billboard(BillboardSubClass),
    UsbTypeCBridge(UsbTypeCBridgeSubClass),
    DiagnosisDevice,
    Bluetooth,
    InterfaceAssociationDescriptor(InterfaceAssociationDescriptorSubClass),
    
    ApplicationSpecific(ApplicationSpecificSubClass),
    VendorSpecific(VendorSpecificSubClass)
}
impl DeviceClass {
    pub fn set(&self, buf: &mut [u8]) {
        match self {
            Self::InterfaceDefined => { buf[0] = 0x00; buf[1] = 0x00; }
            Self::Audio(sub) => { buf[0] = 0x01; buf[1] = *sub as u8; }
            Self::Cdc(sub) => { buf[0] = 0x02; buf[1] = *sub as u8; }
            Self::Hid(sub) => { buf[0] = 0x03; buf[1] = *sub as u8; }
            Self::Physical(sub) => { buf[0] = 0x05; buf[1] = *sub as u8; }
            Self::Imaging(sub) => { buf[0] = 0x06; buf[1] = *sub as u8; }
            Self::Printer(sub) => { buf[0] = 0x07; buf[1] = *sub as u8; }
            Self::MassStorage(sub) => { buf[0] = 0x08; buf[1] = *sub as u8; }
            Self::Hub(sub) => { buf[0] = 0x09; buf[1] = *sub as u8; }
            Self::CdcData(sub) => { buf[0] = 0x0a; buf[1] = *sub as u8; }
            Self::SmartCard(sub) => { buf[0] = 0x0b; buf[1] = *sub as u8; }
            Self::ContentSecurity(sub) => { buf[0] = 0x0d; buf[1] = *sub as u8; }
            Self::Video(sub) => { buf[0] = 0x0e; buf[1] = *sub as u8; }
            Self::PersonalHealthcare(sub) => { buf[0] = 0x0f; buf[1] = *sub as u8; }
            Self::Av(sub) => { buf[0] = 0x10; buf[1] = *sub as u8; }
            Self::Billboard(sub) => { buf[0] = 0x11; buf[1] = *sub as u8; }
            Self::UsbTypeCBridge(sub) => { buf[0] = 0x12; buf[1] = *sub as u8; }
            Self::DiagnosisDevice => { buf[0] = 0xdc; buf[1] = 0x00; }
            Self::Bluetooth => { buf[0] = 0xe0; buf[1] = 0x00; }
            Self::InterfaceAssociationDescriptor(sub) => { buf[0] = 0xef; buf[1] = *sub as u8; }
            Self::ApplicationSpecific(sub) => { buf[0] = 0xfe; buf[1] = *sub as u8; }
            Self::VendorSpecific(sub) => { buf[0] = 0xff; buf[1] = *sub as u8; }
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AudioSubClass {
    Undefined=0x00, AudioControl=0x01, AudioStreaming=0x02, MidiStreaming=0x03
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CdcSubClass {
    Undefined=0x00, DirectLineControl=0x01, Acm=0x02, TelephoneControl=0x03,
    MultiChannelControl=0x04, CapiControl=0x05, EthernetNetworking=0x06, AtmNetworking=0x07,
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum HidSubClass {
    None=0x00, BootInterfaceSubclass=0x01,
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PhysicalSubClass {
    PhysicalInterfaceDevice=0x00,
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ImagingSubClass {
    StillImageCapture=0x01, Video=0x02, Scanner=0x03,
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PrinterSubClass {
    Printer=0x01, PrintingSupport=0x02, VendorSpecific=0xff,
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MassStorageSubClass {
    Undefined=0x00, Rbc=0x01, Atapi=0x02, Qic157=0x03,
    Ufi=0x04, Sff8070i=0x05, ScsiTransparentCommandSet=0x06,
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum HubSubClass {
    FullSpeedHub=0x00, HiSpeedHub=0x01,
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CdcDataSubClass {
    None=0x00,
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SmartCardSubClass {
    SmartCard=0x00
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ContentSecuritySubClass {
    ContentSecurity=0x00
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum VideoSubClass {
    Undefined=0x00, VideoControl=0x01, VideoStreaming=0x02
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PersonalHealthcareSubClass {
    PersonalHealthcare=0x00
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AvSubClass {
    AvControl=0x00
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BillboardSubClass {
    BillboardDevice=0x00
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UsbTypeCBridgeSubClass {
    UsbTypeCBridge=0x00
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum InterfaceAssociationDescriptorSubClass {
    InterfaceAssociationDescriptor=0x02
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ApplicationSpecificSubClass {
    None=0x00
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum VendorSpecificSubClass {
    VendorSpecific=0xff
}

fn _set(buf: &mut [u8], value: u16, is_little_endian: bool) {
    let head = (value >> 8) as u8;
    let tail = (value & 0xff) as u8;
    (buf[0], buf[1]) = if is_little_endian { (tail, head) } else { (head, tail) }
}