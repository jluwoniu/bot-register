use std::fs::File;
use std::io::Write;
use rsa::{RsaPublicKey, pkcs1::DecodeRsaPublicKey, pkcs1::DecodeRsaPrivateKey, PaddingScheme, PublicKey, RsaPrivateKey};
use rsa::pkcs1::{EncodeRsaPrivateKey, EncodeRsaPublicKey, LineEnding};

const PRIVATE_KEY: &str = "-----BEGIN RSA PRIVATE KEY-----
MIIEowIBAAKCAQEAvVkZhnP9Up5ObdGckcGcrcrCKScsA4jNAafPUjWAkWtsFb8n
MS5DTDpmfuQNbthiniOGLLP8fRGczcxLvFdcVC26Kwd+cr/lznKdIVpZSpNL8e/y
IAGvDr2bjoAwY1WH8Mf2rVCg9Gpe3yYHoY7A4aLquWdlFWT7Hd3Vey84Za8P3iso
53fwRW5Tq21lrrR7DsH7EytRJ4kHuLkvIrK59IJQbqPcwBCCfyQg51oU9aG3tnwY
SoGBWbQChbCkeQU4aY9wBlFbfVfaI63N/E//U1DQ+0+YM/9gFmt+bgLC9wEU7In7
yzbfcS0gMJJ1qrVLTp2LTKmcD0ZCfAwoeQFynQIDAQABAoIBAH7RTE1HJmXNbY4U
FB9zY3LrGDvj5pC/cT8om6zd9KxxHEU8cA16hMxrWPan229gMjzkYhtSuUszHqif
qiINCOPAdK4ZKNHGQS6SiTcrgh52pSj3ol1Zg1wGFkfNB5ugT5ou88KE/Ch3je87
U2UbbYgyS6+91nAL4DzStQb6S3WeMWXSZ54IGbAIN0TRbTCY3M6gsdmUBwnOU/+F
5qkzDVAmzQ9S9aYFCWTW4cFqwrNTOWtj2DZcW+g/oPhxPVcGsuB82KGQqX5DzsGh
cfDhajMl0Q+70wv4gsLIpAcp96yPG3/g064eunQdRl+pbnqVbIsoguKHlFXHJtKE
h69AKUECgYEA0Z858Fq8kL+foFqBN9lcFUzeHlLL0Oe9iZF7GaTh1vlF+Jb7RMbg
HYvmW+yelREuJFfDLROMjXaB2cG9CA2XmrAoPY5N7RU0IG1XlxluAju2VCyR5g06
LOariKZo7dSyDmYgFL+xHPGZcsoMQBlIAnEaraiTpLHArgTHUuFqb3kCgYEA5z2U
JEk4Oy/mMl7/XfX4cvbrbgTkDWbf4idTIyHpz4SefhbnNuhdaTJeJYNGDKWgia9W
+YCgU0+YtMMNXg9TSjwX6EHQEQQED+d9WeatbxXyb7YE0j3YXz9QUJO3p1kRnJsg
u+GHLfJMZ5VMxcmUh2sclHC8criE71WBCJEv30UCgYBo2QeV3QysKeyhvCMd2C5i
yQvghvFDEGaRRJ8Z+IqtiAkjigEsKX03yh3ypKu9S9aUfwE0rq6Co1/XlzwBpyrJ
SIsYHvGDQDPOdGJ3RVTs8QkFmTVwp/UNOGNb+kBJlTVqVlJ9jlP8ciuaas0t0nde
IUyCx+rxWadSYzl/jxg5oQKBgAaj9A8esyiv9XJY3Ah8ucfDY3tjVJE19RPvgSmv
HaHB8zM3wSgIgDY3JhD5thdC4ltR17OOiAwcaww11bpvwjqNgzo+OZVjgiJayXS9
dRmNjcq7dWP8Nqx1iv7lXw7FNgZDRitfDJy7KoqG9Q9w/HzvDIEOw7qfpeGhmPUA
BlnpAoGBAIsPjGoy/svja6UItgQfI1rFoPuyzVlmMkWppOhMUf1jslJ1Jvl3lhGn
D1J1qhzPqyG1VUFRv99lp0Etln2ro6t5mHJlqizv0XMJbJxkKvgRH9j0M32LdIt8
mh0rkHDcunVPGw67rMKvhvcnauqhTGaNxuOPHn78pLiYvDjo+kbJ
-----END RSA PRIVATE KEY-----
";

const PUBLIC_KEY :&str = "-----BEGIN RSA PUBLIC KEY-----
MIIBCgKCAQEAvVkZhnP9Up5ObdGckcGcrcrCKScsA4jNAafPUjWAkWtsFb8nMS5D
TDpmfuQNbthiniOGLLP8fRGczcxLvFdcVC26Kwd+cr/lznKdIVpZSpNL8e/yIAGv
Dr2bjoAwY1WH8Mf2rVCg9Gpe3yYHoY7A4aLquWdlFWT7Hd3Vey84Za8P3iso53fw
RW5Tq21lrrR7DsH7EytRJ4kHuLkvIrK59IJQbqPcwBCCfyQg51oU9aG3tnwYSoGB
WbQChbCkeQU4aY9wBlFbfVfaI63N/E//U1DQ+0+YM/9gFmt+bgLC9wEU7In7yzbf
cS0gMJJ1qrVLTp2LTKmcD0ZCfAwoeQFynQIDAQAB
-----END RSA PUBLIC KEY-----
";

pub fn encrypt(content: &str) -> String{

    let mut rng = rand::thread_rng();
    let public_key = RsaPublicKey::from_pkcs1_pem(PUBLIC_KEY).unwrap();
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let enc_data = public_key.encrypt(&mut rng, padding, content.as_bytes()).unwrap();
    let b64 = base64::encode(enc_data);
    return b64;
}

pub fn decrypt(content:&str) -> String{

    let private_key = RsaPrivateKey::from_pkcs1_pem(PRIVATE_KEY).unwrap();
    let enc_data = base64::decode(content).unwrap();
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let dec_data = private_key.decrypt(padding, &enc_data).unwrap();
    return String::from_utf8(dec_data).unwrap();
}

pub fn generate_pem(){
    let mut rng = rand::thread_rng();

    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut rng, bits).unwrap();
    let public_key = RsaPublicKey::from(&private_key);
    let private_pem_str = private_key.to_pkcs1_pem(LineEnding::LF).unwrap().to_string();
    let mut private_pem = File::create("./assets/private.pem").unwrap();
    private_pem.write_all(private_pem_str.as_bytes());
    let mut public_pem = File::create("./assets/public.pem").unwrap();
    public_pem.write_all(public_key.to_pkcs1_pem(LineEnding::LF).unwrap().to_string().as_bytes());

}