pub struct SDNRecord {
    /// unique record identifier / unique listing identifier
    pub ent_num: u32,
    /// name of SDN
    pub sdn_name: String,
    /// type of SDN
    pub sdn_type: String,
    /// sanctions program name
    pub program: String,
    /// title of an individual
    pub title: String,
    /// vessel call sign
    pub call_sign: String,
    /// vessel type
    pub vess_type: String,
    /// vessel tonnage
    pub tonnage: String,
    /// gross registered tonnage
    pub grt: String,
    /// vessel flag
    pub vess_flag: String,
    /// vessel owner
    pub vess_owner: String,
    /// remarks on SDN*
    pub remarks: String,
}
