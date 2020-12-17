//! Initializes course work particular interrupts. Touching not recommended.
// Use board support package (BSP) foreign-function interface (FFI) functions,
// eg. for initializing interrupts
use crate::{button_handler, tick_handler, tick_handler_1, xil};
use core::{ffi::c_void, ptr::null_mut};

struct TmrCntrSetup {
    output_hz: u32,           /* Output frequency */
    interval: xil::XInterval, /* interval value */
    prescaler: u8,            /* prescaler value */
    options: u16,             /* Option settings */
}

// First timer
const TTC_TICK_DEVICE_ID: u32 = xil::XPAR_XTTCPS_0_DEVICE_ID;
const TTC_TICK_INTR_ID: u32 = xil::XPAR_XTTCPS_0_INTR;
static mut TICKSETUP: TmrCntrSetup = TmrCntrSetup {
    output_hz: 800,
    interval: 0,
    prescaler: 0,
    options: 0,
}; //800hz

// Second timer
const TTC_TICK_DEVICE1_ID: u32 = xil::XPAR_XTTCPS_1_DEVICE_ID;
const TTC_TICK_INTR1_ID: u32 = xil::XPAR_XTTCPS_1_INTR;
static mut TICKSETUP_1: TmrCntrSetup = TmrCntrSetup {
    output_hz: 10,
    interval: 0,
    prescaler: 0,
    options: 0,
}; //10hz

// GPIO definitions
const GPIO_BANK: u32 = xil::XGPIOPS_BANK2;
const GPIO_DEVICE_ID: u32 = xil::XPAR_XGPIOPS_0_DEVICE_ID;
const INTC_DEVICE_ID: u32 = xil::XPAR_SCUGIC_SINGLE_DEVICE_ID;
const GPIO_INTERRUPT_ID: u32 = xil::XPAR_XGPIOPS_0_INTR;

static mut TTC_PS_INST: xil::XTtcPs = xil::XTtcPs {
    Config: xil::XTtcPs_Config {
        DeviceId: 0,
        BaseAddress: 0,
        InputClockHz: 0,
    },
    IsReady: 0,
};

static mut TTC_PS_INST_1: xil::XTtcPs = xil::XTtcPs {
    Config: xil::XTtcPs_Config {
        DeviceId: 0,
        BaseAddress: 0,
        InputClockHz: 0,
    },
    IsReady: 0,
};

/* The Instance of the GPIO Driver */
static mut GPIO: xil::XGpioPs = xil::XGpioPs {
    GpioConfig: xil::XGpioPs_Config {
        DeviceId: 0,
        BaseAddr: 0,
    },
    IsReady: 0,
    Handler: None,
    CallBackRef: null_mut(),
    Platform: 0,
    MaxPinNum: 0,
    MaxBanks: 0,
};

/* The Instance of the Interrupt Controller Driver */
static mut INTC: xil::XScuGic = xil::XScuGic {
    Config: null_mut(),
    IsReady: 0,
    UnhandledInterrupts: 0,
};

/// Initialize the board interrupts specific to this project
pub fn init() {
    setup_gpio_interrupt_system();
    setup_timer_int();
    setup_timer_int1();
}

unsafe extern "C" fn scu_gic_intr_h_cb(data: *mut c_void) {
    xil::XScuGic_InterruptHandler(data as *mut xil::XScuGic);
}

unsafe extern "C" fn gpio_ps_intr_h_cb(data: *mut c_void) {
    xil::XGpioPs_IntrHandler(data as *mut xil::XGpioPs);
}

fn setup_timer_int() {
    unsafe {
        let config = xil::XTtcPs_LookupConfig(TTC_TICK_DEVICE_ID as u16);
        xil::XTtcPs_CfgInitialize((&mut TTC_PS_INST) as *mut _, config, (*config).BaseAddress);

        let timer_setup = &mut TICKSETUP;
        // Set options, no PWM output , interrupt with interval
        timer_setup.options |=
            xil::XTTCPS_OPTION_INTERVAL_MODE as u16 | xil::XTTCPS_OPTION_WAVE_DISABLE as u16;
        xil::XTtcPs_SetOptions((&mut TTC_PS_INST) as *mut _, timer_setup.options as u32);

        xil::XTtcPs_CalcIntervalFromFreq(
            (&mut TTC_PS_INST) as *mut _,
            timer_setup.output_hz,
            (&mut timer_setup.interval) as *mut xil::XInterval,
            (&mut timer_setup.prescaler) as *mut u8,
        );

        xil::XTtcPs_SetInterval((&mut TTC_PS_INST) as *mut _, timer_setup.interval as u32);
        xil::XTtcPs_SetPrescaler((&mut TTC_PS_INST) as *mut _, timer_setup.prescaler);

        xil::XScuGic_Connect(
            (&mut INTC) as *mut _,
            TTC_TICK_INTR_ID,
            Some(tick_handler),
            (&mut TTC_PS_INST) as *mut _ as *mut c_void,
        );
        xil::XScuGic_Enable((&mut INTC) as *mut _, TTC_TICK_INTR_ID);

        xil::XTtcPs_EnableInterrupts((&mut TTC_PS_INST) as *mut _, xil::XTTCPS_IXR_INTERVAL_MASK);

        xil::XTtcPs_Start((&mut TTC_PS_INST) as *mut _);
    }
}

