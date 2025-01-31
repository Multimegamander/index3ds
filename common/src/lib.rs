use serde::{Deserialize, Deserializer, Serialize, Serializer};

fn as_base64<S>(data: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&base64::encode(&data[..]))
}

fn from_base64<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    String::deserialize(deserializer)
        .and_then(|string| base64::decode(&string).map_err(|err| Error::custom(err.to_string())))
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NcchInfo {
    pub id: String,
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    pub ncch_signature: Vec<u8>,
    pub content_size: u32,
    pub partition_id: String,
    pub maker_code: String,
    pub ncch_verson: u16,
    pub program_id: String,
    pub product_code: String,
    pub secondary_key_slot: u8,
    pub platform: u8,
    pub content_is_data: bool,
    pub content_is_executable: bool,
    pub content_category: u8,
    pub content_unit_size: u8,
    pub fixed_key: bool,
    pub no_romfs: bool,
    pub no_crypto: bool,
    pub seed_crypto: bool,

    pub exheader_name: Option<String>,
    pub sd_app: Option<bool>,
    pub remaster_version: Option<u16>,
    pub dependencies: Option<Vec<String>>,
    pub save_data_size: Option<u64>,
    pub jump_id: Option<String>,
    pub exheader_program_id: Option<String>,
    pub core_version: Option<u32>,
    pub enable_l2_cache: Option<bool>,
    pub high_cpu_speed: Option<bool>,
    pub system_mode: Option<u8>,
    pub n3ds_system_mode: Option<u8>,
    pub ideal_processor: Option<u8>,
    pub affinity_mask: Option<u8>,
    pub thread_priority: Option<u8>,
    pub resource_limit_desc: Option<Vec<u16>>,
    pub extdata_id: Option<String>,
    pub system_savedata_id0: Option<String>,
    pub system_savedata_id1: Option<String>,
    pub storage_access_id: Option<String>,
    pub filesystem_flag: Option<u64>,
    pub services: Option<Vec<String>>,
    pub resource_limit_category: Option<u8>,
    pub kernel_desc: Option<Vec<u32>>,
    pub arm9_flag: Option<u32>,
    pub arm9_flag_version: Option<u8>,

    pub short_title: Option<Vec<String>>,
    pub long_title: Option<Vec<String>>,
    pub publisher: Option<Vec<String>>,
    pub ratings: Option<Vec<u8>>,
    pub region_lockout: Option<u32>,
    pub match_maker_id: Option<String>,
    pub match_maker_bit_id: Option<String>,
    pub smdh_flags: Option<u32>,
    pub eula_version: Option<u16>,
    pub cec_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "status")]
pub enum NcchInfoResponse {
    Ok(NcchInfo),
    NotFound,
    InternalServerError,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppendRequest {
    pub session_id: u32,
    pub offset: usize,
    pub len: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NcchExist {
    pub ncch_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "status")]
pub enum PostNcchResponse {
    Finished(NcchExist),
    AlreadyFinished,
    AppendNeeded(AppendRequest),
    UnexpectedLength,
    UnexpectedFormat,
    VerificationFailed,
    Busy,
    Conflict(NcchExist),
    InternalServerError,
    NotFound,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct NcchFilterParam {
    pub keyword: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NcchQueryParam {
    pub offset: i64,
    pub limit: i64,
    #[serde(flatten)]
    pub filter: NcchFilterParam,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NcchInfoVec {
    pub ncchs: Vec<NcchInfo>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "status")]
pub enum NcchQueryResponse {
    Ok(NcchInfoVec),
    InternalServerError,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NcchCount {
    pub count: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "status")]
pub enum NcchQueryCountResponse {
    Ok(NcchCount),
    InternalServerError,
}

pub mod url {

    pub fn post_ncch() -> &'static str {
        "/post_ncch"
    }

    pub fn append_ncch(session_id: &str) -> String {
        format!("/append_ncch/{}", session_id)
    }

    pub fn ncch_info(ncch_id: &str, info_type: &str) -> String {
        format!("/ncch/{}/{}", ncch_id, info_type)
    }

    pub fn submit_ncch() -> &'static str {
        "/submit_ncch"
    }

    pub fn ncch() -> &'static str {
        "/ncch"
    }

    pub fn ncch_list() -> &'static str {
        "/"
    }

    pub fn query_ncch() -> &'static str {
        "/query_ncch"
    }

    pub fn query_ncch_count() -> &'static str {
        "/query_ncch_count"
    }

    pub fn not_found_small() -> &'static str {
        "/notfound24.png"
    }

    pub fn not_found_large() -> &'static str {
        "/notfound48.png"
    }

}
