use crate::hci::{BufWriter, WriteHCI, gap::AdvData};
use embedded_io::Write;

// const OGF_LINK_CONTROL_COMMAND: u16 = 0x01;
// const OGF_LINK_POLICY_COMMAND: u16 = 0x02;
const OGF_CONTROL_AND_BASEBAND_COMMAND: u16 = 0x03;
// const OGF_INFORMATIONAL_PARAMETERS_COMMAND: u16 = 0x04;
// const OGF_STATUS_PARAMETERS_COMMAND: u16 = 0x05;
// const OGF_TESTING_COMMAND: u16 = 0x06;
const OGF_LE_CONTROLLER_COMMAND: u16 = 0x08;

pub trait Command {
    const TYPE: u8 = 0x01;
    const OGF: u16;
    const OCF: u16;
    const OPCODE: u16 = (Self::OGF << 10) | Self::OCF;
    const PARAM_LEN: u8;

    fn write_into<W: Write>(&self, w: &mut W) -> Result<(), W::Error> {
        Self::TYPE.write_into(w)?;
        Self::OPCODE.write_into(w)?;
        Self::PARAM_LEN.write_into(w)?;
        self.write_payload(w)?;

        Ok(())
    }

    fn write_payload<W: Write>(&self, w: &mut W) -> Result<(), W::Error>;
}

macro_rules! command {
    (
        OGF = $OGF:expr;
        OCF = $OCF:expr;
        struct $name:ident <$lt:lifetime> {
            $($field:ident : $ty:ty),* $(,)?
        }
    ) => {
        pub struct $name <$lt> {
            $(pub $field: $ty),*
        }

        impl <$lt> $name <$lt> {
            pub fn new($($field: $ty),*) -> Self {
                Self {
                    $($field),*
                }
            }
        }

        impl <$lt> Command for $name <$lt> {
            const OGF: u16 = $OGF;
            const OCF: u16 = $OCF;
            const PARAM_LEN: u8 = (0 $( + core::mem::size_of::<$ty>())*) as u8;

            fn write_payload<W: Write>(&self, w: &mut W) -> Result<(), W::Error> {
                $(self.$field.write_into(w)?;)*

                Ok(())
            }
        }
    };
    (
        OGF = $OGF:expr;
        OCF = $OCF:expr;
        struct $name:ident {
            $($field:ident : $ty:ty),* $(,)?
        }
    ) => {
        pub struct $name {
            $(pub $field: $ty),*
        }

        impl $name {
            pub fn new($($field: $ty),*) -> Self {
                Self {
                    $($field),*
                }
            }
        }


        impl Command for $name {
            const OGF: u16 = $OGF;
            const OCF: u16 = $OCF;
            const PARAM_LEN: u8 = (0 $( + core::mem::size_of::<$ty>())*) as u8;

            fn write_payload<W: Write>(&self, #[allow(unused)] w: &mut W) -> Result<(), W::Error> {
                $(self.$field.write_into(w)?;)*

                Ok(())
            }
       }
    };
}

command! {
    OGF = OGF_CONTROL_AND_BASEBAND_COMMAND;
    OCF = 0x03;
    struct Reset {}
}

command! {
    OGF = OGF_LE_CONTROLLER_COMMAND;
    OCF = 0x06;
    struct SetAdvParameters {
        interval_min: u16,
        interval_max: u16,
        advertising_type: u8,
        own_address_type: u8,
        peer_address_type: u8,
        peer_address: [u8; 6],
        advertising_channel_map: u8,
        advertising_filter_policy: u8,
    }
}

pub struct SetAdvData<'a> {
    pub data: &'a [AdvData],
}

impl<'a> SetAdvData<'a> {
    pub fn new(data: &'a [AdvData]) -> Self {
        Self { data }
    }
}

impl<'a> Command for SetAdvData<'_> {
    const OGF: u16 = OGF_LE_CONTROLLER_COMMAND;
    const OCF: u16 = 0x08;
    const PARAM_LEN: u8 = 32;

    fn write_payload<W: Write>(&self, w: &mut W) -> Result<(), W::Error> {
        let mut buf = [0; (Self::PARAM_LEN - 1) as usize];
        let mut writer = BufWriter::new(&mut buf);

        for ad in self.data {
            ad.write_into(&mut writer).unwrap();
        }

        (writer.pos as u8).write_into(w)?;
        writer.buf.write_into(w)?;

        Ok(())
    }
}

pub struct SetScanResponseData<'a> {
    pub data: &'a [AdvData],
}

impl<'a> SetScanResponseData<'a> {
    pub fn new(data: &'a [AdvData]) -> Self {
        Self { data }
    }
}

impl<'a> Command for SetScanResponseData<'_> {
    const OGF: u16 = OGF_LE_CONTROLLER_COMMAND;
    const OCF: u16 = 0x09;
    const PARAM_LEN: u8 = 32;

    fn write_payload<W: Write>(&self, w: &mut W) -> Result<(), W::Error> {
        let mut buf = [0; (Self::PARAM_LEN - 1) as usize];
        let mut writer = BufWriter::new(&mut buf);

        for ad in self.data {
            ad.write_into(&mut writer).unwrap();
        }

        (writer.pos as u8).write_into(w)?;
        writer.buf.write_into(w)?;

        Ok(())
    }
}

command! {
    OGF = OGF_LE_CONTROLLER_COMMAND;
    OCF = 0x0A;
    struct SetAdvEnable {
        enable: u8,
    }
}