fn setup_timer_int1() {
    unsafe {
        let config: *mut xil::XTtcPs_Config = xil::XTtcPs_LookupConfig(TTC_TICK_DEVICE1_ID as u16);
        xil::XTtcPs_CfgInitialize(
            (&mut TTC_PS_INST_1) as *mut _,
            config,
            (*config).BaseAddress,
        );

        let timer_setup = &mut TICKSETUP_1;
        // Set options, no PWM output , interrupt with interval
        timer_setup.options |=
            xil::XTTCPS_OPTION_INTERVAL_MODE as u16 | xil::XTTCPS_OPTION_WAVE_DISABLE as u16;
        xil::XTtcPs_SetOptions((&mut TTC_PS_INST_1) as *mut _, timer_setup.options as u32);

        xil::XTtcPs_CalcIntervalFromFreq(
            (&mut TTC_PS_INST_1) as *mut _,
            timer_setup.output_hz,
            (&mut timer_setup.interval) as *mut xil::XInterval,
            (&mut timer_setup.prescaler) as *mut u8,
        );

        xil::XTtcPs_SetInterval((&mut TTC_PS_INST_1) as *mut _, timer_setup.interval as u32);
        xil::XTtcPs_SetPrescaler((&mut TTC_PS_INST_1) as *mut _, timer_setup.prescaler);

        xil::XScuGic_Connect(
            (&mut INTC) as *mut _,
            TTC_TICK_INTR1_ID,
            Some(tick_handler_1),
            (&mut TTC_PS_INST_1) as *mut _ as *mut c_void,
        );
        xil::XScuGic_Enable((&mut INTC) as *mut _, TTC_TICK_INTR1_ID);

        xil::XTtcPs_EnableInterrupts(
            (&mut TTC_PS_INST_1) as *mut _,
            xil::XTTCPS_IXR_INTERVAL_MASK,
        );

        xil::XTtcPs_Start((&mut TTC_PS_INST_1) as *mut _);
    }
}

// TODO: initializes both interrupts and gpio, which is confusing; separate them
fn setup_gpio_interrupt_system() -> u32 {
    unsafe {
        let config_ptr = xil::XGpioPs_LookupConfig(GPIO_DEVICE_ID as u16);

        if config_ptr.is_null() {
            return xil::XST_FAILURE;
        }

        xil::XGpioPs_CfgInitialize((&mut GPIO) as *mut _, config_ptr, (*config_ptr).BaseAddr);

        xil::Xil_ExceptionInit();

        let intc_config = xil::XScuGic_LookupConfig(INTC_DEVICE_ID as u16);
        xil::XScuGic_CfgInitialize(
            (&mut INTC) as *mut _,
            intc_config,
            (*intc_config).CpuBaseAddress,
        );

        xil::Xil_ExceptionRegisterHandler(
            xil::XIL_EXCEPTION_ID_INT,
            Some(scu_gic_intr_h_cb),
            (&mut INTC) as *mut _ as *mut c_void,
        );

        // Connect GPIO handler to INTC
        xil::XScuGic_Connect(
            (&mut INTC) as *mut _,
            GPIO_INTERRUPT_ID,
            Some(gpio_ps_intr_h_cb),
            (&mut GPIO) as *mut _ as *mut c_void,
        );

        //set Interrupt type (in this case low edge, on buttons, both on switch). For
        // more info, see xilinx API Driver documentation
        xil::XGpioPs_SetIntrType((&mut GPIO) as *mut _, GPIO_BANK as u8, 0xFF, 0x00, 0x30);

        //connect handler to gpio driver
        xil::XGpioPs_SetCallbackHandler(
            (&mut GPIO) as *mut _,
            (&mut GPIO) as *mut _ as *mut c_void,
            Some(button_handler),
        );

        //enable interrputs connected to bank (2)
        xil::XGpioPs_IntrEnable((&mut GPIO) as *mut _, GPIO_BANK as u8, 0b111111);

        //enable GPIO interrupts
        xil::XScuGic_Enable((&mut INTC) as *mut _, GPIO_INTERRUPT_ID);

        //after init, clearing gpio0,1 int registers is mandatory, because for some
        // reason bank0 and 1 interrupts are set.
        xil::XGpioPs_IntrClear((&mut GPIO) as *mut _, 0, 0xffffffff);
        xil::XGpioPs_IntrClear((&mut GPIO) as *mut _, 1, 0xffffffff);
    }

    // Return 0 for success
    0
}
#[allow(dead_code)]
pub fn change_freq(freq: u32) {
    if freq != 0 {
        unsafe {
            let timer_setup = &mut TICKSETUP_1;

            timer_setup.output_hz = freq;

            xil::XTtcPs_CalcIntervalFromFreq(
                (&mut TTC_PS_INST_1) as *mut _,
                timer_setup.output_hz,
                (&mut timer_setup.interval) as *mut xil::XInterval,
                (&mut timer_setup.prescaler) as *mut u8,
            );

            xil::XTtcPs_SetInterval((&mut TTC_PS_INST_1) as *mut _, timer_setup.interval as u32);
            xil::XTtcPs_SetPrescaler((&mut TTC_PS_INST_1) as *mut _, timer_setup.prescaler);
        }
    }
}
