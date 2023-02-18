pub mod consts {
  pub const URB_BULK_OUT_1: u8 = 0x01;
  pub const URB_BULK_OUT_2: u8 = 0x02;
  pub const URB_BULK_IN_1: u8 = 0x81;
  pub const URB_BULK_IN_2: u8 = 0x82;
  pub const SETUP_1: [u8; 2] = [0x01,0x02];
  pub const SETUP_2: [u8; 10] = [0x0e,0x0a,0x88,0x00,0x00,0x00,0x00,0x00,0x00,0x00];
  pub const SETUP_3: [u8; 3] = [0x0e,0x03,0x00];
  pub const SETUP_4: [u8; 7] = [0x03,0x07,0xe9,0x00,0xce,0x00,0x01];
  pub const SETUP_5: [u8; 6] = [0x06,0x06,0x9a,0x8e,0x1f,0x00];
  pub const CHK: [u8; 2] = [0x0c,0x02];
  pub const IDX: [u8; 10] = [0x0b,0x0a,0x00,0x80,0x00,0x00,0x00,0x00,0x00,0x00];
  pub const ACK: [u8; 3] = [0x0b,0x03,0x00];
}

pub struct EvaHandshake {
  pub transfers: Vec<EvaTransfer>,
}

pub struct EvaTransfer {
  pub endpoint: u8,
  pub data: Option<Vec<u8>>,
}

impl EvaHandshake {

  fn new() -> EvaHandshake {
    return EvaHandshake { transfers: Vec::new() };
  }

  fn add_transfer_out(&mut self, endpoint: u8, data: Vec<u8>){
    self.transfers.push(
      EvaTransfer::new(endpoint, Some(data))
    )
  }

  fn add_transfer_in(&mut self, endpoint: u8){
    self.transfers.push(
      EvaTransfer::new(endpoint, None)
    )
  }

  pub fn sequence() -> EvaHandshake {
    let mut handshake = EvaHandshake::new();
    handshake.add_transfer_out(consts::URB_BULK_OUT_1, consts::SETUP_1.to_vec());
    handshake.add_transfer_in(consts::URB_BULK_IN_1);
    handshake.add_transfer_out(consts::URB_BULK_OUT_1, consts::SETUP_2.to_vec());
    handshake.add_transfer_in(consts::URB_BULK_IN_1);
    handshake.add_transfer_out(consts::URB_BULK_OUT_2, consts::SETUP_3.to_vec());
    handshake.add_transfer_in(consts::URB_BULK_IN_2);
    handshake.add_transfer_out(consts::URB_BULK_OUT_1, consts::SETUP_4.to_vec());
    handshake.add_transfer_in(consts::URB_BULK_IN_1);
    handshake.add_transfer_out(consts::URB_BULK_OUT_1, consts::SETUP_5.to_vec());
    handshake.add_transfer_in(consts::URB_BULK_IN_1);
    return handshake;
  }

}

impl EvaTransfer {
  pub fn new(endpoint: u8, data: Option<Vec<u8>>) -> EvaTransfer {
    return EvaTransfer { endpoint: endpoint, data: data };
  }
}