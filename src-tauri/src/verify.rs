use base64::{engine::general_purpose, Engine as _};
use rsa::{pkcs8::DecodePublicKey, Pkcs1v15Sign, RsaPublicKey};
use sha2::{Digest, Sha256};

use crate::{utils::resolve_resource_path, uuid::get_uuid};

// 使用公钥验证签名
pub fn verify_signature(pub_key: &RsaPublicKey, data: &[u8], signature: &[u8]) -> bool {
    let dencoed = general_purpose::STANDARD_NO_PAD.decode(&signature);
    match dencoed {
        Ok(dencoed_val) => {
            let mut hasher = Sha256::new(); // 创建 SHA-256 哈希实例
            hasher.update(data); // 对数据进行哈希处理
            let hashed_data = hasher.finalize();

            // 验证签名
            pub_key
                .verify(Pkcs1v15Sign::new_unprefixed(), &hashed_data, &dencoed_val)
                .is_ok()
        }
        Err(_) => false,
    }
}

#[tauri::command]
pub fn use_verify_signature(data: &str, signature: Vec<u8>) -> bool {
    let uuid = get_uuid().expect("获取系统 UUID 失败");

    if uuid != data {
        return false;
    }

    let pub_key_path = resolve_resource_path("../public_key.pem");
    let pub_key_pem = RsaPublicKey::read_public_key_pem_file(pub_key_path).expect("读取公钥失败");
    verify_signature(&pub_key_pem, &data.as_bytes().to_vec(), &signature)
}
