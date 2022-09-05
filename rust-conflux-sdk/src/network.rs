use once_cell::sync::Lazy;
use std::sync::atomic::AtomicU16;
use std::sync::atomic::Ordering;
use parking_lot::lock_api::{Mutex};
use parking_lot::RawMutex;

pub static NETWORK_ID: AtomicU16 = AtomicU16::new(70);

static CUSTOM_NETWORK: Lazy<Mutex<RawMutex, String>> = Lazy::new(||{
     Mutex::new(String::new())
});

pub fn set_network(network_id: Network, custom_network: Option<&str>) {
    NETWORK_ID.store(network_id.get_network_id(), Ordering::Relaxed);
    let mut lock = CUSTOM_NETWORK.lock();
    *lock = custom_network.unwrap_or("").to_owned()
}

pub static CONFLUX_NETWORK: Lazy<String> = Lazy::new(|| {
    let lock = CUSTOM_NETWORK.lock();
    if lock.as_str() != "" {
        lock.to_string()
    } else {
        let network = Network::new(NETWORK_ID.load(Ordering::Relaxed));
        network.get_network_url()
    }
});

pub static CONFLUX_ENV: Lazy<String> = Lazy::new(|| {
    let env = Env::new(NETWORK_ID.load(Ordering::Relaxed));
    env.get_env()
});

pub enum Env {
    CFX,
    ETH,
}

impl Env {
    pub fn new(network_id: u16) -> Self {
        match network_id {
            70 | 1029 => Env::CFX,
            71 | 1030 => Env::ETH,
            _ => Env::CFX,
        }
    }

    pub fn get_env(&self) -> String {
        match self {
            Env::CFX => String::from("cfx"),
            Env::ETH => String::from("eth"),
        }
    }
}

#[repr(u16)]
pub enum Network {
    CfxTest = 70,
    CfxMain = 1029,
    EthTest = 71,
    EthMain = 1030,
}

impl Network {
    pub fn new(network_id: u16) -> Self {
        match network_id {
            70 => Network::CfxTest,
            1029 => Network::CfxMain,
            71 => Network::EthTest,
            1030 => Network::EthMain,
            _ => Network::CfxTest,
        }
    }

    pub fn get_network_url(&self) -> String {
        match self {
            Network::CfxTest => String::from("https://test.confluxrpc.com"),
            Network::CfxMain => String::from("https://main.confluxrpc.com"),
            Network::EthTest => String::from("https://evmtestnet.confluxrpc.com"),
            Network::EthMain => String::from("https://evm.confluxrpc.com"),
        }
    }

    pub fn get_network_id(&self) -> u16 {
        match self {
            Network::CfxTest => 70,
            Network::CfxMain => 1029,
            Network::EthTest => 71,
            Network::EthMain => 1030,
        }
    }
}
