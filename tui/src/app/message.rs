use derive_new::new;

#[derive(new)]
pub struct Message {
    pub text: String,
    pub from: MsgFrom,
}

pub enum MsgFrom {
    User,
    Ai,
}
