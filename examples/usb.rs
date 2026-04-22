#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt_rtt as _;
use panic_probe as _;

use embedded_hal::delay::DelayNs;

use ra6m5_pac::{self as pac, NVIC};
use ra6m5_hal::{delay, fcache, mstp, sysc};
use ra6m5_hal::icu::{Icu, IcuEvent, iel::Iel10};
use ra6m5_hal::sysc::clock::{ClocksConfig, ClocksDiv, EK_RA6M5_XTAL_HZ, MoscConfig, MoscSource, PliDiv, PllConfig, PllMul, UsbClkConfig, UsbClkDiv};
use ra6m5_hal::usb::UsbPeripheral;
use ra6m5_hal::usb::interrupt::{ControlTransferStage, DescriptorType, DeviceState, StandardRequest, UsbInterrupts, VbusInput, on_usbi_interrupt};

use core::sync::atomic::{AtomicU32, Ordering};

static COUNT: AtomicU32 = AtomicU32::new(0);

defmt::timestamp!("{=u32}", COUNT.fetch_add(1, Ordering::Relaxed));

use pac::interrupt;
const USBFS_IRQ_CH: pac::Interrupt = pac::Interrupt::IEL10;

const DEVICE_DESCRIPTOR: [u8; 18] = [
    18,    // bLength
    0x01,  // bDescriptorType = Device
    0x00, 0x02, // bcdUSB = 2.00
    0x02,  // bDeviceClass (0 = each interface specifies)
    0x02,  // bDeviceSubClass
    0x01,  // bDeviceProtocol
    64,    // bMaxPacketSize0 (EP0 max packet)
    0x34, 0x12, // idVendor  = 0x1234 （仮）
    0x78, 0x56, // idProduct = 0x5678 （仮）
    0x00, 0x01, // bcdDevice = 0x0100
    0x01, // iManufacturer
    0x02, // iProduct
    0x03, // iSerialNumber
    0x01, // bNumConfigurations
];

#[interrupt]
fn IEL10() {
    if let Ok(irqs) = on_usbi_interrupt() {
        defmt_interrupt(&irqs);

        if let Some(packet) = irqs.usb_request {
            if let Some(std_req) = packet.to_standard_request() {
                match std_req {
                    StandardRequest::GetDescriptor {
                        descriptor_type, index, language_id, length
                    } => {
                        defmt::info!(
                            "[Main] Standard GET_DESCRIPTOR: {:?}, index={}, lang=0x{:04x}, len={}",
                            descriptor_type as u8, index, language_id, length
                        );

                        // ★ Device Descriptor だけに応答する
                        if descriptor_type == DescriptorType::Device && index == 0 {
                            // 実際に送る長さは wLength とディスクリプタ長の小さい方
                            let n = core::cmp::min(DEVICE_DESCRIPTOR.len(), length as usize);
                            let data = &DEVICE_DESCRIPTOR[..n];

                            defmt::info!("[Main] Send Device Descriptor ({} bytes)", n);
                            // usb.send_ep0_in(data).unwrap();
                        } else {
                            defmt::info!("[Main] (他のディスクリプタは未実装)");
                        }
                    }
                    StandardRequest::SetAddress { address } => {
                        defmt::info!(
                            "[Main] SET_ADDRESS requested: {} (STATUS後に反映予定)",
                            address
                        );
                        // ★ ここではまだ USBADDR に書かない。STATUSフェーズまで待つ必要があるため。
                    }
                    StandardRequest::SetConfiguration { configuration_value } => {
                        defmt::info!(
                            "[Main] SET_CONFIGURATION requested: {} (未実装)",
                            configuration_value
                        );
                    }
                    _ => {
                        defmt::info!("[Main] Standard request (未実装)");
                    }
                }
            } else { defmt::info!("[Main] Non-standard or unsupported request"); }
        }
    }
    Iel10::on_interrupt().unwrap();
}

