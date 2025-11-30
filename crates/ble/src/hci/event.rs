use core::fmt::Debug;

use crate::hci::{BufReader, MacAddr, ReadHCI};

pub(crate) trait Event<'a> {
    const CODE: u8;

    fn from_reader(reader: &mut BufReader<'a>) -> Self;
}

macro_rules! event {
    (
        CODE = $CODE:expr;
        struct $name:ident <$lt:lifetime> {
            $($field:ident : $ty:ty),* $(,)?
        }
    ) => {
        #[derive(Debug)]
        pub struct $name <$lt> {
            $(pub $field: $ty),*
        }

        impl <$lt> Event <$lt>  for $name <$lt> {
            const CODE: u8 = $CODE;

            fn from_reader(r: &mut BufReader <$lt>) -> Self {
                Self {
                    $(
                        $field: <$ty>::read_from(r),
                    )*
                }
            }
        }
    };
    (
          CODE = $CODE:expr;
          struct $name:ident {
          $($field:ident : $ty:ty),* $(,)?
      }) => {
          #[derive(Debug)]
          pub struct $name  {
              $(pub $field: $ty),*
          }

          impl Event<'_> for $name {
              const CODE: u8 = $CODE;

              fn from_reader(r: &mut BufReader<'_>) -> Self {
                  Self {
                      $(
                          $field: <$ty>::read_from(r),
                      )*
                  }
              }
          }
      };
}

#[derive(Debug)]
pub enum HCIEvent<'a> {
    CommandComplete(CommandComplete<'a>),
    DisconnectionComplete(DisconnectionComplete),
    LEMeta(LEMeta<'a>),
    Unknown { ev_code: u8, params: &'a [u8] },
}

impl<'a> HCIEvent<'a> {
    pub const TYPE: u8 = 0x04;

    pub(crate) fn from_reader(reader: &mut BufReader<'a>) -> Self {
        let ev_code = reader.read_u8();

        match ev_code {
            CommandComplete::CODE => Self::CommandComplete(CommandComplete::from_reader(reader)),
            DisconnectionComplete::CODE => {
                Self::DisconnectionComplete(DisconnectionComplete::from_reader(reader))
            }
            LEMeta::CODE => Self::LEMeta(LEMeta::from_reader(reader)),
            ev_code => Self::Unknown {
                ev_code,
                params: reader.read_bytes(reader.remaining()),
            },
        }
    }
}

event! {
    CODE = 0x0E;
    struct CommandComplete<'a> {
        num_hci_command_packets: u8,
        command_opcode: u16,
        return_parameters: &'a [u8],
    }
}

event! {
    CODE = 0x05;
    struct DisconnectionComplete {
        status: u8,
        connection_handle: u16,
        reason: u8,
    }
}

#[derive(Debug)]
pub enum LEMeta<'a> {
    ConnectionComplete(ConnectionComplete),
    ScanRequestReceived(ScanRequestReceived),
    EnhancedConnectionComplete(EnhancedConnectionComplete),
    Unknown { subev_code: u8, params: &'a [u8] },
}

impl<'a> LEMeta<'a> {
    pub const CODE: u8 = 0x3E;

    pub(crate) fn from_reader(reader: &mut BufReader<'a>) -> Self {
        let subev_code: u8 = reader.read_u8();

        match subev_code {
            ConnectionComplete::CODE => {
                Self::ConnectionComplete(ConnectionComplete::from_reader(reader))
            }
            ScanRequestReceived::CODE => {
                Self::ScanRequestReceived(ScanRequestReceived::from_reader(reader))
            }
            EnhancedConnectionComplete::CODE => {
                Self::EnhancedConnectionComplete(EnhancedConnectionComplete::from_reader(reader))
            }
            _ => Self::Unknown {
                subev_code,
                params: reader.read_bytes(reader.remaining()),
            },
        }
    }
}

event! {
    CODE= 0x01;
    struct ConnectionComplete {
        status: u8,
        connection_handle: u16,
        role: u8,
        peer_address_type: u8,
        peer_address: MacAddr,
        connection_interval: u16,
        peripheral_latency: u16,
        supervision_timeout: u16,
        central_clock_accuracy: u8,
    }
}

event! {
   CODE = 0x13;
   struct ScanRequestReceived {
       advertising_handle: u8,
       scanner_address_type: u8,
       scanner_address: MacAddr,
   }
}

event! {
   CODE = 0x0A;
   struct EnhancedConnectionComplete {
       status: u8,
       connection_handle: u16,
       role: u8,
       peer_addr_type: u8,
       peer_addr: MacAddr,
       local_resolvable_private_addr: MacAddr,
       peer_resolvable_private_addr: MacAddr,
       conection_interval: u16,
       peripheral_latency: u16,
       supervision_timeout: u16,
       central_clock_accuracy: u8,
   }
}
