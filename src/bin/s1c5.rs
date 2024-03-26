use anyhow::Result;
use cryptopals::Bytes;

/// Implement repeating-key XOR
/// Write a function that takes two equal-length buffers and produces their XOR
/// combination. 
fn main() -> Result<()> {
    let input = Bytes::from_ascii(
        "Burning 'em, if you ain't quick and nimble\n\
        I go crazy when I hear a cymbal");
    let key = Bytes::from_ascii("ICE");
    let output = (input ^ key).to_hex();
    assert_eq!(output, "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343\
    c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b202831\
    65286326302e27282f");
    println!("{output}");
    Ok(())
}
