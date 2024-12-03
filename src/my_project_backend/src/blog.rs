use candid::CandidType;
use ic_cdk::api::time;

#[derive(Clone, CandidType)]
pub struct Blog{
    title: String,
    date: u64, //unsigned, nieujemny zaczyna sie od u8
    //u8 = 0-254 (2^8), u32 = (2^32)
    content: String,
    tags: Vec<String>
}

impl Blog{
    pub fn new(title: String, content: String, tags: Vec<String>) -> Self {
        Self{
            title,
            date: time(),
            content,
            tags
        }
    }
}