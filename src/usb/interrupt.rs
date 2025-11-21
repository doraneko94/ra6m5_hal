use crate::pac;
use crate::RegisterError;
use super::USBFS_CELL;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct UsbInterrupts {
    pub usb_request: Option<SetupPacket>,
    pub buffer_ready: Option<()>,
    pub not_ready: Option<()>,
    pub buffer_empty: Option<()>,
    pub control_transfer_stage_transition: Option<ControlTransferStage>,
    pub device_state_transition: Option<DeviceState>,
    pub start_of_frame: Option<()>,
    pub resume: Option<()>,
    pub vbus: Option<VbusInput>,
}

impl UsbInterrupts {
    fn _new() -> Self {
        Self {
            usb_request: None, buffer_ready: None, not_ready: None, buffer_empty: None,
            control_transfer_stage_transition: None, device_state_transition: None, 
            start_of_frame: None, resume: None, vbus: None
        }
    } 
}

pub fn on_usbi_interrupt() -> Result<UsbInterrupts, RegisterError> {
    let mut irqs = UsbInterrupts::_new();
    critical_section::with(|cs| unsafe {
        let mut bor = USBFS_CELL.borrow(cs).borrow_mut();
        let usbfs = bor.as_mut().expect("Usbfs is not initialized");

        let r0 = usbfs.intsts0().read();
        let mut reset0 = r0
            .valid().set(pac::usbfs::intsts0::Valid::_1)
            .ctrt().set(pac::usbfs::intsts0::Ctrt::_1)
            .dvst().set(pac::usbfs::intsts0::Dvst::_1)
            .sofr().set(pac::usbfs::intsts0::Sofr::_1)
            .resm().set(pac::usbfs::intsts0::Resm::_1)
            .vbint().set(pac::usbfs::intsts0::Vbint::_1);
        if r0.valid().get() == pac::usbfs::intsts0::Valid::_1 {
            let usb_req = usbfs.usbreq().read();
            let bm_request_type = usb_req.bmrequesttype().get();
            let b_request = usb_req.brequest().get();
            let w_value = usbfs.usbval().read().wvalue().get();
            let w_index = usbfs.usbindx().read().windex().get();
            let w_length = usbfs.usbleng().read().wlentuh().get();

            let setup = SetupPacket {
                bm_request_type,
                b_request,
                w_value,
                w_index,
                w_length
            };

            irqs.usb_request = Some(setup);

            //Self::_store_last_setup(setup);
            //self._store_last_setup(steup);

            reset0 = reset0.valid().set(pac::usbfs::intsts0::Valid::_0);
        }
        if r0.brdy().get() == pac::usbfs::intsts0::Brdy::_1 { irqs.buffer_ready = Some(()); }
        if r0.nrdy().get() == pac::usbfs::intsts0::Nrdy::_1 { irqs.not_ready = Some(()); }
        if r0.bemp().get() == pac::usbfs::intsts0::Bemp::_1 { irqs.buffer_empty = Some(()); }
        if r0.ctrt().get() == pac::usbfs::intsts0::Ctrt::_1 {
            irqs.control_transfer_stage_transition = ControlTransferStage::from_u8(r0.ctsq().get().0);
            reset0 = reset0.ctrt().set(pac::usbfs::intsts0::Ctrt::_0);
        }
        if r0.dvst().get() == pac::usbfs::intsts0::Dvst::_1 {
            irqs.device_state_transition = DeviceState::from_u8(r0.dvsq().get().0);
            reset0 = reset0.dvst().set(pac::usbfs::intsts0::Dvst::_0);
        }
        if r0.sofr().get() == pac::usbfs::intsts0::Sofr::_1 {
            irqs.start_of_frame = Some(());
            reset0 = reset0.sofr().set(pac::usbfs::intsts0::Sofr::_0);
        }
        if r0.resm().get() == pac::usbfs::intsts0::Resm::_1 {
            irqs.resume = Some(());
            reset0 = reset0.resm().set(pac::usbfs::intsts0::Resm::_0);
        }
        if r0.vbint().get() == pac::usbfs::intsts0::Vbint::_1 {
            irqs.vbus = VbusInput::from_u8(r0.vbint().get().0);
            reset0 = reset0.vbint().set(pac::usbfs::intsts0::Vbint::_0);
        }
        usbfs.intsts0().write(reset0);
    });
    Ok(irqs)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct  SetupPacket {
    pub bm_request_type: u8,
    pub b_request: u8,
    pub w_value: u16,
    pub w_index: u16,
    pub w_length: u16
}
impl SetupPacket {
    pub fn to_standard_request(&self) -> Option<StandardRequest> {
        let bm = self.bm_request_type;
        let req = self.b_request;
        let w_value = self.w_value;
        let w_index = self.w_index;
        let w_length = self.w_length;

        let _direction_in = (bm & 0x80) != 0;
        let request_type = (bm >> 5) & 0b11;
        let _recipient = bm & 0x1f;

        if request_type != 0 { return None; }

        match req {
            0x00 => Some(StandardRequest::GetStatus),
            0x01 => Some(StandardRequest::ClearFeature),
            0x03 => Some(StandardRequest::SetFeature),
            0x05 => {
                let addr = (w_value & 0x7f) as u8;
                Some(StandardRequest::SetAddress { address: addr })
            }
            0x06 => {
                let desc_type = ((w_value >> 8) & 0xff) as u8;
                let desc_index = (w_value & 0xff) as u8;

                let descriptor_type = match DescriptorType::from_u8(desc_type) {
                    Some(ty) => ty,
                    None => return None,
                };
                Some(StandardRequest::GetDescriptor {
                    descriptor_type,
                    index: desc_index, 
                    language_id: w_index, 
                    length: w_length
                })
            }
            0x07 => Some(StandardRequest::SetDescriptor),
            0x08 => Some(StandardRequest::GetConfiguration),
            0x09 => {
                let cfg = (w_value & 0xff) as u8;
                Some(StandardRequest::SetConfiguration {
                    configuration_value: cfg,
                })
            }
            0x0A => Some(StandardRequest::GetInterface),
            0x0B => Some(StandardRequest::SetInterface),
            0x0C => Some(StandardRequest::SyncFrame),
            _ => None
        }
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DescriptorType {
    Device = 1,
    Configuration = 2,
    String = 3,
    Interface = 4,
    Endpoint = 5,
    DeviceQualifier = 6,
    OtherSpeedConfiguration = 7,
    InterfacePower = 8,
    OtgDescriptor = 9,
    Debug = 10,
    InterfaceAssociationDescriptor = 11,
    BosDescriptor = 15,
    DeviceCapability = 16,
}
impl DescriptorType {
    #[inline] pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            1=>Some(Self::Device), 2=>Some(Self::Configuration), 3=>Some(Self::String),
            4=>Some(Self::Interface), 5=>Some(Self::Endpoint), 6=>Some(Self::DeviceQualifier),
            7=>Some(Self::OtherSpeedConfiguration), 8=>Some(Self::InterfacePower), 9=>Some(Self::OtgDescriptor),
            10=>Some(Self::Debug), 11=>Some(Self::InterfaceAssociationDescriptor), 15=>Some(Self::BosDescriptor),
            16=>Some(Self::DeviceCapability), _=>None
        }
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum StandardRequest {
    GetStatus, // = 0x00,
    ClearFeature, // = 0x01,
    SetFeature, // = 0x03,
    SetAddress { address: u8 }, // = 0x05,
    GetDescriptor {
        descriptor_type: DescriptorType,
        index: u8,
        language_id: u16,
        length: u16
    }, // = 0x06,
    SetDescriptor, // = 0x07,
    GetConfiguration, // = 0x08,
    SetConfiguration { configuration_value: u8 }, // = 0x09,
    GetInterface, // = 0x0a,
    SetInterface, // = 0x0b,
    SyncFrame, // = 0x0c,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ControlTransferStage {
    IdleSetup = 0b000,
    ReadData = 0b001,
    ReadStatus = 0b010,
    WriteData = 0b011,
    WriteStatus = 0b100,
    WriteNoDataStatus = 0b101,
    TransferSequenceError = 0b110
}
impl ControlTransferStage {
    #[inline] pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0b000=>Some(Self::IdleSetup), 0b001=>Some(Self::ReadData), 0b010=>Some(Self::ReadStatus), 0b011=>Some(Self::WriteData),
            0b100=>Some(Self::WriteStatus), 0b101=>Some(Self::WriteNoDataStatus), 0b110=>Some(Self::TransferSequenceError), _=>None
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DeviceState { Powered=0b000, Default=0b001, Address=0b010, Configured=0b011, Suspended=0b100 }
impl DeviceState {
    #[inline] pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0b000=>Some(Self::Powered), 0b001=>Some(Self::Default), 0b010=>Some(Self::Address),
            0b011=>Some(Self::Configured), 0b100|0b101|0b110|0b111=>Some(Self::Suspended), _=>None
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum VbusInput { Low=0, High=1 }
impl VbusInput {
    #[inline] pub fn from_u8(value: u8) -> Option<Self> {
        match value { 0=>Some(Self::Low), 1=>Some(Self::High), _=>None }
    }
}