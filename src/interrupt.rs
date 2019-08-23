#[cfg(feature = "rt")]
global_asm!(
    "
                DH_TRAMPOLINE:
                    br #DEFAULT_HANDLER
                "
);
#[cfg(feature = "rt")]
global_asm ! ( "\n.weak PORT4\nPORT4 = DH_TRAMPOLINE\n.weak PORT3\nPORT3 = DH_TRAMPOLINE\n.weak PORT2\nPORT2 = DH_TRAMPOLINE\n.weak PORT1\nPORT1 = DH_TRAMPOLINE\n.weak SAC1_SAC3\nSAC1_SAC3 = DH_TRAMPOLINE\n.weak SAC0_SAC2\nSAC0_SAC2 = DH_TRAMPOLINE\n.weak ECOMP0_ECOMP1\nECOMP0_ECOMP1 = DH_TRAMPOLINE\n.weak ADC\nADC = DH_TRAMPOLINE\n.weak EUSCI_B1\nEUSCI_B1 = DH_TRAMPOLINE\n.weak EUSCI_B0\nEUSCI_B0 = DH_TRAMPOLINE\n.weak EUSCI_A1\nEUSCI_A1 = DH_TRAMPOLINE\n.weak EUSCI_A0\nEUSCI_A0 = DH_TRAMPOLINE\n.weak WDT\nWDT = DH_TRAMPOLINE\n.weak RTC\nRTC = DH_TRAMPOLINE\n.weak TIMER3_B1\nTIMER3_B1 = DH_TRAMPOLINE\n.weak TIMER3_B0\nTIMER3_B0 = DH_TRAMPOLINE\n.weak TIMER2_B1\nTIMER2_B1 = DH_TRAMPOLINE\n.weak TIMER2_B0\nTIMER2_B0 = DH_TRAMPOLINE\n.weak TIMER1_B1\nTIMER1_B1 = DH_TRAMPOLINE\n.weak TIMER1_B0\nTIMER1_B0 = DH_TRAMPOLINE\n.weak TIMER0_B1\nTIMER0_B1 = DH_TRAMPOLINE\n.weak TIMER0_B0\nTIMER0_B0 = DH_TRAMPOLINE\n.weak UNMI\nUNMI = DH_TRAMPOLINE\n.weak SYSNMI\nSYSNMI = DH_TRAMPOLINE" ) ;
#[cfg(feature = "rt")]
extern "msp430-interrupt" {
    fn PORT4();
    fn PORT3();
    fn PORT2();
    fn PORT1();
    fn SAC1_SAC3();
    fn SAC0_SAC2();
    fn ECOMP0_ECOMP1();
    fn ADC();
    fn EUSCI_B1();
    fn EUSCI_B0();
    fn EUSCI_A1();
    fn EUSCI_A0();
    fn WDT();
    fn RTC();
    fn TIMER3_B1();
    fn TIMER3_B0();
    fn TIMER2_B1();
    fn TIMER2_B0();
    fn TIMER1_B1();
    fn TIMER1_B0();
    fn TIMER0_B1();
    fn TIMER0_B0();
    fn UNMI();
    fn SYSNMI();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "msp430-interrupt" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
#[used]
pub static INTERRUPTS: [Vector; 45] = [
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: PORT4 },
    Vector { _handler: PORT3 },
    Vector { _handler: PORT2 },
    Vector { _handler: PORT1 },
    Vector {
        _handler: SAC1_SAC3,
    },
    Vector {
        _handler: SAC0_SAC2,
    },
    Vector {
        _handler: ECOMP0_ECOMP1,
    },
    Vector { _handler: ADC },
    Vector { _handler: EUSCI_B1 },
    Vector { _handler: EUSCI_B0 },
    Vector { _handler: EUSCI_A1 },
    Vector { _handler: EUSCI_A0 },
    Vector { _handler: WDT },
    Vector { _handler: RTC },
    Vector {
        _handler: TIMER3_B1,
    },
    Vector {
        _handler: TIMER3_B0,
    },
    Vector {
        _handler: TIMER2_B1,
    },
    Vector {
        _handler: TIMER2_B0,
    },
    Vector {
        _handler: TIMER1_B1,
    },
    Vector {
        _handler: TIMER1_B0,
    },
    Vector {
        _handler: TIMER0_B1,
    },
    Vector {
        _handler: TIMER0_B0,
    },
    Vector { _handler: UNMI },
    Vector { _handler: SYSNMI },
];
#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
pub enum Interrupt {
    #[doc = "21 - 0xFFCE"]
    PORT4,
    #[doc = "22 - 0xFFD0"]
    PORT3,
    #[doc = "23 - 0xFFD2"]
    PORT2,
    #[doc = "24 - 0xFFD4"]
    PORT1,
    #[doc = "25 - 0xFFD6"]
    SAC1_SAC3,
    #[doc = "26 - 0xFFD8"]
    SAC0_SAC2,
    #[doc = "27 - 0xFFDA"]
    ECOMP0_ECOMP1,
    #[doc = "28 - 0xFFDC"]
    ADC,
    #[doc = "29 - 0xFFDE"]
    EUSCI_B1,
    #[doc = "30 - 0xFFE0"]
    EUSCI_B0,
    #[doc = "31 - 0xFFE2"]
    EUSCI_A1,
    #[doc = "32 - 0xFFE4"]
    EUSCI_A0,
    #[doc = "33 - 0xFFE6"]
    WDT,
    #[doc = "34 - 0xFFE8"]
    RTC,
    #[doc = "35 - 0xFFEA"]
    TIMER3_B1,
    #[doc = "36 - 0xFFEC"]
    TIMER3_B0,
    #[doc = "37 - 0xFFEE"]
    TIMER2_B1,
    #[doc = "38 - 0xFFF0"]
    TIMER2_B0,
    #[doc = "39 - 0xFFF2"]
    TIMER1_B1,
    #[doc = "40 - 0xFFF4"]
    TIMER1_B0,
    #[doc = "41 - 0xFFF6"]
    TIMER0_B1,
    #[doc = "42 - 0xFFF8"]
    TIMER0_B0,
    #[doc = "43 - 0xFFFA"]
    UNMI,
    #[doc = "44 - 0xFFFC"]
    SYSNMI,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::PORT4 => 21,
            Interrupt::PORT3 => 22,
            Interrupt::PORT2 => 23,
            Interrupt::PORT1 => 24,
            Interrupt::SAC1_SAC3 => 25,
            Interrupt::SAC0_SAC2 => 26,
            Interrupt::ECOMP0_ECOMP1 => 27,
            Interrupt::ADC => 28,
            Interrupt::EUSCI_B1 => 29,
            Interrupt::EUSCI_B0 => 30,
            Interrupt::EUSCI_A1 => 31,
            Interrupt::EUSCI_A0 => 32,
            Interrupt::WDT => 33,
            Interrupt::RTC => 34,
            Interrupt::TIMER3_B1 => 35,
            Interrupt::TIMER3_B0 => 36,
            Interrupt::TIMER2_B1 => 37,
            Interrupt::TIMER2_B0 => 38,
            Interrupt::TIMER1_B1 => 39,
            Interrupt::TIMER1_B0 => 40,
            Interrupt::TIMER0_B1 => 41,
            Interrupt::TIMER0_B0 => 42,
            Interrupt::UNMI => 43,
            Interrupt::SYSNMI => 44,
        }
    }
}
#[derive(Debug, Copy, Clone)]
pub struct TryFromInterruptError(());
impl Interrupt {
    #[inline]
    pub fn try_from(value: u8) -> Result<Self, TryFromInterruptError> {
        match value {
            21 => Ok(Interrupt::PORT4),
            22 => Ok(Interrupt::PORT3),
            23 => Ok(Interrupt::PORT2),
            24 => Ok(Interrupt::PORT1),
            25 => Ok(Interrupt::SAC1_SAC3),
            26 => Ok(Interrupt::SAC0_SAC2),
            27 => Ok(Interrupt::ECOMP0_ECOMP1),
            28 => Ok(Interrupt::ADC),
            29 => Ok(Interrupt::EUSCI_B1),
            30 => Ok(Interrupt::EUSCI_B0),
            31 => Ok(Interrupt::EUSCI_A1),
            32 => Ok(Interrupt::EUSCI_A0),
            33 => Ok(Interrupt::WDT),
            34 => Ok(Interrupt::RTC),
            35 => Ok(Interrupt::TIMER3_B1),
            36 => Ok(Interrupt::TIMER3_B0),
            37 => Ok(Interrupt::TIMER2_B1),
            38 => Ok(Interrupt::TIMER2_B0),
            39 => Ok(Interrupt::TIMER1_B1),
            40 => Ok(Interrupt::TIMER1_B0),
            41 => Ok(Interrupt::TIMER0_B1),
            42 => Ok(Interrupt::TIMER0_B0),
            43 => Ok(Interrupt::UNMI),
            44 => Ok(Interrupt::SYSNMI),
            _ => Err(TryFromInterruptError(())),
        }
    }
}
#[cfg(feature = "rt")]
#[macro_export]
#[doc = r" Assigns a handler to an interrupt"]
#[doc = r""]
#[doc = r" This macro takes two arguments: the name of an interrupt and the path to the"]
#[doc = r" function that will be used as the handler of that interrupt. That function"]
#[doc = r" must have signature `fn()`."]
#[doc = r""]
#[doc = r" Optionally, a third argument may be used to declare interrupt local data."]
#[doc = r" The handler will have exclusive access to these *local* variables on each"]
#[doc = r" invocation. If the third argument is used then the signature of the handler"]
#[doc = r" function must be `fn(&mut $NAME::Locals)` where `$NAME` is the first argument"]
#[doc = r" passed to the macro."]
#[doc = r""]
#[doc = r" # Example"]
#[doc = r""]
#[doc = r" ``` ignore"]
#[doc = r" interrupt!(TIM2, periodic);"]
#[doc = r""]
#[doc = r" fn periodic() {"]
#[doc = r#"     print!(".");"#]
#[doc = r" }"]
#[doc = r""]
#[doc = r" interrupt!(TIM3, tick, locals: {"]
#[doc = r"     tick: bool = false;"]
#[doc = r" });"]
#[doc = r""]
#[doc = r" fn tick(locals: &mut TIM3::Locals) {"]
#[doc = r"     locals.tick = !locals.tick;"]
#[doc = r""]
#[doc = r"     if locals.tick {"]
#[doc = r#"         println!("Tick");"#]
#[doc = r"     } else {"]
#[doc = r#"         println!("Tock");"#]
#[doc = r"     }"]
#[doc = r" }"]
#[doc = r" ```"]
macro_rules ! interrupt { ( $ NAME : ident , $ path : path , locals : { $ ( $ lvar : ident : $ lty : ty = $ lval : expr ; ) * } ) => { # [ allow ( non_snake_case ) ] mod $ NAME { pub struct Locals { $ ( pub $ lvar : $ lty , ) * } } # [ allow ( non_snake_case ) ] # [ no_mangle ] pub extern "msp430-interrupt" fn $ NAME ( ) { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; static mut LOCALS : self :: $ NAME :: Locals = self :: $ NAME :: Locals { $ ( $ lvar : $ lval , ) * } ; let f : fn ( & mut self :: $ NAME :: Locals ) = $ path ; f ( unsafe { & mut LOCALS } ) ; } } ; ( $ NAME : ident , $ path : path ) => { # [ allow ( non_snake_case ) ] # [ no_mangle ] pub extern "msp430-interrupt" fn $ NAME ( ) { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; let f : fn ( ) = $ path ; f ( ) ; } } }
