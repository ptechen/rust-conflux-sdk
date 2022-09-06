mod balance;
mod epoch_number;
mod gas_price;
mod get_admin;
mod get_status;
mod get_transaction_by_hash;
mod get_transaction_receipt;
mod web3_version;
mod get_block_by_hash;

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
