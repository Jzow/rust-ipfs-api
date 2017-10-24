pub type BlockGetResponse = Vec<u8>;


#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BlockPutResponse {
    pub key: String,
    pub size: usize,
}


#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BlockRmResponse {
    pub hash: String,
    pub error: String,
}


#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BlockStatResponse {
    pub key: String,
    pub size: usize,
}


#[cfg(test)]
mod tests {
    deserialize_test!(v0_block_stat_0, BlockStatResponse);
}