#[entry]
fn main() -> ! {
    defmt::info!("usb_step0: start");

    let dp = pac::Peripherals::take().unwrap();

    let mut cp = cortex_m::Peripherals::take().unwrap();
    let mut clocks = sysc::Sysc::init(dp.SYSC).unwrap();
    
    let mut mstp = mstp::Mstp::init(dp.MSTP).unwrap();
    let mut fcache = fcache::Fcache::init(dp.FCACHE).unwrap();
    let mut iel = Icu::init(dp.ICU).unwrap();

    if clocks.mosc.is_running() { clocks.mosc.stop().unwrap(); }
    let mosc_config = MoscConfig::new(EK_RA6M5_XTAL_HZ, MoscSource::Oscillator).unwrap();
    clocks.mosc.set_config(&mosc_config).unwrap();
    clocks.mosc.run().unwrap();
    while !clocks.mosc.is_stable() {}

    if clocks.pll.is_running() { clocks.pll.stop().unwrap(); }
    let pll_config = PllConfig::new(&mut clocks.mosc, PliDiv::Div3, PllMul::Mul25_0).unwrap();
    clocks.pll.set_config(&pll_config).unwrap();
    clocks.pll.run().unwrap();
    while !clocks.pll.is_stable() {}

    let config = ClocksConfig::new(
        &mut clocks.pll, ClocksDiv::Div1, ClocksDiv::Div2, ClocksDiv::Div4, ClocksDiv::Div4, ClocksDiv::Div2, 
        ClocksDiv::Div4, ClocksDiv::Div2, true, false, false
    ).unwrap();

    clocks.set_config(&config, &mut mstp, &mut fcache, &mut cp.DCB, &mut cp.DWT).unwrap();

    let mut d = delay::Delay::new(cp.SYST, &clocks);

    if clocks.pll2.is_running() { clocks.pll2.stop().unwrap(); }
    let pll2_config = PllConfig::new(&mut clocks.mosc, PliDiv::Div2, PllMul::Mul12_0).unwrap();
    clocks.pll2.set_config(&pll2_config).unwrap();
    clocks.pll2.run().unwrap();
    while !clocks.pll2.is_stable() {}

    let usbclk_config = UsbClkConfig::new(&mut clocks.pll2, UsbClkDiv::Div3).unwrap();
    clocks.usbclk.set_config(usbclk_config).unwrap();


    
    let mut usb = UsbPeripheral::<pac::Usbfs>::init(dp.USBFS).unwrap();
    defmt::info!("usb enable: {}", usb.is_enabled(&mut mstp));
    usb.enable(&mut mstp).unwrap();
    defmt::info!("usb enable: {}", usb.is_enabled(&mut mstp));

    defmt::info!("interrupt enable: {}", usb.is_usbi_interrupt_enabled());
    usb.enable_usbi_interrupt().unwrap();
    defmt::info!("interrupt enable: {}", usb.is_usbi_interrupt_enabled());

    unsafe {
        NVIC::unmask(USBFS_IRQ_CH);
    }

    defmt::info!("event number: {}", match iel.iel10.get_event(){ Some(v) => v as u32, None => 0xfff });
    iel.iel10.set_event(IcuEvent::USBFS0_USBI).unwrap();
    defmt::info!("event number: {}", iel.iel10.get_event().unwrap() as u32);

    usb.connect(&mut mstp).unwrap();
    defmt::info!("usb connected.");

    loop {
        usb.poll();

        
        defmt::info!("living");
        d.delay_ms(1000);
    }
}

fn defmt_interrupt(irqs: &UsbInterrupts) {
    defmt::info!("Interrupt fired!");
    if let Some(packet) = irqs.usb_request {
        defmt::info!(
            "[Interrupt] control setup: packet\nbm_request_type: {}\nb_requrest: {}\nw_value: {}\nw_index: {}\nw_length: {}",
            packet.bm_request_type,
            packet.b_request,
            packet.w_value,
            packet.w_index,
            packet.w_length
        );
        if let Some(std_req) = packet.to_standard_request() {
            match std_req {
                StandardRequest::GetDescriptor {
                    descriptor_type, 
                    index, 
                    language_id, 
                    length 
                } => {
                    defmt::info!(
                        "[Interrupt] Standard GET_DESCRIPTOR: {:?}, index={}, lang=0x{:04x}, len={}",
                        descriptor_type as u8,
                        index,
                        language_id,
                        length
                    );
                }
                StandardRequest::SetAddress { address } => {
                    defmt::info!(
                        "[Interrupt] Standard SET_ADDRESS: {}",
                        address
                    )
                }
                StandardRequest::SetConfiguration { configuration_value } => {
                    defmt::info!(
                        "[Interrupt] Standard SET_CONFIGURATION: {}",
                        configuration_value
                    )
                }
                _ => {
                    defmt::info!("[Interrupt] Standard request (not implemented)");
                }
            }
        } else {
            defmt::info!("[Interrupt] Non-standard or unsupported request");
        }
    }
    if let Some(()) = irqs.buffer_ready { defmt::info!("[Interrupt] buffer ready."); }
    if let Some(()) = irqs.not_ready { defmt::info!("[Interrupt] not ready."); }
    if let Some(()) = irqs.buffer_empty { defmt::info!("[Interrupt] buffer empty."); }
    if let Some(stage) = irqs.control_transfer_stage_transition {
        defmt::info!("[Interrupt] control transfer state: {}", match stage {
            ControlTransferStage::IdleSetup => "Idle or Setup",
            ControlTransferStage::ReadData => "Read data",
            ControlTransferStage::ReadStatus => "Read status",
            ControlTransferStage::WriteData => "Write data",
            ControlTransferStage::WriteStatus => "Write status",
            ControlTransferStage::WriteNoDataStatus => "Write (no data) status",
            ControlTransferStage::TransferSequenceError => "Transfer squence error",
        });
    }
    if let Some(state) = irqs.device_state_transition {
        defmt::info!("[Interrupt] device state transition: {}", match state {
            DeviceState::Powered => "Powered", 
            DeviceState::Default => "Default", 
            DeviceState::Address => "Address", 
            DeviceState::Configured => "Configured",
            DeviceState::Suspended => "Suspended",
        });
    }
    if let Some(()) = irqs.start_of_frame { defmt::info!("[Interrupt] start of time."); }
    if let Some(()) = irqs.resume { defmt::info!("[Interrupt] resume.") }
    if let Some(input) = irqs.vbus {
        defmt::info!("[Interrupt] vbus: {}", match input {
            VbusInput::Low => "Low",
            VbusInput::High => "High",
        })
    }
}