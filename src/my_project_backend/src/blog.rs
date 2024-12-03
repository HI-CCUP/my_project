use candid::CandidType;
#[derive(Clone, CandidType)]

pub struct Blog{
    title: String,
    date: u32, //unsigned, nieujemny zaczyna sie od u8
    //u8 = 0-254 (2^8), u32 = (2^32)
    content: String,
    tags: Vec<String>
}

impl Blog{
    pub fn new(title: String, date: u32, content: String, tags: Vec<String>) -> Self {
        Self{
            title,
            date,
            content,
            tags
        }
    }
}