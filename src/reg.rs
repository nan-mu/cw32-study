use tock_registers::{register_bitfields, register_structs, registers};

register_structs! {
    pub Sysctrl {// 下面的数字是寄存器的地址偏移
        (0x00 => _reserved0),//保留，留空的
        (0x30 => pub ahben: registers::ReadWrite<u32, ahben::Register>),// 记载了寄存器位的功能，读写权限、和长度（32）
        (0x34 => @END),
    }
}

register_structs! {
    pub Gpioc {//同理
        (0x00 => pub dir: registers::ReadWrite<u32, dir::Register>),
        (0x04 => pub opendrain: registers::ReadWrite<u32, opendrain::Register>),
        (0x08 => pub speed: registers::ReadWrite<u32, speed::Register>),
        (0x0c => _reserved0), //pdr
        (0x10 => _reserved1), //pur
        (0x14 => _reserved2), //afrh
        (0x18 => _reserved3), //afrl
        (0x1c => pub analog: registers::ReadWrite<u32, analog::Register>),
        (0x20 => pub driver: registers::ReadWrite<u32, driver::Register>),
        (0x24 => _reserved4), //riseie
        (0x28 => _reserved5), //fallie
        (0x2c => _reserved6), //highie
        (0x30 => _reserved7), //lowie
        (0x34 => _reserved8), //isr
        (0x38 => _reserved9), //icr
        (0x3c => pub lckr: registers::ReadWrite<u32, lckr::Register>),
        (0x40 => _reserved10), //filter
        // (0x40 => _reserved10), //filter
        // (0x40 => _reserved10), //filter
        // (0x40 => _reserved10), //filter
        (0x50 => _reserved11), //idr
        (0x54 => pub odr: registers::ReadWrite<u32, odr::Register>),
        (0x58 => @END),
    }
}

// 这里是寄存器位的详细说明
register_bitfields![u32,
    pub ahben [
        GPIOC OFFSET(6) NUMBITS(1) [// 比如我希望启用这个功能，我就需要操作第7位的一位，选择enable写入1
            enable = 1,
            disable = 0,
        ],
    ],
    pub lckr [
        KEYPIN13 OFFSET(0) NUMBITS(32) [ // 比如我希望启用这个功能，我就需要操作第1位的32位，也就是全部，选择unlock写入0x5A5AFFFF
            unlock = 0x5A5AFFFF,
            lock = 0x0000,
        ],
    ],
    pub analog [
        PIN13 OFFSET(13) NUMBITS(1) [
            analog = 1,
            digital = 0,
        ]
    ],
    pub driver [
        PIN13 OFFSET(13) NUMBITS(1) [
            high = 0,
            low = 1,
        ]
    ],
    pub dir [
        PIN13 OFFSET(13) NUMBITS(1) [
            input = 1,
            output = 0,
        ]
    ],
    pub opendrain [
        PIN13 OFFSET(13) NUMBITS(1) [
            open_drain = 1,
            push_pull = 0,
        ]
    ],
    pub speed [
        PIN13 OFFSET(13) NUMBITS(1) [
            low = 0,
            high = 1,
        ]
    ],
    pub odr [
        PIN13 OFFSET(13) NUMBITS(1) [
            high = 1,
            low = 0,
        ]
    ],
];
