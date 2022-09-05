mod balance;
mod web3_version;
mod epoch_number;
mod get_status;
mod gas_price;
mod get_admin;
mod get_transaction_receipt;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
